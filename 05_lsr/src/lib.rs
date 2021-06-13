use chrono::{DateTime, Local};
use clap::{App, Arg};
use std::error::Error;
use std::fs::{self, Metadata};
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use users::{get_group_by_gid, get_user_by_uid};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pub entries: Vec<String>,
    pub long: bool,
    pub all: bool,
}

#[derive(Debug)]
pub struct FileInfo {
    pub path: String,
    pub metadata: Metadata,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("lsr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust ls")
        .arg(
            Arg::with_name("entries")
                .value_name("ENTRY")
                .help("Files and/or directories")
                .required(true)
                .default_value(".")
                .min_values(1),
        )
        .arg(
            Arg::with_name("long")
                .value_name("LONG")
                .takes_value(false)
                .help("Long listing")
                .short("l")
                .long("long"),
        )
        .arg(
            Arg::with_name("all")
                .value_name("ALL")
                .takes_value(false)
                .help("Show all files")
                .short("a")
                .long("all"),
        )
        .get_matches();

    Ok(Config {
        entries: matches.values_of_lossy("entries").unwrap(),
        long: matches.is_present("long"),
        all: matches.is_present("all"),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let (entries, errors) = find_files(&config)?;

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
    match config.long {
        true => {
            let modified: DateTime<Local> =
                DateTime::from(file.metadata.modified()?);
            let uid = file.metadata.uid();
            let user = match get_user_by_uid(uid) {
                Some(u) => u.name().to_string_lossy().into_owned(),
                _ => uid.to_string(),
            };
            let gid = file.metadata.gid();
            let group = match get_group_by_gid(gid) {
                Some(g) => g.name().to_string_lossy().into_owned(),
                _ => gid.to_string(),
            };

            Ok(format!(
                "{}{} {:2} {:8} {:8} {:8} {} {}",
                if file.metadata.is_dir() { "d" } else { "-" },
                format_mode(file.metadata.permissions().mode() as u16),
                file.metadata.nlink(),
                user,
                group,
                file.metadata.len(),
                modified.format("%b %d %y %H:%M").to_string(),
                file.path.to_string()
            ))
        }
        _ => Ok(file.path.to_string()),
    }
}

// --------------------------------------------------
fn find_files(config: &Config) -> MyResult<(Vec<FileInfo>, Vec<String>)> {
    let mut results = vec![];
    let mut errors = vec![];

    for path in &config.entries {
        if let Ok(meta) = fs::metadata(path) {
            if meta.is_file() {
                results.push(FileInfo {
                    path: path.to_string(),
                    metadata: meta,
                });
            } else if let Ok(entries) = fs::read_dir(path) {
                for entry in entries {
                    let entry = entry?;
                    if let Ok(meta) = entry.metadata() {
                        let basename =
                            entry.file_name().to_string_lossy().to_string();
                        let hidden = basename.starts_with('.');
                        if !hidden || (hidden && config.all) {
                            results.push(FileInfo {
                                path: entry.path().display().to_string(),
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

    Ok((results, errors))
}

/// Given a file mode in octal format like 0o751,
/// return a string like "rwxr-x--x"
pub fn format_mode(mode: u16) -> String {
    format!(
        "{}{}{}",
        mk_triple(mode, 0o400, 0o200, 0o100),
        mk_triple(mode, 0o040, 0o020, 0o010),
        mk_triple(mode, 0o004, 0o002, 0o001),
    )
}

/// Given an octal number like 0o500 and three mask values,
/// return a string like "r-x"
pub fn mk_triple(mode: u16, read: u16, write: u16, execute: u16) -> String {
    format!(
        "{}{}{}",
        if mode & read == 0 { "-" } else { "r" },
        if mode & write == 0 { "-" } else { "w" },
        if mode & execute == 0 { "-" } else { "x" },
    )
}

// --------------------------------------------------
#[cfg(test)]
mod test {
    use super::{format_mode, mk_triple};

    #[test]
    fn test_mk_triple() {
        assert_eq!(mk_triple(0o751, 0o400, 0o200, 0o100), "rwx");
        assert_eq!(mk_triple(0o751, 0o040, 0o020, 0o010), "r-x");
        assert_eq!(mk_triple(0o751, 0o004, 0o002, 0o001), "--x");
        assert_eq!(mk_triple(0o600, 0o004, 0o002, 0o001), "---");
    }

    #[test]
    fn test_format_mode() {
        assert_eq!(format_mode(0o755), "rwxr-xr-x");
        assert_eq!(format_mode(0o421), "r---w---x");
    }
}
