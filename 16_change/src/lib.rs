extern crate clap;

pub mod coin;

use clap::{App, Arg};
use coin::Coin;
//use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    amount: u32,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("change")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("First Bank of Change")
        .arg(
            Arg::with_name("amount")
                .value_name("INT")
                .help("Change amount between 1 and 100 cents")
                .required(true),
        )
        .get_matches();

    let amount: u32 = parse_int(matches.value_of("amount").unwrap())?;

    if amount < 1 || amount > 100 {
        Err(From::from(format!(
            "Amount \"{}\" must be between 1 and 100",
            amount
        )))
    } else {
        Ok(Config { amount })
    }
}

// --------------------------------------------------
fn parse_int<T: FromStr>(val: &str) -> MyResult<T> {
    val.trim()
        .parse::<T>()
        .or(Err(From::from(format!("\"{}\" is not an integer", val))))
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let amount = config.amount;
    let pennies: Vec<Coin> = (0..=amount).map(Coin::Penny).collect();
    let nickels: Vec<Coin> = (0..=(amount / 5)).map(Coin::Nickel).collect();
    let dimes: Vec<Coin> = (0..=(amount / 10)).map(Coin::Dime).collect();
    let quarters: Vec<Coin> =
        (0..=(amount / 25)).map(Coin::Quarter).collect();

    // Manually create Cartesian product
    let mut combos: Vec<Vec<Coin>> = vec![];
    for p in &pennies {
        for n in &nickels {
            for d in &dimes {
                for q in &quarters {
                    let sum = p.value() + n.value() + d.value() + q.value();
                    if sum == config.amount {
                        combos.push(vec![
                            p.clone(),
                            n.clone(),
                            d.clone(),
                            q.clone(),
                        ]);
                    }
                }
            }
        }
    }

    //// Use clever function
    //let combos = cartesian_product(&vec![pennies, nickels, dimes, quarters]);
    //let values: Vec<u32> = combos
    //    .iter()
    //    .map(|combo| combo.iter().map(|c| c.value()).sum())
    //    .collect();

    //let combos: Vec<Vec<Coin>> = combos
    //    .into_iter()
    //    .zip(values)
    //    .into_iter()
    //    .filter_map(
    //        |(combo, sum)| if sum == amount { Some(combo) } else { None },
    //    )
    //    .collect();

    combos.sort();
    if combos.len() > 0 {
        println!(
            "For {} cent{}, I can give you the following:",
            amount,
            if amount == 1 { "" } else { "s" }
        );

        for combo in combos {
            let mut coins: Vec<Coin> =
                combo.into_iter().filter(|c| c.value() > 0).collect();
            coins.sort();

            println!(
                "* {}",
                coins
                    .iter()
                    .map(|c| format!("{}", c))
                    .collect::<Vec<String>>()
                    .join(", ")
            );
        }
    } else {
        println!("I cannot make change for {} cents", amount);
    }

    Ok(())
}

// --------------------------------------------------
fn cartesian_product<T: Copy>(lists: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut res = vec![];

    let mut list_iter = lists.iter();
    if let Some(first_list) = list_iter.next() {
        for &i in first_list {
            res.push(vec![i]);
        }
    }

    for l in list_iter {
        let mut tmp = vec![];
        for r in res {
            for &el in l {
                let mut tmp_el = r.clone();
                tmp_el.push(el);
                tmp.push(tmp_el);
            }
        }
        res = tmp;
    }
    res
}

//// --------------------------------------------------
//fn display_coin(coin: Coin) -> Option<String> {
//    match coin {
//        Coin::Penny(val) => match val {
//            0 => None,
//            1 => Some(format!("{} penny", val)),
//            _ => Some(format!("{} pennies", val)),
//        },
//        Coin::Nickel(val) => match val {
//            0 => None,
//            1 => Some(format!("{} nickel", val)),
//            _ => Some(format!("{} nickels", val)),
//        },
//        Coin::Dime(val) => match val {
//            0 => None,
//            1 => Some(format!("{} dime", val)),
//            _ => Some(format!("{} dimes", val)),
//        },
//        Coin::Quarter(val) => match val {
//            0 => None,
//            1 => Some(format!("{} quarter", val)),
//            _ => Some(format!("{} quarters", val)),
//        },
//    }
//}

//// --------------------------------------------------
//#[test]
//fn test_format() {
//    assert!(display_coin(Coin::Penny(0)).is_none());
//    if let Some(p1) = display_coin(Coin::Penny(1)) {
//        assert_eq!(p1, "1 penny".to_string());
//    }

//    if let Some(p2) = display_coin(Coin::Penny(2)) {
//        assert_eq!(p2, "2 pennies".to_string());
//    }
//}
