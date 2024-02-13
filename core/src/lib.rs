use std::{
    fs::{create_dir, File},
    path::{Path, PathBuf},
};

use directories::ProjectDirs;

pub mod daemon;
mod reminders;
mod task;
mod watcher;
// TODO: fix error propagation/handling in general
// its a shitshow right now
// TODO: more documentation
// TODO: testing

// TODO:
// fix PathBuf return
pub fn get_dir() -> anyhow::Result<PathBuf> {
    // TODO:
    // fix this unwrap since its on an Option
    let project_dir = ProjectDirs::from("com", "dvub", "remind-me").unwrap();
    let data_dir = project_dir.data_dir();
    if !data_dir.exists() {
        println!("configuring data directory...");
        create_dir(data_dir)?;
    }
    println!("{:?}", data_dir);
    Ok(data_dir.to_path_buf())
}

/// Uses the directory from `env::current_dir()`
/// to check for (or create) a configuration directory
/// which contains the toml file to read from.
/// This function returns a path to the toml file
fn configure_toml_file(dir: &Path) -> anyhow::Result<PathBuf> {
    println!("configuring reminder file...");
    let path = dir.join("Config.toml");
    if !path.exists() {
        println!("didn't find an existing toml file, creating an empty one...");
        File::create(&path)?;
    } else {
        println!("found existing toml file")
    }
    Ok(path)
}
