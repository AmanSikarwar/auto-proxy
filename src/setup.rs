use std::error::Error;
use std::fs::{self, File};
use std::io::prelude::*;

fn do_setup() -> Result<(), Box<dyn Error>> {
    create_config_files()?;
    Ok(())
}

fn check_config_files() -> Result<(), Box<dyn Error>> {
    let config_dir = dirs::home_dir().unwrap().join(".auto-proxy/config");
    let config_file = config_dir.join("config.yaml");
    let profiles_file = config_dir.join("proxy-profiles.yaml");

    if !config_file.exists() || !profiles_file.exists() {
        return Err("Config files not found".into());
    }

    Ok(())
}

fn create_config_files() -> std::io::Result<()> {
    let config_dir = dirs::home_dir().unwrap().join(".auto-proxy/config");
    fs::create_dir_all(&config_dir)?;

    let config_file = config_dir.join("config.yaml");
    let mut file = File::create(&config_file)?;
    file.write_all(b"config file content")?;

    let profiles_file = config_dir.join("proxy-profiles.yaml");
    let mut file = File::create(&profiles_file)?;
    file.write_all(b"proxy profiles content")?;

    Ok(())
}
