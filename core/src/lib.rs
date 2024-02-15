use crate::reminders::read_all_reminders;
use crate::task::collect_and_run_tasks;
use crate::watcher::gen_watcher_receiver;
use notify::{RecursiveMode, Watcher};
use std::sync::Arc;
use std::{
    fs::{create_dir, File},
    path::{Path, PathBuf},
};
use tokio::sync::Mutex;

use directories::ProjectDirs;

pub mod daemon;
mod reminders;
mod task;
mod watcher;
// TODO: fix error propagation/handling in general
// its a shitshow right now
// TODO: more documentation
// TODO: testing

// fix pathbuf
pub fn get_dir() -> anyhow::Result<PathBuf> {
    // TODO:
    // fix this unwrap since its on an Option
    let project_dir = ProjectDirs::from("com", "dvub", "remind-me").unwrap();
    let data_dir = project_dir.data_dir();
    if !data_dir.exists() {
        println!("directory does not exist; creating data directory...");
        create_dir(data_dir)?;
    }
    Ok(data_dir.to_path_buf())
}

// call it db??
pub fn get_path() -> anyhow::Result<PathBuf> {
    let data_dir = get_dir()?;

    let path = data_dir.join("Config.toml");
    if !path.exists() {
        println!("didn't find an existing toml file, creating an empty one...");
        File::create(&path)?;
    }
    Ok(path)
}

// important note:
// the actual entry function (main()) cannot be marked by tokio
// or else daemonize will NOT WORK!
// source: https://stackoverflow.com/questions/76042987/having-problem-in-rust-with-tokio-and-daemonize-how-to-get-them-to-work-togethe
// instead, this function contains all the program logic
// and is marked as tokio's entry point

#[tokio::main]
pub async fn run(file: &Path) -> anyhow::Result<()> {
    let reminders = read_all_reminders(file)?;
    let arc = Arc::new(Mutex::new(reminders));
    let debouncer = gen_watcher_receiver(arc.clone(), file.to_path_buf());
    debouncer?
        .watcher()
        .watch(file, RecursiveMode::NonRecursive)?;
    collect_and_run_tasks(arc.clone()).unwrap();
    Ok(())
}
