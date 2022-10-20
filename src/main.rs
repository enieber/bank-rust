mod account;
mod bank;
mod document;
use crate::bank::open_account;

fn main() {
    let my_account = open_account(String::from("Enieber Cunha"), String::from("123"), 10.90);
    match my_account {
        Ok(account) => println!("{}", account.to_string()),
        Err(err) => println!("{}", err),
    }

    let outher_account = open_account(String::from("Marcelo "), String::from("321"), 90.60);
    match outher_account {
        Ok(account) => println!("{}", account.to_string()),
        Err(err) => println!("{}", err),
    }

    let invalid_account = open_account(String::from("Marcelo "), String::from("1"), 90.60);
    match invalid_account {
        Ok(account) => println!("{}", account.to_string()),
        Err(err) => println!("{}", err),
    }
}
