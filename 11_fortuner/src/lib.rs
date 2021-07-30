use clap::{App, Arg};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    file: String,
    seed: Option<u64>,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("fortuner")
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

    Ok(Config {
        file: matches.value_of("file").unwrap().to_string(),
        seed: parse_int(matches.value_of("seed"))?,
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let fortunes = read_fortunes(&config.file)?;
    println!("{}", pick_fortune(&fortunes, &config.seed));
    Ok(())
}

// --------------------------------------------------
fn parse_int<T: FromStr>(val: Option<&str>) -> MyResult<Option<T>> {
    match val {
        Some(v) => match v.trim().parse::<T>() {
            Ok(n) => Ok(Some(n)),
            Err(_) => Err(From::from(format!("Invalid integer \"{}\"", v))),
        },
        None => Ok(None),
    }
}

// --------------------------------------------------
fn read_fortunes(filename: &str) -> MyResult<Vec<String>> {
    let file = BufReader::new(
        File::open(filename).map_err(|e| format!("{}: {}", filename, e))?,
    );
    let mut fortunes = vec![];
    let mut buffer = vec![];

    for line in file.lines() {
        let line = &line?.trim().to_string();

        if line == "%" {
            if !buffer.is_empty() {
                fortunes.push(buffer.join("\n"));
                buffer.clear();
            }
        } else {
            buffer.push(line.to_string());
        }
    }

    Ok(fortunes)
}

// --------------------------------------------------
fn pick_fortune(fortunes: &[String], seed: &Option<u64>) -> String {
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
    use super::{pick_fortune, read_fortunes};

    #[test]
    fn test_read_fortunes_bad_file() {
        assert!(read_fortunes(&"/file/does/not/exist".to_string()).is_err())
    }

    #[test]
    fn test_read_fortunes_good_file() {
        let fortunes = read_fortunes("tests/inputs/fortunes");
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

            let expected =
                concat!("If you put garbage in a computer nothing ",
            "comes out but garbage. But this garbage, having passed through ",
            "a very expensive machine, is somehow ennobled and none dare ",
            "criticize it.");
            assert_eq!(pick_fortune(&fortunes, &Some(1)), expected);
        }
    }
}
