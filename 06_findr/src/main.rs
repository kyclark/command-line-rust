fn main() {
    if let Err(e) = findr::get_args().and_then(|config| findr::run(config)) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
