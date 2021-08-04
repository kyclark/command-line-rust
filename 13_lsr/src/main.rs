fn main() {
    if let Err(e) = lsr::get_args().and_then(|config| lsr::run(config)) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
