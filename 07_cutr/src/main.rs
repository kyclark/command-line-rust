fn main() {
    let config = match cutr::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = cutr::run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
