extern crate ls;
use std::process;

fn main() {
    let config = match ls::get_args() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = ls::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
