use clap::{Args, Parser, Subcommand};
use std::process; // For exiting with error code

mod storage; // Declare the storage module
use storage::{
    add_account, get_accounts, get_selected_profile, set_selected_profile, GitAccount,
};

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
    /// List existing git account profiles
    List,
    /// Set the active git account profile
    Use(UseArgs),
    // Add other commands like Remove later
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

#[derive(Args)]
struct UseArgs {
    /// The name of the profile to activate
    profile_name: String,
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
        Commands::List => {
            match (get_accounts(), get_selected_profile()) {
                (Ok(accounts), Ok(selected)) => {
                    if accounts.is_empty() {
                        println!("No accounts configured yet. Use 'add' command to add one.");
                    } else {
                        println!("Available git account profiles:");
                        let selected_name = selected.as_deref(); // Get &str or None
                        for account in accounts {
                            let marker = if Some(account.profile_name.as_str()) == selected_name {
                                "*" // Mark the selected profile
                            } else {
                                "-"
                            };
                            println!(
                                "{} {}: {} <{}>",
                                marker, account.profile_name, account.user_name, account.user_email
                            );
                        }
                        if selected_name.is_none() {
                            println!("\nNo profile currently selected. Use 'use <profile_name>' to select one.");
                        }
                    }
                    Ok(())
                }
                (Err(e), _) | (_, Err(e)) => {
                    eprintln!("Error loading data: {}", e);
                    Err(e)
                }
            }
        }
        Commands::Use(args) => {
            match set_selected_profile(&args.profile_name) {
                Ok(_) => {
                    println!("Successfully set active profile to '{}'.", args.profile_name);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Error setting profile: {}", e);
                    Err(e)
                }
            }
        }
    };

    if result.is_err() {
        process::exit(1); // Exit with a non-zero code if any command failed
    }
}
