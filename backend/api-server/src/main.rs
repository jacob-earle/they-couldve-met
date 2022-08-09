#[macro_use]
extern crate actix_web;
extern crate people_database;

use dotenv::dotenv;
use std::env;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web::cookie::Key;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
use actix_cors::Cors;
use people_database::{establish_connection, init_database_with_migrations};
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};

mod api;
mod auth;

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
    // Setting environment variables
    dotenv().ok();
    let host = env::var("HOST").expect("HOST not set.");
    let port = env::var("PORT").expect("PORT not set.");
    let secret_key = Key::generate();
    
    // Creating database management pool
    let pool = get_pool();
    
    // Connect to database and perform migrations
    // Attempting to connect to database
    println!("Connecting to database...");
    let conn = establish_connection();
    
    // Perform database migrations
    println!("Successfully connected to database. Running migrations.");
    init_database_with_migrations(&conn);
    
    HttpServer::new(move || {
        // Constructing CORS policy
        let cors = Cors::permissive();

        App::new()
            .wrap(
                SessionMiddleware::new(
                    CookieSessionStore::default(),
                    secret_key.clone()
                )
            )
            .wrap(cors)
            .configure(api::config_api)
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
