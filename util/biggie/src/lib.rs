use clap::{App, Arg};
use rand::{distributions::Alphanumeric, Rng};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use thousands::Separable;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    outfile: String,
    lines: usize,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("biggie")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Make big text files")
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("100000"),
        )
        .arg(
            Arg::with_name("outfile")
                .short("o")
                .long("outfile")
                .value_name("FILE")
                .help("Output filename")
                .default_value("out"),
        )
        .get_matches();

    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("--lines \"{}\" must be greater than 0", e))?;

    Ok(Config {
        lines: lines.unwrap(),
        outfile: matches.value_of("outfile").unwrap().to_string(),
    })
}

// --------------------------------------------------
fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let mut file = File::create(&config.outfile)?;
    for _ in 0..config.lines {
        let num_words = rand::thread_rng().gen_range(7..15);
        let mut words = vec![];
        for _ in 0..num_words {
            words.push(random_string());
        }
        writeln!(file, "{}", words.join(" "))?;
    }

    println!(
        "Done, wrote {} line{} to \"{}\".",
        config.lines.separate_with_commas(),
        if config.lines == 1 { "" } else { "s" },
        config.outfile
    );

    Ok(())
}

// --------------------------------------------------
fn random_string() -> String {
    let length = rand::thread_rng().gen_range(2..8);
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
