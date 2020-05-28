use pi_calc::series::*;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
struct BenchResult {
    thread_count: usize,
    duration: Duration,
    acceleration: f64,
}

fn bench() {
    const SAMPLES_COUNT: u32 = 10;
    const PRECISION: u32 = 30_000;
    let cores = num_cpus::get() as u64;
    let mut best_durations = vec![None; cores as usize];

    for i in 0..cores {
        println!(
            "Starting testing for {} threads with {} samples",
            i + 1,
            SAMPLES_COUNT
        );

        for _ in 0..SAMPLES_COUNT {
            let start_time = SystemTime::now();
            calc_series(PRECISION, i + 1);
            let end_time = SystemTime::now();
            let duration = end_time.duration_since(start_time).unwrap();

            if best_durations[i as usize].is_none()
                || best_durations[i as usize].unwrap() > duration
            {
                best_durations[i as usize] = Some(duration);
            }
        }
    }

    let mut results: Vec<BenchResult> = Vec::with_capacity(cores as usize);
    let one_thread_duration = best_durations[0].unwrap().as_millis() as f64;

    for i in 0..best_durations.len() {
        let current_duration = best_durations[i as usize].unwrap().as_millis() as f64;
        let expected_duration = one_thread_duration / (i + 1) as f64;
        let acceleration = 100f64 * expected_duration / current_duration;

        results.push(BenchResult {
            thread_count: i + 1,
            duration: best_durations[i as usize].unwrap(),
            acceleration,
        })
    }

    println!("{:?}", results);
}

fn main() {
    bench();
}
