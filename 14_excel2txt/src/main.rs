extern crate excel2txt;
use std::process;

fn main() {
    let config = match excel2txt::get_args() {
        Ok(c) => c,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = excel2txt::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
