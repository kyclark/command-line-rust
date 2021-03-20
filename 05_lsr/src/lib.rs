// Cf https://endler.dev/2018/ls/

extern crate chrono;
extern crate clap;
extern crate libc;

use chrono::{DateTime, Local};
use clap::{App, Arg};
use std::error::Error;
use std::fs::{self, Metadata};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

//use std::fs::File;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    long: bool,
    all: bool,
}

#[derive(Debug)]
pub struct FileInfo {
    basename: String,
    path: String,
    metadata: Metadata,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("ls")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust ls")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .default_value(".")
                .min_values(1),
        )
        .arg(
            Arg::with_name("long")
                .value_name("LONG")
                .takes_value(false)
                .help("Long listing")
                .short("-l")
                .long("--long"),
        )
        .arg(
            Arg::with_name("all")
                .value_name("ALL")
                .takes_value(false)
                .help("Show all files")
                .short("-a")
                .long("--all"),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("file").unwrap(),
        long: matches.is_present("long"),
        all: matches.is_present("all"),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let (entries, errors) = find_files(&config.files, &config);

    for error in errors {
        eprintln!("{}", error);
    }

    for entry in entries {
        println!("{}", format_output(&entry, &config)?);
    }

    Ok(())
}

// --------------------------------------------------
fn format_output(file: &FileInfo, config: &Config) -> MyResult<String> {
    // Cf https://man7.org/linux/man-pages/man3/strftime.3.html

    match config.long {
        true => {
            let modified: DateTime<Local> =
                DateTime::from(file.metadata.modified()?);
            Ok(format!(
                "{}{} {:8} {} {}",
                if file.metadata.is_dir() { "d" } else { "-" },
                parse_permissions(file.metadata.permissions().mode() as u16),
                file.metadata.len(),
                modified.format("%b %d %y %H:%M").to_string(),
                file.path.to_string()
            ))
        }
        _ => Ok(file.path.to_string()),
    }
}

// --------------------------------------------------
fn find_files(
    paths: &Vec<String>,
    config: &Config,
) -> (Vec<FileInfo>, Vec<String>) {
    let mut results = vec![];
    let mut errors = vec![];

    for path in paths {
        if let Ok(meta) = fs::metadata(path) {
            if meta.is_file() {
                let path_info = Path::new(path);
                if let Some(basename) = path_info
                    .file_name()
                    .and_then(|e| Some(e.to_string_lossy().to_string()))
                {
                    results.push(FileInfo {
                        basename: basename,
                        path: path.to_string(),
                        metadata: meta,
                    });
                }
            } else if let Ok(entries) = fs::read_dir(path) {
                for entry in entries {
                    let entry = entry.expect("entry");
                    if let Ok(meta) = entry.metadata() {
                        let basename =
                            entry.file_name().to_string_lossy().to_string();
                        let hidden = basename.starts_with('.');
                        if !hidden || (hidden && config.all) {
                            results.push(FileInfo {
                                path: entry.path().display().to_string(),
                                basename: basename,
                                metadata: meta,
                            });
                        }
                    }
                }
            }
        } else {
            errors.push(format!("{}: No such file or directory", path));
        }
    }

    (results, errors)
}

// --------------------------------------------------
fn parse_permissions(mode: u16) -> String {
    let user = show_perm(mode, libc::S_IRUSR, libc::S_IWUSR, libc::S_IXUSR);
    let group = show_perm(mode, libc::S_IRGRP, libc::S_IWGRP, libc::S_IXGRP);
    let other = show_perm(mode, libc::S_IROTH, libc::S_IWOTH, libc::S_IXOTH);
    [user, group, other].join("")
}

// --------------------------------------------------
fn show_perm(mode: u16, read: u16, write: u16, execute: u16) -> String {
    match (mode & read, mode & write, mode & execute) {
        (0, 0, 0) => "---",
        (_, 0, 0) => "r--",
        (0, _, 0) => "-w-",
        (0, 0, _) => "--x",
        (_, 0, _) => "r-x",
        (_, _, 0) => "rw-",
        (0, _, _) => "-wx",
        (_, _, _) => "rwx",
    }
    .to_string()
}

// --------------------------------------------------
#[test]
fn test_show_perm() {
    assert_eq!(
        show_perm(33188u16, libc::S_IRUSR, libc::S_IWUSR, libc::S_IXUSR),
        "rw-"
    );
    assert_eq!(
        show_perm(33188u16, libc::S_IRGRP, libc::S_IWGRP, libc::S_IXGRP),
        "r--"
    );
    assert_eq!(
        show_perm(32768u16, libc::S_IRGRP, libc::S_IWGRP, libc::S_IXGRP),
        "---"
    );
    assert_eq!(
        show_perm(33279u16, libc::S_IROTH, libc::S_IWOTH, libc::S_IXOTH),
        "rwx"
    );
}
