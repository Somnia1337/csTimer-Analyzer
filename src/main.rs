use std::io::{self, Write};
use std::time::Instant;

use env_logger::{Builder, Env};
use log::{error, info};

use cstimer_analyzer::analyze::*;
use cstimer_analyzer::files::*;
use cstimer_analyzer::options::*;

/// Waits for an ENTER press to keep the console alive.
fn wait() {
    print!("Press ENTER to continue...");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Reads and parses options
    let options_file_path = "options.txt";
    let options = read_txt(options_file_path).map_err(|e| {
        format!(
            "Failed to read options from \"{}\": {}",
            options_file_path, e
        )
    })?;
    let options = sanitize_options(options);
    info!("Imported options from \"{}\"", options_file_path);

    let (options, ob_flavor) = parse_options(options);
    info!(
        "Parsed {} options and {} Obsidian-flavored Markdown",
        options.len(),
        if ob_flavor { "ENABLED" } else { "DISABLED" }
    );

    // Reads data
    info!("Matching csTimer data file");

    let data_file_path =
        match_data_file().map_err(|e| format!("Failed to match data file: {}", e))?;

    info!("Selected data file \"{}\"", data_file_path);

    let data = read_txt(&data_file_path)
        .map_err(|e| format!("Failed to read data from \"{}\": {}", data_file_path, e))?;

    info!("Imported data");

    // Splits sessions
    info!("Parsing sessions and records");

    let start_time = Instant::now();
    let sessions = split_sessions(&data);

    info!(
        "[{:.2?}] Parsed {} sessions ({} records)",
        start_time.elapsed(),
        sessions.len(),
        sessions.iter().map(|s| s.records().len()).sum::<usize>(),
    );

    // Analyzes sessions
    info!("Analyzing data");

    let start_time = Instant::now();
    analyze(&sessions, &options, &data_file_path, ob_flavor)
        .map_err(|e| format!("Failed to analyze sessions: {}", e))?;

    info!("[{:.2?}] Generated analysis", start_time.elapsed());

    Ok(())
}

fn main() {
    // Initializes logger
    let env = Env::default().filter_or("RUST_LOG", "info");
    Builder::from_env(env).init();

    // Sets a panic hook to catch any unexpected panics
    std::panic::set_hook(Box::new(|panic_info| {
        error!("cstimer-analyzer panicked: {}", panic_info);
        wait();
    }));

    // Catches any error
    if let Err(e) = run() {
        error!("{}", e);
    }

    // Keeps the console alive
    wait();
}
