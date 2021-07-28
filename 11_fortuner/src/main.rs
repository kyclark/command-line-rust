fn main() {
    let config = match fortuner::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = fortuner::run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
