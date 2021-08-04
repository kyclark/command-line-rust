fn main() {
    if let Err(e) = wcr::get_args().and_then(|config| wcr::run(config)) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
