#![feature(optin_builtin_traits)]

use criterion::*;
use bigdecimal::*;
use std::cell::*;
use std::thread;
use std::sync::*;

fn factorial(n: u32) -> BigDecimal {
    let mut result = bigdecimal::One::one();

    for i in 2..=n {
        result *= BigDecimal::from(i);
    }

    result
}

fn pow(b: &BigDecimal, power: u32) -> BigDecimal {
    let mut res = bigdecimal::One::one();

    for _ in 1..=power {
        res *= b;
    }

    res
}

fn calc_series_no_threads_no_cache(n: u32) -> BigDecimal {
    let mut pi = bigdecimal::Zero::zero();

    let a = BigDecimal::from(1103);
    let b = BigDecimal::from(26390);
    let c = BigDecimal::from(396);

    for k in 0..=n {
        pi += (factorial(4 * k) * (&a + &b * BigDecimal::from(k))) /
              (pow(&factorial(k), 4) * pow(&c, 4 * k));
    }
    
    pi *= (BigDecimal::from(2) * BigDecimal::from(2).sqrt().unwrap()) / BigDecimal::from(9801);
    pi = 1 / pi;
    pi
}

struct FactorialCalculator {
    cache: Mutex<Vec<BigDecimal>>
}

impl FactorialCalculator {
    fn new() -> Self {
        FactorialCalculator {
            cache: Mutex::new(vec![
                BigDecimal::from(1),
                BigDecimal::from(1)
            ])
        }
    }

    fn is_calculated(&self, n: u32) -> bool {
        self.cache.lock().unwrap().len() >= ((n + 1) as usize)
    }

    fn calc(&self, n: u32) -> BigDecimal {
        if !self.is_calculated(n) {
            let prev = self.calc(n - 1);
            self.cache.lock().unwrap().push(prev * BigDecimal::from(n));
        }
        
        let cache_borrow = self.cache.lock().unwrap();
        cache_borrow[(n as usize)].clone()
    }
}

fn calc_series_no_threads_with_cache(n: u32) -> BigDecimal {
    let mut pi = bigdecimal::Zero::zero();

    let factorial_calculator = FactorialCalculator::new();
    let a = BigDecimal::from(1103);
    let b = BigDecimal::from(26390);
    let c = BigDecimal::from(396);

    for k in 0..=n {
        pi += (factorial_calculator.calc(4 * k) * (&a + &b * BigDecimal::from(k))) /
              (pow(&factorial_calculator.calc(k), 4) * pow(&c, 4 * k));
    }
    
    pi *= (BigDecimal::from(2) * BigDecimal::from(2).sqrt().unwrap()) / BigDecimal::from(9801);
    pi = 1 / pi;
    pi
}

fn calc_series_for_range(start_index: u32, end_index: u32) -> BigDecimal {
    let mut pi = bigdecimal::Zero::zero();

    let a = BigDecimal::from(1103);
    let b = BigDecimal::from(26390);
    let c = BigDecimal::from(396);

    for k in start_index..=end_index {
        pi += (factorial(4 * k) * (&a + &b * BigDecimal::from(k))) /
              (pow(&factorial(k), 4) * pow(&c, 4 * k));
    }
    
    pi *= (BigDecimal::from(2) * BigDecimal::from(2).sqrt().unwrap()) / BigDecimal::from(9801);
    pi = 1 / pi;
    pi
}

fn calc_series_with_threads_no_cache(n: u32) -> BigDecimal {
    if n < 4 {
        return calc_series_no_threads_no_cache(n);
    }

    let mut handles = vec![];
    const THREAD_COUNT: u32 = 4u32;
    let jobs_per_thread = n / THREAD_COUNT;
    let remaining_jobs = n % THREAD_COUNT;

    for i in 0..=(THREAD_COUNT - 1) {
        let start_index = i * jobs_per_thread;
        let end_index = (i + 1) * jobs_per_thread - 1;

        handles.push(thread::spawn(move || {
            calc_series_for_range(start_index, end_index)
        }));
    }

    handles.push(thread::spawn(move || {
        calc_series_for_range(n - remaining_jobs, n)
    }));

    let mut result = bigdecimal::Zero::zero();

    for handle in handles {
        result += handle.join().expect("Thread finished with error");
    }

    result
}


