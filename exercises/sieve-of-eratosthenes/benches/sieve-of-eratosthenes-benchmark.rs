use criterion::{Criterion, criterion_group, criterion_main};
use sieve_of_eratosthenes::SieveOfEratosthenes;
use std::hint::black_box;

fn benchmark_sieve_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sieve Creation");

    // Benchmark finding primes up to 1,000
    group.bench_function("limit_1000", |b| {
        b.iter(|| SieveOfEratosthenes::new(black_box(1_000)))
    });

    // Benchmark finding primes up to 10,000
    group.bench_function("limit_10000", |b| {
        b.iter(|| SieveOfEratosthenes::new(black_box(10_000)))
    });

    // Benchmark finding primes up to 1,000,000
    group.bench_function("limit_1m", |b| {
        b.iter(|| SieveOfEratosthenes::new(black_box(1_000_000)))
    });

    group.finish();
}

criterion_group!(benches, benchmark_sieve_creation);
criterion_main!(benches);
