use std::{fs::OpenOptions, io::Write, thread, time::Duration};
use crate::location::{fetch_location, LocationEntry};
use crate::config::load_config;
use chrono::Utc;
use serde_json;
use open;

pub async fn start_logging() {
    let config = load_config();
    let interval = Duration::from_secs(config.interval);

    println!("📍 Logging every {} seconds...", config.interval);

    loop {
        if let Some(loc) = fetch_location(&config.api_url).await {
            let json = serde_json::to_string(&loc).unwrap();

            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&config.log_path)
                .unwrap();

            writeln!(file, "{}", json).unwrap();
            println!("[{}] Logged: {}, {}", Utc::now(), loc.lat, loc.lon);
        } else {
            println!("⚠️ Failed to fetch location");
        }

        thread::sleep(interval);
    }
}

pub fn list_logs() {
    let config = load_config();
    if let Ok(content) = std::fs::read_to_string(&config.log_path) {
        for line in content.lines().rev().take(10) {
            if let Ok(loc) = serde_json::from_str::<LocationEntry>(line) {
                println!(
                    "[{}] {}, {} ({}, {})",
                    loc.timestamp,
                    loc.lat,
                    loc.lon,
                    loc.city.unwrap_or_default(),
                    loc.country.unwrap_or_default()
                );
            } else {
                eprintln!("Warning: Could not parse log entry: {}", line);
            }
        }
    } else {
        println!("No log file found at '{}'. Run 'cargo run -- start' to begin logging.", config.log_path);
    }
}

pub fn generate_map() -> std::io::Result<()> {
    let config = load_config();
    let content = std::fs::read_to_string(&config.log_path)?;
    let mut points = vec![];

    // Note: To draw a line in chronological order, we need the points
    // in the order they were logged. `content.lines()` provides this.
    for line in content.lines() {
        if let Ok(loc) = serde_json::from_str::<LocationEntry>(line) {
            points.push(format!("[{}, {}]", loc.lat, loc.lon));
        }
    }

    let points_str = points.join(",\n");

    let html = format!(
        r#"<!DOCTYPE html>
<html>
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
        var map = L.map('map').setView([0, 0], 2);
        L.tileLayer('https://{{s}}.tile.openstreetmap.org/{{z}}/{{x}}/{{y}}.png', {{
            maxZoom: 18,
            attribution: '© OpenStreetMap contributors'
        }}).addTo(map);

        var heatData = [
            {}
        ];

        // 1. Heatmap Layer with darker red
        var heat = L.heatLayer(heatData, {{
            radius: 25,
            // Define custom color gradient:
            // [0.0, 0.5, 1.0] represent intensity thresholds (from low to high)
            // 'red', 'darkred', 'rgb(139, 0, 0)' are HTML color names or RGB values
            colors: ['red', 'darkred', 'rgb(139, 0, 0)'] // Adjust these as you like
        }}).addTo(map);

        // 2. Polyline Layer to connect points chronologically
        if (heatData.length > 1) {{ // Need at least two points to draw a line
            var polyline = L.polyline(heatData, {{
                color: 'blue', // Color of the line
                weight: 3,     // Thickness of the line
                opacity: 0.7   // Transparency of the line
            }}).addTo(map);

            // Fit map bounds to the polyline (which correctly represents all points)
            map.fitBounds(polyline.getBounds());
        }} else if (heatData.length === 1) {{
            // If only one point, just center the map on it
            map.setView(heatData[0], 10); // Center on single point with zoom level 10
        }}


        // Original fitBounds logic (using tempLayer) is replaced by polyline.getBounds()
        // if (heatData.length > 0) {{
        //     var tempLayer = L.featureGroup();
        //     heatData.forEach(function(p) {{
        //         L.marker(p).addTo(tempLayer);
        //     }});
        //     map.fitBounds(tempLayer.getBounds());
        // }}
    </script>
</body>
</html>"#,
        points_str
    );

    std::fs::create_dir_all("map")?;
    std::fs::write("map/index.html", html)?;
    open::that("map/index.html")?;

    Ok(())
}