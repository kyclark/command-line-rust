extern crate clap;

use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    text: String,
    newline: bool,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("echo")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true),
        )
        .arg(
            Arg::with_name("no_newline")
                .help("Do not print newline")
                .takes_value(false)
                .short("n"),
        )
        .get_matches();

    Ok(Config {
        text: matches.value_of("text").unwrap().to_string(),
        newline: !matches.is_present("no_newline"),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    print!("{}{}", &config.text, if config.newline { "\n" } else { "" });
    Ok(())
}
