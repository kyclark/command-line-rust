fn main() {
    if let Err(e) = cutr::get_args().and_then(|config| cutr::run(config)) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
