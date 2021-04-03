extern crate cutr;
use std::process;

fn main() {
    let config = match cutr::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = cutr::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
