fn main() {
    if let Err(e) = grepr::get_args().and_then(|config| grepr::run(config)) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
