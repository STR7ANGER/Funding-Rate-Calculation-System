# Calculation Pipeline

## 1-Second Calculation Loop

Every second, the system:

1. Fetches mark price from internal oracle
2. Fetches index price from Pyth Network (with Switchboard fallback)
3. Calculates premium index: `(Mark Price - Index Price) / Index Price`
4. Adds interest rate component (0.01% daily / 86400 seconds)
5. Clamps rate to ±0.05%
6. Stores in Redis cache
7. Stores in database
8. Updates on-chain state

## Hourly Payment Distribution

Every hour:

1. Aggregates 3600 samples (1 per second)
2. Calculates average funding rate
3. Queries all open positions
4. Calculates funding payment: `Position Size × Avg Rate × 1 hour`
5. Updates position margin
6. Records payment in database
7. Emits payment event

## Performance Targets

- Calculation latency: < 100ms for 50 symbols
- Oracle fetch: < 100ms
- Payment processing: < 1 minute for 10,000 positions

