#[macro_use]
extern crate actix_web;
extern crate people_database;

use dotenv::dotenv;
use std::env;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

mod api;

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

// Get the connection to the Postgres database
fn get_pool() -> PostgresPool {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("No database URL found.");
    let conn_manager: ConnectionManager<PgConnection> = ConnectionManager::new(url);
    
    r2d2::Pool::builder()
        .build(conn_manager)
        .expect("Could not build connection pool.")
}

// Base route
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    // Setting environment variables
    let host = env::var("HOST").expect("HOST not set.");
    let port = env::var("PORT").expect("PORT not set.");
    
    // Creating database management pool
    let pool = get_pool();
    
    
    HttpServer::new(move || {
        App::new()
            .configure(api::config_api)
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
