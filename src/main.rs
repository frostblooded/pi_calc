mod calc;
use clap::{App, Arg};

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
        .arg(Arg::with_name("iterations").short("i").takes_value(true))
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

    const DEFAULT_ITERATIONS_INPUT: &str = "1000";
    let iterations: u64 = matches
        .value_of("iterations")
        .unwrap_or(DEFAULT_ITERATIONS_INPUT)
        .parse()
        .expect("failed to parse iterations to a number");

    // The precision that rug uses is the length of the mantissa in bits,
    // but the input precision is in digits after the dot. Here we convert
    // the input precision into the corresponding mantissa bit length
    // by multiplying the input by log2(10).
    let log = 10f32.log2();
    let precision = ((precision as f32) * log).floor() as u32;

    let pi = calc::calc_series(precision, thread_count, iterations);
    println!("{}", pi);
}
