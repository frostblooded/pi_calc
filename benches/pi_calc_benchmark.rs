use criterion::*;
use pi_calc::series::*;

fn calc_series_benchmark(c: &mut Criterion) {
    const MAX_ITERATIONS: u64 = 1000000;
    const MAX_THREADS: u64 = 4;
    const TEST_PRECISION: u32 = 100;

    println!(
        "This should be Pi: {}",
        calc_series(TEST_PRECISION, MAX_THREADS, MAX_ITERATIONS)
    );

    const SAMPLE_SIZE: usize = 10;
    let mut group = c.benchmark_group("calc series");
    let custom_group = group.sample_size(SAMPLE_SIZE);
    let keypoints = (100..1000).step_by(100);

    for i in keypoints {
        custom_group.bench_function(BenchmarkId::new("single thread", i), |b| {
            b.iter(|| calc_series(i, 1, MAX_ITERATIONS))
        });

        custom_group.bench_function(BenchmarkId::new("many threads", i), |b| {
            b.iter(|| calc_series(i, MAX_THREADS, MAX_ITERATIONS))
        });
    }
}

criterion_group!(benches, calc_series_benchmark);
criterion_main!(benches);
