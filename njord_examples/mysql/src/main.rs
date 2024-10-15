use std::fmt::Error;

use crate::schema::NearEarthObject;
use njord::column::Column;
use njord::keys::AutoIncrementPrimaryKey;
use njord::mysql;
use reqwest::header::ACCEPT;
use serde::Deserialize;
use serde_json::Value;

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
async fn main() -> Result<(), Error> {
    let _ = insert().await;
    let _ = select();

    Ok(())
}

fn select() -> Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://njord_user:njord_password@localhost:3306/njord_db";
    let mut conn = mysql::open(url).unwrap();

    let results = mysql::select(&mut conn, vec![Column::Text("id".to_string())])
        .from(NearEarthObject::default())
        .build();

    match results {
        Ok(data) => println!("Selected: {:#?}", data.len()),
        Err(err) => eprintln!("Error: {}", err),
    }

    Ok(())
}

async fn insert() -> Result<(), Box<dyn std::error::Error>> {
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

    let url = "mysql://user:password@localhost:3306/neo";
    let mut conn = mysql::open(url).unwrap();

    match mysql::insert(&mut conn, near_earth_objects) {
        Ok(_) => println!("Near Earth Objects inserted successfully"),
        Err(err) => eprintln!("Error: {}", err),
    };

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

    // println!("response = {:#?}", response_text);

    let v: Value = serde_json::from_str(&response_text)?;

    Ok(v)
}
