fn main() {
    let config = match lsr::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = lsr::run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
