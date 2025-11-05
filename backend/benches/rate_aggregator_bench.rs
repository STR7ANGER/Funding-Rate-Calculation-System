use criterion::{criterion_group, criterion_main, Criterion, black_box};
use funding_rate_backend::services::rate_aggregator::RateAggregator;
use funding_rate_backend::types::funding_rate::FundingRate;

fn bench_rate_aggregation(c: &mut Criterion) {
    let aggregator = RateAggregator::new();

    let mut samples = Vec::with_capacity(10_000);
    for i in 0..10_000u64 {
        samples.push(FundingRate {
            symbol: "BTC-PERP".to_string(),
            funding_rate: 0.0001 + (i as f64 % 10.0) * 1e-6,
            premium_index: 0.00005,
            mark_price: 50_000.0 + (i % 20) as f64,
            index_price: 49_975.0 + (i % 20) as f64,
            timestamp: i as i64,
        });
    }

    c.bench_function("aggregate_hourly_10k", |b| {
        b.iter(|| {
            let avg = aggregator.aggregate_hourly(black_box(&samples)).unwrap();
            black_box(avg)
        })
    });

    c.bench_function("get_statistics_10k", |b| {
        b.iter(|| {
            let stats = aggregator.get_statistics(black_box(&samples));
            black_box(stats)
        })
    });
}

criterion_group!(benches, bench_rate_aggregation);
criterion_main!(benches);


