use anyhow::Result;
use super::models::FundingRateRow;

pub struct Repository;

impl Repository {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn insert_funding_rate(&self, _row: &FundingRateRow) -> Result<()> {
        Ok(())
    }
}

