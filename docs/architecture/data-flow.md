# Data Flow

## Real-time Flow

```
[1-second Timer]
    ↓
[Oracle Manager] → Mark Price (Internal)
    ↓
[Oracle Manager] → Index Price (Pyth/Switchboard)
    ↓
[Funding Calculator] → Premium Index + Interest Rate
    ↓
[Rate Clamping] → ±0.05%
    ↓
[Redis Cache] ← Store current rate
    ↓
[PostgreSQL] ← Store historical rate
    ↓
[WebSocket] → Broadcast to clients
```

## Hourly Flow

```
[Hourly Timer]
    ↓
[Rate Aggregator] → Aggregate 3600 samples
    ↓
[Payment Calculator] → Calculate per-position payments
    ↓
[Solana Program] → Update on-chain positions
    ↓
[PostgreSQL] ← Store payment records
    ↓
[WebSocket] → Notify clients
```

