use serde::{Serialize, Deserialize};
use people_database::*;
use people_database::models::{Person};
use diesel::pg::PgConnection;
use chrono::naive::NaiveDate;

// Types used to deserialize the response from the sparql query
#[derive(Serialize, Deserialize, Debug)] 
pub struct DBPediaResponse {
    pub head: Head,
    pub results: Results,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Head {
    pub link: Vec<String>,
    pub vars: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Results {
    pub distinct: bool,
    pub ordered: bool,
    pub bindings: Vec<PersonInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonInfo {
    name: NameInfo,
    birth: BirthDeathInfo,
    death: Option<BirthDeathInfo>,
    description_en: DescriptionInfo,
    #[serde(rename = "wikipediaLink")]
    wikipedia_link: LinkInfo,
    #[serde(rename = "thumbnailLink")]
    thumbnail_link: LinkInfo,
    #[serde(rename = "linksToPage")]
    links_to_page: LinkCountInfo,
    person: LinkInfo,
}

impl PersonInfo {
    pub fn insert_into_database(&self, conn: &PgConnection) -> Person {
        create_person(
            conn, 
            self.name.to_string(), 
            self.birth.to_date(), 
            self.death.clone().map(|x| x.to_date()), 
            self.description_en.to_string(), 
            self.wikipedia_link.to_string(), 
            self.thumbnail_link.to_string(), 
            self.links_to_page.to_int()
        )
    }
}

// Note, we have to manually rename some fields because of rust's naming conventions
#[derive(Serialize, Deserialize, Debug)]
pub struct NameInfo {
    #[serde(rename = "type")]
    type_field: String,
    #[serde(rename = "xml:lang")]
    xml_lang: String,
    value: String,
}

impl NameInfo {
    pub fn to_string(&self) -> String {
        self.value.clone()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BirthDeathInfo {
    #[serde(rename = "type")]
    type_field: String,
    datatype: String,
    value: String,
}

impl BirthDeathInfo {
    pub fn to_date(&self) -> NaiveDate {
        NaiveDate::parse_from_str(&self.value, "%Y-%m-%d").expect("Could not properly parse date.")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DescriptionInfo {
    #[serde(rename = "type")]
    type_field: String,
    #[serde(rename = "xml:lang")]
    xml_lang: String,
    value: String,
}

impl DescriptionInfo {
    pub fn to_string(&self) -> String {
        self.value.clone()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkInfo {
    #[serde(rename = "type")]
    type_field: String,
    value: String,
}

impl LinkInfo {
    pub fn to_string(&self) -> String {
        self.value.clone()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkCountInfo {
    #[serde(rename = "type")]
    type_field: String,
    datatype: String,
    value: String,
}

impl LinkCountInfo {
    pub fn to_int(&self) -> i32 {
        self.value.parse::<i32>().unwrap_or(0)
    }
}