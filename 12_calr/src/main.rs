fn main() {
    let config = match calr::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = calr::run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
