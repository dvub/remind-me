use std::{fs::create_dir, path::PathBuf};

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
