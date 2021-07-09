use std::process;

fn main() {
    let config = match rot13::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = rot13::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
