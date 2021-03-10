extern crate wcr;
use std::process;

fn main() {
    let config = match wcr::get_args() {
        Ok(c) => c,
        Err(_) => {
            process::exit(1);
        }
    };

    if let Err(e) = wcr::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
