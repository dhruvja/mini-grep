use std::env;
use std::process;
use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("The progam failed with {}", err);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        eprintln!("Application Error with {}", e);
        process::exit(1);
    }
}