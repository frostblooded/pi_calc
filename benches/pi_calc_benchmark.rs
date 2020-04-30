use criterion::*;
use pi_calc::calc;

fn calc_series_benchmark(c: &mut Criterion) {
    const TEST_ITERATIONS: u64 = 1000;
    const MAX_THREADS: u64 = 4;
    const PRECISION: u32 = 10000;

    println!(
        "{}",
        calc::calc_series(PRECISION, MAX_THREADS, TEST_ITERATIONS)
    );

    const SAMPLE_SIZE: usize = 10;
    let mut group = c.benchmark_group("calc series");
    let custom_group = group.sample_size(SAMPLE_SIZE);
    let keypoints = (10000..100000).step_by(10000);

    for i in keypoints {
        custom_group.bench_function(BenchmarkId::new("single thread", i), |b| {
            b.iter(|| calc::calc_series(PRECISION, 1, i))
        });

        custom_group.bench_function(BenchmarkId::new("many threads", i), |b| {
            b.iter(|| calc::calc_series(PRECISION, MAX_THREADS, i))
        });
    }
}

criterion_group!(benches, calc_series_benchmark);
criterion_main!(benches);
