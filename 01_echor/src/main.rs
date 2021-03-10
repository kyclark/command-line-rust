extern crate echor;
use std::process;

fn main() {
    let config = match echor::get_args() {
        Ok(c) => c,
        Err(_) => {
            process::exit(1);
        }
    };

    if let Err(e) = echor::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
