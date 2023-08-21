use std::env;
use std::error::Error;
use std::fs;
use std::process;

use rusty_krab::Config;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|_| {
        eprintln!("Error parsing arguments: {args:#?}");
        process::exit(1);
    });

    println!("Searching {} for {}\n", config.file_path, config.query);

    if let Err(error) = rusty_krab::run(config) {
        eprintln!("Application Error: {error:#?}");
        process::exit(1);
    }
}
