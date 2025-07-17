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
    println!("Map command not yet implemented.");
    Ok(())
}