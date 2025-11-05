-- System performance metrics
CREATE TABLE calculation_metrics (
    id BIGSERIAL PRIMARY KEY,
    metric_type VARCHAR(50) NOT NULL,
    symbol VARCHAR(20),
    duration_ms NUMERIC(10, 3),
    success BOOLEAN NOT NULL,
    error_message TEXT,
    metadata JSONB,
    timestamp TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_metrics_type_timestamp ON calculation_metrics(metric_type, timestamp DESC);
CREATE INDEX idx_metrics_success ON calculation_metrics(success, timestamp DESC) WHERE success = false;

