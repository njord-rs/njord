#[allow(unused_imports)]
use njord::table::Table;
use njord_derive::Table;
use serde::Deserialize;

#[derive(Table, Deserialize, Debug)]
#[table_name = "neo"]
pub struct NearEarthObject {
    pub id: String,
    pub neo_reference_id: String,
    pub name: String,
    pub name_limited: String,
    pub designation: String,
    pub nasa_jpl_url: String,
    pub absolute_magnitude_h: f64,
    pub is_potentially_hazardous_asteroid: bool,
    pub is_sentry_object: bool,
}
