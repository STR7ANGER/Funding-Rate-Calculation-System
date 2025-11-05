-- Aggregated hourly funding history
CREATE TABLE funding_history (
    id BIGSERIAL PRIMARY KEY,
    symbol VARCHAR(20) NOT NULL,
    hour_start TIMESTAMPTZ NOT NULL,
    hour_end TIMESTAMPTZ NOT NULL,
    avg_funding_rate NUMERIC(20, 10) NOT NULL,
    min_funding_rate NUMERIC(20, 10) NOT NULL,
    max_funding_rate NUMERIC(20, 10) NOT NULL,
    avg_mark_price NUMERIC(30, 8) NOT NULL,
    avg_index_price NUMERIC(30, 8) NOT NULL,
    total_long_oi NUMERIC(30, 8),
    total_short_oi NUMERIC(30, 8),
    sample_count INT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_funding_history_symbol_hour ON funding_history(symbol, hour_start);
CREATE INDEX idx_funding_history_timestamp ON funding_history(hour_start DESC);

