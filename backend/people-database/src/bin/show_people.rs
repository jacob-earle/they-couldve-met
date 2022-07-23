extern crate people_database;
extern crate diesel;

use self::people_database::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use people_database::schema::people::dsl::*;

    let connection = establish_connection();
    let results = people
        .limit(5)
        .load::<Person>(&connection)
        .expect("Error loading people.");

    println!("Displaying {} people.", results.len());
    for person in results {
        println!("{}", person.name);
        println!("----------\n");
        println!("{}\n", person.description_en);
    }
}