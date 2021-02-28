extern crate head;
use std::process;

fn main() {
    let config = match head::get_args() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = head::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
