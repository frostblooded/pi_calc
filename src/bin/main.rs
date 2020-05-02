use clap::{App, Arg, ArgMatches};
use pi_calc::series::*;

fn get_app_matches<'a>() -> ArgMatches<'a> {
    App::new("Pi calc program")
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
        .arg(Arg::with_name("debug_log").short("d"))
        .get_matches()
}

fn get_parsed_args() -> (u64, u32, bool) {
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

    let debug_log: bool = matches.is_present("debug_log");

    (thread_count, precision, debug_log)
}

fn main() {
    let (thread_count, precision, debug_log) = get_parsed_args();

    if debug_log {
        simple_logger::init().expect("Failed to initialize simple_logger");
    }

    let pi = calc_series(precision, thread_count);
    println!("{}", pi);
}
