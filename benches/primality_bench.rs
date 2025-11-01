use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use erato::PrimalityRegistry;

/// Generate test numbers for small range
fn generate_small_primes() -> Vec<u64> {
    vec![
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139,
        149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    ]
}

/// Generate test numbers for medium range
fn generate_medium_primes() -> Vec<u64> {
    vec![
        10007, 10009, 10037, 10039, 10061, 10067, 10069, 10079, 10091, 10093,
        100003, 100019, 100043, 100049, 100057, 100069, 100103, 100109, 100129, 100151,
    ]
}

/// Generate test numbers for large range
fn generate_large_primes() -> Vec<u64> {
    vec![
        1_000_003, 1_000_033, 1_000_037, 1_000_039, 1_000_081,
        10_000_019, 10_000_079, 10_000_139, 10_000_169, 10_000_189,
        100_000_007, 100_000_037, 100_000_039, 100_000_049, 100_000_061,
        1_000_000_007, 1_000_000_009, 1_000_000_021, 1_000_000_033, 1_000_000_087,
    ]
}

/// Generate test numbers for very large range
fn generate_very_large_primes() -> Vec<u64> {
    vec![
        10_000_000_019, 10_000_000_033, 10_000_000_061, 10_000_000_069, 10_000_000_097,
        100_000_000_003, 100_000_000_019, 100_000_000_037, 100_000_000_039, 100_000_000_049,
    ]
}

/// Generate challenging composites (products of large primes)
fn generate_very_large_composites() -> Vec<u64> {
    vec![
        // Semiprimes (products of 2 large primes) - hardest case for trial division
        9_999_997_000_029_991,  // 9999991 * 1000003
        10_000_056_000_703,     // 10000019 * 1000037
        10_000_001_400_000_063, // 100000007 * 100000009
    ]
}

/// Generate composite numbers for correctness testing
fn generate_small_composites() -> Vec<u64> {
    vec![
        4, 6, 8, 9, 10, 12, 14, 15, 16, 18,
        20, 21, 22, 24, 25, 26, 27, 28, 30, 32,
    ]
}

/// Generate larger composite numbers
fn generate_medium_composites() -> Vec<u64> {
    vec![
        1000, 1001, 1002, 10000, 10001, 10002, 100000, 100001, 100002,
        1000000, 1000001, 1000002, 10000000, 10000001, 10000002,
    ]
}

/// Benchmark small primes (< 200)
fn bench_small_primes(c: &mut Criterion) {
    let registry = PrimalityRegistry::<u64>::with_all_algorithms();
    let test_numbers = generate_small_primes();

    let mut group = c.benchmark_group("small_primes");
    group.sample_size(1000);
    group.measurement_time(std::time::Duration::from_secs(30));

    for algo in registry.algorithms() {
        group.bench_with_input(
            BenchmarkId::from_parameter(algo.name()),
            &test_numbers,
            |b, numbers| {
                b.iter(|| {
                    for &n in numbers {
                        black_box(algo.is_prime(black_box(n)));
                    }
                });
            },
        );
    }
    group.finish();
}

/// Benchmark small composites (< 50)
fn bench_small_composites(c: &mut Criterion) {
    let registry = PrimalityRegistry::<u64>::with_all_algorithms();
    let test_numbers = generate_small_composites();

    let mut group = c.benchmark_group("small_composites");
    group.sample_size(1000);
    group.measurement_time(std::time::Duration::from_secs(30));

    for algo in registry.algorithms() {
        group.bench_with_input(
            BenchmarkId::from_parameter(algo.name()),
            &test_numbers,
            |b, numbers| {
                b.iter(|| {
                    for &n in numbers {
                        black_box(algo.is_prime(black_box(n)));
                    }
                });
            },
        );
    }
    group.finish();
}

/// Benchmark medium primes (10^3 to 10^5)
fn bench_medium_primes(c: &mut Criterion) {
    let registry = PrimalityRegistry::<u64>::with_all_algorithms();
    let test_numbers = generate_medium_primes();

    let mut group = c.benchmark_group("medium_primes");
    group.sample_size(500);
    group.measurement_time(std::time::Duration::from_secs(40));

    for algo in registry.algorithms() {
        group.bench_with_input(
            BenchmarkId::from_parameter(algo.name()),
            &test_numbers,
            |b, numbers| {
                b.iter(|| {
                    for &n in numbers {
                        black_box(algo.is_prime(black_box(n)));
                    }
                });
            },
        );
    }
    group.finish();
}

