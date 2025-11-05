use anyhow::{anyhow, Result};
use serde::Deserialize;

pub struct HistoricalManager;

#[derive(Deserialize)]
struct BinanceFundingRateItem {
    #[serde(rename = "fundingRate")]
    funding_rate: String,
    #[serde(rename = "fundingTime")]
    funding_time: i64,
}

impl HistoricalManager {
    pub fn new() -> Self {
        Self {}
    }

    fn to_binance_symbol(symbol: &str) -> String {
        let base = symbol.replace("-PERP", "");
        if base.ends_with("USDT") {
            base
        } else {
            format!("{}USDT", base)
        }
    }

    pub async fn get_recent_funding_rates(&self, symbol: &str, limit: usize) -> Result<Vec<(f64, i64)>> {
        let binance_symbol = Self::to_binance_symbol(symbol);
        let url = format!(
            "https://fapi.binance.com/fapi/v1/fundingRate?symbol={}&limit={}",
            binance_symbol,
            limit
        );
        let resp = reqwest::Client::new()
            .get(url)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await?;
        if !resp.status().is_success() {
            return Err(anyhow!("binance fundingRate http status {}", resp.status()));
        }
        let items: Vec<BinanceFundingRateItem> = resp.json().await?;
        let mut out = Vec::with_capacity(items.len());
        for it in items {
            if let Ok(rate) = it.funding_rate.parse::<f64>() {
                // Binance returns ms
                out.push((rate, it.funding_time / 1000));
            }
        }
        Ok(out)
    }
}

