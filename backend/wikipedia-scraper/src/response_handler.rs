use serde::{Serialize, Deserialize};

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
    pub bindings: Vec<Person>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
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

// Note, we have to manually rename some fields because of rust's naming conventions
#[derive(Serialize, Deserialize, Debug)]
pub struct NameInfo {
    #[serde(rename = "type")]
    type_field: String,
    #[serde(rename = "xml:lang")]
    xml_lang: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BirthDeathInfo {
    #[serde(rename = "type")]
    type_field: String,
    datatype: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DescriptionInfo {
    #[serde(rename = "type")]
    type_field: String,
    #[serde(rename = "xml:lang")]
    xml_lang: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkInfo {
    #[serde(rename = "type")]
    type_field: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkCountInfo {
    #[serde(rename = "type")]
    type_field: String,
    datatype: String,
    value: String,
}