fn main() {
    let config = match grepr::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = grepr::run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
