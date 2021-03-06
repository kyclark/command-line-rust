extern crate cut;
use std::process;

fn main() {
    let config = match cut::get_args() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = cut::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
