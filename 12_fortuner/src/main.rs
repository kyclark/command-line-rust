extern crate fortuner;
use std::process;

fn main() {
    let config = match fortuner::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = fortuner::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
