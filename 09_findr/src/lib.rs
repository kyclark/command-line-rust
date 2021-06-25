use clap::{App, Arg};
use regex::Regex;
use std::error::Error;
use std::fs;
use walkdir::{DirEntry, WalkDir};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
enum EntryType {
    Any,
    Dir,
    File,
}

#[derive(Debug)]
pub struct Config {
    dirs: Vec<String>,
    entry_type: EntryType,
    names: Option<Vec<Regex>>,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("findr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust find")
        .arg(
            Arg::with_name("dirs")
                .value_name("DIR")
                .help("Search directory")
                .default_value(".")
                .min_values(1),
        )
        .arg(
            Arg::with_name("type")
                .value_name("TYPE")
                .help("Entry type")
                .short("t")
                .long("type")
                .possible_values(&["f", "d"])
                .takes_value(true),
        )
        .arg(
            Arg::with_name("name")
                .value_name("NAME")
                .help("Name")
                .short("n")
                .long("name")
                .takes_value(true)
                .multiple(true),
        )
        .get_matches();

    let mut dirs = vec![];
    if let Some(vals) = matches.values_of_lossy("dirs") {
        for dir in vals {
            match fs::metadata(&dir) {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        dirs.push(dir);
                    } else {
                        return Err(From::from(format!(
                            "\"{}\" is not a directory",
                            &dir
                        )));
                    }
                }
                Err(e) => {
                    return Err(From::from(format!("\"{}\": {}", &dir, e)))
                }
            }
        }
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

    let mut names = vec![];
    if let Some(vals) = matches.values_of_lossy("name") {
        for name in vals {
            match Regex::new(&name) {
                Ok(re) => names.push(re),
                _ => {
                    return Err(From::from(format!(
                        "Invalid --name regex \"{}\"",
                        name
                    )))
                }
            }
        }
    }

    Ok(Config {
        dirs,
        entry_type,
        names: if names.is_empty() { None } else { Some(names) },
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    //println!("{:#?}", &config);

    let name_filter = |entry: &DirEntry| match &config.names {
        Some(names) => names
            .iter()
            .any(|re| re.is_match(&entry.file_name().to_string_lossy())),
        _ => true,
    };

    let type_filter = |entry: &DirEntry| match &config.entry_type {
        &EntryType::Any => true,
        &EntryType::Dir => entry.file_type().is_dir(),
        &EntryType::File => entry.file_type().is_file(),
    };

    for dir in &config.dirs {
        let entries: Vec<String> = WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect();

        println!("{}", entries.join("\n"));
    }

    Ok(())
}
