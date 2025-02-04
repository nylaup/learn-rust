use std::fs;
use std::path::Path;
use std::error::Error;
use std::process;

use chrono::{DateTime, Local};

fn main() {
    if let Err(ref e) = run(Path::new(".")) {
        println!("{}", e);
        process::exit(1);
    }
}

fn run(dir: &Path) -> Result<(), Box<dyn Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let file_name = entry
                .file_name()
                .into_string()
                .or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
            let metadata = entry.metadata()?;
            let modified: DateTime<Local> = DateTime::from(metadata.modified()?);
            println!("{} - {}", modified.format("%_d %b %H:%M").to_string(), file_name);
        }
    }
    Ok(())
}
