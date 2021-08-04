fn main() {
    if let Err(e) = commr::get_args().and_then(|config| commr::run(config)) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
