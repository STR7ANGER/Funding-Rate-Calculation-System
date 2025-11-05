use std::sync::Arc;
use std::time::Instant;
use tokio::task::JoinSet;
use anyhow::Result;
use serde::Serialize;

use crate::types::funding_rate::FundingRate;
use crate::services::oracle_manager::OracleManager;
use crate::cache::redis_client::RedisClient;
use crate::monitoring::alerts::AlertService;

#[derive(Debug, Serialize)]
pub struct PerformanceMetrics {
    pub symbol_count: usize,
    pub duration_ms: f64,
    pub average_per_symbol: f64,
    pub updates_per_second: f64,
    pub target_met: bool,
}

#[derive(Clone)]
pub struct FundingCalculatorConfig {
    pub interest_rate_daily: f64,
    pub rate_min: f64,
    pub rate_max: f64,
    pub target_calculation_time_ms: u64,
}

pub struct FundingRatePerformance {
    pub config: FundingCalculatorConfig,
    pub oracle: Arc<OracleManager>,
    pub redis: Arc<RedisClient>,
    pub alert_service: Arc<AlertService>,
}

impl FundingRatePerformance {
    pub fn new(
        config: FundingCalculatorConfig,
        oracle: Arc<OracleManager>,
        redis: Arc<RedisClient>,
        alert_service: Arc<AlertService>,
    ) -> Self {
        Self {
            config,
            oracle,
            redis,
            alert_service,
        }
    }

    pub async fn calculate_all_funding_rates(
        &self,
        symbols: &[String],
    ) -> Result<PerformanceMetrics> {
        let start = Instant::now();
        let mut tasks = JoinSet::new();

        // Parallel calculation for all symbols
        for symbol in symbols {
            let symbol = symbol.clone();
            let oracle = self.oracle.clone();
            let config = self.config.clone();

            tasks.spawn(async move {
                let mark_price = oracle.get_mark_price(&symbol).await?;
                let index_price = oracle.get_index_price(&symbol).await?;

                // Calculate premium index
                let premium_index = if index_price > 0.0 {
                    (mark_price - index_price) / index_price
                } else {
                    0.0
                };

                // Interest rate: 0.01% / 24 hours / 3600 seconds
                let interest_rate = config.interest_rate_daily / 86400.0;

                let funding_rate = premium_index + interest_rate;

                // Clamp to reasonable bounds (-0.05% to +0.05%)
                let clamped_rate = funding_rate.max(config.rate_min).min(config.rate_max);

                Ok::<FundingRate, anyhow::Error>(FundingRate {
                    symbol,
                    funding_rate: clamped_rate,
                    premium_index,
                    mark_price,
                    index_price,
                    timestamp: chrono::Utc::now().timestamp(),
                })
            });
        }

        // Collect results
        let mut funding_rates = Vec::new();
        while let Some(result) = tasks.join_next().await {
            match result {
                Ok(Ok(rate)) => funding_rates.push(rate),
                Ok(Err(e)) => tracing::error!("Funding rate calculation error: {}", e),
                Err(e) => tracing::error!("Task join error: {}", e),
            }
        }

        let duration = start.elapsed();
        let duration_ms = duration.as_secs_f64() * 1000.0;

        // Store in Redis for fast access
        self.cache_funding_rates(&funding_rates).await?;

        // Alert if performance target not met
        if duration_ms > self.config.target_calculation_time_ms as f64 {
            self.alert_service
                .send_alert(&format!(
                    "Funding rate calculation exceeded target: {}ms > {}ms",
                    duration_ms, self.config.target_calculation_time_ms
                ))
                .await?;
        }

        Ok(PerformanceMetrics {
            symbol_count: symbols.len(),
            duration_ms,
            average_per_symbol: duration_ms / symbols.len() as f64,
            updates_per_second: symbols.len() as f64 / duration.as_secs_f64().max(0.001),
            target_met: duration_ms <= self.config.target_calculation_time_ms as f64,
        })
    }

    async fn cache_funding_rates(&self, rates: &[FundingRate]) -> Result<()> {
        for rate in rates {
            let key = format!("funding_rate:{}", rate.symbol);
            let value = serde_json::to_string(rate)?;
            self.redis.set_ex(&key, &value, 60).await?; // 60 second TTL
        }
        Ok(())
    }
}

// Simple calculator for single symbol
pub struct FundingCalculator {
    pub config: FundingCalculatorConfig,
}

impl FundingCalculator {
    pub fn new(config: FundingCalculatorConfig) -> Self {
        Self { config }
    }

    pub fn calculate(&self, symbol: &str, mark_price: f64, index_price: f64, timestamp: i64) -> FundingRate {
        let premium_index = if index_price > 0.0 {
            (mark_price - index_price) / index_price
        } else {
            0.0
        };
        let interest_rate_per_sec = self.config.interest_rate_daily / 86400.0;
        let funding_rate = (premium_index + interest_rate_per_sec)
            .max(self.config.rate_min)
            .min(self.config.rate_max);

        FundingRate {
            symbol: symbol.to_string(),
            funding_rate,
            premium_index,
            mark_price,
            index_price,
            timestamp,
        }
    }
}

