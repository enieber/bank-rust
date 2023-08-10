use crate::document::*;

pub struct Personal {
    pub name: String,
    pub social_name: String,
    pub document: Document,
    pub document_identification: String,
    pub passport_number: String,
    pub parent_first: Parent,
    pub parent_last: Parent,
    pub birth_date: Date,
    pub sex: Sex,
    pub civil_state: CivilState,
    pub nationality: Coutry,
    pub abroad_document: Some(String),
}

pub enum CivilState {
    Married,
    Single,
    Divorced,
    Widowed
}

pub struct Parent {
    pub name: String,
    pub sex: Sex,
}

pub enum Sex {
    Male,
    Female,
    Outhers,
}

pub struct Contact {
    pub phone: String,
    pub email: String,
}

pub struct Address {
    pub street: String,
    pub zip_code: String,
    pub number: String,
}

pub struct AccountPersonal {
    pub balance: Amount,
    pub balance_blocked: Amount,
    pub owner: Personal,
}

pub struct Amount {
    pub value: f64,
    pub currency: Currency,
}

pub struct AccountToTransfer {
    pub agency: String,
    pub account: String,
    pub bank_info: BankToTransfer,
}

pub struct Currency {
    pub coutry: Coutry,
    pub simbol: String,
}

pub enum Coutry {
    Brasil,
}