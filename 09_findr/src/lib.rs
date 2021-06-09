use clap::{App, Arg};
use regex::Regex;
use std::error::Error;
use std::fs;
use walkdir::{DirEntry, WalkDir};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
enum EntryType {
    Any,
    Dir,
    File,
}

#[derive(Debug)]
pub struct Config {
    dir: String,
    entry_type: EntryType,
    name: Option<Regex>,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("find")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust find")
        .arg(
            Arg::with_name("dir")
                .value_name("DIR")
                .help("Search directory")
                .default_value("."),
        )
        .arg(
            Arg::with_name("type")
                .value_name("TYPE")
                .help("Entry type")
                .short("t")
                .long("type")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("name")
                .value_name("NAME")
                .help("Name")
                .short("n")
                .long("name")
                .takes_value(true),
        )
        .get_matches();

    let dir = matches.value_of("dir").unwrap().to_string();
    match fs::metadata(&dir) {
        Ok(metadata) => {
            if !metadata.is_dir() {
                return Err(From::from(format!(
                    "\"{}\" is not a directory",
                    &dir
                )));
            }
        }
        Err(e) => return Err(From::from(format!("\"{}\": {}", &dir, e))),
    }

    let entry_type: EntryType = match matches.value_of("type") {
        Some(val) => match val {
            "f" => EntryType::File,
            "d" => EntryType::Dir,
            _ => {
                return Err(From::from(format!("Unknown --type \"{}\"", val)))
            }
        },
        _ => EntryType::Any,
    };

    let name = match matches.value_of("name") {
        Some(pattern) => match Regex::new(pattern) {
            Ok(re) => Some(re),
            _ => {
                return Err(From::from(format!(
                    "Invalid --name regex \"{}\"",
                    pattern
                )))
            }
        },
        _ => None,
    };

    Ok(Config {
        dir,
        entry_type,
        name,
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    //println!("{:?}", &config);

    let name_filter = |entry: &DirEntry| match &config.name {
        Some(re) => re.is_match(&entry.path().display().to_string()),
        _ => true,
    };

    let type_filter = |entry: &DirEntry| {
        config.entry_type == EntryType::Any
            || (entry.file_type().is_dir()
                && config.entry_type == EntryType::Dir)
            || (entry.file_type().is_file()
                && config.entry_type == EntryType::File)
    };

    let entries: Vec<String> = WalkDir::new(&config.dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(type_filter)
        .filter(name_filter)
        .map(|entry| entry.path().display().to_string())
        .collect();

    println!("{}", entries.join("\n"));

    Ok(())
}
