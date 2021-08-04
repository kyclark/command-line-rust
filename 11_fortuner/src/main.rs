fn main() {
    if let Err(e) =
        fortuner::get_args().and_then(|config| fortuner::run(config))
    {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
