use clap::{Args, Parser, Subcommand};
use std::io; // Import io for error handling
use std::process::{self, Command, Stdio}; // Import Command, Stdio, and the process module itself

mod storage; // Declare the storage module
use storage::{
    add_account, get_account_by_profile_name, get_accounts, get_selected_profile,
    set_selected_profile, GitAccount,
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

fn run_git_config(key: &str, value: &str) -> io::Result<()> {
    let status = Command::new("git")
        .arg("config")
        .arg("--global") // Apply globally
        .arg(key)
        .arg(value)
        .stdout(Stdio::null()) // Suppress git command output
        .stderr(Stdio::null()) // Suppress git command errors (we'll handle based on status)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Failed to execute 'git config --global {} \"{}\"'. Is git installed and in PATH?",
                key, value
            ),
        ))
    }
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
                    println!(
                        "Successfully set active profile to '{}' in storage.",
                        args.profile_name
                    );
                    match get_account_by_profile_name(&args.profile_name) {
                        Ok(Some(account)) => {
                            println!(
                                "Applying git config: user.name='{}', user.email='{}'",
                                account.user_name, account.user_email
                            );
                            if let Err(e) = run_git_config("user.name", &account.user_name) {
                                eprintln!("Error setting git user.name: {}", e);
                                Err(e)
                            } else if let Err(e) = run_git_config("user.email", &account.user_email) {
                                eprintln!("Error setting git user.email: {}", e);
                                Err(e) // Propagate the error
                            } else {
                                println!("Git config updated successfully.");
                                Ok(())
                            }
                        }
                        Ok(None) => {
                            eprintln!(
                                "Internal error: Profile '{}' set but could not be retrieved.",
                                args.profile_name
                            );
                            Err(io::Error::new(io::ErrorKind::NotFound, "Profile inconsistency"))
                        }
                        Err(e) => {
                            eprintln!("Error retrieving profile details: {}", e);
                            Err(e)
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error setting profile: {}", e);
                    if e.kind() == std::io::ErrorKind::InvalidData {
                        eprintln!("Hint: The storage file might be corrupted. You may need to delete or fix it manually.");
                        eprintln!(
                            "Storage file location: {:?}",
                            storage::get_storage_path().unwrap_or_default()
                        );
                    }
                    Err(e)
                }
            }
        }
    };

    if result.is_err() {
        process::exit(1); // Exit with a non-zero code if any command failed
    }
}
