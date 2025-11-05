#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use funding_rate_backend::services::{
        funding_calculator::{FundingRatePerformance, FundingCalculatorConfig},
        oracle_manager::OracleManager,
    };
    use funding_rate_backend::cache::redis_client::RedisClient;
    use funding_rate_backend::monitoring::alerts::AlertService;

    #[tokio::test]
    #[ignore]
    async fn test_calculate_all_funding_rates() {
        let config = FundingCalculatorConfig {
            interest_rate_daily: 0.0001,
            rate_min: -0.0005,
            rate_max: 0.0005,
            target_calculation_time_ms: 1000, // High threshold for test
        };

        let oracle = Arc::new(OracleManager::new());
        let redis = Arc::new(RedisClient::new());
        let alert_service = Arc::new(AlertService::new());

        let calculator = FundingRatePerformance::new(config, oracle, redis, alert_service);

        let symbols = vec!["BTC-PERP".to_string(), "ETH-PERP".to_string()];
        let result = calculator.calculate_all_funding_rates(&symbols).await;

        assert!(result.is_ok());
        let metrics = result.unwrap();
        assert_eq!(metrics.symbol_count, 2);
    }
}


