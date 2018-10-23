use super::schema::fundings;
use diesel::prelude::Insertable;

#[derive(Queryable, Serialize)]
pub struct Funding {
    pub id: i32,
    pub user_id: String,
    pub amount: i32,
}

#[derive(Insertable)]
#[table_name="fundings"]
pub struct NewFunding<'a> {
    pub user_id: &'a str,
    pub amount: i32,
}