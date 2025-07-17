
# 📍 Locust: Geolocation Logger & Visualizer

Locust is a robust and user-friendly command-line interface (CLI) application built with Rust. Its primary function is to periodically fetch your public IP address's geographical location, log this data persistently, and then provide powerful visualization tools to display your tracked locations on an interactive world map. This includes both a heatmap for density analysis and a chronological path to trace movement.

## ✨ Features

Locust offers a suite of features designed for efficient and insightful geolocation tracking:

  * **Automated IP Geolocation Tracking**: At a user-defined interval, Locust automatically sends a request to a public geolocation API (`http://ip-api.com/json` by default) to determine your current public IP address's geographical coordinates (latitude, longitude) along with associated city and country information.
  * **Configurable Logging Interval**: The frequency of location logging is fully customizable. By default, Locust logs your location every 30 minutes, but this can be easily adjusted via the `config.json` file to suit your tracking needs (e.g., every 60 seconds for more granular data).
  * **Persistent Data Storage**: All fetched geolocation entries are appended to a `JSON Lines` (`.jsonl`) formatted file (default: `locations.jsonl`). This ensures that your historical location data is preserved across application runs and can be easily processed by other tools. Each line in this file is a self-contained JSON object.
  * **Recent Log Listing**: A dedicated CLI command allows you to quickly retrieve and display the last 10 (or fewer, if not enough exist) logged location entries directly in your terminal. This provides a convenient overview of your most recent movements.
  * **Interactive Map Visualization**: Locust generates a self-contained `index.html` file within a `map/` directory. This HTML file, when opened in a web browser, renders an interactive world map, powered by Leaflet.js, showcasing your tracked locations:
      * **Geolocation Heatmap**: A visual representation of location density. Areas where more points have been logged (indicating more time spent or more frequent logging) appear with higher intensity and a **darker red color**. The heatmap uses a custom color gradient (`['red', 'darkred', 'rgb(139, 0, 0)']`) for a distinct visual.
      * **Chronological Track Line**: A clear **blue polyline** connects all logged coordinates in the exact order they were recorded. This feature allows you to visually trace your movement path over time. The line has a `weight` (thickness) of `3` pixels and an `opacity` (transparency) of `0.7`.
      * **Automatic Map Centering**: The generated map intelligently zooms and pans to automatically fit all your logged points within the view, ensuring your entire track is visible without manual adjustment.
  * **Robust Error Handling**: The application is designed to handle common errors gracefully. Instead of crashing due to file I/O issues, network problems, or malformed data, it provides informative error messages to the user via `eprintln!` and attempts to recover or exit cleanly.
  * **Flexible Configuration**: All key parameters, such as the logging interval, the log file path, and the geolocation API URL, can be easily overridden by creating an optional `config.json` file in the project's root directory.

-----

## 💻 Technologies Used

Locust is built on a modern and robust technology stack, primarily centered around the Rust programming language and its rich ecosystem.

### 🦀 Rust (The Core Language)

  * **Role in Locust**: Rust forms the bedrock of the entire application. It's used for:
      * **CLI Logic**: Defining and parsing command-line arguments.
      * **Network Requests**: Making efficient, low-level HTTP requests to the geolocation API.
      * **File I/O**: Reading from and writing to the log file (`.jsonl`) and generating the HTML map file.
      * **Data Processing**: Structuring, serializing, and deserializing location data.
      * **Overall Application Logic**: Orchestrating the flow between different modules and handling errors. Its compile-time safety features contribute significantly to the reliability and stability of Locust.

### 🚀 Tokio (Asynchronous Runtime for Rust)

  * **Description**: Tokio is the de facto standard asynchronous runtime for Rust. It provides the necessary infrastructure for writing fast, reliable, and highly concurrent network applications. This includes an event loop, a task scheduler, and asynchronous versions of I/O primitives (like `tokio::fs` for async file operations, though `std::fs` is used for simplicity in some parts of Locust).
  * **Role in Locust**: `tokio` is crucial for handling the non-blocking nature of network requests. The `#[tokio::main]` attribute macro on the `main` function sets up the Tokio runtime, allowing `async` functions like `fetch_location` and `start_logging` to use the `await` keyword. This means Locust can initiate a network request and then pause its execution *without blocking the entire program*, allowing the operating system to perform other tasks until the network response is ready.

