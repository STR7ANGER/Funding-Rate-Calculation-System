-- Oracle price history
CREATE TABLE oracle_prices (
    id BIGSERIAL PRIMARY KEY,
    symbol VARCHAR(20) NOT NULL,
    source VARCHAR(20) NOT NULL,
    price NUMERIC(30, 8) NOT NULL,
    confidence NUMERIC(30, 8),
    slot BIGINT,
    timestamp TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_oracle_prices_symbol_timestamp ON oracle_prices(symbol, timestamp DESC);
CREATE INDEX idx_oracle_prices_source ON oracle_prices(source, timestamp DESC);

