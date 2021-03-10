extern crate uniqr;
use std::process;

fn main() {
    let config = match uniqr::get_args() {
        Ok(c) => c,
        Err(_) => {
            process::exit(1);
        }
    };

    if let Err(e) = uniqr::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