### 🤝 Clap (Command-Line Argument Parser)

  * **Description**: `clap` (Command-Line Argument Parser) is a popular, feature-rich, and easy-to-use library for building command-line interfaces in Rust. It allows developers to define their CLI structure (commands, subcommands, arguments, flags) declaratively using Rust structs and enums with procedural macros (`#[derive(Parser)]`, `#[derive(Subcommand)]`). It automatically generates help messages, validates input, and handles common CLI patterns.
  * **Role in Locust**: `clap` is fundamental to Locust's user interaction. It defines the application's top-level commands (`start`, `list`, `map`, `config`) and their descriptions. This makes Locust intuitive to use from the terminal, providing clear instructions and handling argument parsing seamlessly.

### 📦 Serde & Serde JSON (Serialization/Deserialization Framework)

  * **Description**: `Serde` (SERialize/DEserialize) is a powerful, generic serialization framework for Rust. It allows Rust data structures to be converted into (serialized) and from (deserialized) various data formats. `serde_json` is a crate that provides specific support for the JSON data format, leveraging the `serde` framework.
  * **Role in Locust**: `serde` and `serde_json` are vital for data persistence and exchange:
      * **Logging**: The `LocationEntry` struct is annotated with `#[derive(Serialize)]`, enabling `serde_json::to_string()` to effortlessly convert a Rust `LocationEntry` instance into a JSON string, which is then written to `locations.jsonl`.
      * **Reading Logs**: The `LocationEntry` struct is also annotated with `#[derive(Deserialize)]`, allowing `serde_json::from_str()` to parse a JSON string from the log file back into a `LocationEntry` struct for display or map generation.

### 🌐 Reqwest (Asynchronous HTTP Client)

  * **Description**: `reqwest` is a robust and user-friendly HTTP client for Rust. It provides a high-level API for making HTTP requests (GET, POST, etc.) and handling responses, supporting both blocking and non-blocking (asynchronous) operations. It integrates well with `tokio`.
  * **Role in Locust**: `reqwest` is the core component for network communication. The `fetch_location` function uses `reqwest::Client` to send GET requests to the `ip-api.com` endpoint. It handles the underlying TCP connections, sending HTTP headers, and receiving the response body, which is then parsed as JSON.

### ⏳ Chrono (Date and Time Library)

  * **Description**: `chrono` is a comprehensive and accurate date and time library for Rust. It provides various types for representing dates, times, and durations, along with utilities for parsing, formatting, and performing calculations. It supports different time zones, including UTC.
  * **Role in Locust**: `chrono` is used to precisely timestamp each location entry. `chrono::Utc::now()` captures the current Coordinated Universal Time (UTC) at the moment the location is fetched. This ensures consistency and accuracy of timestamps in the `locations.jsonl` file, regardless of the user's local machine time zone.

### 🌍 Open (Cross-Platform File Opener)

  * **Description**: The `open` crate is a lightweight, cross-platform utility that simplifies the task of opening files or URLs using the operating system's default application. For example, it can open a `.txt` file in Notepad, a `.pdf` in a PDF viewer, or an `.html` file in the default web browser.
  * **Role in Locust**: In the `generate_map` function, the `open::that("map/index.html")?` call is used to automatically launch the user's default web browser and display the newly generated interactive map, providing a seamless user experience.

