mod cli;
mod config;
mod logger;
mod location; // Declare the location module

use cli::Cli;
use clap::Parser;

#[tokio::main] // Marks the main function as an asynchronous entry point using Tokio
async fn main() {
    let cli = Cli::parse(); // Parse command-line arguments into our Cli struct

    match cli.command {
        cli::Commands::Start => {
            logger::start_logging().await; // Call the asynchronous start_logging function
        }
        cli::Commands::List => {
            logger::list_logs(); // Call the (currently stubbed) list_logs function
        }
        cli::Commands::Map => {
            logger::generate_map().unwrap(); // Call the (currently stubbed) generate_map function
        }
        cli::Commands::Config => {
            config::print_config(); // Call the print_config function
        }
    }
}