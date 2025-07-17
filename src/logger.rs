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

// Stubs for later phases
pub fn list_logs() {
    println!("List command not yet implemented.");
}

pub fn generate_map() -> std::io::Result<()> {
    println!("Map command not yet implemented.");
    Ok(())
}