extern crate clap;

use chrono::prelude::*;
use chrono::{Datelike, NaiveDate};
use clap::{App, Arg};
use itertools::izip;
use std::error::Error;
use std::str::FromStr;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    month: Option<u32>,
    year: i32,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("calr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cal")
        .arg(
            Arg::with_name("month")
                .value_name("MONTH")
                .short("m")
                .help("Month number or name"),
        )
        .arg(Arg::with_name("year").value_name("YEAR").help("Year"))
        .get_matches();

    let local: DateTime<Local> = Local::now();

    let mut month = match matches.value_of("month") {
        Some(m) => {
            let month = parse_month(m)?;
            Some(month)
        }
        _ => None,
    };

    let mut year = match matches.value_of("year") {
        Some(y) => {
            let year: i32 = parse_int(y)?;
            if year < 1 || year > 9999 {
                return Err(From::from(format!(
                    "year \"{}\" not in the range 1..9999",
                    y
                )));
            }
            Some(year)
        }
        _ => None,
    };

    if month.is_none() && year.is_none() {
        month = Some(local.month());
        year = Some(local.year());
    }

    Ok(Config {
        month: month,
        year: year.unwrap(),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let month_nums: Vec<u32> = match config.month {
        Some(m) => vec![m],
        _ => (1..=12).collect(),
    };
    let show_year = month_nums.len() < 12;
    let months: Vec<Vec<String>> = month_nums
        .iter()
        .map(|month| format_month(config.year, *month, show_year))
        .collect();

    if !show_year {
        println!("{:32}", config.year);
    }

    let default = " ".repeat(22);
    let wanted = 8;
    for chunk in months.chunks(3) {
        match chunk {
            [m1] => println!("{}\n{}", m1.join("\n"), default),
            [m1, m2, m3] => {
                let mut block1 = m1.to_vec();
                let mut block2 = m2.to_vec();
                let mut block3 = m3.to_vec();
                while block1.len() < wanted {
                    block1.push(default.clone());
                }
                while block2.len() < wanted {
                    block2.push(default.clone());
                }
                while block3.len() < wanted {
                    block3.push(default.clone());
                }

                for lines in izip!(block1, block2, block3) {
                    println!("{}{}{}", lines.0, lines.1, lines.2);
                }
                println!("");
            }
            _ => {}
        };
    }

    Ok(())
}

// --------------------------------------------------
fn parse_month(month: &str) -> MyResult<u32> {
    let names = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    if let Ok(num) = parse_int(&month) {
        if num > 0 && num < 13 {
            return Ok(num);
        } else {
            return Err(From::from(format!("Invalid month \"{}\"", num)));
        }
    }

    let lower = &month.to_lowercase();
    let matches: Vec<usize> = names
        .iter()
        .enumerate()
        .filter_map(|(i, name)| {
            if name.to_lowercase().starts_with(lower) {
                Some(i + 1)
            } else {
                None
            }
        })
        .collect();

    if matches.len() == 1 {
        Ok(matches[0] as u32)
    } else {
        Err(From::from(format!("Unknown month \"{}\"", month)))
    }
}

// --------------------------------------------------
#[test]
fn test_parse_month() {
    let one = parse_month("1");
    assert!(one.is_ok());
    if let Ok(val) = one {
        assert_eq!(val, 1);
    }

    let twelve = parse_month("12");
    assert!(twelve.is_ok());
    if let Ok(val) = twelve {
        assert_eq!(val, 12);
    }

    let zero = parse_month("0");
    assert!(zero.is_err());

    let thirteen = parse_month("13");
    assert!(thirteen.is_err());

    let jan = parse_month("jan");
    assert!(jan.is_ok());
    if let Ok(val) = jan {
        assert_eq!(val, 1);
    }

    let bad = parse_month("foo");
    assert!(bad.is_err());
}

// --------------------------------------------------
fn parse_int<T: FromStr>(val: &str) -> MyResult<T> {
    val.trim()
        .parse::<T>()
        .or(Err(From::from(format!("\"{}\" is not an integer", val))))
}

// --------------------------------------------------
#[test]
fn test_parse_int() {
    let one = parse_int::<usize>("1");
    assert!(one.is_ok());

    if let Ok(val) = one {
        assert_eq!(val, 1);
    }

    let bad = parse_int::<usize>("foo");
    assert!(bad.is_err());
}

// --------------------------------------------------
fn last_day_in_month(year: i32, month: u32) -> NaiveDate {
    // the first day of the next month...
    let (y, m) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    NaiveDate::from_ymd(y, m, 1).pred()
}

// --------------------------------------------------
fn format_month(year: i32, month: u32, print_year: bool) -> Vec<String> {
    let names = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let first = NaiveDate::from_ymd(year, month, 1);
    let last = last_day_in_month(year, month);
    let mut days: Vec<String> = (1..first.weekday().number_from_sunday())
        .collect::<Vec<u32>>()
        .iter()
        .map(|_| "  ".to_string())
        .collect();
    let nums: Vec<String> = (first.day()..=last.day())
        .collect::<Vec<u32>>()
        .iter()
        .map(|d| format!("{:2}", d))
        .collect();
    days.extend(nums);

    let mut lines: Vec<String> = vec![];
    if let Some(month_name) = names.iter().nth(month as usize - 1) {
        lines.push(format!(
            "{:^20}  ",
            if print_year {
                format!("{} {}", month_name, year)
            } else {
                month_name.to_string()
            }
        ));
        lines.push(format!("Su Mo Tu We Th Fr Sa  "));

        for week in days.chunks(7) {
            lines.push(format!("{:20}  ", week.join(" ")));
        }
    }

    lines
}

// --------------------------------------------------
//fn paste(vals: Vec<Vec<String>>, sep: &str) -> String {
//    let mut ret: Vec<String> = vec![];
//    println!("{:?}", multizip(*vals));

//    return "foo".to_string();
//}
