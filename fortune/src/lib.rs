extern crate clap;
extern crate rand;

use clap::{App, Arg};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type MyResult<T> = Result<T, Box<dyn Error>>;
type Fortune = String;
type Fortunes = Vec<Fortune>;

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
    println!("{}", pick_fortune(&fortunes, &config.seed));
    Ok(())
}

// --------------------------------------------------
fn read_fortunes(filename: &String) -> MyResult<Fortunes> {
    let file = File::open(filename)?;
    let file = BufReader::new(file);
    let mut fortunes: Vec<String> = vec![];
    let mut buffer: Vec<String> = vec![];

    for line in file.lines() {
        let line = &line?.trim().to_string();

        if line == &"%".to_string() {
            fortunes.push(buffer.join("\n"));
            buffer = vec![];
        } else {
            buffer.push(line.to_string());
        }
    }

    Ok(fortunes)
}

// --------------------------------------------------
fn pick_fortune(fortunes: &Fortunes, seed: &Option<u64>) -> Fortune {
    let range = 0..fortunes.len();
    let i: usize = match &seed {
        Some(seed) => StdRng::seed_from_u64(*seed).gen_range(range),
        _ => rand::thread_rng().gen_range(range),
    };
    fortunes[i].to_string()
}

// --------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_read_fortunes_bad_file() {
        assert!(read_fortunes(&"/file/does/not/exist".to_string()).is_err())
    }

    #[test]
    fn test_read_fortunes_good_file() {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = manifest_dir.join(PathBuf::from("tests/inputs/fortunes"));
        let fortunes = read_fortunes(&file.to_str().unwrap().to_string());
        assert!(fortunes.is_ok());

        if let Ok(fortunes) = fortunes {
            assert_eq!(fortunes.len(), 5437);
            let first = concat!(
                "You cannot achieve the impossible without ",
                "attempting the absurd."
            );
            assert_eq!(fortunes[0], first);

            let last =
                concat!("There is no material safety data sheet for ",
                "astatine. If there were, it would just be the word \"NO\" ",
                "scrawled over and over in charred blood.\n",
                "-- Randall Munroe, \"What If?\"");
            assert_eq!(fortunes.last().unwrap(), last);

            let expected = concat!("If you put garbage in a computer nothing ",
            "comes out but garbage. But this garbage, having passed through ",
            "a very expensive machine, is somehow ennobled and none dare ",
            "criticize it.");
            assert_eq!(pick_fortune(&fortunes, &Some(1)), expected);
        }
    }
}
