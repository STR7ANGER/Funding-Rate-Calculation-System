-- Funding rates table (1-second granularity)
CREATE TABLE funding_rates (
    id BIGSERIAL PRIMARY KEY,
    symbol VARCHAR(20) NOT NULL,
    funding_rate NUMERIC(20, 10) NOT NULL,
    premium_index NUMERIC(20, 10) NOT NULL,
    interest_rate NUMERIC(20, 10) NOT NULL,
    mark_price NUMERIC(30, 8) NOT NULL,
    index_price NUMERIC(30, 8) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for fast queries
CREATE INDEX idx_funding_rates_symbol_timestamp ON funding_rates(symbol, timestamp DESC);
CREATE INDEX idx_funding_rates_timestamp ON funding_rates(timestamp DESC);

-- Partitioning by time (optional, for high volume)
-- ALTER TABLE funding_rates PARTITION BY RANGE (timestamp);

