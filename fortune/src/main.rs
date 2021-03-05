extern crate fortune;
use std::process;

fn main() {
    let config = match fortune::get_args() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = fortune::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
