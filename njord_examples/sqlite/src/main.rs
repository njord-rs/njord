use crate::schema::NearEarthObject;
use njord::sqlite::{self, SqliteError};
use reqwest::header::ACCEPT;
use serde_json::Value;
use std::path::Path;

mod schema;

const API_URL: &str = "https://api.nasa.gov/neo/rest/v1";

#[tokio::main]
async fn main() -> Result<(), SqliteError> {
    // Setting up a SQLite DB and Connection
    let db_relative_path = "./njord_examples/sqlite/neo.db";
    let db_path = Path::new(&db_relative_path);

    let neo = get_near_earth_objects(0, 10).await;
    let mut near_earth_objects: Vec<NearEarthObject> = Vec::new();

    match neo {
        Ok(data) => {
            for obj in data["near_earth_objects"].as_array().unwrap() {
                let near_earth_obj: NearEarthObject = serde_json::from_value(obj.clone()).unwrap();
                println!("{:#?}", near_earth_obj);
                near_earth_objects.push(near_earth_obj);
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }

    let conn = sqlite::open(db_path).unwrap();
    sqlite::insert(conn, near_earth_objects)?;

    Ok(())
}

async fn get_near_earth_objects(page: u32, size: u32) -> Result<Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let endpoint = format!(
        "{}/neo/browse?page={}&size={}&api_key=DEMO_KEY",
        API_URL, page, size
    );

    let response = client
        .get(endpoint)
        .header(ACCEPT, "application/json")
        .send()
        .await?;

    let response_text = response.text().await?;

    let v: Value = serde_json::from_str(&response_text)?;

    Ok(v)
}
