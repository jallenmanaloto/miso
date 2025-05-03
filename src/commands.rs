use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

use clap::Subcommand;
use keyring::Entry;

#[derive(Subcommand)]
pub enum Commands {
    Create {
        label: String,
        password: String,
        #[arg(short, long)]
        force: bool,
    },
    Get {
        label: String,
    },
    List,
    Delete {
        label: String,
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
    if labels.is_empty() {
        println!("No saved passwords found.");
    }
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
