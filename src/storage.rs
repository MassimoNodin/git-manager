use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{self, BufReader, BufWriter},
    path::PathBuf,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitAccount {
    pub profile_name: String,
    pub user_name: String,
    pub user_email: String,
}

fn get_storage_path() -> io::Result<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "YourName", "GitManager") {
        let config_dir = proj_dirs.config_dir();
        fs::create_dir_all(config_dir)?; // Ensure the directory exists
        Ok(config_dir.join("accounts.json"))
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Could not find config directory",
        ))
    }
}

pub fn load_accounts() -> io::Result<Vec<GitAccount>> {
    let path = get_storage_path()?;
    if !path.exists() {
        return Ok(Vec::new()); // Return empty list if file doesn't exist
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

pub fn save_accounts(accounts: &[GitAccount]) -> io::Result<()> {
    let path = get_storage_path()?;
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, accounts)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

pub fn add_account(account: GitAccount) -> io::Result<()> {
    let mut accounts = load_accounts()?;
    // Optional: Check for duplicate profile names
    if accounts
        .iter()
        .any(|a| a.profile_name == account.profile_name)
    {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("Profile '{}' already exists", account.profile_name),
        ));
    }
    accounts.push(account);
    save_accounts(&accounts)
}
