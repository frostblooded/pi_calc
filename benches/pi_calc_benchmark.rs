use criterion::*;
use pi_calc::factorial_calculator::FactorialCalculator;
use pi_calc::series::*;

fn calc_series_benchmark(c: &mut Criterion) {
    const TEST_PRECISION: u32 = 100;
    let max_threads = num_cpus::get() as u64;

    println!(
        "This should be Pi: {}",
        calc_series(TEST_PRECISION, max_threads)
    );

    const SAMPLE_SIZE: usize = 10;
    let mut group = c.benchmark_group("calc series");
    let custom_group = group.sample_size(SAMPLE_SIZE);
    let keypoints = (10_000..100_000).step_by(1_000);

    for i in keypoints {
        custom_group.bench_function(BenchmarkId::new("single thread", i), |b| {
            b.iter(|| calc_series(i, 1))
        });

        custom_group.bench_function(BenchmarkId::new("many threads", i), |b| {
            b.iter(|| calc_series(i, max_threads))
        });

        custom_group.bench_function(BenchmarkId::new("factorial calculator", i), |b| {
            b.iter(|| {
                FactorialCalculator::new(
                    TEST_PRECISION,
                    ((i as f64 / 7.) * std::f64::consts::LOG2_10) as u64,
                )
            })
        });
    }
}

criterion_group!(benches, calc_series_benchmark);
criterion_main!(benches);
