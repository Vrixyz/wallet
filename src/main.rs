#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate actix_web;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
pub mod database;

use actix_web::{http, server, App, Path, Responder};

fn add_fundings(info: Path<(String, u32)>) -> impl Responder {
    match database::add_funding(&info.0, info.1) {
        Err(_) => format!("error"),
        Ok(_) => format!("success"),
    }
}
fn get_fundings(info: Path<(String)>) -> impl Responder {
    match database::get_funding(&info) {
        Err(_) => format!("error"),
        Ok(f) => serde_json::to_string(&f).unwrap(),
    }
}

fn main() {
    server::new(
        || App::new()
            .resource("/fundings/{user_id}/add/{amount}", |r| r.method(http::Method::GET).with(add_fundings))
            .resource("/fundings/{user_id}/get", |r| r.method(http::Method::GET).with(get_fundings)))
        .bind("127.0.0.1:8080").unwrap()
        .run();
}