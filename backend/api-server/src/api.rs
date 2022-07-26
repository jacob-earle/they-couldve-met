use actix_web::{web, Responder, HttpResponse};
use diesel::{RunQueryDsl, QueryDsl, BoolExpressionMethods, ExpressionMethods, PgTextExpressionMethods};
use people_database::schema::people::dsl::*;
use people_database::models::Person;
use serde::Deserialize;
use crate::PostgresPool;

// function that will configure our API routes to attach to the router
pub fn config_api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(api_greeter)
            .service(search_for_people)
            .service(get_person_by_id)
            .service(get_matches_for_person_with_id)
    );
}

#[get("/")]
async fn api_greeter() -> impl Responder {
    HttpResponse::Ok().body("This is the API.")
}

#[derive(Deserialize)]
struct SearchQuery {
    // The fragment to be matched in the search query
    q: String,
}

/// Search people in the database by name
#[get("/search")]
async fn search_for_people(pool: web::Data<PostgresPool>, query: web::Query<SearchQuery>) -> impl Responder {
    let conn = pool.get().expect("Could not establish connection to database from pool.");
    
    let search_string = "%".to_string() + &query.q + "%";
    
    // Find all people with names containing the search string
    let person_list: Vec<Person> = people
        .filter(name.ilike(&search_string))
        .get_results(&conn)
        .expect("Couldn't load people from database.");

    HttpResponse::Ok().json(person_list)
}

/// Get record of an individual person by id
#[get("/{person_id}")]
async fn get_person_by_id(pool: web::Data<PostgresPool>, person_id: web::Path<i32>) -> impl Responder {
    let person_id_deref = person_id.into_inner();
    
    let conn = pool.get().expect("Could not establish connection to database from pool.");
    
    let person: Person = people
        .find(person_id_deref)
        .first(&conn)
        .expect("Error loading person.");
    
    HttpResponse::Ok().json(person)
}

/// Find all people a person could have met
/// I.e. all people who were alive during a person's life
/// So, the people who died after a person was born and who were born before the person died
#[get("/match/{person_id}")]
async fn get_matches_for_person_with_id(pool: web::Data<PostgresPool>, person_id: web::Path<i32>) -> impl Responder {
    let conn = pool.get().expect("Could not establish connection to database from pool.");
    
    let person_id_deref = person_id.into_inner();

    // Retrieve person with given id from database
    let person: Person = people
        .find(person_id_deref)
        .first(&conn)
        .expect("Couldn't find person with id specified.");
        
    // Find people who were alive at the same time as the person
    let matching_people_list: Vec<Person> = match person.death {
        Some(death_date) => {
            people
                .filter(birth.le(death_date).and(death.is_null().or(death.ge(person.birth))).and(id.ne(person_id_deref)))
                .order(birth.desc())
                .get_results(&conn)
                .expect("Could not load matches")
        },
        None => {
            people
                .filter(death.ge(person.birth))
                .or_filter(death.is_null())
                .order(birth.desc())
                .get_results(&conn)
                .expect("Could not load matches.")
        }
    };
    
    HttpResponse::Ok().json(matching_people_list)
}