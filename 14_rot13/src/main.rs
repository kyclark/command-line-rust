fn main() {
    if let Err(e) = rot13::get_args().and_then(rot13::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
