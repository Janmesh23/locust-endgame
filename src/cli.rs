use clap :: {Parser, Subcommand};
#[derive(Parser)]
#[command(name = "Locust", version, author, about)]

pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
/// Start logging location every 30 mins
    Start,
/// List recent logged locations
    List,
/// Generate map.html and open it
    Map,
    /// Show config values
    Config,

}
