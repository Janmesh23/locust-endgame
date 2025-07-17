use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use reqwest::Client; // Import Client from reqwest

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationEntry {
    pub timestamp: DateTime<Utc>,
    pub lat: f64,
    pub lon: f64,
    pub city: Option<String>,    // Optional fields
    pub country: Option<String>, // Optional fields
}

pub async fn fetch_location(api_url: &str) -> Option<LocationEntry> {
    let client = Client::new(); // Create a new HTTP client
    let res = client.get(api_url).send().await.ok()?; // Make the GET request and await the response. Use `?` for error propagation with Option.
    let json: serde_json::Value = res.json().await.ok()?; // Parse the response body as JSON.

    // Extract fields, returning None if any required field is missing
    Some(LocationEntry {
        timestamp: Utc::now(), // Record the current UTC timestamp
        lat: json["lat"].as_f64()?, // Get latitude as f64
        lon: json["lon"].as_f64()?, // Get longitude as f64
        city: json["city"].as_str().map(|s| s.to_string()), // Get city as Option<String>
        country: json["country"].as_str().map(|s| s.to_string()), // Get country as Option<String>
    })
}