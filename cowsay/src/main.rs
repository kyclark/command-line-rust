extern crate cowsay;
use std::process;

fn main() {
    let config = match cowsay::get_args() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = cowsay::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
