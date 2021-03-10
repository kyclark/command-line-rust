extern crate cutr;
use std::process;

fn main() {
    let config = match cutr::get_args() {
        Ok(c) => c,
        Err(_) => {
            process::exit(1);
        }
    };

    if let Err(e) = cutr::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
