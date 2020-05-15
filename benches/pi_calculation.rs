use pi_calc::series::*;
use std::time::SystemTime;

fn bench() {
    const SAMPLES_COUNT: u32 = 10;
    const PRECISION: u32 = 10_000;
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

    println!("{:?}", best_durations);
}

fn main() {
    bench();
}
