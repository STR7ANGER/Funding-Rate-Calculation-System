use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FundingRateRow {
    pub symbol: String,
    pub funding_rate: f64,
    pub premium_index: f64,
    pub mark_price: f64,
    pub index_price: f64,
    pub timestamp: i64,
}

