use crate::task::collect_and_run_tasks;
use crate::watcher::gen_watcher_receiver;

use notify::{RecursiveMode, Watcher};
use reminders::commands::read_all_reminders;
use reminders::Reminder;

use std::collections::hash_map::DefaultHasher;

use std::hash::{Hash, Hasher};

use std::path::PathBuf;

use directories::ProjectDirs;

pub mod config;
pub mod error;
pub mod reminders;
pub mod task;
pub mod watcher;

// TODO: fix error propagation/handling in general
// its a shitshow right now
// TODO: more documentation - in progress
// TODO: testing - huge improvements - in progress

// TODO: rename or fix
// right now this is a rather silly wrapper
// necessary because you can't have pub fns in lib/main marked with #[tauri::comand]

pub mod commands {
    use std::{
        fs::{create_dir_all, File},
        path::PathBuf,
    };

    use crate::{error::CommandError, get_project_dirs};

    #[tauri::command]
    #[specta::specta]
    // call it db??
    pub fn get_path() -> Result<PathBuf, CommandError> {
        // TODO: fix this unwrap since its on an Option
        let project_dir = get_project_dirs();
        let data_dir = project_dir.data_dir();
        if !data_dir.exists() {
            println!("directory does not exist; creating data directory...");
            create_dir_all(data_dir)?;
        }

        let path = data_dir.join("Reminders.toml");
        if !path.exists() {
            println!("didn't find an existing reminders.toml file, creating an empty one...");
            File::create(&path)?;
        }
        Ok(path)
    }
}

// TODO: this note can probably be removed since this codebase no longer uses daemonize.

// important note:
// the actual entry function (main()) cannot be marked by tokio
// or else daemonize will NOT WORK!
// source: https://stackoverflow.com/questions/76042987/having-problem-in-rust-with-tokio-and-daemonize-how-to-get-them-to-work-togethe
// instead, this function contains all the program logic
// and is marked as tokio's entry point

// thank you kyillingene
// for helping me learn about async rust programming
// this would have taken hours without help

#[tokio::main]
pub async fn run(path: PathBuf) -> anyhow::Result<()> {
    let (mut debouncer, mut rx) = gen_watcher_receiver()?;
    debouncer
        .watcher()
        .watch(&path, RecursiveMode::NonRecursive)?;

    let mut reminders = read_all_reminders(path.clone())?;
    let mut tasks = collect_and_run_tasks(reminders.clone());
    loop {
        // at the moment, we don't care about what the message is
        // we just need to wait for a change to happen
        let _ = rx.recv().await.unwrap();
        // now that we know there's been a change, restart tasks

        let new_reminders = read_all_reminders(path.clone())?;

        let reminders_to_abort: Vec<_> = reminders
            .iter()
            .filter(|r| !new_reminders.contains(r))
            .collect::<Vec<_>>();
        let to_abort = get_hashes(reminders_to_abort);

        let indices_to_remove: Vec<usize> = tasks
            .iter()
            .enumerate()
            .filter_map(|(index, (handle, hash))| {
                if to_abort.contains(hash) {
                    handle.abort();
                    println!("aborted a task: {}", hash);
                    Some(index)
                } else {
                    None
                }
            })
            .collect();

        for &index in indices_to_remove.iter().rev() {
            tasks.remove(index);
        }

        let to_start: Vec<_> = new_reminders
            .iter()
            .filter(|x| !reminders.contains(*x))
            .cloned()
            .collect();
        //
        tasks.append(&mut collect_and_run_tasks(to_start));
        reminders = new_reminders;
    }
}
/// Hashes a vec of reminders. (this was abstracted as such for consistency)
pub fn get_hashes(reminders: Vec<&Reminder>) -> Vec<u64> {
    reminders
        .iter()
        .map(|reminder| {
            let mut hasher = DefaultHasher::new();
            reminder.hash(&mut hasher);
            hasher.finish()
        })
        .collect::<Vec<_>>()
}

fn get_project_dirs() -> ProjectDirs {
    ProjectDirs::from("com", "dvub", "remind-me").unwrap()
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_data_path_exists() {
        assert!(super::commands::get_path().is_ok());
    }
}
