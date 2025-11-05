# REST API Endpoints

## Health Check

```
GET /health
```

Response:
```json
{
  "status": "ok"
}
```

## Get Current Funding Rate

```
GET /funding/current/:symbol
```

Response:
```json
{
  "symbol": "BTC-PERP",
  "funding_rate": 0.0001,
  "premium_index": 0.00005,
  "mark_price": 50000.0,
  "index_price": 49975.0,
  "timestamp": 1234567890
}
```

## Get Funding History

```
GET /funding/history/:symbol
```

## Get Payment History

```
GET /funding/payments/:position
```

## Get Funding Statistics

```
GET /funding/stats/:symbol
```

## Get Next Payment Time

```
GET /funding/next-payment
```

Response:
```json
{
  "next_payment_time": 1234567890,
  "seconds_remaining": 1800
}
```

