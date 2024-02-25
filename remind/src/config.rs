use std::{
    fs::{self, create_dir, File},
    io::Write,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::get_project_dirs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub start_minimized: bool,
    pub run_backend_on_gui_start: bool,
}

pub fn get_config_path() -> Result<PathBuf, std::io::Error> {
    let project_dir = get_project_dirs();
    let pref_dir = project_dir.preference_dir();
    if !pref_dir.exists() {
        create_dir(pref_dir)?;
    }

    let path = pref_dir.join("Config.toml");
    if !path.exists() {
        println!("didn't find an existing config toml file, creating an empty one...");
        let mut f = File::create(&path)?;
        f.write_all(b"start_minimized = true\nrun_backend_on_gui_start = true")?;
    }
    Ok(path)
}

pub fn read_config(path: &Path) -> std::result::Result<Config, toml::de::Error> {
    let toml_str_content = fs::read_to_string(path).unwrap();
    toml::from_str::<Config>(&toml_str_content)
}
#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use tempfile::tempdir;

    #[test]
    fn test_start_minimized() {
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path().join("Test.toml");
        let mut file = File::create(&path).unwrap();
        file.write_all(b"start_minimized = true\nrun_backend_on_gui_start = false")
            .unwrap();
        let res = super::read_config(&path).unwrap();
        assert!(res.start_minimized);
        assert!(!res.run_backend_on_gui_start);
    }
}
