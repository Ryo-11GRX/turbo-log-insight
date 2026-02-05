use anyhow::Result;
use clap::Parser;
// Import modules defined in lib.rs
use turbo_log_insight::config::Config;
use turbo_log_insight::run;

fn main() -> Result<()> {
    // 1. Parse arguments (instantiates ownership)
    let config = Config::parse();

    // 2. Execute application logic
    // Ownership of 'config' moves to the run function here.
    // Consequently, 'config' will no longer be accessible within the main function.
    if let Err(e) = run(config) {
        // Display error message to standard error (stderr)
        eprintln!("Application Error: {:#}", e);
        std::process::exit(1);
    }

    Ok(())
}