fn main() {
    let config = match findr::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = findr::run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
