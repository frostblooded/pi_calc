use criterion::*;
use bigdecimal::*;
use std::cell::*;

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

fn calc_series(n: u32) -> BigDecimal {
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
    cache: RefCell<Vec<BigDecimal>>
}

impl FactorialCalculator {
    fn new() -> Self {
        FactorialCalculator {
            cache: RefCell::new(vec![
                BigDecimal::from(1),
                BigDecimal::from(1)
            ])
        }
    }

    fn is_calculated(&self, n: u32) -> bool {
        self.cache.borrow().len() >= ((n + 1) as usize)
    }

    fn calc(&self, n: u32) -> BigDecimal {
        if !self.is_calculated(n) {
            let prev = self.calc(n - 1);
            self.cache.borrow_mut().push(prev * BigDecimal::from(n));
        }
        
        let cache_borrow = self.cache.borrow();
        cache_borrow[(n as usize)].clone()
    }
}

fn calc_series_cached_factorial(n: u32) -> BigDecimal {
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

fn pi_calc_benchmark(c: &mut Criterion) {
    c.bench_function("1 iteration", |b| b.iter(|| calc_series(1)));
    c.bench_function("10 iterations", |b| b.iter(|| calc_series(10)));
    c.bench_function("50 iterations", |b| b.iter(|| calc_series(50)));
    c.bench_function("100 iterations", |b| b.iter(|| calc_series(100)));
}

fn pi_calc_benchmark_with_cache(c: &mut Criterion) {
    c.bench_function("1 iteration with cache", |b| b.iter(|| calc_series_cached_factorial(1)));
    c.bench_function("10 iterations with cache", |b| b.iter(|| calc_series_cached_factorial(10)));
    c.bench_function("50 iterations with cache", |b| b.iter(|| calc_series_cached_factorial(50)));
    c.bench_function("100 iterations with cache", |b| b.iter(|| calc_series_cached_factorial(100)));
}

criterion_group!(benches, pi_calc_benchmark, pi_calc_benchmark_with_cache);
criterion_main!(benches);