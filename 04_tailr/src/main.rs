use std::process;

fn main() {
    let config = match tailr::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = tailr::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
