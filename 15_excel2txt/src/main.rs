fn main() {
    if let Err(e) = excel2txt::get_args().and_then(excel2txt::run) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
