#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
extern crate chrono;

use chrono::naive::NaiveDate;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::{env, time, thread};

use self::models::{Person, NewPerson};

embed_migrations!();

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATA_URL must be set.");
    
    // If we cannot immediately establish connection, wait 5 seconds and try again
    loop {
        match PgConnection::establish(&database_url) {
            Ok(conn) => {
                return conn;
            },
            Err(_) => {
                println!("Error connecting to {}. Will attempt to reconnect in 1 second.", &database_url);
                thread::sleep(time::Duration::from_secs(5));
            } 
        }
    }
}

/// Run migrations on connected database
pub fn init_database_with_migrations(conn: &PgConnection) {
    embedded_migrations::run(conn).expect("Error performing diesel migrations!");
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