/// Benchmark medium composites (10^3 to 10^7)
fn bench_medium_composites(c: &mut Criterion) {
    let registry = PrimalityRegistry::<u64>::with_all_algorithms();
    let test_numbers = generate_medium_composites();

    let mut group = c.benchmark_group("medium_composites");
    group.sample_size(200);
    group.measurement_time(std::time::Duration::from_secs(40));

    for algo in registry.algorithms() {
        group.bench_with_input(
            BenchmarkId::from_parameter(algo.name()),
            &test_numbers,
            |b, numbers| {
                b.iter(|| {
                    for &n in numbers {
                        black_box(algo.is_prime(black_box(n)));
                    }
                });
            },
        );
    }
    group.finish();
}

/// Benchmark large primes (10^6 to 10^9)
fn bench_large_primes(c: &mut Criterion) {
    let registry = PrimalityRegistry::<u64>::with_all_algorithms();
    let test_numbers = generate_large_primes();

    let mut group = c.benchmark_group("large_primes");
    group.sample_size(100);
    group.measurement_time(std::time::Duration::from_secs(45));

    for algo in registry.algorithms() {
        group.bench_with_input(
            BenchmarkId::from_parameter(algo.name()),
            &test_numbers,
            |b, numbers| {
                b.iter(|| {
                    for &n in numbers {
                        black_box(algo.is_prime(black_box(n)));
                    }
                });
            },
        );
    }
    group.finish();
}

/// Benchmark very large primes (10^9 to 10^11)
fn bench_very_large_primes(c: &mut Criterion) {
    let registry = PrimalityRegistry::<u64>::with_all_algorithms();
    let test_numbers = generate_very_large_primes();

    let mut group = c.benchmark_group("very_large_primes");
    group.sample_size(50);
    group.measurement_time(std::time::Duration::from_secs(50));

    for algo in registry.algorithms() {
        group.bench_with_input(
            BenchmarkId::from_parameter(algo.name()),
            &test_numbers,
            |b, numbers| {
                b.iter(|| {
                    for &n in numbers {
                        black_box(algo.is_prime(black_box(n)));
                    }
                });
            },
        );
    }
    group.finish();
}

/// Benchmark very large composites (semiprimes - hardest case for trial division)
fn bench_very_large_composites(c: &mut Criterion) {
    let registry = PrimalityRegistry::<u64>::with_all_algorithms();
    let test_numbers = generate_very_large_composites();

    let mut group = c.benchmark_group("very_large_composites");
    group.sample_size(30);
    group.measurement_time(std::time::Duration::from_secs(60));

    for algo in registry.algorithms() {
        group.bench_with_input(
            BenchmarkId::from_parameter(algo.name()),
            &test_numbers,
            |b, numbers| {
                b.iter(|| {
                    for &n in numbers {
                        black_box(algo.is_prime(black_box(n)));
                    }
                });
            },
        );
    }
    group.finish();
}

/// Benchmark single prime number for each algorithm
fn bench_single_prime(c: &mut Criterion) {
    let registry = PrimalityRegistry::<u64>::with_all_algorithms();
    let prime = 1_000_000_007u64;

    let mut group = c.benchmark_group("single_large_prime");
    group.sample_size(500);

    for algo in registry.algorithms() {
        group.bench_function(algo.name(), |b| {
            b.iter(|| {
                black_box(algo.is_prime(black_box(prime)));
            });
        });
    }
    group.finish();
}

/// Benchmark single composite number for each algorithm
fn bench_single_composite(c: &mut Criterion) {
    let registry = PrimalityRegistry::<u64>::with_all_algorithms();
    let composite = 1_000_000_000u64;

    let mut group = c.benchmark_group("single_large_composite");
    group.sample_size(500);

    for algo in registry.algorithms() {
        group.bench_function(algo.name(), |b| {
            b.iter(|| {
                black_box(algo.is_prime(black_box(composite)));
            });
        });
    }
    group.finish();
}

/// Benchmark edge cases
fn bench_edge_cases(c: &mut Criterion) {
    let registry = PrimalityRegistry::<u64>::with_all_algorithms();
    let edge_cases = vec![0, 1, 2, 3, 4, u64::MAX];

    let mut group = c.benchmark_group("edge_cases");
    group.sample_size(1000);

    for algo in registry.algorithms() {
        group.bench_with_input(
            BenchmarkId::from_parameter(algo.name()),
            &edge_cases,
            |b, numbers| {
                b.iter(|| {
                    for &n in numbers {
                        black_box(algo.is_prime(black_box(n)));
                    }
                });
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_small_primes,
    bench_small_composites,
    bench_medium_primes,
    bench_medium_composites,
    bench_large_primes,
    bench_very_large_primes,
    bench_very_large_composites,
    bench_single_prime,
    bench_single_composite,
    bench_edge_cases,
);

criterion_main!(benches);
