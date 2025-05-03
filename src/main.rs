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
        Commands::Get { label } => {
            commands::get(label);
        }
        Commands::List => match commands::list() {
            Ok(labels) => {
                println!("Saved passwords:");
                for label in labels {
                    println!("- {}", label);
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
    }
}
