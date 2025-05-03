mod commands;

use clap::Parser;
use commands::Commands;

#[derive(Parser)]
#[command(name = "miso", version = "0.1", about = "Local password manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create {
            label,
            password,
            force,
        } => match commands::create(label.clone(), password, force) {
            Ok(()) => println!("Successfully created password for '{}'", label),
            Err(e) => eprintln!("Error: {}", e),
        },
        Commands::Get { label, copy } => match commands::get(label.clone()) {
            Ok(pass) => {
                if copy {
                    let mut clipboard = arboard::Clipboard::new().unwrap_or_else(|e| {
                        eprintln!("Clipboard error: {}", e);
                        std::process::exit(1);
                    });
                    clipboard.set_text(&pass).unwrap_or_else(|e| {
                        eprintln!("Failed to copy to clipboard: {}", e);
                        std::process::exit(1);
                    });
                    println!("Password for '{}' copied to clipboard", &label);
                } else {
                    println!("Password for '{}': {}", label, pass);
                }
            }
            Err(_) => {
                println!("No password found for '{}'", label)
            }
        },
        Commands::List => match commands::list() {
            Ok(labels) => {
                if labels.is_empty() {
                    println!("No saved passwords found");
                } else {
                    println!("Saved passwords:");
                    for label in labels {
                        println!("- {}", label);
                    }
                }
            }
            Err(_) => {
                println!("Unexpected error occured when fetching passwords.");
            }
        },
        Commands::Delete { label } => match commands::delete(label.clone()) {
            Ok(_) => {
                println!("Successfully deleted password for '{}'", &label);
            }
            Err(_) => {
                println!("Failed to delete password for '{}'", label);
            }
        },
        Commands::Search { query } => match commands::search(query.clone()) {
            Ok(matches) => {
                if matches.is_empty() {
                    println!("No matches found for '{}'", &query)
                } else {
                    println!("Matches found:");
                    for label in matches {
                        println!("- {}", label);
                    }
                }
            }
            Err(_) => {
                println!("Unexpected error occured when searching for '{}'", query);
            }
        },
    }
}
