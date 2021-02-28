extern crate cat;
use std::process;

fn main() {
    let config = match cat::get_args() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = cat::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
