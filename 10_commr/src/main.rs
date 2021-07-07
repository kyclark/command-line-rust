extern crate commr;
use std::process;

fn main() {
    let config = match commr::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = commr::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
