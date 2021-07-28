fn main() {
    let config = match tailr::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = tailr::run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
