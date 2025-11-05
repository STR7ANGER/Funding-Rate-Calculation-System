#[cfg(test)]
mod tests {
    use funding_rate_backend::services::rate_aggregator::RateAggregator;
    use funding_rate_backend::types::funding_rate::FundingRate;

    #[test]
    fn test_rate_aggregation() {
        let aggregator = RateAggregator::new();

        let samples = vec![
            FundingRate {
                symbol: "BTC-PERP".to_string(),
                funding_rate: 0.0001,
                premium_index: 0.00005,
                mark_price: 50000.0,
                index_price: 49975.0,
                timestamp: 1000,
            },
            FundingRate {
                symbol: "BTC-PERP".to_string(),
                funding_rate: 0.0002,
                premium_index: 0.00015,
                mark_price: 50010.0,
                index_price: 49980.0,
                timestamp: 1001,
            },
        ];

        let avg = aggregator.aggregate_hourly(&samples).unwrap();
        assert!((avg - 0.00015).abs() < 1e-12);

        let (avg_stat, min_stat, max_stat) = aggregator.get_statistics(&samples);
        assert!((avg_stat - 0.00015).abs() < 1e-12);
        assert!((min_stat - 0.0001).abs() < 1e-12);
        assert!((max_stat - 0.0002).abs() < 1e-12);
    }
}


