# Funding Rate Calculations

## Premium Index

```
Premium Index = (Mark Price - Index Price) / Index Price
```

Example:
- Mark Price: $50,000
- Index Price: $49,975
- Premium Index: (50000 - 49975) / 49975 = 0.0005 (0.05%)

## Interest Rate Component

```
Interest Rate = 0.01% / 86400 seconds
             = 0.0001 / 86400
             ≈ 1.157e-9 per second
```

## Funding Rate

```
Funding Rate = Premium Index + Interest Rate
```

## Clamped Rate

```
Clamped Rate = max(-0.05%, min(0.05%, Funding Rate))
```

## Funding Payment

```
Payment = Position Size × Average Funding Rate × Hours
```

Example:
- Position Size: 1 BTC
- Average Rate: 0.0001 (0.01%)
- Hours: 1
- Payment: 1 × 0.0001 × 1 = 0.0001 BTC

If rate is positive, longs pay shorts.
If rate is negative, shorts pay longs.

