use crate::types::funding_rate::FundingRate;
use anyhow::Result;

pub struct RateAggregator;

impl RateAggregator {
    pub fn new() -> Self {
        Self {}
    }

    /// Aggregate 3600 samples (1 per second for 1 hour) and calculate hourly average
    pub fn aggregate_hourly(&self, samples: &[FundingRate]) -> Result<f64> {
        if samples.is_empty() {
            return Ok(0.0);
        }

        let sum: f64 = samples.iter().map(|r| r.funding_rate).sum();
        Ok(sum / samples.len() as f64)
    }

    pub fn get_statistics(&self, samples: &[FundingRate]) -> (f64, f64, f64) {
        if samples.is_empty() {
            return (0.0, 0.0, 0.0);
        }

        let rates: Vec<f64> = samples.iter().map(|r| r.funding_rate).collect();
        let avg = rates.iter().sum::<f64>() / rates.len() as f64;
        let min = rates.iter().fold(f64::MAX, |a, &b| a.min(b));
        let max = rates.iter().fold(f64::MIN, |a, &b| a.max(b));

        (avg, min, max)
    }
}

