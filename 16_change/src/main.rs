extern crate change;
use std::process;

fn main() {
    let config = match change::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = change::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
