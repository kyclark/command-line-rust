#[cfg(test)]
use change::coin::Coin;

#[test]
fn test_value_penny() {
    assert_eq!(Coin::Penny(0).value(), 0);
    assert_eq!(Coin::Penny(1).value(), 1);
    assert_eq!(Coin::Penny(4).value(), 4);
}

#[test]
fn test_value_nickel() {
    assert_eq!(Coin::Nickel(0).value(), 0);
    assert_eq!(Coin::Nickel(1).value(), 5);
    assert_eq!(Coin::Nickel(4).value(), 20);
}

#[test]
fn test_value_dime() {
    assert_eq!(Coin::Dime(0).value(), 0);
    assert_eq!(Coin::Dime(1).value(), 10);
    assert_eq!(Coin::Dime(4).value(), 40);
}

#[test]
fn test_value_quarter() {
    assert_eq!(Coin::Quarter(0).value(), 0);
    assert_eq!(Coin::Quarter(1).value(), 25);
    assert_eq!(Coin::Quarter(4).value(), 100);
}

#[test]
fn test_display_penny() {
    assert_eq!(format!("{}", Coin::Penny(0)), "".to_string());
    assert_eq!(format!("{}", Coin::Penny(1)), "1 penny".to_string());
    assert_eq!(format!("{}", Coin::Penny(3)), "3 pennies".to_string());
}

#[test]
fn test_display_nickel() {
    assert_eq!(format!("{}", Coin::Nickel(0)), "".to_string());
    assert_eq!(format!("{}", Coin::Nickel(1)), "1 nickel".to_string());
    assert_eq!(format!("{}", Coin::Nickel(3)), "3 nickels".to_string());
}

#[test]
fn test_display_dime() {
    assert_eq!(format!("{}", Coin::Dime(0)), "".to_string());
    assert_eq!(format!("{}", Coin::Dime(1)), "1 dime".to_string());
    assert_eq!(format!("{}", Coin::Dime(3)), "3 dimes".to_string());
}

#[test]
fn test_display_quarter() {
    assert_eq!(format!("{}", Coin::Quarter(0)), "".to_string());
    assert_eq!(format!("{}", Coin::Quarter(1)), "1 quarter".to_string());
    assert_eq!(format!("{}", Coin::Quarter(3)), "3 quarters".to_string());
}
