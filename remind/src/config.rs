use std::{
    fs::{self, File},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::get_project_dirs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub start_minimized: bool,
}

pub fn get_config_path() -> Result<PathBuf, std::io::Error> {
    let project_dir = get_project_dirs();
    let pref_dir = project_dir.preference_dir();

    let path = pref_dir.join("Config.toml");
    if !path.exists() {
        println!("didn't find an existing config toml file, creating an empty one...");
        File::create(&path)?;
    }
    Ok(path)
}

pub fn read_config() -> std::result::Result<Config, toml::de::Error> {
    let path = get_config_path().unwrap();
    let toml_str_content = fs::read_to_string(path).unwrap();
    toml::from_str::<Config>(&toml_str_content)
}
