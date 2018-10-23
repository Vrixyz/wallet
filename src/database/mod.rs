use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

use self::models::{Funding,NewFunding};
use self::schema::fundings;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_funding<'a>(user_id: &'a str) -> Result<Funding, diesel::result::Error> {
    internal_get_funding(&establish_connection(), user_id)
}
pub fn internal_get_funding<'a>(connection: &PgConnection, user_id: &'a str) -> Result<Funding, diesel::result::Error> {
    use self::schema::fundings::dsl;
    
    Ok(dsl::fundings.filter(dsl::user_id.eq(String::from(user_id))).limit(1).get_result::<Funding>(connection)?)
}

pub fn add_funding<'a>(user_id: &'a str, amount_to_add: u32) -> Result<(), ()> {
    use self::schema::fundings::dsl;
    let connection = establish_connection();

    let amount_to_add = amount_to_add as i32;
    
    match diesel::update(dsl::fundings.filter(dsl::user_id.eq(String::from(user_id))))
        .set(dsl::amount.eq(dsl::amount + amount_to_add))
        .get_result::<Funding>(&connection) {
            Ok(f) => Ok(()),
            Err(_) => {
                println!("Must create new funding.");
                let new_funding = NewFunding {
                    user_id: &user_id.to_string(),
                    amount: amount_to_add,
                };
                diesel::insert_into(fundings::table)
                    .values(&new_funding)
                    .get_result::<Funding>(&connection)
                    .expect("Error saving new post");
                    Ok(())
            }
        }
}

#[cfg(test)]
mod test {
    use crate::database::*;

    fn remove_funding(funding: Funding) {
        use self::schema::fundings::dsl;

        let connection = establish_connection();
        diesel::delete(dsl::fundings.find(funding.id)).execute(&connection).expect(&format!("Error removing funding with id {}", funding.id));
    }

    #[test]
    fn add_and_get_funding() {
        add_funding("user1", 2);
        add_funding("user1", 2);
        add_funding("user1", 2);
        let funding = internal_get_funding(&establish_connection(), "user1").expect("failed to get funding");
        assert_eq!(funding.amount, 6);
        remove_funding(funding);
    }
}

