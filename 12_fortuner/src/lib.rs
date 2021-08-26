use clap::{App, Arg};
use rand::{rngs::StdRng, Rng, SeedableRng};
use regex::{Regex, RegexBuilder};
use std::{
    error::Error,
    ffi::OsStr,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
};
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    sources: Vec<String>,
    pattern: Option<Regex>,
    seed: Option<u64>,
}

#[derive(Debug)]
pub struct Fortune {
    source: String,
    text: String,
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
                .multiple(true)
                .required(true)
                .help("Input files or directories"),
        )
        .arg(
            Arg::with_name("pattern")
                .value_name("PATTERN")
                .short("m")
                .long("pattern")
                .help("Pattern"),
        )
        .arg(
            Arg::with_name("insensitive")
                .short("i")
                .long("insensitive")
                .help("Case-insensitive pattern matching")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("seed")
                .value_name("SEED")
                .short("s")
                .long("seed")
                .help("Random seed"),
        )
        .get_matches();

    let pattern = matches
        .value_of("pattern")
        .map(|val| {
            RegexBuilder::new(val)
                .case_insensitive(matches.is_present("insensitive"))
                .build()
                .map_err(|_| format!("Invalid --pattern \"{}\"", val))
        })
        .transpose()?;

    Ok(Config {
        sources: matches.values_of_lossy("sources").unwrap(),
        seed: matches.value_of("seed").map(parse_u64).transpose()?,
        pattern,
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let files = find_files(&config.sources)?;
    let fortunes = read_fortunes(&files, &config.pattern)?;

    match config.pattern.is_some() {
        true => {
            let mut last_source: Option<String> = None;
            for fortune in fortunes {
                if last_source.map_or(true, |s| s != fortune.source) {
                    eprintln!("({})\n%", fortune.source);
                }
                println!("{}\n%", fortune.text);
                last_source = Some(fortune.source.clone());
            }
        }
        _ => {
            if let Some(fortune) = pick_fortune(&fortunes, &config.seed) {
                println!("{}", fortune);
            }
        }
    };

    Ok(())
}

// --------------------------------------------------
fn parse_u64(val: &str) -> MyResult<u64> {
    val.trim()
        .parse()
        .map_err(|_| format!("\"{}\" not a valid integer", val).into())
}

// --------------------------------------------------
fn read_fortunes(
    paths: &[PathBuf],
    pattern: &Option<Regex>,
) -> MyResult<Vec<Fortune>> {
    let mut fortunes = vec![];
    let mut buffer = vec![];

    let is_match =
        |text: &str| pattern.as_ref().map_or(true, |re| re.is_match(text));

    for path in paths {
        let source = path.file_name().unwrap().to_string_lossy().into_owned();
        for line in BufReader::new(File::open(path)?)
            .lines()
            .filter_map(Result::ok)
            .map(|line| line.trim_end().to_owned())
        {
            if line == "%" {
                if !buffer.is_empty() {
                    let text = buffer.join("\n");
                    buffer.clear();

                    if is_match(&text) {
                        fortunes.push(Fortune {
                            source: source.clone(),
                            text,
                        });
                    }
                }
            } else {
                buffer.push(line.to_string());
            }
        }
    }
    Ok(fortunes)
}

// --------------------------------------------------
fn find_files(sources: &[String]) -> MyResult<Vec<PathBuf>> {
    let dat = OsStr::new("dat");
    let mut results: Vec<PathBuf> = vec![];

    for source in sources {
        fs::metadata(source).map_err(|e| format!("{}: {}", source, e))?;
        results.extend(
            WalkDir::new(source)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_type().is_file()
                        && e.path().extension() != Some(dat)
                })
                .map(|e| Into::into(e.path())),
            //.map(|e| PathBuf::from(e.path())),
            //.map(|e| e.path().into()),
        );
    }
    results.sort();

    Ok(results)
}

// --------------------------------------------------
fn pick_fortune(fortunes: &[Fortune], seed: &Option<u64>) -> Option<String> {
    match fortunes.is_empty() {
        true => None,
        _ => {
            let range = 0..fortunes.len();
            let i: usize = match &seed {
                Some(seed) => StdRng::seed_from_u64(*seed).gen_range(range),
                _ => rand::thread_rng().gen_range(range),
            };
            fortunes.get(i).map(|fortune| fortune.text.to_string())
        }
    }
}

// --------------------------------------------------
#[cfg(test)]
mod tests {
    use super::{
        find_files, parse_u64, pick_fortune, read_fortunes, Fortune,
    };
    use regex::Regex;
    use std::path::PathBuf;

    #[test]
    fn test_parse_u64() {
        let res = parse_u64("a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "\"a\" not a valid integer");

        let res = parse_u64("0");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 0);

        let res = parse_u64("4");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 4);
    }

    #[test]
    fn test_find_files() {
        // Verify that the function finds a file known to exist
        let res = find_files(&["./tests/inputs/fortunes".to_string()]);
        assert!(res.is_ok());

        let files = res.unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(
            files.get(0).unwrap().to_string_lossy(),
            "./tests/inputs/fortunes"
        );

        // Fails to find a bad file
        let res = find_files(&["/path/does/not/exist".to_string()]);
        assert!(res.is_err());

        // Finds all the input files, excludes ".dat"
        let res = find_files(&["./tests/inputs".to_string()]);
        assert!(res.is_ok());

        // Check number and order of files
        let files = res.unwrap();
        assert_eq!(files.len(), 5);
        let first = files.get(0).unwrap().display().to_string();
        assert!(first.contains("ascii-art"));
        let last = files.last().unwrap().display().to_string();
        assert!(last.contains("startrek"));
    }

    #[test]
    fn test_read_fortunes() {
        // Parses all the fortunes without a filter
        let res =
            read_fortunes(&[PathBuf::from("./tests/inputs/fortunes")], &None);
        assert!(res.is_ok());

        if let Ok(fortunes) = res {
            // Correct number and sorting
            assert_eq!(fortunes.len(), 5433);
            assert_eq!(
                fortunes.iter().nth(0).unwrap().text,
                "You cannot achieve the impossible without \
                attempting the absurd."
            );
            assert_eq!(
                fortunes.last().unwrap().text,
                "There is no material safety data sheet for \
                astatine. If there were, it would just be the word \
                \"NO\" scrawled over and over in charred blood.\n\
                -- Randall Munroe, \"What If?\""
            );
        }

        // Filters for matching text
        let res = read_fortunes(
            &[PathBuf::from("./tests/inputs/fortunes")],
            &Some(Regex::new("Yogi Berra").unwrap()),
        );
        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 2);
    }

    #[test]
    fn test_pick_fortune() {
        // Create a vector of fortunes
        let fortunes = &[
            Fortune {
                source: "fortunes".to_string(),
                text: "You cannot achieve the impossible without \
                      attempting the absurd."
                    .to_string(),
            },
            Fortune {
                source: "fortunes".to_string(),
                text: "Assumption is the mother of all screw-ups."
                    .to_string(),
            },
            Fortune {
                source: "fortunes".to_string(),
                text: "Neckties strangle clear thinking.".to_string(),
            },
        ];

        // Pick a fortune with a seed
        assert_eq!(
            pick_fortune(fortunes, &Some(1)).unwrap(),
            "Neckties strangle clear thinking.".to_string()
        );
    }
}