### 🗺️ Leaflet.js (Interactive Maps - JavaScript Library)

  * **Description**: Leaflet.js is a leading open-source JavaScript library specifically designed for building mobile-friendly interactive maps. It's known for being lightweight (around 40 KB JS), easy to use, and highly extensible with a vast ecosystem of plugins. It handles map tiles, markers, popups, and basic map interactions.
  * **Role in Locust**: Leaflet.js is embedded directly into the `map/index.html` file. It's responsible for:
      * Initializing the map (`L.map`).
      * Adding the base map tiles from OpenStreetMap (`L.tileLayer`).
      * Handling user interactions like zooming and panning.
      * Providing the foundation upon which the `Leaflet.heat` plugin and `L.polyline` are built.

### 🔥 Leaflet.heat (Heatmap Plugin for Leaflet - JavaScript Library)

  * **Description**: `Leaflet.heat` is a small, fast, and simple Leaflet plugin for visualizing large datasets of points as a heatmap. It's built on top of `simpleheat` and provides an efficient way to show data density on a map.
  * **Role in Locust**: This plugin is crucial for the visual analysis of your logged locations. It consumes the `[latitude, longitude]` data points generated by the Rust application and renders them as a heatmap on the Leaflet map. The `colors` option is specifically configured to display a gradient from `red` to `darkred` (`rgb(139, 0, 0)`), highlighting areas with higher concentrations of logged points.

-----

## 🛠️ Setup

To get Locust up and running on your system, follow these steps:

### Prerequisites

You need to have **Rust and Cargo** installed on your machine. If you don't already have them, the recommended way to install is via `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen instructions. After installation, you might need to restart your terminal or run `source "$HOME/.cargo/env"` to ensure Cargo's binaries are in your PATH.

### Installation Steps

1.  **Clone the Repository**:
    Open your terminal and clone the Locust GitHub repository:

    ```bash
    git clone https://github.com/Janmesh23/locust.git
    cd locust
    ```

    This command will create a `locust/` directory and navigate you into it.

2.  **Verify Project Structure**:
    Ensure your project directory structure matches the expected layout. This is important for Rust's module system to find all source files.

    ```
    locust/
    ├── Cargo.toml          # Rust project manifest and dependencies
    ├── .gitignore          # Specifies files/directories to ignore in Git
    ├── config.json         # Optional: for custom configuration settings
    ├── locations.jsonl     # Created by the app; stores logged geolocation data
    └── src/                # Contains all Rust source code files
        ├── main.rs         # Main entry point of the application
        ├── cli.rs          # Defines command-line interface structure
        ├── logger.rs       # Handles logging, listing, and map generation logic
        ├── config.rs       # Manages application configuration loading
        └── location.rs     # Responsible for fetching geolocation data
    ```

3.  **Build the Project**:
    While in the `locust/` directory (where `Cargo.toml` is located), build the project. This command will automatically download all necessary dependencies (as defined in `Cargo.toml`) and compile your Rust code into an executable binary.

    ```bash
    cargo build --release
    ```

    *The `--release` flag is highly recommended for production use, as it compiles an optimized binary that runs significantly faster and is smaller in size compared to a debug build.*

-----

## 🚀 Usage

Locust is operated through simple command-line subcommands. Navigate to your `locust/` project directory in your terminal to run these commands.

### 1\. `start` - Begin Geolocation Logging

This command initiates the continuous logging of your public IP's geolocation. It will run indefinitely until manually stopped.

```bash
cargo run -- start
```

  * **Behavior**: Locust will periodically query the configured geolocation API (default: `http://ip-api.com/json`). Each successful fetch will be appended as a new JSON line to your `locations.jsonl` file (or the `log_path` specified in `config.json`).
  * **Output**: You will see console messages indicating when a location is logged or if fetching failed.
  * **Stopping**: Press `Ctrl + C` in the terminal where `start` is running to terminate the logging process.

### 2\. `list` - View Recent Logs

This command displays a summary of the most recent geolocation entries from your log file.

```bash
cargo run -- list
```

  * **Behavior**: It reads the `locations.jsonl` file (or your custom `log_path`) and prints the last 10 entries.
  * **Output**: Each entry will show the timestamp, latitude, longitude, and (if available) city and country.
  * **Error Handling**: If the log file does not exist or cannot be read, an informative message will be displayed.

