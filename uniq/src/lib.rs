extern crate clap;

use clap::{App, Arg};
//use itertools::Itertools;
//use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    file: Option<String>,
    count: bool,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniq")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust uniq")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file(s)"),
        )
        .arg(
            Arg::with_name("count")
                .value_name("COUNT")
                .help("Show counts")
                .short("c")
                .long("counts")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        file: matches.value_of("file").and_then(|v| Some(v.to_string())),
        count: matches.is_present("count"),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    println!("config {:?}", &config);
    let file: Box<dyn BufRead> = match &config.file {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => {
            Box::new(BufReader::new(File::open(filename).unwrap()))
        }
    };

    let lines = fs::read_to_string(&config.file)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse())
        .collect();
    println!("{:?}", lines);

    //let lines: Vec<&str> = file.lines().map(|e| e.ok()).collect();
    //let lines = io::BufReader::new(file).lines();
    //for line in lines {
    //    let line = line?;
    //    println!("{}", line);
    //}

    Ok(())
}

//let mut prev = "".to_string();
//let mut count = 0;
//for line in file.lines() {
//    let line = line?;
//    let current = line.trim().to_string();

//    if &prev == &current {
//        //count = count + 1;
//        count += 1;
//    } else {
//        if config.count {
//            println!("{:5} {}", &count, &current);
//        } else {
//            println!("{}", &current);
//        }
//        prev = current.to_string();
//        count = 1;
//    }
//}

//for (line1, line2) in file.lines().into_iter().tuples() {
//    println!("line1 {:?}", line1);
//    println!("line2 {:?}", line2);
//}

//let mut prev: String = "".to_string();
////let mut last: Option<String> = None;
////let mut curr = "".to_string();
//let mut count = 0;
//let mut lines = file.lines();
//while let Some(line) = &lines.next() {
//    if let Ok(val) = line {
//        let val = val.to_string();
//        let new_group = prev.len() == 0 || prev != val;
//        //println!("line {:?}", val);
//        //println!("prev {:?}", prev);
//        //println!("new  {:?}", new_group);
//        //println!("--");
//        if new_group {
//            let (show, show_count) = if prev.len() == 0 {
//                (&val, 1)
//            } else {
//                (&prev, count)
//            };
//            if config.count {
//                println!("{:5} {}", &show_count, &show);
//            } else {
//                println!("{}", &show);
//            }
//            prev = val.to_string();
//            count = 0;
//        } else {
//            count += 1;
//        }
//    }
//}

//if config.count {
//    println!("{:5} {}", &count, &prev);
//} else {
//    println!("{}", &prev);
//}

//let mut prev: Option<String> = None;
//let mut count = 0;
//for line in file.lines() {
//    let line = line?;
//    let current = Some(line.trim().to_string());

//    println!("prev {:?}", &prev);
//    println!("curr {:?}", &current);

//    if Some(&prev) == Some(&current) {
//        count = count + 1;
//    } else {
//        if config.count {
//            println!("{} {:?}", &count, &current);
//        } else {
//            println!("{:?}", &current);
//        }
//        prev = Some(current.unwrap().to_string());
//        count = 1;
//    }
//}

//let mut seen: HashMap<String, u32> = HashMap::new();
//for line in file.lines() {
//    let line = line?;
//    let count = seen.entry(line.trim().to_string()).or_insert(1);
//    *count += 1;
//}
//println!("seen {:?}", &seen);
