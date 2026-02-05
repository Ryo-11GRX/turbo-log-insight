use clap::Parser;

/// Program configuration (generated from CLI arguments)
/// Uses owned `String` instead of borrowed `&str` to ensure data persistence.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// The keyword to search for
    #[arg(short, long)]
    pub query: String,

    /// Path to the log file to be analyzed
    #[arg(short, long)]
    pub file_path: String,
}