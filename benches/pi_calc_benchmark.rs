use criterion::*;
use bigdecimal::*;

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

fn pi_calc_benchmark(c: &mut Criterion) {
    c.bench_function("1 iteration", |b| b.iter(|| calc_series(1)));
    c.bench_function("10 iterations", |b| b.iter(|| calc_series(10)));
    c.bench_function("30 iterations", |b| b.iter(|| calc_series(30)));
    c.bench_function("50 iterations", |b| b.iter(|| calc_series(50)));
}

criterion_group!(benches, pi_calc_benchmark);
criterion_main!(benches);