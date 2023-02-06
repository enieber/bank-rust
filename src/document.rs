extern crate cpf_cnpj;

use cpf_cnpj::cpf;

#[derive(Debug, PartialEq)]
pub struct Document {
    str: String,
}

impl Document {
    pub fn new(str: String) -> Option<Self> {
        if cpf::validate(&str) {
            Some(Document { str })
        } else {
            None
        }
    }
}
