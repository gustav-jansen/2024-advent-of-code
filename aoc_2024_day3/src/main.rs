use std::process;
use std::env;

use aoc_2024_day3::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Can't parse arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = aoc_2024_day3::run(config) {
        println!("Application error: {err}");
        process::exit(1);
    }
}