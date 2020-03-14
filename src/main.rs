use bigdecimal::*;
use std::time::*;

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
    let mut pi: BigDecimal = bigdecimal::Zero::zero();

    let a = BigDecimal::from(1103);
    let b = BigDecimal::from(26390);
    let c = BigDecimal::from(396);

    for k in 0..=n {
        pi += (factorial(4 * k) * (&a + &b * BigDecimal::from(k))) /
              (pow(&factorial(k), 4) * pow(&c, 4 * k));
    }
    
    pi *= (BigDecimal::from(2) * BigDecimal::from(2).sqrt().unwrap()) / BigDecimal::from(9801);
    1 / pi
}

fn print_pi(n: u32) {
    let start_time: Instant = Instant::now();
    let pi = calc_series(n);
    let taken_time: Duration = Instant::now() - start_time;
    println!("{} - {} iterations and took {:?}", pi, n, taken_time);
}

fn main() {
    print_pi(1);
    print_pi(5);
    print_pi(10);
    print_pi(100);
}
