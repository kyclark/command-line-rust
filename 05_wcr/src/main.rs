use std::process;

fn main() {
    let config = match wcr::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = wcr::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
