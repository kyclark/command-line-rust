fn main() {
    if let Err(e) = fortuner::get_args().and_then(fortuner::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
