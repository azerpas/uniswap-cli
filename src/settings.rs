use std::fs;
use std::io::prelude::*;
use anyhow::{Result, Context};
use directories::UserDirs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    webhook: String,
    password: String,
}

pub fn get_password_from_settings() -> Result<String> {
    let user_dirs = UserDirs::new().context("Failed to get user directories")?;
    let mut path = user_dirs.home_dir().to_path_buf();
    path = path.join(".uniswap-cli/settings.json");
    let file_content = fs::read_to_string(&path).context("Failed to read settings file")?;

    let settings: Settings = serde_json::from_str(&file_content)
        .map_err(|e| anyhow::anyhow!("Failed to deserialize settings: {}", e))?;
    Ok(settings.password)
}

pub fn save_settings_to_file(settings: &Settings) -> Result<()> {
    let user_dirs = UserDirs::new().context("Failed to get user directories")?;
    let mut path = user_dirs.home_dir().to_path_buf();
    path = path.join(".uniswap-cli");
    fs::create_dir_all(&path)?;
    path = path.join("settings.json");

    let json = serde_json::to_string_pretty(settings)
        .map_err(|e| anyhow::anyhow!("Failed to serialize settings: {}", e))?;

    let mut file = fs::File::create(&path)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}
