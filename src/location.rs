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
    // Uncomment ONE of these 'return Some(...)' blocks at a time,
    // run 'cargo run -- start', then stop it, and repeat for the next.

    // 1. New York, USA
    
    // return Some(LocationEntry {
    //     timestamp: Utc::now(),
    //     lat: 40.7128,
    //     lon: -74.0060,
    //     city: Some("New York".into()),
    //     country: Some("United States".into()),
    // });
    

    // 2. London, UK
    
    // return Some(LocationEntry {
    //     timestamp: Utc::now(),
    //     lat: 51.5074,
    //     lon: -0.1278,
    //     city: Some("London".into()),
    //     country: Some("United Kingdom".into()),
    // });
    

    // 3. Tokyo, Japan
    
    // return Some(LocationEntry {
    //     timestamp: Utc::now(),
    //     lat: 35.6895,
    //     lon: 139.6917,
    //     city: Some("Tokyo".into()),
    //     country: Some("Japan".into()),
    // });
    

    // 4. Sydney, Australia
    
    // return Some(LocationEntry {
    //     timestamp: Utc::now(),
    //     lat: -33.8688,
    //     lon: 151.2093,
    //     city: Some("Sydney".into()),
    //     country: Some("Australia".into()),
    // });
    

    // 5. Rio de Janeiro, Brazil
    
    // return Some(LocationEntry {
    //     timestamp: Utc::now(),
    //     lat: -22.9068,
    //     lon: -43.1729,
    //     city: Some("Rio de Janeiro".into()),
    //     country: Some("Brazil".into()),
    // });
    

    // 6. Cairo, Egypt
    /*
    return Some(LocationEntry {
        timestamp: Utc::now(),
        lat: 30.0444,
        lon: 31.2357,
        city: Some("Cairo".into()),
        country: Some("Egypt".into()),
    });
    */

    // 7. Cape Town, South Africa
    
    // return Some(LocationEntry {
    //     timestamp: Utc::now(),
    //     lat: -33.9249,
    //     lon: 18.4241,
    //     city: Some("Cape Town".into()),
    //     country: Some("South Africa".into()),
    // });
    

    // 8. Moscow, Russia
    
    // return Some(LocationEntry {
    //     timestamp: Utc::now(),
    //     lat: 55.7558,
    //     lon: 37.6173,
    //     city: Some("Moscow".into()),
    //     country: Some("Russia".into()),
    // });
    

    // 9. Reykjavik, Iceland
    
    // return Some(LocationEntry {
    //     timestamp: Utc::now(),
    //     lat: 64.1265,
    //     lon: -21.8174,
    //     city: Some("Reykjavik".into()),
    //     country: Some("Iceland".into()),
    // });
    

    // 10. Bangkok, Thailand
    
    // return Some(LocationEntry {
    //     timestamp: Utc::now(),
    //     lat: 13.7563,
    //     lon: 100.5018,
    //     city: Some("Bangkok".into()),
    //     country: Some("Thailand".into()),
    // });
   

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