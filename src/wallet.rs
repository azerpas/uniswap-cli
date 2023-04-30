use std::{
    fs::{create_dir, File},
    io::{stdin, stdout, Write},
    path::{Path, PathBuf},
};

use crate::{crypto::{decrypt, encrypt, nonce}};
use anyhow::{Context, Result};
use directories::ProjectDirs;
use ethers::{
    prelude::{k256::ecdsa::SigningKey},
    signers::{coins_bip39::English, MnemonicBuilder, Wallet},
};

/// Ask the user for a password to encrypt/decrypt the mnemonic
/// 
/// ## Returns
/// The password as a string
fn get_password() -> Result<String> {
    let password =
        rpassword::prompt_password("Your password: ").context("Failed to read password")?;

    Ok(password.trim().to_string())
}

/// Get the mnemonic from the encrypted file and decrypt it with the password
/// given by the user
/// 
/// ## Returns
/// The Wallet ready to be used by ethers-rs
pub fn decrypt_wallet_data(password: Option<String>) -> Result<Wallet<SigningKey>> {
    let mut path = get_path_to_directory();
    path = path.join("mnemonic.enc");
    let encrypted_data = match std::fs::read(path.clone()) {
        Ok(data) => data,
        Err(_) => match save_mnemonic(path) {
            Ok(data) => data,
            Err(e) => return Err(e),
        },
    };
    let password: String = match password {
        Some(password) => password,
        None => get_password()?,
    };
    let decrypted = decrypt(bs58::decode(&encrypted_data).into_vec().unwrap(), password)?;
    let ph = std::str::from_utf8(&decrypted).unwrap();
    let index = 0u32;
    let wallet = MnemonicBuilder::<English>::default()
        .phrase(ph)
        .index(index)?
        .build()?;
    Ok(wallet)
}

/// Ask the user for a mnemonic/seed phrase/recovery phrase/... 
/// 
/// More informations here: https://www.ledger.com/academy/glossary/seed-phrase
/// 
/// ## Returns
/// The mnemonic as a string
fn ask_for_mnemonic() -> Result<String> {
    print!("Enter mnemonic: ");
    stdout().flush()?;

    let mut mnemonic = String::new();
    stdin().read_line(&mut mnemonic)?;

    Ok(mnemonic.trim().to_string())
}

/// Encrypt the mnemonic with the password given by the user and save it in a file
/// 
/// ## Arguments
/// * `path` - The path to the file where the mnemonic will be saved
/// 
/// ## Returns
/// The encrypted mnemonic as a vector of base58 encoded bytes
fn save_mnemonic(path: PathBuf) -> Result<Vec<u8>> {
    let mnemonic = ask_for_mnemonic().context("Could not read the mnemonic from stdin")?;
    let password = get_password().context("Could not read the password from stdin")?;
    let nonce = nonce().context("Could not generate a nonce")?;
    let encrypted_data =
        encrypt(mnemonic, password, nonce).context("Could not encrypt the mnemonic")?;
    let mut file = File::create(path).context("Could not create the mnemonic file")?;
    let data = bs58::encode(encrypted_data.clone()).into_vec();
    file.write_all(&data)
        .context("Could not write the mnemonic file")?;
    Ok(data)
}

/// Get the path to the configuration directory
/// 
/// ## Returns
/// The path to the configuration directory
pub fn get_path_to_directory() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("com", "azerpas", "dca-onchain") {
        let path: &Path = proj_dirs.config_dir();
        if !path.exists() {
            create_dir(path).expect("Could not create the configuration directory");
        }
        path.to_path_buf()
    } else {
        panic!("Could not retrieve the configuration path. Please raise an issue on GitHub with your configuration");
    }
}
