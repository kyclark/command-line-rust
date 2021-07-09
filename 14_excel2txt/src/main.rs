use std::process;

fn main() {
    let config = match excel2txt::get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = excel2txt::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