fn calc_series_for_range_with_cache(start_index: u32, end_index: u32, factorial_calculator: Arc<FactorialCalculator>) -> BigDecimal {
    let mut pi = bigdecimal::Zero::zero();

    let a = BigDecimal::from(1103);
    let b = BigDecimal::from(26390);
    let c = BigDecimal::from(396);

    for k in start_index..=end_index {
        pi += (factorial_calculator.calc(4 * k) * (&a + &b * BigDecimal::from(k))) /
              (pow(&factorial_calculator.calc(k), 4) * pow(&c, 4 * k));
    }
    
    pi *= (BigDecimal::from(2) * BigDecimal::from(2).sqrt().unwrap()) / BigDecimal::from(9801);
    pi = 1 / pi;
    pi
}

fn calc_series_with_threads_with_cache(n: u32) -> BigDecimal {
    if n < 4 {
        return calc_series_no_threads_no_cache(n);
    }

    let mut handles = vec![];
    const THREAD_COUNT: u32 = 4u32;
    let jobs_per_thread = n / THREAD_COUNT;
    let remaining_jobs = n % THREAD_COUNT;
    let factorial_calculator = Arc::new(FactorialCalculator::new());

    for i in 0..=(THREAD_COUNT - 1) {
        let start_index = i * jobs_per_thread;
        let end_index = (i + 1) * jobs_per_thread - 1;
        let factorial_calculator_clone = factorial_calculator.clone();

        handles.push(thread::spawn(move || {
            calc_series_for_range_with_cache(start_index, end_index, factorial_calculator_clone)
        }));
    }

    let factorial_calculator_clone = factorial_calculator.clone();

    handles.push(thread::spawn(move || {
        calc_series_for_range_with_cache(n - remaining_jobs, n, factorial_calculator_clone)
    }));

    let mut result = bigdecimal::Zero::zero();

    for handle in handles {
        result += handle.join().expect("Thread finished with error");
    }

    result
}

fn calc_series_no_threads_no_cache_benchmark(c: &mut Criterion) {
    c.bench_function("1 iteration no threads no cache", |b| b.iter(|| calc_series_no_threads_no_cache(1)));
    c.bench_function("10 iterations no threads no cache", |b| b.iter(|| calc_series_no_threads_no_cache(10)));
    c.bench_function("50 iterations no threads no cache", |b| b.iter(|| calc_series_no_threads_no_cache(50)));
    c.bench_function("100 iterations no threads no cache", |b| b.iter(|| calc_series_no_threads_no_cache(100)));
}

fn calc_series_no_threads_with_cache_benchmark(c: &mut Criterion) {
    c.bench_function("1 iteration no threads with cache", |b| b.iter(|| calc_series_no_threads_with_cache(1)));
    c.bench_function("10 iterations no threads with cache", |b| b.iter(|| calc_series_no_threads_with_cache(10)));
    c.bench_function("50 iterations no threads with cache", |b| b.iter(|| calc_series_no_threads_with_cache(50)));
    c.bench_function("100 iterations no threads with cache", |b| b.iter(|| calc_series_no_threads_with_cache(100)));
}

fn calc_series_with_threads_no_cache_benchmark(c: &mut Criterion) {
    c.bench_function("1 iteration with threads no cache", |b| b.iter(|| calc_series_with_threads_no_cache(1)));
    c.bench_function("10 iterations with threads no cache", |b| b.iter(|| calc_series_with_threads_no_cache(10)));
    c.bench_function("50 iterations with threads no cache", |b| b.iter(|| calc_series_with_threads_no_cache(50)));
    c.bench_function("100 iterations with threads no cache", |b| b.iter(|| calc_series_with_threads_no_cache(100)));
}

fn calc_series_with_threads_with_cache_benchmark(c: &mut Criterion) {
    c.bench_function("1 iteration with threads with cache", |b| b.iter(|| calc_series_with_threads_with_cache(1)));
    c.bench_function("10 iterations with threads with cache", |b| b.iter(|| calc_series_with_threads_with_cache(10)));
    c.bench_function("50 iterations with threads with cache", |b| b.iter(|| calc_series_with_threads_with_cache(50)));
    c.bench_function("100 iterations with threads with cache", |b| b.iter(|| calc_series_with_threads_with_cache(100)));
}

criterion_group!(benches,
                //  calc_series_no_threads_no_cache_benchmark,
                 calc_series_no_threads_with_cache_benchmark,
                //  calc_series_with_threads_no_cache_benchmark,
                //  calc_series_with_threads_with_cache_benchmark
                );
criterion_main!(benches);