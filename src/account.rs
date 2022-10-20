use crate::document::*;

#[derive(Debug, PartialEq)]
pub struct Owner {
    pub name: String,
    pub document: Document,
}

#[derive(Debug, PartialEq)]
pub struct Account {
    pub amount: f64,
    pub owner: Owner,
}

impl ToString for Account {
    fn to_string(&self) -> String {
        return format!("The Owner {} has amount {}", &self.owner.name, &self.amount);
    }
}
