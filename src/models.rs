use diesel::prelude::*;

#[derive(Queryable)]
pub struct Account {
    pub id: i32,
    pub : String,
    pub body: String,
    pub published: bool,
}
