extern crate find;
use std::process;

fn main() {
    let config = match find::get_args() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = find::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
