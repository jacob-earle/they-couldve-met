use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;

use super::schema::{people, users};

#[derive(Queryable, Debug, Identifiable, Serialize)]
#[table_name = "people"]
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

#[derive(Queryable, Debug, Identifiable, Serialize)]
#[primary_key(userid)]
#[table_name = "users"]
pub struct User {
    pub userid: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub pw: String,
    pub birthday: NaiveDate,
    pub admin_role: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub pw: String,
    pub birthday: NaiveDate,
    pub admin_role: bool,
    pub created_at: NaiveDateTime,
}