use std::process;

fn main() {
    let config = match lsr::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = lsr::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
