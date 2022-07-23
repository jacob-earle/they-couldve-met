use chrono::NaiveDate;

use super::schema::people;

#[derive(Queryable)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub birth: NaiveDate,
    pub death: Option<NaiveDate>,
    pub description_en: String,
    pub wikipedia_link: String,
    pub thumbnail_link: String,
    pub links_to_page: i32,
}

#[derive(Insertable)]
#[table_name="people"]
pub struct NewPerson {
    pub name: String,
    pub birth: NaiveDate,
    pub death: Option<NaiveDate>,
    pub description_en: String,
    pub wikipedia_link: String,
    pub thumbnail_link: String,
    pub links_to_page: i32,
}