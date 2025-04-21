use clap::{Parser, Subcommand};

/// Simple CLI tool template in Rust
#[derive(Parser)]
#[command(name = "git-manager")]
#[command(version = "0.1.0")]
#[command(about = "A CLI tool template in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Print hello world
    Hello,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Hello) => {
            println!("Hello, world!");
        }
        None => {
            println!("Use --help to see available commands.");
        }
    }
}
