#[cfg(test)]
mod tests {
    use funding_rate_backend::services::funding_calculator::{FundingCalculator, FundingCalculatorConfig};

    #[test]
    fn test_funding_rate_calculation() {
        let config = FundingCalculatorConfig {
            interest_rate_daily: 0.0001,
            rate_min: -0.0005,
            rate_max: 0.0005,
            target_calculation_time_ms: 100,
        };
        let calculator = FundingCalculator::new(config);

        let mark_price = 50000.0;
        let index_price = 49975.0;
        let timestamp = chrono::Utc::now().timestamp();

        let result = calculator.calculate("BTC-PERP", mark_price, index_price, timestamp);

        // Premium index should be positive (mark > index)
        assert!(result.premium_index > 0.0);
        assert!(result.funding_rate >= -0.0005);
        assert!(result.funding_rate <= 0.0005);
        assert_eq!(result.symbol, "BTC-PERP");
    }

    #[test]
    fn test_funding_rate_clamping() {
        let config = FundingCalculatorConfig {
            interest_rate_daily: 0.0001,
            rate_min: -0.0005,
            rate_max: 0.0005,
            target_calculation_time_ms: 100,
        };
        let calculator = FundingCalculator::new(config);

        // Extreme case: mark price much higher than index
        let result = calculator.calculate("TEST", 60000.0, 50000.0, chrono::Utc::now().timestamp());
        assert!(result.funding_rate <= 0.0005);
    }
}

