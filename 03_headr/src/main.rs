extern crate headr;
use std::process;

fn main() {
    let config = match headr::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = headr::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
