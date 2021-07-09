use std::process;

fn main() {
    let config = match hangman::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = hangman::run(config) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
