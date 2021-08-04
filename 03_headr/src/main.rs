fn main() {
    if let Err(e) = headr::get_args().and_then(|config| headr::run(config)) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
