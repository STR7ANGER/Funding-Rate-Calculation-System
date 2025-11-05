# WebSocket Streams

## Connection

Connect to: `ws://localhost:8080/ws`

## Real-time Funding Rate Updates

Subscribe to funding rate updates for a symbol:

```json
{
  "type": "subscribe",
  "symbol": "BTC-PERP"
}
```

Receive updates every second:

```json
{
  "type": "funding_rate",
  "symbol": "BTC-PERP",
  "funding_rate": 0.0001,
  "timestamp": 1234567890
}
```

## Hourly Payment Notifications

```json
{
  "type": "payment",
  "symbol": "BTC-PERP",
  "avg_rate": 0.0001,
  "timestamp": 1234567890
}
```

## Premium/Discount Alerts

```json
{
  "type": "alert",
  "symbol": "BTC-PERP",
  "message": "High premium detected",
  "premium_index": 0.001
}
```

