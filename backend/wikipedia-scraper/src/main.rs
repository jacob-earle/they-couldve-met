extern crate people_database;
extern crate diesel;
extern crate chrono;

mod response_handler;

use people_database::establish_connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Attempting to connect to database
    println!("Connecting to database...");
    let conn = establish_connection();

    // Loading and encoding the sparql query for use with the http request
    let graph_uri = String::from("http://dbpedia.org");
    let sparql_query = std::fs::read_to_string("./sparql-query")?;
    let format = String::from("application/sparql-results+json");

    
    // Making http request to dbpedia's public API endpoint
    println!("Querying DBPedia...");
    let client = reqwest::Client::new();
    let resp = client.get("http://dbpedia.org/sparql")
        .query(&[
            ("default-graph-uri", &graph_uri),
            ("query", &sparql_query),
            ("format", &format)
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
    Ok(())
}