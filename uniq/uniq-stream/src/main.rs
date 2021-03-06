extern crate uniq;
use std::process;

fn main() {
    let config = match uniq::get_args() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = uniq::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
