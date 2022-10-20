#[derive(Debug, PartialEq)]
pub struct Document {
    str: String,
}

impl Document {
    pub fn new(str: String) -> Option<Self> {
        if str.chars().count() <= 1 {
            None
        } else {
            Some(Document { str })
        }
    }
}