### 3\. `map` - Generate and Open Interactive Map

This command processes all logged geolocation data and creates an interactive HTML map visualization.

```bash
cargo run -- map
```

  * **Behavior**:
    1.  Reads all entries from your `locations.jsonl` file.
    2.  Extracts latitude and longitude for each entry.
    3.  Generates an `index.html` file within a new `map/` subdirectory. This HTML file includes embedded JavaScript that uses Leaflet.js and Leaflet.heat to render the map.
    4.  Automatically opens `map/index.html` in your system's default web browser.
  * **Visualization**: The map will display:
      * A **heatmap** (in shades of red) showing the density of your logged points.
      * A **blue polyline** connecting all points in chronological order, illustrating your movement path.
      * The map will automatically zoom and pan to encompass all your logged locations.
  * **Error Handling**: If the log file is missing or corrupted, an error message will be shown, and the map generation will not proceed.

### 4\. `config` - Show Current Configuration

This command displays the configuration values that Locust is currently using.

```bash
cargo run -- config
```

  * **Behavior**: It loads the configuration (from `config.json` if present, otherwise using defaults from `config.rs`) and prints the `interval`, `log_path`, and `api_url`.
  * **Purpose**: Useful for verifying your settings and understanding how the application will behave.

### Customizing Configuration (`config.json` - Optional)

Locust allows you to override its default settings by creating a `config.json` file in the root `locust/` directory (the same directory as `Cargo.toml`). If this file exists, Locust will load its values and merge them with the built-in defaults.

**Example `config.json` for common customizations**:

```json
{
    "interval": 60,                     // Log location every 60 seconds (1 minute)
    "log_path": "my_custom_logs.jsonl", // Store logs in 'my_custom_logs.jsonl'
    "api_url": "http://ip-api.com/json" // Use the default geolocation API
}
```

  * **`interval` (number)**: Sets the delay in seconds between consecutive location fetches.
      * Default: `1800` (30 minutes).
      * Example: `60` for logging every minute.
  * **`log_path` (string)**: Specifies the filename for storing location data.
      * Default: `"locations.jsonl"`.
      * Example: `"travel_history.jsonl"` to keep different logs separated.
  * **`api_url` (string)**: Defines the endpoint for the geolocation API.
      * Default: `"http://ip-api.com/json"`.
      * **Tracking a Specific Public IP**: You can modify this to query the location of a specific **public IP address** (e.g., your other device's current public IP, if it's static or you update it manually) by appending the IP to the URL:
        ```json
        "api_url": "http://ip-api.com/json/203.0.113.45"
        ```
        **Important Note**: This method tracks the location of the *specified IP address*, not necessarily the device running Locust. For dynamic IPs (like mobile devices), this requires manual updates to `config.json` whenever the target IP changes. For consistent tracking of a mobile device, consider running Locust directly on that device.

### Clearing Log Data

To start fresh with your geolocation logs, simply delete the log file that Locust is currently using:

```bash
rm locations.jsonl
# Or if you configured a different log_path in config.json:
# rm your_custom_log_file.jsonl
```

On Windows, use `del locations.jsonl` in Command Prompt or `Remove-Item locations.jsonl` in PowerShell. After deleting, run `cargo run -- start` again to begin a new log file.

-----

## 🤝 Contributing

Contributions, issues, and feature requests are highly welcome\! If you have ideas for improvements, encounter bugs, or want to add new functionalities, please feel free to:

  * **Report Issues**: Use the [GitHub issues page](https://github.com/Janmesh23/locust/issues) to report any bugs or suggest new features.
  * **Submit Pull Requests**: If you've implemented a fix or a new feature, feel free to open a pull request.

-----



**Developed by Janmesh**

  * GitHub Profile: [https://github.com/Janmesh23](https://github.com/Janmesh23)
  * Project Repository: [https://github.com/Janmesh23/locust](https://github.com/Janmesh23/locust)

-----
