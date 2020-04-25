use clap::{App, Arg};
use std::sync::*;
use std::thread;

type BigNum = rug::Float;

fn new_num(precision: u32, n: u64) -> BigNum {
    BigNum::with_val(precision, n)
}

fn pow(b: &BigNum, power: u64) -> BigNum {
    let mut res = new_num(b.prec(), 1);

    for _ in 1..=power {
        res *= b;
    }

    res
}

struct FactorialCalculator {
    cache: Vec<BigNum>,
}
impl FactorialCalculator {
    fn new(precision: u32, n: u64) -> Self {
        let mut cache_builder: Vec<BigNum> = vec![];
        cache_builder.push(new_num(precision, 1));

        for i in 1..=n {
            cache_builder.push(&cache_builder[(i - 1) as usize] * new_num(precision, i));
        }

        FactorialCalculator {
            cache: cache_builder,
        }
    }

    fn get(&self, i: u64) -> &BigNum {
        &self.cache[i as usize]
    }
}

fn calc_series_no_threads(precision: u32, n: u64) -> BigNum {
    let mut pi = new_num(precision, 0);
    let a = new_num(precision, 1103);
    let b = new_num(precision, 26390);
    let c = new_num(precision, 396);
    let factorial_calculator = FactorialCalculator::new(precision, 4 * n);

    for k in 0..=n {
        pi += (factorial_calculator.get(4 * k) * (&a + &b * new_num(precision, k)))
            / (pow(&factorial_calculator.get(k), 4) * pow(&c, 4 * k));
    }

    pi *= (new_num(precision, 2) * new_num(precision, 2).sqrt()) / new_num(precision, 9801);
    1 / pi
}

fn calc_series_for_range(
    precision: u32,
    start_index: u64,
    end_index: u64,
    factorial_calculator: Arc<FactorialCalculator>,
) -> BigNum {
    let mut pi = new_num(precision, 0);
    let a = new_num(precision, 1103);
    let b = new_num(precision, 26390);
    let c = new_num(precision, 396);

    for k in start_index..=end_index {
        pi += (factorial_calculator.get(4 * k) * (&a + &b * new_num(precision, k)))
            / (pow(&factorial_calculator.get(k), 4) * pow(&c, 4 * k));
    }

    pi
}

fn calc_series_with_threads(precision: u32, thread_count: u64, n: u64) -> BigNum {
    if n < thread_count {
        return calc_series_no_threads(precision, n);
    }

    let mut handles = vec![];
    let jobs_per_thread = n / thread_count;
    let remaining_jobs = n % thread_count;
    let factorial_calculator = Arc::new(FactorialCalculator::new(precision, 4 * n));

    for i in 0..(thread_count - 1) {
        let start_index = i * jobs_per_thread;
        let end_index = (i + 1) * jobs_per_thread - 1;
        let factorial_calculator_clone = factorial_calculator.clone();

        handles.push(thread::spawn(move || {
            calc_series_for_range(
                precision,
                start_index,
                end_index,
                factorial_calculator_clone,
            )
        }));
    }

    handles.push(thread::spawn(move || {
        calc_series_for_range(
            precision,
            n - jobs_per_thread - remaining_jobs,
            n,
            factorial_calculator,
        )
    }));

    let mut result = new_num(precision, 0);

    for handle in handles {
        result += handle.join().expect("Thread finished with error");
    }

    result *= (new_num(precision, 2) * new_num(precision, 2).sqrt()) / new_num(precision, 9801);
    1 / result
}

fn main() {
    let matches = App::new("Pi calc program")
        .version("1.0")
        .author("Nikolay Danailov")
        .about("Efficiently calculating Pi in a multithreaded way")
        .arg(
            Arg::with_name("thread_count")
                .short("t")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("precision")
                .short("p")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let thread_count: u64 = matches
        .value_of("thread_count")
        .expect("thread_count argument not passed in and clap didn't detect it for some reason")
        .parse()
        .expect("failed to parse thread_count to a number");

    let precision: u32 = matches
        .value_of("precision")
        .expect("precision argument not passed in and clap didn't detect it for some reason")
        .parse()
        .expect("failed to parse precision to a number");

    println!("{:?}", precision);
    println!("{:?}", thread_count);

    let pi = calc_series_with_threads(precision, thread_count, 500);

    println!("{:?}", pi);
}
