// Expose module
pub mod config;

use anyhow::{Context, Result};
use config::Config;
use std::fs;

/// Core execution logic of the application
pub fn run(config: Config) -> Result<()> {
    // Read the file
    // The '?' operator returns Err immediately if an error occurs.
    // .with_context() appends information about which file caused the failure.
    let content = fs::read_to_string(&config.file_path)
        .with_context(|| format!("Failed to read file: {}", config.file_path))?;

    println!("--------------------------------------------------");
    println!("Search target: {}", config.query);
    println!("Target file:   {}", config.file_path);
    println!("--------------------------------------------------");

    // Simple search logic (to be optimized for performance later)
    for (i, line) in content.lines().enumerate() {
        if line.contains(&config.query) {
            println!("{}: {}", i + 1, line);
        }
    }

    Ok(())
}