use anyhow::{anyhow, Result};
use serde::Deserialize;

#[derive(Clone)]
pub struct OracleManager;

#[derive(Deserialize)]
struct BinancePremiumIndex {
    #[serde(rename = "markPrice")]
    mark_price: String,
    #[serde(rename = "indexPrice")]
    index_price: String,
    #[serde(rename = "time")]
    _time: i64,
}

impl OracleManager {
    pub fn new() -> Self {
        Self {}
    }

    fn to_binance_symbol(&self, symbol: &str) -> String {
        // Accepts variants like "BTC", "BTC-PERP", "BTCUSDT" and normalizes to Binance futures symbol
        let base = symbol.replace("-PERP", "");
        if base.ends_with("USDT") {
            base
        } else {
            format!("{}USDT", base)
        }
    }

    async fn fetch_binance_premium_index(&self, symbol: &str) -> Result<BinancePremiumIndex> {
        let binance_symbol = self.to_binance_symbol(symbol);
        let url = format!(
            "https://fapi.binance.com/fapi/v1/premiumIndex?symbol={}",
            binance_symbol
        );
        let resp = reqwest::Client::new()
            .get(url)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await?;
        if !resp.status().is_success() {
            return Err(anyhow!("binance premiumIndex http status {}", resp.status()));
        }
        let data: BinancePremiumIndex = resp.json().await?;
        Ok(data)
    }

    pub async fn get_mark_price(&self, symbol: &str) -> Result<f64> {
        let data = self.fetch_binance_premium_index(symbol).await?;
        let price: f64 = data
            .mark_price
            .parse()
            .map_err(|e| anyhow!("failed to parse mark_price: {}", e))?;
        Ok(price)
    }

    pub async fn get_index_price(&self, symbol: &str) -> Result<f64> {
        let data = self.fetch_binance_premium_index(symbol).await?;
        let price: f64 = data
            .index_price
            .parse()
            .map_err(|e| anyhow!("failed to parse index_price: {}", e))?;
        Ok(price)
    }
}

