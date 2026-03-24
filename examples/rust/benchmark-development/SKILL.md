---
name: benchmark-development
description: >
  Patterns for developing and running Criterion.rs benchmarks for performance
  validation. Use when creating benchmarks, profiling, detecting regressions,
  or tuning performance parameters.
allowed-tools: Read, Glob, Grep, Bash
user-invocable: true
---

# Benchmark Development

## Criterion.rs Setup

### Cargo.toml

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "benchmarks"
harness = false
```

### Benchmark Structure

```rust
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn operation_benchmark(c: &mut Criterion) {
    let setup = setup_test_data(100_000);

    let mut group = c.benchmark_group("operation_name");

    for param in [10, 20, 50, 100] {
        group.bench_with_input(BenchmarkId::new("param", param), &param, |b, &p| {
            b.iter(|| setup.operation(p))
        });
    }

    group.finish();
}

criterion_group!(benches, operation_benchmark);
criterion_main!(benches);
```

---

## Commands

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench operation_name

# Compare against baseline
cargo bench -- --baseline main

# Save baseline
cargo bench -- --save-baseline main

# Generate HTML report
cargo bench  # Opens target/criterion/report/index.html
```

---

## Regression Detection

```rust
// In CI, configure thresholds
group.significance_level(0.05);
group.sample_size(100);
group.measurement_time(std::time::Duration::from_secs(10));
```

Fail CI if performance regresses >10% from baseline.

---

## Memory Profiling

```bash
# Linux
perf record cargo bench && perf report

# macOS
cargo instruments -t "Time Profiler" --bench benchmarks

# Heap profiling
heaptrack cargo bench operation_name
```
