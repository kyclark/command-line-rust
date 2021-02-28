extern crate grep;
use std::process;

fn main() {
    let config = match grep::get_args() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = grep::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
