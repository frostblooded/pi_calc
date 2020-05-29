use clap::{App, Arg, ArgMatches};
use pi_calc::series::*;
use std::fs;

fn get_app_matches<'a>() -> ArgMatches<'a> {
    App::new("Pi calc program")
        .version("1.0")
        .author("Nikolay Danailov")
        .about("Efficiently calculating Pi in a multithreaded way")
        .arg(
            Arg::with_name("thread_count")
                .short("t")
                .required(true)
                .takes_value(true)
                .help("Count of threads to be spawned for the calculation."),
        )
        .arg(
            Arg::with_name("precision")
                .short("p")
                .required(true)
                .takes_value(true)
                .help("Digits to be calculated."),
        )
        .arg(
            Arg::with_name("debug_log")
                .short("d")
                .help("Whether debug logs should be printed."),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .help("Whether the result should be printed."),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .help("To which file the output should be written. Default is STDOUT.")
                .takes_value(true),
        )
        .get_matches()
}

fn get_parsed_args() -> (u64, u32, bool, bool, Option<String>) {
    let matches = get_app_matches();

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

    let debug_log = matches.is_present("debug_log");
    let quiet = matches.is_present("quiet");
    let output = matches.value_of("output").map(|s| s.to_string());

    (thread_count, precision, debug_log, quiet, output)
}

fn main() {
    let (thread_count, precision, debug_log, quiet, output) = get_parsed_args();

    if debug_log {
        simple_logger::init().expect("Failed to initialize simple_logger");
    }

    let pi = calc_series(precision, thread_count);

    if quiet {
        return;
    }

    // There are some digits at the back of the number
    // that are remainders from the computation and aren't
    // accurate, so we don't want to display them.
    // We display only as much digits as is the input precision.
    let mut truncated_pi = pi.to_string();

    // Add 1 to account for the dot in the float
    let truncated_length = precision + 1;

    truncated_pi.truncate(truncated_length as usize);

    if let Some(file_path) = output {
        fs::write(file_path, truncated_pi).expect("Unable to write to file");
    } else {
        println!("{}", truncated_pi);
    }
}
