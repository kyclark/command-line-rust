fn main() {
    let config = match headr::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = headr::run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
