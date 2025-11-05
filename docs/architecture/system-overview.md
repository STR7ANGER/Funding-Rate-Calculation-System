# System Overview

## Architecture

The Funding Rate Calculation System is composed of:

1. **Solana Smart Contract (Anchor Program)**
   - Stores funding rate state on-chain
   - Processes funding payments
   - Maintains hourly samples

2. **Rust Backend Service**
   - High-frequency funding rate calculations
   - Oracle price aggregation
   - Payment distribution
   - REST API and WebSocket streams

3. **Database (PostgreSQL)**
   - Historical funding rate data
   - Payment records
   - Oracle price history
   - Performance metrics

4. **Cache (Redis)**
   - Fast access to current funding rates
   - Reduces database load

## Data Flow

```
Oracle Prices → Calculator → Redis Cache → Database
                              ↓
                         REST API / WebSocket
                              ↓
                         On-chain Updates
```

## Key Components

- **Funding Calculator**: Calculates rates every 1 second
- **Rate Aggregator**: Aggregates 3600 samples for hourly payments
- **Payment Distributor**: Processes hourly payments
- **Oracle Manager**: Fetches and validates prices from Pyth/Switchboard

