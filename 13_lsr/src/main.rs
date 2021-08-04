fn main() {
    if let Err(e) = lsr::get_args().and_then(lsr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
