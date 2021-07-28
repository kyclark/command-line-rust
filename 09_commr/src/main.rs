fn main() {
    let config = match commr::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = commr::run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
