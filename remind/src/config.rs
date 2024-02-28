use std::{
    fs::{self, create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::{error::CommandError, get_project_dirs};
use specta::Type;
#[derive(Debug, Serialize, Deserialize, Type)]
pub struct Config {
    pub start_minimized: bool,
    pub run_backend_on_gui_start: bool,
}

#[tauri::command]
#[specta::specta]
pub fn get_config_path() -> Result<PathBuf, CommandError> {
    let project_dir = get_project_dirs();
    let pref_dir = project_dir.preference_dir();
    if !pref_dir.exists() {
        create_dir_all(pref_dir)?;
    }

    let path = pref_dir.join("Config.toml");
    if !path.exists() {
        println!("didn't find an existing config toml file, creating an empty one...");
        let mut f = File::create(&path)?;
        f.write_all(b"start_minimized = false\nrun_backend_on_gui_start = true")?;
    }
    Ok(path)
}

#[tauri::command]
#[specta::specta]
pub fn read_config(path: PathBuf) -> Result<Config, CommandError> {
    let toml_str_content = fs::read_to_string(path).unwrap();
    let res = toml::from_str::<Config>(&toml_str_content)?;
    Ok(res)
}

#[tauri::command]
#[specta::specta]
pub fn update_start_minimized(path: PathBuf, new_val: bool) -> Result<(), CommandError> {
    let toml_str_content = fs::read_to_string(&path).unwrap();
    let mut res = toml::from_str::<Config>(&toml_str_content)?;
    res.start_minimized = new_val;
    fs::write(&path, toml::to_string(&res).unwrap())?;
    Ok(())
}
#[tauri::command]
#[specta::specta]
pub fn update_run_backend_with_gui(path: PathBuf, new_val: bool) -> Result<(), CommandError> {
    let toml_str_content = fs::read_to_string(&path).unwrap();
    let mut res = toml::from_str::<Config>(&toml_str_content)?;
    res.run_backend_on_gui_start = new_val;
    fs::write(&path, toml::to_string(&res).unwrap())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, create_dir_all, File},
        io::Write,
    };

    use tempfile::tempdir;

    use super::Config;

    #[test]
    fn test_read_config() {
        let temp_dir = tempdir().unwrap();
        create_dir_all(temp_dir.path()).unwrap();
        let path = temp_dir.path().join("Test.toml");
        let mut file = File::create(&path).unwrap();
        file.write_all(b"start_minimized = true\nrun_backend_on_gui_start = false")
            .unwrap();
        let res = super::read_config(path).unwrap();
        assert!(res.start_minimized);
        assert!(!res.run_backend_on_gui_start);
    }

    #[test]
    fn update_values() {
        let temp_dir = tempdir().unwrap();
        create_dir_all(temp_dir.path()).unwrap();
        let path = temp_dir.path().join("Test.toml");
        let mut file = File::create(&path).unwrap();
        file.write_all(b"start_minimized = true\nrun_backend_on_gui_start = false")
            .unwrap();
        super::update_start_minimized(path.clone(), false).unwrap();
        let toml_str_content = fs::read_to_string(&path).unwrap();
        let res = toml::from_str::<Config>(&toml_str_content).unwrap();
        assert!(!res.start_minimized);
    }
}
