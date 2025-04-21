use clap::{Args, Parser, Subcommand};
use std::process; // For exiting with error code

mod storage; // Declare the storage module
use storage::{add_account, load_accounts, GitAccount};

/// Simple CLI tool template in Rust
#[derive(Parser)]
#[command(name = "git-manager")]
#[command(version = "0.1.0")]
#[command(about = "A CLI tool to manage git user configurations", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands, // Changed: Command is now required
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new git account profile
    Add(AddArgs),
    List,
    // Add other commands like Use, Remove later
}

#[derive(Args)]
struct AddArgs {
    /// A unique name for this profile (e.g., 'work', 'personal')
    #[arg(short, long)]
    profile: String,

    /// The git user.name for this profile
    #[arg(short, long)]
    name: String,

    /// The git user.email for this profile
    #[arg(short, long)]
    email: String,
}

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Add(args) => {
            let new_account = GitAccount {
                profile_name: args.profile.clone(),
                user_name: args.name.clone(),
                user_email: args.email.clone(),
            };
            match add_account(new_account) {
                Ok(_) => {
                    println!("Profile '{}' added successfully.", args.profile);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Error adding profile: {}", e);
                    Err(e)
                }
            }
        }
        Commands::List => match load_accounts() {
            Ok(accounts) => {
                if accounts.is_empty() {
                    println!("No accounts configured yet. Use 'add' command to add one.");
                } else {
                    println!("Available git account profiles:");
                    for account in accounts {
                        println!(
                            "- {}: {} <{}>",
                            account.profile_name, account.user_name, account.user_email
                        );
                    }
                }
                Ok(())
            }
            Err(e) => {
                eprintln!("Error loading accounts: {}", e);
                Err(e)
            }
        },
    };

    if result.is_err() {
        process::exit(1); // Exit with a non-zero code if any command failed
    }
}
