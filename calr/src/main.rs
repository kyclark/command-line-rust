extern crate calr;
use std::process;

fn main() {
    let config = match calr::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = calr::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
