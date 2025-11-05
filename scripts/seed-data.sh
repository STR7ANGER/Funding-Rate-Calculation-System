#!/bin/bash

set -e

echo "Seeding test data..."

DATABASE_URL=${DATABASE_URL:-postgresql://funding_user:funding_pass@localhost:5432/funding_rates}

# Check if psql is available
if ! command -v psql &> /dev/null; then
    echo "psql not found. Please install PostgreSQL client tools."
    exit 1
fi

# Seed some test symbols and funding rates
psql "$DATABASE_URL" << EOF
-- Insert test symbols if not exists
INSERT INTO funding_rates (symbol, funding_rate, premium_index, interest_rate, mark_price, index_price, timestamp)
VALUES 
    ('BTC-PERP', 0.0001, 0.00005, 0.0001, 50000.0, 49975.0, NOW()),
    ('ETH-PERP', 0.0002, 0.00015, 0.0001, 3000.0, 2995.0, NOW()),
    ('SOL-PERP', -0.0001, -0.00015, 0.0001, 100.0, 100.15, NOW())
ON CONFLICT DO NOTHING;

SELECT 'Test data seeded successfully!' as status;
EOF

echo "Seed data script complete!"

