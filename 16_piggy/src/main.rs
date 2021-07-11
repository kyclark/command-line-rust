use std::process;

fn main() {
    let config = match piggy::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = piggy::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
