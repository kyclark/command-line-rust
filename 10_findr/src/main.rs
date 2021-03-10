extern crate findr;
use std::process;

fn main() {
    let config = match findr::get_args() {
        Ok(c) => c,
        Err(_) => {
            process::exit(1);
        }
    };

    if let Err(e) = findr::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
