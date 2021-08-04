fn main() {
    if let Err(e) = tailr::get_args().and_then(|config| tailr::run(config)) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
