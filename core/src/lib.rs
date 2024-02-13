use crate::reminders::read_all_reminders;
use crate::task::collect_and_run_tasks;
use crate::watcher::gen_watcher_receiver;
use notify::{RecursiveMode, Watcher};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
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
pub fn configure_toml_file(dir: &Path) -> anyhow::Result<PathBuf> {
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

// important note:
// the actual entry function (main()) cannot be marked by tokio
// or else daemonize will NOT WORK!
// source: https://stackoverflow.com/questions/76042987/having-problem-in-rust-with-tokio-and-daemonize-how-to-get-them-to-work-togethe
// instead, this function contains all the program logic
// and is marked as tokio's entry point

#[tokio::main]
pub async fn run(file: &Path) -> anyhow::Result<()> {
    let (mut debouncer, mut rx) = gen_watcher_receiver()?;
    debouncer
        .watcher()
        .watch(file, RecursiveMode::NonRecursive)?;

    let mut reminders = read_all_reminders(file)?;
    let mut tasks = collect_and_run_tasks(reminders.clone());
    loop {
        // at the moment, we don't care about what the message is
        // we just need to wait for a change to happen
        let _ = rx.recv().await.unwrap();
        // now that we know there's been a change, restart tasks

        let new_reminders = read_all_reminders(file)?;
        let mut hasher = DefaultHasher::new();
        let to_abort: Vec<_> = reminders
            .iter()
            .filter(|r| !new_reminders.contains(r))
            .map(|reminder| {
                reminder.hash(&mut hasher);
                hasher.finish()
            })
            .collect();

        for (handle, hash) in &tasks {
            if to_abort.iter().any(|abort_hash| abort_hash == hash) {
                handle.abort();
                println!("aborted a task: {hash}");
            }
        }

        let to_start: Vec<_> = new_reminders
            .iter()
            .filter(|x| !reminders.contains(*x))
            .cloned()
            .collect();
        //
        tasks = collect_and_run_tasks(to_start);
        reminders = new_reminders;
    }
}
