#[cfg(test)]
mod tests {
    use funding_rate_backend::services::oracle_manager::OracleManager;

    #[tokio::test]
    async fn test_oracle_manager() {
        let oracle = OracleManager::new();

        // Test mark price fetch
        let mark_price = oracle.get_mark_price("BTC-PERP").await;
        assert!(mark_price.is_ok());

        // Test index price fetch
        let index_price = oracle.get_index_price("BTC-PERP").await;
        assert!(index_price.is_ok());
    }
}

