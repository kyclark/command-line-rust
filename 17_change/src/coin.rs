use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Copy, Clone, Eq)]
pub enum Coin {
    Penny(u32),
    Nickel(u32),
    Dime(u32),
    Quarter(u32),
}

impl fmt::Display for Coin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match *self {
            Coin::Penny(val) => match val {
                0 => None,
                1 => Some(format!("{} penny", val)),
                _ => Some(format!("{} pennies", val)),
            },
            Coin::Nickel(val) => match val {
                0 => None,
                1 => Some(format!("{} nickel", val)),
                _ => Some(format!("{} nickels", val)),
            },
            Coin::Dime(val) => match val {
                0 => None,
                1 => Some(format!("{} dime", val)),
                _ => Some(format!("{} dimes", val)),
            },
            Coin::Quarter(val) => match val {
                0 => None,
                1 => Some(format!("{} quarter", val)),
                _ => Some(format!("{} quarters", val)),
            },
        };
        match out {
            Some(s) => write!(f, "{}", s),
            _ => write!(f, ""),
        }
    }
}

impl Coin {
    pub fn value(&self) -> u32 {
        match *self {
            Coin::Penny(p) => p,
            Coin::Nickel(n) => n * 5,
            Coin::Dime(d) => d * 10,
            Coin::Quarter(q) => q * 25,
        }
    }
}

impl PartialEq for Coin {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Ord for Coin {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Coin {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value().cmp(&other.value()))
    }
}
