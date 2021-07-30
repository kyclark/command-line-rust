use clap::{App, Arg};
use rand::{rngs::StdRng, Rng, SeedableRng};
use regex::{Regex, RegexBuilder};
use std::{
    error::Error,
    fs::{self, File},
    io::{BufRead, BufReader},
};
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    sources: Vec<String>,
    pattern: Option<Regex>,
    seed: Option<u64>,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("fortuner")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust fortune")
        .arg(
            Arg::with_name("sources")
                .value_name("FILE")
                .help("Input files or directories")
                .min_values(1)
                .required(true),
        )
        .arg(
            Arg::with_name("pattern")
                .value_name("PATTERN")
                .help("Pattern")
                .short("m")
                .long("pattern"),
        )
        .arg(
            Arg::with_name("insensitive")
                .value_name("INSENSITIVE")
                .help("Case-insensitive pattern matching")
                .takes_value(false)
                .short("i")
                .long("insensitive"),
        )
        .arg(
            Arg::with_name("seed")
                .value_name("SEED")
                .help("Random seed")
                .short("s")
                .long("seed"),
        )
        .get_matches();

    let insensitive = matches.is_present("insensitive");
    let pattern = matches
        .value_of("pattern")
        .map(|val| {
            RegexBuilder::new(val)
                .case_insensitive(insensitive)
                .build()
                .map_err(|_| format!("Invalid --pattern \"{}\"", val))
        })
        .transpose()?;

    Ok(Config {
        sources: matches.values_of_lossy("sources").unwrap(),
        seed: matches.value_of("seed").map(parse_int).transpose()?,
        pattern,
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    // println!("{:#?}", config);

    let files = find_files(&config.sources);
    let fortunes = read_fortunes(&files, &config.pattern)?;
    if let Some(fortune) = pick_fortune(&fortunes, &config.seed) {
        println!("{}", fortune);
    }

    Ok(())
}

// --------------------------------------------------
fn parse_int(val: &str) -> MyResult<u64> {
    match val.trim().parse() {
        Ok(n) => Ok(n),
        Err(_) => Err(From::from(format!("\"{}\" not a valid integer", val))),
    }
}

// --------------------------------------------------
fn read_fortunes(
    filenames: &Vec<String>,
    pattern: &Option<Regex>,
) -> MyResult<Vec<String>> {
    let mut fortunes = vec![];
    let mut buffer = vec![];

    for filename in filenames {
        match File::open(filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(file) => {
                let file = BufReader::new(file);
                for line in file.lines() {
                    let line = &line?.trim().to_string();

                    if line == "%" {
                        if !buffer.is_empty() {
                            let fortune = buffer.join("\n");
                            buffer.clear();

                            if pattern
                                .as_ref()
                                .map_or(true, |re| re.is_match(&fortune))
                            {
                                fortunes.push(fortune);
                            }

                            //if let Some(re) = pattern {
                            //        fortunes.push(fortune);
                            //    }
                            //} else {
                            //    fortunes.push(fortune);
                            //}
                        }
                    } else {
                        buffer.push(line.to_string());
                    }
                }
            }
        }
    }

    Ok(fortunes)
}

// --------------------------------------------------
fn find_files(sources: &Vec<String>) -> Vec<String> {
    let mut results = vec![];

    for path in sources {
        match fs::metadata(path) {
            Err(e) => eprintln!("{}: {}", path, e),
            Ok(meta) => {
                if meta.is_file() {
                    results.push(path.to_owned());
                } else if meta.is_dir() {
                    for entry in WalkDir::new(path)
                        .into_iter()
                        .filter_map(|e| e.ok())
                        .filter(|e| e.file_type().is_file())
                    {
                        results.push(entry.path().display().to_string());
                    }
                }
            }
        }
    }

    results
}

// --------------------------------------------------
fn pick_fortune(
    fortunes: &Vec<String>,
    seed: &Option<u64>,
) -> Option<String> {
    match fortunes.is_empty() {
        true => None,
        _ => {
            let range = 0..fortunes.len();
            let i: usize = match &seed {
                Some(seed) => StdRng::seed_from_u64(*seed).gen_range(range),
                _ => rand::thread_rng().gen_range(range),
            };
            fortunes.get(i).map(|v| v.to_string())
        }
    }
}

// --------------------------------------------------
#[cfg(test)]
mod tests {
    use super::{find_files, parse_int, pick_fortune, read_fortunes};

    #[test]
    fn test_parse_int() {
        let res = parse_int("a");
        assert!(res.is_err());

        let res = parse_int("0");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 0);

        let res = parse_int("4");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 4);
    }

    #[test]
    fn test_find_files() {
        // Verify that the function finds a file known to exist
        let input = vec!["./tests/inputs/fortunes".to_string()];
        let files = find_files(&input);
        assert_eq!(files.len(), 1);
        assert_eq!(files.iter().nth(0).unwrap(), &"./tests/inputs/fortunes");

        let input = vec!["./tests/inputs".to_string()];
        let files = find_files(&input);
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_read_fortunes() {
        let res = read_fortunes(&vec!["/file/does/not/exist"]);
        assert!(res.is_ok());

        let res = read_fortunes(&vec!["tests/inputs/fortunes"]);
        assert!(res.is_ok());

        if let Ok(fortunes) = res {
            assert_eq!(fortunes.len(), 5437);
            assert_eq!(
                fortunes[0],
                "You cannot achieve the impossible \
                without attempting the absurd."
            );

            assert_eq!(
                fortunes.last().unwrap(),
                "There is no material safety data sheet for \
                astatine. If there were, it would just be the word \"NO\" \
                scrawled over and over in charred blood.\n\
                -- Randall Munroe, \"What If?\""
            );

            assert_eq!(
                pick_fortune(&fortunes, &Some(1)),
                Some(
                    "If you put garbage in a computer nothing \
                    comes out but garbage. But this garbage, having \
                    passed through a very expensive machine, is somehow \
                    ennobled and none dare criticize it."
                        .to_string()
                )
            );
        }
    }
}
