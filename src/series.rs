use crossbeam_channel::Receiver;
use log::debug;
use rug::ops::Pow;
use std::sync::*;
use std::thread;
use std::time::SystemTime;

use crate::big_num::*;
use crate::factorial_calculator::*;
use crate::utils::*;

fn calc_series_helper_for_range(
    precision: u32,
    start_index: u64,
    end_index: u64,
    factorial_calculator: Arc<FactorialCalculator>,
) -> BigNum {
    let mut pi = new_num(precision, 0);
    let mut last_pi: BigNum;
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

pub fn calc_series(input_precision: u32, thread_count: u64) -> BigNum {
    let total_start_time = SystemTime::now();
    let n = ((input_precision as f32) / 7.).ceil() as u64;

    const ADDITIONAL_PRECISION: u32 = 10;
    let increased_precision = input_precision + ADDITIONAL_PRECISION;

    debug!("Input precision: {}", input_precision);
    debug!("Used precision: {}", increased_precision);
    debug!("Iterations: {}", n);

    // The precision that rug uses is the length of the mantissa in bits,
    // but the input precision is in digits after the dot. Here we convert
    // the input precision into the corresponding mantissa bit length
    // by multiplying the input by log2(10).
    let used_increased_precision =
        ((increased_precision as f32) * std::f32::consts::LOG2_10).floor() as u32;
    let used_input_precision =
        ((input_precision as f32) * std::f32::consts::LOG2_10).floor() as u32;

    // Because of the used formula, we know that 4 * n is the biggest factorial
    // that we are going to need.
    let factorial_calculator = Arc::new(FactorialCalculator::new(used_increased_precision, 4 * n));

    let mut result = if n < thread_count || thread_count == 1 {
        calc_series_helper_for_range(used_increased_precision, 0, n - 1, factorial_calculator)
    } else {
        calc_series_helper_with_threads(
            used_increased_precision,
            thread_count,
            n,
            factorial_calculator,
        )
    };

    result *= (new_num(used_increased_precision, 2) * new_num(used_increased_precision, 2).sqrt())
        / new_num(used_increased_precision, 9801);
    result = result.recip();
    result.set_prec(used_input_precision);

    let total_end_time = SystemTime::now();

    debug!(
        "Total execution done in {:?}!",
        total_end_time.duration_since(total_start_time)
    );

    result
}

fn handle_thread(
    i: u64,
    receiver: Receiver<(u64, u64)>,
    factorial_calculator: Arc<FactorialCalculator>,
    precision: u32,
) -> BigNum {
    let mut tasks_done_in_thread = 0;
    let start_time_thread = SystemTime::now();
    let mut res = new_num(precision, 0);

    while let Ok((start_index, end_index)) = receiver.recv() {
        let start_time_job = SystemTime::now();

        debug!(
            "Thread {} starting on range ({}, {})!",
            i, start_index, end_index
        );

        res += calc_series_helper_for_range(
            precision,
            start_index,
            end_index,
            Arc::clone(&factorial_calculator),
        );

        tasks_done_in_thread += 1;
        let end_time_job = SystemTime::now();

        debug!(
            "Thread {} done with ({}, {}) in {:?}!",
            i,
            start_index,
            end_index,
            end_time_job.duration_since(start_time_job)
        );
    }

    let end_time_thread = SystemTime::now();

    debug!(
        "Thread {} done in {:?}! It did {} tasks.",
        i,
        end_time_thread.duration_since(start_time_thread),
        tasks_done_in_thread
    );

    res
}

fn calc_series_helper_with_threads(
    precision: u32,
    thread_count: u64,
    n: u64,
    factorial_calculator: Arc<FactorialCalculator>,
) -> BigNum {
    let mut handles = vec![];
    let (start_indexes, end_indexes) = range_to_subranges(n, 10);
    let (sender, receiver) = crossbeam_channel::unbounded();

    for i in 0..thread_count {
        let factorial_calculator_clone = Arc::clone(&factorial_calculator);
        let receiver_clone = receiver.clone();

        handles.push(thread::spawn(move || {
            handle_thread(i, receiver_clone, factorial_calculator_clone, precision)
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

    result
}
