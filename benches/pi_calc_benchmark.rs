#![feature(optin_builtin_traits)]

use criterion::*;
use bigdecimal::*;
use std::thread;
use std::sync::*;

fn new_num(n: u64) -> BigDecimal {
    BigDecimal::from(n)
}

fn factorial(n: u64) -> BigDecimal {
    let mut result = new_num(1);

    for i in 2..=n {
        result *= new_num(i);
    }

    result
}

fn pow(b: &BigDecimal, power: u64) -> BigDecimal {
    let mut res = new_num(1);

    for _ in 1..=power {
        res *= b;
    }

    res
}

fn calc_series_no_threads_no_cache(n: u64) -> BigDecimal {
    let mut pi = new_num(0);
    let a = new_num(1103);
    let b = new_num(26390);
    let c = new_num(396);

    for k in 0..=n {
        pi += (factorial(4 * k) * (&a + &b * new_num(k))) /
              (pow(&factorial(k), 4) * pow(&c, 4 * k));
    }
    
    pi *= (new_num(2) * new_num(2).sqrt().unwrap()) / new_num(9801);
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
                new_num(1),
                new_num(1)
            ])
        }
    }

    fn is_calculated(&self, n: u64) -> bool {
        self.cache.lock().unwrap().len() >= ((n + 1) as usize)
    }

    fn calc(&self, n: u64) -> BigDecimal {
        if !self.is_calculated(n) {
            let prev = self.calc(n - 1);
            self.cache.lock().unwrap().push(prev * new_num(n));
        }
        
        let cache_borrow = self.cache.lock().unwrap();
        cache_borrow[(n as usize)].clone()
    }
}

fn calc_series_no_threads_with_cache(n: u64) -> BigDecimal {
    let mut pi = new_num(0);
    let a = new_num(1103);
    let b = new_num(26390);
    let c = new_num(396);
    let factorial_calculator = FactorialCalculator::new();

    for k in 0..=n {
        pi += (factorial_calculator.calc(4 * k) * (&a + &b * new_num(k))) /
              (pow(&factorial_calculator.calc(k), 4) * pow(&c, 4 * k));
    }
    
    pi *= (new_num(2) * new_num(2).sqrt().unwrap()) / new_num(9801);
    pi = 1 / pi;
    pi
}

fn calc_series_for_range(start_index: u64, end_index: u64) -> BigDecimal {
    let mut pi = new_num(0);
    let a = new_num(1103);
    let b = new_num(26390);
    let c = new_num(396);

    for k in start_index..=end_index {
        pi += (factorial(4 * k) * (&a + &b * new_num(k))) /
              (pow(&factorial(k), 4) * pow(&c, 4 * k));
    }
    
    pi * (new_num(2) * new_num(2).sqrt().unwrap()) / new_num(9801)
}

fn calc_series_with_threads_no_cache(n: u64) -> BigDecimal {
    if n < 4 {
        return calc_series_no_threads_no_cache(n);
    }

    let mut handles = vec![];
    const THREAD_COUNT: u64 = 4u64;
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

    result = 1 / result;
    result
}


fn calc_series_for_range_with_cache(start_index: u64, end_index: u64, factorial_calculator: Arc<FactorialCalculator>) -> BigDecimal {
    let mut pi = new_num(0);
    let a = new_num(1103);
    let b = new_num(26390);
    let c = new_num(396);

    for k in start_index..=end_index {
        pi += (factorial_calculator.calc(4 * k) * (&a + &b * new_num(k))) /
              (pow(&factorial_calculator.calc(k), 4) * pow(&c, 4 * k));
    }
    
    pi * (new_num(2) * new_num(2).sqrt().unwrap()) / new_num(9801)
}

fn calc_series_with_threads_with_cache(n: u64) -> BigDecimal {
    if n < 4 {
        return calc_series_no_threads_no_cache(n);
    }

    let mut handles = vec![];
    const THREAD_COUNT: u64 = 4u64;
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

    result = 1 / result;
    result
}

fn calc_series_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("calc series");

    let keypoints: Vec<u64> = vec![5, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    for i in keypoints {
        group.bench_function(BenchmarkId::new("no threads no cache", i), |b| b.iter(|| calc_series_no_threads_no_cache(i)));
        group.bench_function(BenchmarkId::new("no threads with cache", i), |b| b.iter(|| calc_series_no_threads_with_cache(i)));
        group.bench_function(BenchmarkId::new("with threads no cache", i), |b| b.iter(|| calc_series_with_threads_no_cache(i)));
        group.bench_function(BenchmarkId::new("with threads with cache", i), |b| b.iter(|| calc_series_with_threads_with_cache(i)));
    }
}

criterion_group!(benches, calc_series_benchmark);
criterion_main!(benches);