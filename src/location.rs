// ... (existing imports)
use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationEntry {
    pub timestamp: DateTime<Utc>,
    pub lat: f64,
    pub lon: f64,
    pub city: Option<String>,
    pub country: Option<String>,
}

pub async fn fetch_location(api_url: &str) -> Option<LocationEntry> {
    // --- TEMPORARY MOCK DATA FOR TESTING HEATMAP ---
    // You would uncomment and modify these lines to simulate different locations
    // For actual IP-based logging, keep the original code below.

    // Static example 1: New York
    // return Some(LocationEntry {
    //     timestamp: Utc::now(),
    //     lat: 40.7128,
    //     lon: -74.0060,
    //     city: Some("New York".into()),
    //     country: Some("United States".into()),
    // });

    // Static example 2: London
    // return Some(LocationEntry {
    //     timestamp: Utc::now(),
    //     lat: 51.5074,
    //     lon: -0.1278,
    //     city: Some("London".into()),
    //     country: Some("United Kingdom".into()),
    // });

    // Static example 3: Tokyo
    // return Some(LocationEntry {
    //     timestamp: Utc::now(),
    //     lat: 35.6895,
    //     lon: 139.6917,
    //     city: Some("Tokyo".into()),
    //     country: Some("Japan".into()),
    // });

    // Example: Cycle through a few locations (more complex, but shows spread)
    // You'd need a static mutable counter or similar for this
    // For simple test, just uncomment one `return Some(...)` at a time, run `start`, stop, uncomment another, run `start` again.

    // --- ORIGINAL CODE FOR REAL IP GEOLOCATION (KEEP THIS FOR PRODUCTION) ---
    let client = Client::new();
    let res = client.get(api_url).send().await.ok()?;
    let json: serde_json::Value = res.json().await.ok()?;

    Some(LocationEntry {
        timestamp: Utc::now(),
        lat: json["lat"].as_f64()?,
        lon: json["lon"].as_f64()?,
        city: json["city"].as_str().map(|s| s.to_string()),
        country: json["country"].as_str().map(|s| s.to_string()),
    })
}