fn main() {
    let config = match uniqr::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = uniqr::run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
