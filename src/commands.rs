use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

use clap::Subcommand;
use keyring::Entry;

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new password for the given label
    Create {
        /// The label for this password (e.g., "github")
        label: String,

        /// The password to save
        password: String,

        /// Overwrite the password if it already exists
        #[arg(short, long)]
        force: bool,
    },

    /// Retrieve the password for a given label
    Get {
        /// The label to retrieve
        label: String,

        /// Copy the password to clipboard
        #[arg(short, long)]
        copy: bool,
    },

    /// List all saved password labels
    List,

    /// Delete the password for a given label
    Delete {
        /// The label to delete
        label: String,
    },

    /// Search saved labels using a keyword
    Search {
        /// The keyword to search for
        query: String,
    },
}

pub fn create(
    label: String,
    password: String,
    force: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let entry = Entry::new("miso", &label)?;
    let mut labels = load_labels()?;

    if labels.contains(&label) && !force {
        return Err(format!(
            "Password for '{}' already exists. Use --force to overwrite.",
            label
        )
        .into());
    }

    if !labels.contains(&label) {
        labels.push(label.clone());
        save_labels(&labels)?;
    }

    entry.set_password(&password)?;
    Ok(())
}

pub fn get(label: String) -> Result<String, Box<dyn std::error::Error>> {
    let entry = Entry::new("miso", &label)?;
    let pass = entry.get_password()?;
    return Ok(pass);
}

pub fn list() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let labels = load_labels()?;
    return Ok(labels);
}

pub fn delete(label: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut labels = load_labels()?;
    if !labels.contains(&label) {
        println!("Password for {} does not exist.", &label);
        return Ok(());
    }

    let entry = Entry::new("miso", &label)?;
    match entry.delete_password() {
        Ok(_) => {
            labels.retain(|l| l != &label);
            save_labels(&labels)?;
        }
        Err(_) => {
            println!("Failed to delete password for {}", &label);
        }
    }
    Ok(())
}

pub fn search(query: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let labels = load_labels()?;
    let matches: Vec<_> = labels
        .into_iter()
        .filter(|label| label.to_lowercase().contains(&query.to_lowercase()))
        .collect();

    Ok(matches)
}

fn labels_file() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("miso")
        .join("labels.json")
}

fn load_labels() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let path = labels_file();
    if !path.exists() {
        return Ok(Vec::new());
    }

    let mut file = fs::File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let labels: Vec<String> = serde_json::from_str(&content)?;
    Ok(labels)
}

fn save_labels(labels: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let path = labels_file();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    file.write_all(serde_json::to_string_pretty(labels)?.as_bytes())?;
    Ok(())
}
