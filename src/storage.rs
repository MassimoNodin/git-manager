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

// New struct to hold all storage data
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct StorageData {
    pub accounts: Vec<GitAccount>,
    pub selected_profile: Option<String>,
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

// Renamed and updated to load StorageData
pub fn load_storage() -> io::Result<StorageData> {
    let path = get_storage_path()?;
    if !path.exists() {
        return Ok(StorageData::default()); // Return default empty storage if file doesn't exist
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

// Renamed and updated to save StorageData
pub fn save_storage(data: &StorageData) -> io::Result<()> {
    let path = get_storage_path()?;
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, data)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

// Updated to use StorageData
pub fn add_account(account: GitAccount) -> io::Result<()> {
    let mut storage = load_storage()?;
    if storage
        .accounts
        .iter()
        .any(|a| a.profile_name == account.profile_name)
    {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("Profile '{}' already exists", account.profile_name),
        ));
    }
    storage.accounts.push(account);
    save_storage(&storage)
}

// New function to get the list of accounts
pub fn get_accounts() -> io::Result<Vec<GitAccount>> {
    load_storage().map(|data| data.accounts)
}

// New function to get the selected profile name
pub fn get_selected_profile() -> io::Result<Option<String>> {
    load_storage().map(|data| data.selected_profile)
}

// New function to set the selected profile name
pub fn set_selected_profile(profile_name: &str) -> io::Result<()> {
    let mut storage = load_storage()?;
    // Check if the profile exists before setting it
    if !storage
        .accounts
        .iter()
        .any(|a| a.profile_name == profile_name)
    {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Profile '{}' not found", profile_name),
        ));
    }
    storage.selected_profile = Some(profile_name.to_string());
    save_storage(&storage)
}
