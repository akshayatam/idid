use chrono::Local;
use clap::{Parser, Subcommand};
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::TryLockResult;

const LOG_DIR: &str = "data";
const LOG_FILE: &str = "data/log.csv";

#[derive(Parser, Debug)]
#[command(
    name = "idid",
    version = "0.1.0",
    about = "A tiny CLI tool for logging app installs and removals"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Install {
        package: String,
        source: Option<String>,
    },
    Remove {
        package: String,
        source: Option<String>,
    },
    History,
}

fn main() {
    let cli = Cli::parse();

    if let Err(err) = ensure_log_file_exists() {
        eprintln!("Error preparing log file: {err}");
        std::process::exit(1);
    }

    match cli.command {
        Commands::Install { package, source } => {
            let source = source.unwrap_or_else(|| "Unknown".to_string());
            if let Err(err) = append_log("install", &package, &source) {
                eprintln!("Error logging install: {err}");
                std::process::exit(1);
            }
            println!("Logged install: {package} from {source}");
        }
        Commands::Remove { package, source } => {
            let source = source.unwrap_or_else(|| "Unknown".to_string());
            if let Err(err) = append_log("remove", &package, &source) {
                eprintln!("Error logging removal: {err}");
                std::process::exit(1);
            }
            println!("Logged removal: {package}");
        }
        Commands::History => {
            if let Err(err) = print_history() {
                eprintln!("Error reading history: {err}");
                std::process::exit(1);
            }
        }
    }
}

fn ensure_log_file_exists() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(LOG_DIR).exists() {
        std::fs::create_dir_all(LOG_DIR)?;
    }

    if !Path::new(LOG_FILE).exists() {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(LOG_FILE)?;

        let mut writer = BufWriter::new(file);
        writeln!(writer, "timestamp,action,package,source")?;
        writer.flush()?;
    }
    Ok(())
}

fn append_log(action: &str, package: &str, source: &str) -> Result<(), Box<dyn std::error::Error>> {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let file = OpenOptions::new().append(true).open(LOG_FILE)?;

    let mut writer = BufWriter::new(file);
    writeln!(
        writer,
        "{},{},{},{}",
        escape_csv_field(&timestamp),
        escape_csv_field(action),
        escape_csv_field(package),
        escape_csv_field(source),
    )?;
    writer.flush()?;
    Ok(())
}

fn print_history() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(LOG_FILE)?;
    println!("{content}");
    Ok(())
}

fn escape_csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}
