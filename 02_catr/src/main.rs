fn main() {
    if let Err(e) = catr::get_args().and_then(|config| catr::run(config)) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
