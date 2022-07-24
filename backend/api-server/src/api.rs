use actix_web::{web, Responder, HttpResponse};
use diesel::{RunQueryDsl, QueryDsl};
use people_database::schema::people::dsl::*;
use people_database::models::Person;
use crate::PostgresPool;

// function that will configure our API routes to attach to the router
pub fn config_api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(api_greeter)
            .service(search_for_people)
            .service(get_person_by_id)
    );
}

#[get("/")]
async fn api_greeter() -> impl Responder {
    HttpResponse::Ok().body("This is the API.")
}

#[get("/search")]
async fn search_for_people(pool: web::Data<PostgresPool>) -> impl Responder {
    let conn = pool.get().expect("Could not establish connection to database from pool.");

    let person_list: Vec<Person> = people
        .load(&conn)
        .expect("Couldn't load people from database.");

    HttpResponse::Ok().json(person_list)
}

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
