use std::{fs::OpenOptions, io::Write, thread, time::Duration};
use crate::location::{fetch_location, LocationEntry}; // Make sure LocationEntry is imported
use crate::config::load_config;
use chrono::Utc;
use serde_json; // Ensure serde_json is imported

pub async fn start_logging() {
    let config = load_config();
    let interval = Duration::from_secs(config.interval);

    println!("📍 Logging every {} seconds...", config.interval);

    loop {
        if let Some(loc) = fetch_location(&config.api_url).await {
            // Serialize LocationEntry to a JSON string
            let json = serde_json::to_string(&loc).unwrap();

            // Open the log file in append mode, create if it doesn't exist
            let mut file = OpenOptions::new()
                .create(true) // Create the file if it doesn't exist
                .append(true) // Append to the file if it exists
                .open(&config.log_path)
                .unwrap(); // Panic if file cannot be opened (for simplicity in this phase)

            // Write the JSON string followed by a newline
            writeln!(file, "{}", json).unwrap(); // Panic if write fails (for simplicity)

            println!("[{}] Logged: {}, {}", Utc::now(), loc.lat, loc.lon);
        } else {
            println!("⚠️ Failed to fetch location");
        }

        // Sleep for the configured interval. Note: `thread::sleep` is blocking,
        // which is fine for a simple interval loop, but for more complex async
        // scenarios, `tokio::time::sleep` would be preferred.
        thread::sleep(interval);
    }
}

pub fn list_logs() {
    let config = load_config();
    // Try to read the content of the log file
    if let Ok(content) = std::fs::read_to_string(&config.log_path) {
        // Iterate over lines, reverse them to get most recent first, and take up to 10
        for line in content.lines().rev().take(10) {
            // Attempt to deserialize each line back into a LocationEntry
            if let Ok(loc) = serde_json::from_str::<LocationEntry>(line) {
                // Print the formatted log entry
                println!(
                    "[{}] {}, {} ({}, {})",
                    loc.timestamp,
                    loc.lat,
                    loc.lon,
                    // Use unwrap_or_default() to safely get city/country,
                    // providing an empty string if they are None
                    loc.city.unwrap_or_default(),
                    loc.country.unwrap_or_default()
                );
            } else {
                // Handle cases where a line might not be valid JSON or not a LocationEntry
                eprintln!("Warning: Could not parse log entry: {}", line);
            }
        }
    } else {
        // If the file doesn't exist or can't be read, inform the user
        println!("No log file found at '{}'. Run 'cargo run -- start' to begin logging.", config.log_path);
    }
}

pub fn generate_map() -> std::io::Result<()> {
    let config = load_config();
    let content = std::fs::read_to_string(&config.log_path)?; // Read the entire log file
    let mut points = vec![]; // Initialize an empty vector to store map points

    // Process each line in the log file
    for line in content.lines() {
        if let Ok(loc) = serde_json::from_str::<LocationEntry>(line) {
            // Format each location as "[latitude, longitude]" for Leaflet.heat
            points.push(format!("[{}, {}]", loc.lat, loc.lon));
        }
    }

    // Join all formatted points with a comma and newline for clean HTML embedding
    let points_str = points.join(",\n");

    // Construct the full HTML content with embedded Leaflet.js and Leaflet.heat.js
    let html = format!(
        r#"<html>
<head>
    <meta charset="utf-8" />
    <title>Locust Map</title>
    <link rel="stylesheet" href="https://unpkg.com/leaflet/dist/leaflet.css" />
    <style> #map {{ height: 100vh; }} </style>
</head>
<body>
    <div id="map"></div>
    <script src="https://unpkg.com/leaflet/dist/leaflet.js"></script>
    <script src="https://unpkg.com/leaflet.heat/dist/leaflet-heat.js"></script>
    <script>
        // Initialize the map, centered at [0,0] with zoom level 2
        var map = L.map('map').setView([0, 0], 2);
        // Add OpenStreetMap tiles
        L.tileLayer('https://{{s}}.tile.openstreetmap.org/{{z}}/{{x}}/{{y}}.png', {{
            maxZoom: 18,
            attribution: '© OpenStreetMap contributors' // Add attribution for good practice
        }}).addTo(map);

        // Define the heatmap data points
        var heat = L.heatLayer([
            {} // This placeholder will be replaced by points_str
        ], {{radius: 25}}).addTo(map);

        // Optional: Fit map bounds to the heatmap data (if points exist)
        if (heat.getLatLngs().length > 0) {{
            map.fitBounds(heat.getLatLngs());
        }}
    </script>
</body>
</html>"#,
        points_str
    );

    // Create a 'map' directory if it doesn't exist
    std::fs::create_dir_all("map")?;
    // Write the HTML content to 'map/index.html'
    std::fs::write("map/index.html", html)?;
    // Open the generated HTML file in the default web browser
    open::that("map/index.html")?;

    Ok(()) // Indicate success
}