extern crate clap;

use clap::{App, Arg};
use std::error::Error;
use std::fs;
use std::str::FromStr;
use textwrap;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    text: String,
    width: usize,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("cowsay")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cowsay")
        .arg(
            Arg::with_name("input")
                .value_name("INPUT")
                .help("Input file or string")
                .required(true),
        )
        .arg(
            Arg::with_name("width")
                .value_name("WIDTH")
                .help("Maximum text width")
                .short("w")
                .long("width")
                .default_value("40"),
        )
        .get_matches();

    let text = matches.value_of("input").unwrap();
    let text = fs::read_to_string(text).unwrap_or(text.trim().to_string());
    let width: usize = parse_int(matches.value_of("width").unwrap())?;

    if width < 1 {
        return Err(From::from("Width must be positive"));
    }

    Ok(Config {
        text: text,
        width: width,
    })
}

// --------------------------------------------------
fn parse_int<T: FromStr>(val: &str) -> MyResult<T> {
    val.trim()
        .parse::<T>()
        .or(Err(From::from(format!("\"{}\" is not an integer", val))))
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let cow = r"
                \   ^__^
                 \  (oo)\_______
                    (__)\       )\/\
                        ||----w |
                        ||     ||
    ";

    let width = config.width;
    let mut lines = textwrap::wrap(&config.text, width);
    let text = if lines.len() == 1 {
        vec![format!("< {} >", &lines.pop().unwrap())]
    } else {
        let mut tmp: Vec<String> = vec![];
        tmp.push(format!("/ {:width$} \\", &lines.remove(0), width = width));

        for line in &lines {
            tmp.push(format!("| {:width$} |", line, width = width));
        }

        tmp.push(format!(
            "\\ {:width$} /",
            &lines.pop().unwrap(),
            width = width
        ));
        tmp
    };

    let longest = &text.iter().map(|t| t.len()).max().unwrap_or(width) - 3;
    let top_bar = "_".repeat(longest);
    let bottom_bar = "-".repeat(longest);
    println!(" {}", top_bar);
    println!("{}", text.join("\n"));
    println!(" {}", bottom_bar);
    println!("{}", cow);

    Ok(())
}
