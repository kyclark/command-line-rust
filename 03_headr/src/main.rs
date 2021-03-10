extern crate headr;
use std::process;

fn main() {
    let config = match headr::get_args() {
        Ok(c) => c,
        Err(_) => {
            process::exit(1);
        }
    };

    if let Err(e) = headr::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
