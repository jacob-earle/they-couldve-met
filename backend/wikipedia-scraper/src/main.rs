extern crate people_database;
extern crate diesel;
extern crate chrono;

mod response_handler;

use people_database::{establish_connection, init_database_with_migrations};
use clap::Parser;

/// Simple script that will run a SPARQL query against DBPedia to get information to populate the database
#[derive(Parser, Debug)]
#[clap(version)]
struct CommandArgs {
    /// Path to the SPARQL query. Searches for a file called "sparql-query" in the current directory if none specified
    #[clap(short, long, value_parser, default_value = "./sparql-query")]
    file: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line args
    let args = CommandArgs::parse();

    // Attempting to connect to database
    println!("Connecting to database...");
    let conn = establish_connection();
    
    // Perform database migrations
    println!("Successfully connected to database. Running migrations.");
    init_database_with_migrations(&conn);
    

    // Loading and encoding the sparql query for use with the http request
    let graph_uri = String::from("http://dbpedia.org");
    let sparql_query = std::fs::read_to_string(&args.file)?;
    let format = String::from("application/sparql-results+json");
    let timeout = String::from("200000");
    let signal_void = String::from("on");
    let signal_unconnected = String::from("on");

    
    // Making http request to dbpedia's public API endpoint
    println!("Querying DBPedia...");
    let client = reqwest::Client::new();
    let resp = client.get("http://dbpedia.org/sparql")
        .query(&[
            ("default-graph-uri", &graph_uri),
            ("query", &sparql_query),
            ("format", &format),
            ("timeout", &timeout),
            ("signal_void", &signal_void),
            ("signal_unconnected", &signal_unconnected)
        ])
        .send()
        .await?
        .json::<response_handler::DBPediaResponse>()
        .await?;
    let parsed_results = resp.results.bindings;
    println!("Request to DBPedia successful. Parsed {} datapoints.", parsed_results.len());
    
    // Inserting the parsed results into the database
    println!("Populating database...");
    for person_info in parsed_results {
        person_info.insert_into_database(&conn);
    }
    println!("Database populated successfully!");
    Ok(())
}