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

pub fn get_funding<'a>(connection: &PgConnection, user_id: &'a str) -> Result<Funding, diesel::result::Error> {
    use self::schema::fundings::dsl;
    
    // TODO: lock db for read and update. I think transaction is not enough ?
    Ok(dsl::fundings.filter(dsl::user_id.eq(String::from(user_id))).limit(1).get_result::<Funding>(connection)?)
}

pub fn add_funding<'a>(user_id: &'a str, amount_to_add: i32) -> Result<(), ()> {
    use self::schema::fundings::dsl;
    let connection = establish_connection();
    
    // TODO: lock db for read and update. I think transaction is not enough ?
    let funding_to_modify = match get_funding(&connection, user_id) {
        Ok(f) => { f},
        Err(_) => {
            println!("Must create new funding.");
            let newFunding = NewFunding {
                user_id: &user_id.to_string(),
                amount: 0,
            };
            diesel::insert_into(fundings::table)
                .values(&newFunding)
                .get_result(&connection)
                .expect("Error saving new post")
        }
    };

    let initial_amout = funding_to_modify.amount;

    let post = diesel::update(dsl::fundings.find(funding_to_modify.id))
        .set(dsl::amount.eq(initial_amout + amount_to_add))
        .get_result::<Funding>(&connection)
        .expect(&format!("Unable to update funding {}", user_id));
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::database::*;
    #[test]
    fn add_and_get_funding() {
        add_funding("user1", 2);
        add_funding("user1", 2);
        add_funding("user1", 2);
        assert_eq!(get_funding(&establish_connection(), "user1").expect("failed to get funding").amount, 6);
    }
}

