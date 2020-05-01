type BigNum = rug::Float;

use rug::ops::Pow;
use std::sync::*;
use std::thread;
use std::time::SystemTime;

fn new_num(precision: u32, n: u64) -> BigNum {
    BigNum::with_val(precision, n)
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

fn calc_series_helper_for_range(
    precision: u32,
    start_index: u64,
    end_index: u64,
    factorial_calculator: Arc<FactorialCalculator>,
) -> BigNum {
    let mut pi = new_num(precision, 0);
    let mut last_pi = new_num(precision, 0);
    let a = new_num(precision, 1103);
    let b = new_num(precision, 26390);
    let c = new_num(precision, 396);

    for k in start_index..=end_index {
        let top1 = factorial_calculator.get(4 * k);
        let top2 = &a + &b * new_num(precision, k);
        let bottom1 = BigNum::with_val(precision, factorial_calculator.get(k).pow(4));
        let bottom2 = BigNum::with_val(precision, (&c).pow(4 * k));

        last_pi = pi.clone();
        pi += (top1 * top2) / (bottom1 * bottom2);

        if last_pi == pi {
            return pi;
        }
    }

    pi
}

pub fn calc_series(precision: u32, thread_count: u64, n: u64) -> BigNum {
    // Because of the used formula, we know that 4 * n is the biggest factorial
    // that we are going to need.
    let factorial_calculator = Arc::new(FactorialCalculator::new(precision, 4 * n));

    let mut result = if n < thread_count || thread_count == 1 {
        calc_series_helper_for_range(precision, 0, n - 1, factorial_calculator)
    } else {
        calc_series_helper_with_threads(precision, thread_count, n, factorial_calculator)
    };

    result *= (new_num(precision, 2) * new_num(precision, 2).sqrt()) / new_num(precision, 9801);
    1 / result
}

fn calc_series_helper_with_threads(
    precision: u32,
    thread_count: u64,
    n: u64,
    factorial_calculator: Arc<FactorialCalculator>,
) -> BigNum {
    let total_start_time = SystemTime::now();
    let mut handles = vec![];
    let tasks_count = 50;
    let iters_per_task = n / tasks_count;
    let remaining_iters = n % tasks_count;

    let iter_range: Vec<_> = (0..(tasks_count - 1)).collect();

    let mut start_indexes: Vec<_> = iter_range.iter().map(|i| i * iters_per_task).collect();
    start_indexes.push(n - iters_per_task - remaining_iters);

    let mut end_indexes: Vec<_> = iter_range
        .iter()
        .map(|i| (i + 1) * iters_per_task - 1)
        .collect();
    end_indexes.push(n - 1);

    let (sender, receiver) = crossbeam_channel::unbounded();

    for i in 0..thread_count {
        let factorial_calculator_clone = Arc::clone(&factorial_calculator);
        let receiver_clone = receiver.clone();

        handles.push(thread::spawn(move || {
            let mut tasks_done_in_thread = 0;
            let start_time_thread = SystemTime::now();
            let mut res = new_num(precision, 0);

            while let Ok((start_index, end_index)) = receiver_clone.recv() {
                let start_time_job = SystemTime::now();

                println!(
                    "Thread {} starting on range ({}, {})!",
                    i, start_index, end_index
                );

                res += calc_series_helper_for_range(
                    precision,
                    start_index,
                    end_index,
                    Arc::clone(&factorial_calculator_clone),
                );

                tasks_done_in_thread += 1;
                let end_time_job = SystemTime::now();

                println!(
                    "Thread {} done with ({}, {}) in {:?}!",
                    i,
                    start_index,
                    end_index,
                    end_time_job.duration_since(start_time_job)
                );
            }

            let end_time_thread = SystemTime::now();

            println!(
                "Thread {} done in {:?}! It did {} tasks.",
                i,
                end_time_thread.duration_since(start_time_thread),
                tasks_done_in_thread
            );

            res
        }));
    }

    for i in 0..start_indexes.len() {
        let start_index = start_indexes[i as usize];
        let end_index = end_indexes[i as usize];
        sender
            .send((start_index, end_index))
            .expect("Failed to send task");
    }

    drop(sender);

    let mut result = new_num(precision, 0);

    for handle in handles {
        result += handle.join().expect("Thread finished with error");
    }

    let total_end_time = SystemTime::now();

    println!(
        "Total execution done in {:?}!",
        total_end_time.duration_since(total_start_time)
    );

    result
}
