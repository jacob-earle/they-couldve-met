#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;

use chrono::naive::NaiveDate;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use self::models::{Person, NewPerson};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATA_URL must be set.");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}.", database_url))
}


pub fn create_person<'a>(conn: &PgConnection, name: String, birth: NaiveDate, death: Option<NaiveDate>, description_en: String, wikipedia_link: String, thumbnail_link: String, links_to_page: i32) -> Person {
    use schema::people;

    let new_person = NewPerson {
       name,
       birth,
       death,
       description_en,
       wikipedia_link,
       thumbnail_link,
       links_to_page, 
    };

    diesel::insert_into(people::table)
        .values(&new_person)
        .get_result(conn)
        .expect("Error saving new person.")
}