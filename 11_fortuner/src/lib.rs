use clap::{App, Arg};
use rand::{rngs::StdRng, Rng, SeedableRng};
use regex::{Regex, RegexBuilder};
use std::{
    cmp::Ordering,
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

#[derive(Debug, Eq)]
pub struct Fortune {
    source: String,
    text: String,
}

impl Ord for Fortune {
    fn cmp(&self, other: &Self) -> Ordering {
        self.source
            .cmp(&other.text)
            .then(self.text.cmp(&other.text))
    }
}

impl PartialOrd for Fortune {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Fortune {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source && self.text == other.text
    }
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
fn parse_int(val: &str) -> MyResult<u64> {
    match val.trim().parse() {
        Ok(n) => Ok(n),
        Err(_) => Err(From::from(format!("\"{}\" not a valid integer", val))),
    }
}

// --------------------------------------------------
fn read_fortunes(
    paths: &Vec<PathBuf>,
    pattern: &Option<Regex>,
) -> MyResult<Vec<Fortune>> {
    let mut fortunes = vec![];
    let mut buffer = vec![];

    for path in paths {
        let file = File::open(path)
            .map_err(|e| format!("{}: {}", path.display(), e))?;
        let file = BufReader::new(file);

        for line in file.lines() {
            let line = &line?.trim_end().to_string();

            if line == "%" {
                if !buffer.is_empty() {
                    let text = buffer.join("\n");
                    buffer.clear();

                    if pattern.as_ref().map_or(true, |re| re.is_match(&text))
                    {
                        fortunes.push(Fortune {
                            source: path
                                .file_name()
                                .unwrap()
                                .to_string_lossy()
                                .into_owned(),
                            text,
                        });
                    }

                    //if let Some(re) = pattern {
                    //        fortunes.push(Fortune {source, text});
                    //    }
                    //} else {
                    //    fortunes.push(Fortune {source, text});
                    //}
                }
            } else {
                buffer.push(line.to_string());
            }
        }
    }

    fortunes.sort();
    Ok(fortunes)
}

// --------------------------------------------------
fn find_files(sources: &Vec<String>) -> Vec<PathBuf> {
    let mut results = vec![];

    for path in sources {
        match fs::metadata(path) {
            Err(e) => eprintln!("{}: {}", path, e),
            Ok(meta) => {
                if meta.is_file() {
                    results.push(PathBuf::from(path));
                } else if meta.is_dir() {
                    for entry in WalkDir::new(path)
                        .into_iter()
                        .filter_map(|e| e.ok())
                        .filter(|e| e.file_type().is_file())
                    {
                        results.push(entry.path().into());
                    }
                }
            }
        }
    }

    // Exclude ".dat" files
    results
        .into_iter()
        .filter(|path| path.as_path().extension() != Some(OsStr::new("dat")))
        .collect()
}

// --------------------------------------------------
fn pick_fortune(
    fortunes: &Vec<Fortune>,
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
            fortunes.get(i).map(|v| v.text.to_string())
        }
    }
}

// --------------------------------------------------
#[cfg(test)]
mod tests {
    use super::{
        find_files, parse_int, pick_fortune, read_fortunes, Fortune,
    };
    use std::path::PathBuf;

    #[test]
    fn test_parse_int() {
        let res = parse_int("a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "\"a\" not a valid integer");

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
        assert_eq!(
            files.iter().nth(0).unwrap().to_string_lossy(),
            "./tests/inputs/fortunes"
        );

        // Fails to find a bad file
        let input = vec!["/path/does/not/exist".to_string()];
        let files = find_files(&input);
        assert_eq!(files.len(), 0);

        // Finds all the input files, excludes ".dat"
        let input = vec!["./tests/inputs".to_string()];
        let files = find_files(&input);
        assert_eq!(files.len(), 5);
    }

    #[test]
    fn test_read_fortunes() {
        let res = read_fortunes(
            &vec![PathBuf::from("/file/does/not/exist")],
            &None,
        );
        assert!(res.is_err());

        let res = read_fortunes(
            &vec![PathBuf::from("./tests/inputs/fortunes")],
            &None,
        );
        assert!(res.is_ok());

        if let Ok(fortunes) = res {
            assert_eq!(fortunes.len(), 5433);
            let first =
                concat!("\"Contrariwise,\" continued Tweedledee, ",
                "\"if it was so, it might be; and if it were so, it would ",
                "be; but as it isn't, it ain't. That's logic!\"\n",
                "-- Lewis Carroll, \"Through the Looking Glass\""
            );

            assert_eq!(fortunes.iter().nth(0).unwrap().text, first);

            assert_eq!(
                fortunes.last().unwrap().text,
                concat!(
                    "listen: there's a hell of a good universe next door;\n",
                    "let's go.\n",
                    "-- ee cummings"
                )
            );
        }
    }

    #[test]
    fn test_pick_fortune() {
        let fortunes = vec![
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

        assert_eq!(
            pick_fortune(&fortunes, &Some(1)).unwrap(),
            "Neckties strangle clear thinking.".to_string()
        );
    }
}
