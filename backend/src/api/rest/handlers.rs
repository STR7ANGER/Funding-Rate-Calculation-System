use axum::{extract::Path, Json};
use crate::services::{historical_manager::HistoricalManager, oracle_manager::OracleManager};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

#[derive(Serialize)]
pub struct FundingRateResponse {
    pub symbol: String,
    pub funding_rate: f64,
    pub premium_index: f64,
    pub mark_price: f64,
    pub index_price: f64,
    pub timestamp: i64,
}

#[derive(Serialize)]
pub struct FundingHistoryResponse {
    pub symbol: String,
    pub rates: Vec<FundingRateResponse>,
}

#[derive(Serialize)]
pub struct PaymentHistoryResponse {
    pub position_pubkey: String,
    pub payments: Vec<FundingPaymentResponse>,
}

#[derive(Serialize)]
pub struct FundingPaymentResponse {
    pub symbol: String,
    pub payment_amount: f64,
    pub payment_type: String,
    pub timestamp: i64,
}

#[derive(Serialize)]
pub struct FundingStatsResponse {
    pub symbol: String,
    pub avg_rate: f64,
    pub min_rate: f64,
    pub max_rate: f64,
    pub current_rate: f64,
}

#[derive(Serialize)]
pub struct NextPaymentResponse {
    pub next_payment_time: i64,
    pub seconds_remaining: i64,
}

pub async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

pub async fn get_current_funding(Path(symbol): Path<String>) -> Json<FundingRateResponse> {
    let now = chrono::Utc::now().timestamp();
    let oracle = OracleManager::new();

    // Try live prices; on error, fall back to zeros
    let (mark_price, index_price) = match (
        oracle.get_mark_price(&symbol).await,
        oracle.get_index_price(&symbol).await,
    ) {
        (Ok(m), Ok(i)) if i > 0.0 => (m, i),
        _ => (0.0, 0.0),
    };

    let premium_index = if index_price > 0.0 {
        (mark_price - index_price) / index_price
    } else {
        0.0
    };

    // Interest rate: 0.01% daily / 86400 seconds
    let interest_rate = 0.0001 / 86400.0;
    let funding_rate = (premium_index + interest_rate).max(-0.0005).min(0.0005);

    Json(FundingRateResponse {
        symbol,
        funding_rate,
        premium_index,
        mark_price,
        index_price,
        timestamp: now,
    })
}

pub async fn get_funding_history(Path(symbol): Path<String>) -> Json<FundingHistoryResponse> {
    let mut rates = Vec::new();
    let oracle = OracleManager::new();
    let hist = HistoricalManager::new();
    let now_index = oracle.get_index_price(&symbol).await.unwrap_or(0.0);

    let interest_rate = 0.0001 / 86400.0;

    match hist.get_recent_funding_rates(&symbol, 24).await {
        Ok(items) => {
            for (funding_rate_raw, ts) in items {
                // funding_rate from Binance is 8-hourly; we still display as-is
                // Approximate premium index as funding_rate - interest_rate
                let premium_index = (funding_rate_raw - interest_rate)
                    .max(-0.001)
                    .min(0.001);

                let index_price = now_index; // best-effort reference
                let mark_price = if index_price > 0.0 {
                    index_price * (1.0 + premium_index)
                } else {
                    0.0
                };

                rates.push(FundingRateResponse {
                    symbol: symbol.clone(),
                    funding_rate: funding_rate_raw,
                    premium_index,
                    mark_price,
                    index_price,
                    timestamp: ts,
                });
            }
        }
        Err(_) => {
            // Fallback: empty
        }
    }

    Json(FundingHistoryResponse { symbol, rates })
}

pub async fn get_payment_history(Path(position): Path<String>) -> Json<PaymentHistoryResponse> {
    // TODO: Fetch from database - For now generating mock payment history
    let mut payments = Vec::new();
    let now = chrono::Utc::now().timestamp();
    
    // Generate last 7 days of hourly payments (168 samples)
    for i in (0..168).rev() {
        let payment_timestamp = now - (i * 3600);
        
        // Mock payment amount (positive = received, negative = paid)
        let base_rate = 0.0001;
        let variation = ((payment_timestamp % 1000) as f64 / 10000.0) - 0.05;
        let funding_rate = base_rate + variation;
        let payment_amount = 1.0 * funding_rate; // Position size of 1
        
        payments.push(FundingPaymentResponse {
            symbol: "BTC-PERP".to_string(), // Mock symbol
            payment_amount: if funding_rate > 0.0 { -payment_amount } else { payment_amount.abs() },
            payment_type: if funding_rate > 0.0 { "PAID".to_string() } else { "RECEIVED".to_string() },
            timestamp: payment_timestamp,
        });
    }
    
    Json(PaymentHistoryResponse {
        position_pubkey: position,
        payments,
    })
}

pub async fn get_funding_stats(Path(symbol): Path<String>) -> Json<FundingStatsResponse> {
    let oracle = OracleManager::new();
    let hist = HistoricalManager::new();

    let now_index = oracle.get_index_price(&symbol).await.unwrap_or(0.0);
    let now_mark = oracle.get_mark_price(&symbol).await.unwrap_or(0.0);
    let premium_index = if now_index > 0.0 {
        (now_mark - now_index) / now_index
    } else {
        0.0
    };
    let interest_rate = 0.0001 / 86400.0;
    let current_rate = (premium_index + interest_rate).max(-0.0005).min(0.0005);

    let rates: Vec<f64> = hist
        .get_recent_funding_rates(&symbol, 24)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|(r, _)| r)
        .collect();

    let (avg_rate, min_rate, max_rate) = if rates.is_empty() {
        (0.0, 0.0, 0.0)
    } else {
        let sum = rates.iter().sum::<f64>();
        let avg = sum / rates.len() as f64;
        let min = rates
            .iter()
            .fold(f64::INFINITY, |a, &b| if b < a { b } else { a });
        let max = rates
            .iter()
            .fold(f64::NEG_INFINITY, |a, &b| if b > a { b } else { a });
        (avg, min, max)
    };

    Json(FundingStatsResponse {
        symbol,
        avg_rate,
        min_rate,
        max_rate,
        current_rate,
    })
}

pub async fn get_next_payment() -> Json<NextPaymentResponse> {
    // Calculate next hour boundary
    let now = chrono::Utc::now();
    let next_hour = (now.timestamp() / 3600 + 1) * 3600;
    let seconds_remaining = next_hour - now.timestamp();

    Json(NextPaymentResponse {
        next_payment_time: next_hour,
        seconds_remaining,
    })
}

