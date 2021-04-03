extern crate clap;

use clap::{App, Arg};
use std::error::Error;
use std::fs;
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    dir: String,
    entry_type: String,
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
                .required(true),
        )
        .arg(
            Arg::with_name("type")
                .value_name("TYPE")
                .help("Entry type")
                .short("t")
                .long("type"),
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

    Ok(Config {
        dir: dir,
        entry_type: matches.value_of("type").unwrap().to_string(),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", &config);

    if let Ok(files) = find_files(&config.dir) {
        for file in files {
            println!("{}", file);
        }
    }

    Ok(())
}

// --------------------------------------------------
fn find_files(path: &String) -> MyResult<Vec<String>> {
    let mut files: Vec<String> = vec![];
    let metadata = fs::metadata(&path)?;
    if metadata.is_dir() {
        let walker = WalkDir::new(path).into_iter();
        for entry in walker.filter_map(|e| e.ok()) {
            if &entry.path().display().to_string() != path {
                if let Ok(mut more) =
                    find_files(&entry.path().display().to_string())
                {
                    files.append(&mut more);
                }
            }
            //let meta = fs::metadata(&entry)?;
            //if meta.is_file() {
            //    files.push(entry.path().display().to_string());
            //}
            //else if meta.is_dir() {
            //}
        }
    } else if metadata.is_file() {
        files.push(path.to_string());
    }

    Ok(files)
}
