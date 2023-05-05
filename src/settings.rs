use std::fs;
use std::io::prelude::*;
use anyhow::{Result, Context};
use directories::UserDirs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    webhook: Option<String>,
    password: Option<String>,
}

/// Get the password from the settings file
/// 
/// ## Returns
/// The password as a string if it exists
pub fn get_password_from_settings() -> Result<Option<String>> {
    let user_dirs = UserDirs::new().context("Failed to get user directories")?;
    let mut path = user_dirs.home_dir().to_path_buf();
    path = path.join(".uniswap-cli/settings.json");
    let file_content = match fs::read_to_string(&path) {
        Ok(data) => data,
        Err(_) => {
            // Create the settings file if it doesn't exist
            save_settings_to_file(&Settings { webhook: None, password: None })?;
            return Ok(None);
        },
    };

    let settings: Settings = serde_json::from_str(&file_content)
        .map_err(|e| anyhow::anyhow!("Failed to deserialize settings: {}\nPlease make sure the settings file is valid.", e))?;
    Ok(settings.password)
}

/// Save the settings to user home directory
/// 
/// ## Arguments
/// * `settings` - The settings to save
pub fn save_settings_to_file(settings: &Settings) -> Result<()> {
    let user_dirs = UserDirs::new().context("Failed to get user directories")?;
    let mut path = user_dirs.home_dir().to_path_buf();
    path = path.join(".uniswap-cli");
    fs::create_dir_all(&path)?;
    path = path.join("settings.json");

    let json = serde_json::to_string_pretty(settings)
        .map_err(|e| anyhow::anyhow!("Failed to serialize settings and create settings: {}", e))?;

    let mut file = fs::File::create(&path)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}
