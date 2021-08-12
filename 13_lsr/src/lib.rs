mod owner;

use chrono::{DateTime, Local};
use clap::{App, Arg};
use owner::Owner;
use std::{
    error::Error,
    fs,
    os::unix::fs::{MetadataExt, PermissionsExt},
    path::PathBuf,
};
use tabular::{Row, Table};
use users::{get_group_by_gid, get_user_by_uid};
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    long: bool,
    show_hidden: bool,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("lsr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust ls")
        .arg(
            Arg::with_name("paths")
                .value_name("PATH")
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
        paths: matches.values_of_lossy("paths").unwrap(),
        long: matches.is_present("long"),
        show_hidden: matches.is_present("all"),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let paths = find_files(&config.paths, config.show_hidden)?;
    //println!("{:?}", paths);
    if config.long {
        println!("{}", format_output(&paths)?);
    } else {
        for path in paths {
            println!("{}", path.display());
        }
    }
    Ok(())
}

// --------------------------------------------------
fn find_files(paths: &[String], show_hidden: bool) -> MyResult<Vec<PathBuf>> {
    let mut results = vec![];
    for path in paths {
        match fs::metadata(path) {
            Err(e) => eprintln!("{}: {}", path, e),
            Ok(meta) => {
                if meta.is_dir() {
                    for entry in WalkDir::new(path).min_depth(1).max_depth(1)
                    {
                        let entry = entry?;
                        let path = entry.path();
                        let is_hidden =
                            path.file_name().map_or(false, |name| {
                                name.to_string_lossy().starts_with(".")
                            });
                        if !is_hidden || show_hidden {
                            results.push(PathBuf::from(entry.path()));
                        }
                    }
                } else {
                    results.push(PathBuf::from(path));
                }
            }
        }
    }
    Ok(results)
}

// --------------------------------------------------
fn format_output(paths: &[PathBuf]) -> MyResult<String> {
    //         1   2     3     4     5     6     7     8
    let fmt = "{:<}{:<}  {:>}  {:<}  {:<}  {:>}  {:<}  {:<}";
    let mut table = Table::new(fmt);

    for path in paths {
        let metadata = path.metadata()?;

        let uid = metadata.uid();
        let user = get_user_by_uid(uid)
            .map(|u| u.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| uid.to_string());

        let gid = metadata.gid();
        let group = get_group_by_gid(gid)
            .map(|g| g.name().to_string_lossy().into_owned())
            .unwrap_or(format!("{}", gid));

        let file_type = if path.is_dir() { "d" } else { "-" };
        let perms = format_mode(metadata.permissions().mode() as u16);
        let modified: DateTime<Local> = DateTime::from(metadata.modified()?);

        table.add_row(
            Row::new()
                .with_cell(file_type) // 1
                .with_cell(perms) // 2
                .with_cell(metadata.nlink()) // 3
                .with_cell(user) // 4
                .with_cell(group) // 5
                .with_cell(metadata.len()) // 6
                .with_cell(modified.format("%b %d %y %H:%M")) // 7
                .with_cell(path.display()), // 8
        );
    }

    Ok(format!("{}", table))
}

// --------------------------------------------------
/// Given a file mode in octal format like 0o751,
/// return a string like "rwxr-x--x"
pub fn format_mode(mode: u16) -> String {
    format!(
        "{}{}{}",
        mk_triple(mode, Owner::User),
        mk_triple(mode, Owner::Group),
        mk_triple(mode, Owner::Other),
    )
}

// --------------------------------------------------
/// Given an octal number like 0o500 and an `Owner`,
/// return a string like "r-x"
pub fn mk_triple(mode: u16, owner: Owner) -> String {
    let [read, write, execute] = owner.masks();
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
    use super::{find_files, format_mode, format_output, mk_triple, Owner};
    use std::{collections::HashSet, path::PathBuf};

    #[test]
    fn test_find_files() {
        let res = find_files(&["tests/inputs".to_string()], false);
        assert!(res.is_ok());

        let paths = res.unwrap();
        assert_eq!(paths.len(), 4);

        let filenames: HashSet<String> = paths
            .iter()
            .map(|path| path.display().to_string())
            .collect();
        let expected: HashSet<String> = [
            "tests/inputs/bustle.txt",
            "tests/inputs/dir",
            "tests/inputs/empty.txt",
            "tests/inputs/fox.txt",
        ]
        .iter()
        .map(|v| v.to_string())
        .collect();
        assert_eq!(filenames, expected);
    }

    #[test]
    fn test_find_files_hidden() {
        let res = find_files(&["tests/inputs".to_string()], true);
        assert!(res.is_ok());

        let paths = res.unwrap();
        assert_eq!(paths.len(), 5);

        let filenames: HashSet<String> = paths
            .iter()
            .map(|path| path.display().to_string())
            .collect();
        let expected: HashSet<String> = [
            "tests/inputs/.hidden",
            "tests/inputs/bustle.txt",
            "tests/inputs/dir",
            "tests/inputs/empty.txt",
            "tests/inputs/fox.txt",
        ]
        .iter()
        .map(|v| v.to_string())
        .collect();
        assert_eq!(filenames, expected);
    }

    fn long_match(
        line: &str,
        path: &str,
        permissions: &str,
        size: Option<&str>,
    ) {
        let parts: Vec<_> = line.split_whitespace().collect();

        if let Some(file_path) = &parts.last() {
            assert_eq!(file_path, &&path);
        }
        if let Some(file_perm) = &parts.get(0) {
            assert_eq!(file_perm, &&permissions);
        }
        if let Some(size) = size {
            if let Some(file_size) = &parts.get(4) {
                assert_eq!(file_size, &&size);
            }
        }
    }

    #[test]
    fn test_format_output_one() {
        let bustle_path = "tests/inputs/bustle.txt";
        let bustle = PathBuf::from(bustle_path);

        let res = format_output(&[bustle]);
        assert!(res.is_ok());

        let out = res.unwrap();
        let lines: Vec<&str> =
            out.split("\n").filter(|s| !s.is_empty()).collect();
        assert_eq!(lines.len(), 1);

        let line1 = lines.first().unwrap();
        long_match(&line1, bustle_path, "-rw-r--r--", Some("193"));
    }

    #[test]
    fn test_format_output_two() {
        let res = format_output(&[
            PathBuf::from("tests/inputs/dir"),
            PathBuf::from("tests/inputs/empty.txt"),
        ]);
        assert!(res.is_ok());

        let out = res.unwrap();
        let mut lines: Vec<&str> =
            out.split("\n").filter(|s| !s.is_empty()).collect();
        lines.sort();
        assert_eq!(lines.len(), 2);

        let empty_line = lines.remove(0);
        long_match(
            &empty_line,
            "tests/inputs/empty.txt",
            "-rw-r--r--",
            Some("0"),
        );

        let dir_line = lines.remove(0);
        long_match(&dir_line, "tests/inputs/dir", "drwxr-xr-x", None);
    }

    #[test]
    fn test_mk_triple() {
        assert_eq!(mk_triple(0o751, Owner::User), "rwx");
        assert_eq!(mk_triple(0o751, Owner::Group), "r-x");
        assert_eq!(mk_triple(0o751, Owner::Other), "--x");
        assert_eq!(mk_triple(0o600, Owner::Other), "---");
    }

    #[test]
    fn test_format_mode() {
        assert_eq!(format_mode(0o755), "rwxr-xr-x");
        assert_eq!(format_mode(0o421), "r---w---x");
    }
}
