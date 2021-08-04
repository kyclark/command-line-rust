fn main() {
    if let Err(e) = uniqr::get_args().and_then(|config| uniqr::run(config)) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
