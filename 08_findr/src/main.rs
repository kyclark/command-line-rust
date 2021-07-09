use std::process;

fn main() {
    let config = match findr::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = findr::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
