fn main() {
    if let Err(e) = calr::get_args().and_then(|config| calr::run(config)) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
