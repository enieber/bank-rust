use crate::account::*;
use crate::document::Document;

pub fn open_account(name: String, doc: String, amount: f64) -> Result<Account, String> {
    match Document::new(doc) {
        Some(document) => {
            let owner = Owner {
                name,
                document,
            };
            Ok(Account {
                amount,
                owner,
            })
        }
        None => Err(String::from("Document invalid")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_document_in_open_account() {
        let my_account = open_account(String::from("Enieber Cunha"), String::from("1"), 10.90);
        assert_eq!(my_account, Err(String::from("Document invalid")))
    }

    #[test]
    fn open_account_with_success() {
        let my_account = open_account(String::from("Enieber Cunha"), String::from("123"), 10.90);
        assert_eq!(
            my_account.unwrap().to_string(),
            String::from("The Owner Enieber Cunha has amount 10.9")
        )
    }
}
