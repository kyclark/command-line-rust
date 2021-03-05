extern crate clap;
extern crate rand;

use clap::{App, Arg};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    file: String,
    seed: Option<u64>,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("fortune")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust fortune")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Fortune file")
                .required(true),
        )
        .arg(
            Arg::with_name("seed")
                .value_name("SEED")
                .help("Random seed")
                .short("s")
                .long("seed"),
        )
        .get_matches();

    let seed = matches
        .value_of("seed")
        .and_then(|v| v.trim().parse::<u64>().ok());

    Ok(Config {
        file: matches.value_of("file").unwrap().to_string(),
        seed: seed,
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let fortunes = read_fortunes(&config.file)?;
    let mut rng = match &config.seed {
        Some(seed) => StdRng::seed_from_u64(*seed),
        _ => rand::thread_rng(),
    };
    let i = rng.gen_range(0..fortunes.len());
    println!("{}", fortunes[i].join("\n"));
    Ok(())
}

// --------------------------------------------------
fn read_fortunes(filename: &String) -> MyResult<Vec<Vec<String>>> {
    let file = File::open(filename)?;
    let file = BufReader::new(file);
    let mut fortunes: Vec<Vec<String>> = vec![];
    let mut buffer: Vec<String> = vec![];

    for line in file.lines() {
        let line = &line?.trim().to_string();

        if line == &"%".to_string() {
            fortunes.push(buffer.clone());
            buffer = vec![];
        } else {
            buffer.push(line.to_string());
        }
    }

    Ok(fortunes)
}
