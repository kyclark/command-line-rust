extern crate grepr;
use std::process;

fn main() {
    let config = match grepr::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = grepr::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
