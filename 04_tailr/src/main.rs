extern crate tailr;
use std::process;

fn main() {
    let config = match tailr::get_args() {
        Ok(c) => c,
        Err(_) => {
            process::exit(1);
        }
    };

    if let Err(e) = tailr::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
