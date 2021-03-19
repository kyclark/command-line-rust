extern crate markov;
use std::process;

fn main() {
    let config = match markov::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = markov::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
