extern crate grepr;
use std::process;

fn main() {
    let config = match grepr::get_args() {
        Ok(c) => c,
        Err(_) => {
            process::exit(1);
        }
    };

    if let Err(e) = grepr::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
