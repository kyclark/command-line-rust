extern crate clap;
extern crate csv;
extern crate regex;
extern crate tempdir;

use calamine::{open_workbook, Reader, Xlsx};
use clap::{App, Arg};
use csv::WriterBuilder;
use regex::Regex;
use std::error::Error;
use std::fs::{self, DirBuilder};
use std::path::{Path, PathBuf};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    outdir: String,
    delimiter: u8,
    normalize: bool,
    make_dirs: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("excel2txt")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Export Excel workbooks into delimited text files")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("File input")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("outdir")
                .short("o")
                .long("outdir")
                .value_name("DIR")
                .default_value("out")
                .help("Output directory"),
        )
        .arg(
            Arg::with_name("delimiter")
                .short("d")
                .long("delimiter")
                .value_name("DELIM")
                .default_value("\t")
                .help("Delimiter for output files"),
        )
        .arg(
            Arg::with_name("normalize")
                .short("n")
                .long("normalize")
                .help("Normalize headers"),
        )
        .arg(
            Arg::with_name("make_dirs")
                .short("m")
                .long("mkdirs")
                .help("Make output directory for each input file"),
        )
        .get_matches();

    let files = matches.values_of_lossy("file").unwrap();

    let bad: Vec<String> = files.iter().cloned().filter(|f| !is_file(f)).collect();

    if !bad.is_empty() {
        let msg = format!(
            "Invalid file{}: {}",
            if bad.len() == 1 { "" } else { "s" },
            bad.join(", ")
        );
        return Err(From::from(msg));
    }

    Ok(Config {
        files: files,
        outdir: matches.value_of("outdir").unwrap().to_string(),
        delimiter: *matches
            .value_of("delimiter")
            .unwrap()
            .as_bytes()
            .first()
            .unwrap(),
        normalize: matches.is_present("normalize"),
        make_dirs: matches.is_present("make_dirs"),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    for (fnum, file) in config.files.into_iter().enumerate() {
        let path = Path::new(&file);
        let basename = path.file_stem().expect("basename");
        let stem = normalize(&basename.to_string_lossy().to_string());

        println!("{}: {}", fnum + 1, basename.to_string_lossy());

        let mut out_dir = PathBuf::from(&config.outdir);
        if config.make_dirs {
            out_dir.push(&stem)
        }
        if !out_dir.is_dir() {
            DirBuilder::new().recursive(true).create(&out_dir)?;
        }

        let mut excel: Xlsx<_> = open_workbook(file)?;
        let sheets = excel.sheet_names().to_owned();
        for sheet in sheets {
            let ext = if config.delimiter == 44 { "csv" } else { "txt" };
            let out_file = format!("{}__{}.{}", &stem, normalize(&sheet), ext);
            let out_path = &out_dir.join(out_file);
            let mut wtr = WriterBuilder::new()
                .delimiter(config.delimiter)
                .from_path(out_path)?;

            println!("\tSheet '{}' -> '{}'", sheet, out_path.display());
            if let Some(Ok(r)) = excel.worksheet_range(&sheet) {
                for (rnum, row) in r.rows().enumerate() {
                    let vals = row
                        .into_iter()
                        .map(|f| format!("{}", f))
                        .map(|f| if rnum == 0 { normalize(&f) } else { f })
                        .collect::<Vec<String>>();

                    wtr.write_record(&vals)?;
                }
            }
            wtr.flush()?;
        }
    }

    Ok(())
}

// --------------------------------------------------
fn normalize(val: &String) -> String {
    let mut new = val.to_string();
    let camel = Regex::new(r"(.*)([a-z])([A-Z].*)").unwrap();

    // First handle FooBar -> Foo_Bar
    loop {
        if let Some(cap) = camel.captures(&new) {
            new = format!("{}{}_{}", &cap[1], &cap[2], &cap[3]);
        } else {
            break;
        }
    }

    let spaces = Regex::new(r"[\s]+").unwrap();
    let non_alphanum = Regex::new(r"[^a-z0-9_]").unwrap();
    let mult_underbar = Regex::new(r"[_]+").unwrap();

    new = new.to_ascii_lowercase();
    new = spaces.replace_all(&new.to_string(), "_").to_string();
    new = non_alphanum.replace_all(&new.to_string(), "").to_string();
    mult_underbar.replace_all(&new.to_string(), "_").to_string()
}

// --------------------------------------------------
fn is_file(path: &String) -> bool {
    if let Ok(meta) = fs::metadata(path) {
        return meta.is_file();
    } else {
        return false;
    }
}

// --------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempdir::TempDir;

    #[test]
    fn test_normalize() {
        assert_eq!(normalize(&"".to_string()), "");
        assert_eq!(normalize(&"ABC".to_string()), "abc");
        assert_eq!(normalize(&"ABC  DEF".to_string()), "abc_def");
        assert_eq!(normalize(&"foo-b*!a,r".to_string()), "foobar");
        assert_eq!(normalize(&"Foo Bar".to_string()), "foo_bar");
        assert_eq!(normalize(&"Foo / Bar".to_string()), "foo_bar");
        assert_eq!(normalize(&"Foo (Bar)".to_string()), "foo_bar");
        assert_eq!(normalize(&"FooBarBAZ".to_string()), "foo_bar_baz");
    }

    #[test]
    fn test_1() {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = manifest_dir.join(PathBuf::from("tests/test1.xlsx"));
        if let Ok(tmp_dir) = TempDir::new("test") {
            let outdir = &tmp_dir.path().display().to_string();
            let conf = Config {
                files: vec![file.display().to_string()],
                outdir: outdir.to_string(),
                delimiter: 9, // tab
                normalize: false,
                make_dirs: false,
            };

            let _res = match run(conf) {
                Ok(_) => {
                    let expected_dir = PathBuf::from(outdir);
                    assert!(expected_dir.is_dir());
                    let expected_file = expected_dir.join("test1__sheet1.txt");
                    assert!(expected_file.is_file());

                    let contents = fs::read_to_string(expected_file).ok().unwrap();
                    let lines: Vec<&str> = contents.split("\n").collect();
                    assert_eq!(lines[0], "name\trank\tserial_number");
                    assert_eq!(lines[1], "Ed\tCaptain\t12345");
                    assert_eq!(lines[2], "Jorge\tMajor\t98765");
                }
                Err(x) => panic!("{:?}", x),
            };
        }
    }

    #[test]
    fn test_2() {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = manifest_dir.join(PathBuf::from("tests/Test 2.xlsx"));
        if let Ok(tmp_dir) = TempDir::new("test") {
            let outdir = &tmp_dir.path().display().to_string();
            let conf = Config {
                files: vec![file.display().to_string()],
                outdir: outdir.to_string(),
                delimiter: 44, // comma
                normalize: true,
                make_dirs: true,
            };

            let _res = match run(conf) {
                Ok(_) => {
                    let expected_dir = PathBuf::from(tmp_dir.path().join("test_2"));
                    assert!(expected_dir.is_dir());
                    let expected_file = expected_dir.join("test_2__sheet1.csv");
                    assert!(expected_file.is_file());

                    let contents = fs::read_to_string(expected_file).ok().unwrap();
                    let lines: Vec<&str> = contents.split("\n").collect();
                    assert_eq!(lines[0], "ice_cream_flavor,peoples_rank");
                    assert_eq!(lines[1], "chocolate,1");
                    assert_eq!(lines[2], "vanilla,2");
                    assert_eq!(lines[3], "stravberry,3");
                }
                Err(x) => panic!("{:?}", x),
            };
        }
    }
}
