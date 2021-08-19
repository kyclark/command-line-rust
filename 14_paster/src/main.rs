fn main() {
    if let Err(e) = paster::get_args().and_then(paster::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
