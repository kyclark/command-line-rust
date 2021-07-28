fn main() {
    let config = match catr::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = catr::run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
