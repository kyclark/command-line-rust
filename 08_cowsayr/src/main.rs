extern crate cowsayr;
use std::process;

fn main() {
    let config = match cowsayr::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = cowsayr::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
