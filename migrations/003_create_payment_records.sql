-- Individual funding payment records
CREATE TABLE funding_payments (
    id BIGSERIAL PRIMARY KEY,
    position_pubkey VARCHAR(44) NOT NULL,
    symbol VARCHAR(20) NOT NULL,
    position_size NUMERIC(30, 8) NOT NULL,
    funding_rate NUMERIC(20, 10) NOT NULL,
    payment_amount NUMERIC(30, 8) NOT NULL,
    payment_type VARCHAR(10) NOT NULL CHECK (payment_type IN ('PAID', 'RECEIVED')),
    hour_start TIMESTAMPTZ NOT NULL,
    hour_end TIMESTAMPTZ NOT NULL,
    transaction_signature VARCHAR(88),
    status VARCHAR(20) NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_funding_payments_position ON funding_payments(position_pubkey, created_at DESC);
CREATE INDEX idx_funding_payments_symbol ON funding_payments(symbol, created_at DESC);
CREATE INDEX idx_funding_payments_status ON funding_payments(status) WHERE status = 'PENDING';

