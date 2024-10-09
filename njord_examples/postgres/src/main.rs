use crate::schema::NearEarthObject;
use njord::{keys::AutoIncrementPrimaryKey, postgres};
use reqwest::header::ACCEPT;
use serde::Deserialize;
use serde_json::Value;
use std::{error::Error, thread};
use tokio; // Import Tokio runtime

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

#[tokio::main] // Use tokio runtime for async execution in the main function
async fn main() -> Result<(), Box<dyn Error>> {
    // Fetch and process NEOs asynchronously
    let near_earth_objects = get_and_process_neos().await?;

    // Handle database connection and insertion synchronously here
    // FIXME: Figure out how to do on the main thread
    match thread::spawn(|| {
        let mut conn =
            postgres::open("postgresql://myuser:mypassword@localhost:5432/mydatabase").unwrap();

        postgres::insert(&mut conn, near_earth_objects).unwrap();
    })
    .join()
    {
        Ok(_) => (),
        Err(err) => eprintln!("Error: {:?}", err),
    }

    Ok(())
}

async fn get_and_process_neos() -> Result<Vec<NearEarthObject>, Box<dyn Error>> {
    let neo = get_near_earth_objects(0, 10).await?;
    let mut near_earth_objects: Vec<NearEarthObject> = Vec::new();

    for obj in neo["near_earth_objects"].as_array().unwrap() {
        let response_obj: NearEarthObjectResponse = serde_json::from_value(obj.clone()).unwrap();

        let near_earth_obj = NearEarthObject {
            id: AutoIncrementPrimaryKey::default(), // Auto-generate id
            neo_id: response_obj.neo_id,            // Map id to neo_id
            neo_reference_id: response_obj.neo_reference_id,
            name: response_obj.name,
            name_limited: response_obj.name_limited,
            designation: response_obj.designation,
            nasa_jpl_url: response_obj.nasa_jpl_url,
            absolute_magnitude_h: response_obj.absolute_magnitude_h,
            is_potentially_hazardous_asteroid: response_obj.is_potentially_hazardous_asteroid,
            is_sentry_object: false, // Set this field as needed
        };

        println!("{:#?}", near_earth_obj);
        near_earth_objects.push(near_earth_obj);
    }

    Ok(near_earth_objects)
}

async fn get_near_earth_objects(page: u32, size: u32) -> Result<Value, Box<dyn Error>> {
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
