extern crate lsr;
use std::process;

fn main() {
    let config = match lsr::get_args() {
        Ok(c) => c,
        Err(_) => {
            process::exit(1);
        }
    };

    if let Err(e) = lsr::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
