extern crate echo;
use std::process;

fn main() {
    let config = match echo::get_args() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = echo::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
