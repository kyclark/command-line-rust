use anyhow::{anyhow, Result};
use chrono::prelude::*;
use chrono::{Datelike, NaiveDate};
use clap::{App, Arg};
use colorize::AnsiColor;
use itertools::izip;
use std::str::FromStr;

#[derive(Debug)]
pub struct Config {
    month: Option<u32>,
    year: i32,
}

static MONTH_NAMES: [&str; 12] = [
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

// --------------------------------------------------
pub fn get_args() -> Result<Config> {
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
                return Err(anyhow!(
                    "year \"{}\" not in the range 1..9999",
                    y
                ));
            }
            Some(year)
        }
        _ => None,
    };

    let today = Utc::today();
    if month.is_none() && year.is_none() {
        month = Some(today.month());
        year = Some(today.year());
    }

    Ok(Config {
        month: month,
        year: year.unwrap_or(today.year()),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> Result<()> {
    let month_nums: Vec<u32> = match config.month {
        Some(m) => vec![m],
        _ => (1..=12).collect(),
    };
    let show_year = month_nums.len() < 12;
    let today = Utc::today().naive_local();
    let months: Vec<Vec<String>> = month_nums
        .iter()
        .map(|month| format_month(config.year, *month, show_year, today))
        .collect();

    if !show_year {
        println!("{:32}", config.year);
    }

    for (i, chunk) in months.chunks(3).enumerate() {
        match chunk {
            [m1] => println!("{}", m1.join("\n")),
            [m1, m2, m3] => {
                for lines in izip!(m1, m2, m3) {
                    println!("{}{}{}", lines.0, lines.1, lines.2);
                }
                if i < 3 {
                    println!("");
                }
            }
            _ => {}
        };
    }

    Ok(())
}

// --------------------------------------------------
fn parse_month(month: &str) -> Result<u32> {
    if let Ok(num) = parse_int(&month) {
        if num > 0 && num < 13 {
            return Ok(num);
        } else {
            return Err(anyhow!("Invalid month \"{}\"", num));
        }
    }

    let lower = &month.to_lowercase();
    let matches: Vec<usize> = MONTH_NAMES
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
        Err(anyhow!("Unknown month \"{}\"", month))
    }
}

// --------------------------------------------------
fn parse_int<T: FromStr>(val: &str) -> Result<T> {
    val.trim()
        .parse::<T>()
        .or(Err(anyhow!("\"{}\" is not an integer", val)))
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
fn format_month(
    year: i32,
    month: u32,
    print_year: bool,
    today: NaiveDate, // Date<Utc>,
) -> Vec<String> {
    let first = NaiveDate::from_ymd(year, month, 1);
    let last = last_day_in_month(year, month);
    let mut days: Vec<String> = (1..first.weekday().number_from_sunday())
        .collect::<Vec<u32>>()
        .iter()
        .map(|_| "  ".to_string())
        .collect();

    let is_today = |n: &u32| {
        year == today.year() && month == today.month() && *n == today.day()
    };

    let placeholder = "XX";
    let nums: Vec<String> = (first.day()..=last.day())
        .collect::<Vec<u32>>()
        .iter()
        .map(|num| {
            let day = format!("{:2}", num);
            if is_today(num) {
                placeholder.to_string()
            } else {
                day
            }
        })
        .collect();
    days.extend(nums);

    let width = 22;
    let mut lines: Vec<String> = vec![];

    if let Some(month_name) = MONTH_NAMES.iter().nth(month as usize - 1) {
        lines.push(format!(
            "{:^20}  ",
            if print_year {
                format!("{} {}", month_name, year)
            } else {
                month_name.to_string()
            }
        ));
        lines.push(format!(
            "{:width$}",
            "Su Mo Tu We Th Fr Sa",
            width = width
        ));

        for week in days.chunks(7) {
            let mut disp =
                format!("{:width$}", week.join(" "), width = width);

            if disp.contains(&placeholder) {
                disp = disp.replace(
                    &placeholder,
                    &format!("{:2}", today.day()).reverse(),
                );
            }
            lines.push(disp);
        }
    }

    while lines.len() < 8 {
        lines.push(" ".repeat(width));
    }

    lines
}

// --------------------------------------------------
#[cfg(test)]
mod tests {
    use super::{format_month, last_day_in_month, parse_int, parse_month};
    use chrono::{NaiveDate, Utc};

    #[test]
    fn test_format_month() {
        let today = Utc::today().naive_local();
        let april = vec![
            "     April 2020       ",
            "Su Mo Tu We Th Fr Sa  ",
            "          1  2  3  4  ",
            " 5  6  7  8  9 10 11  ",
            "12 13 14 15 16 17 18  ",
            "19 20 21 22 23 24 25  ",
            "26 27 28 29 30        ",
            "                      ",
        ];
        assert_eq!(format_month(2020, 4, true, today), april);

        let may = vec![
            "      May 2020        ",
            "Su Mo Tu We Th Fr Sa  ",
            "                1  2  ",
            " 3  4  5  6  7  8  9  ",
            "10 11 12 13 14 15 16  ",
            "17 18 19 20 21 22 23  ",
            "24 25 26 27 28 29 30  ",
            "31                    ",
        ];
        assert_eq!(format_month(2020, 5, true, today), may);

        let april_hl = vec![
            "     April 2021       ",
            "Su Mo Tu We Th Fr Sa  ",
            "             1  2  3  ",
            " 4  5  6 \u{1b}[7m 7\u{1b}[0;39;49m  8  9 10  ",
            "11 12 13 14 15 16 17  ",
            "18 19 20 21 22 23 24  ",
            "25 26 27 28 29 30     ",
            "                      ",
        ];
        let today2 = NaiveDate::from_ymd(2021, 4, 7);
        assert_eq!(format_month(2021, 4, true, today2), april_hl);
    }

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

    #[test]
    fn test_last_day_in_month() {
        assert_eq!(
            last_day_in_month(2020, 1),
            NaiveDate::from_ymd(2020, 1, 31)
        );
        assert_eq!(
            last_day_in_month(2020, 2),
            NaiveDate::from_ymd(2020, 2, 29)
        );
        assert_eq!(
            last_day_in_month(2020, 4),
            NaiveDate::from_ymd(2020, 4, 30)
        );
    }
}
