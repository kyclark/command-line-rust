fn main() {
    if let Err(e) = calr::get_args().and_then(calr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
