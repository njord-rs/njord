use crate::schema::NearEarthObject;
use njord::keys::AutoIncrementPrimaryKey;
use njord::sqlite::{self, SqliteError};
use reqwest::header::ACCEPT;
use serde::Deserialize;
use serde_json::Value;
use std::path::Path;

mod schema;

const API_URL: &str = "https://api.nasa.gov/neo/rest/v1";

#[derive(Debug, Deserialize)]
pub struct NearEarthObjectResponse {
    #[serde(rename = "id")]
    pub neo_id: String,
    pub neo_reference_id: String,
    pub name: String,
    pub name_limited: String,
    pub designation: String,
    pub nasa_jpl_url: String,
    pub absolute_magnitude_h: f64,
    pub is_potentially_hazardous_asteroid: bool,
}

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
                let response_obj: NearEarthObjectResponse =
                    serde_json::from_value(obj.clone()).unwrap();

                let near_earth_obj = NearEarthObject {
                    id: AutoIncrementPrimaryKey::default(), // Auto-generate id
                    neo_id: response_obj.neo_id,            // Map id to neo_id
                    neo_reference_id: response_obj.neo_reference_id,
                    name: response_obj.name,
                    name_limited: response_obj.name_limited,
                    designation: response_obj.designation,
                    nasa_jpl_url: response_obj.nasa_jpl_url,
                    absolute_magnitude_h: response_obj.absolute_magnitude_h,
                    is_potentially_hazardous_asteroid: response_obj
                        .is_potentially_hazardous_asteroid,
                    is_sentry_object: false, // Set this field as needed
                };
                println!("{:#?}", near_earth_obj);
                near_earth_objects.push(near_earth_obj);
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }

    let conn = sqlite::open(db_path)?;
    sqlite::insert(&conn, near_earth_objects)?;

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

    println!("response = {:#?}", response_text);

    let v: Value = serde_json::from_str(&response_text)?;

    Ok(v)
}
