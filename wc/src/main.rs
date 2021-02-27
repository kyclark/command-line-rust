extern crate wc;
use std::process;

fn main() {
    let config = match wc::get_args() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = wc::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
