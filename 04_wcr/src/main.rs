fn main() {
    let config = match wcr::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = wcr::run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
