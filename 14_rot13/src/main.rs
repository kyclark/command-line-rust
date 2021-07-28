fn main() {
    let config = match rot13::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = rot13::run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
