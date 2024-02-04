use core::collect_reminders_from_file;
use daemonize::Daemonize;
use notify::{RecursiveMode, Watcher};
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::path::Path;

use crate::task::collect_and_run_tasks;
use crate::watcher::gen_watcher_receiver;

// thank you kyillingene
// for helping me learn about async rust programming
// this would have taken hours without help

/// Takes a `Daemonize` and a target config file. Runs the program as a daemon
/// reading reminders from the target config file.
pub fn start_daemon(daemon: Daemonize<()>, config_file: &Path) -> anyhow::Result<()> {
    match daemon.start() {
        Ok(_) => {
            println!("successfully started daemon. reminders are now running in the background.");
        }
        Err(e) => eprintln!("there was an error starting the daemon: {e}"),
    }
    run(config_file)?;
    Ok(())
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

    let mut reminders = collect_reminders_from_file(file)?;
    let mut tasks = collect_and_run_tasks(reminders.clone());
    loop {
        // at the moment, we don't care about what the message is
        // we just need to wait for a change to happen
        let _ = rx.recv().await.unwrap();
        // now that we know there's been a change, restart tasks

        let new_reminders = collect_reminders_from_file(file)?;
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

/// Configure and return a `Daemonize<()>`.
pub fn configure_daemon(current_dir: &Path) -> anyhow::Result<Daemonize<()>> {
    let stdout = File::create("/tmp/daemon.out")?;
    let stderr = File::create("/tmp/daemon.err")?;

    // TODO: add more options
    let daemonize = Daemonize::new()
        .stdout(stdout)
        .stderr(stderr)
        .pid_file(current_dir.join("daemon.pid"))
        .working_directory(current_dir);
    Ok(daemonize)
}
/// Uses the directory from `env::current_dir()`
/// to check for (or create) a configuration directory
/// which contains the toml file to read from.
/// This function returns a path to the toml file
pub fn setup_file(file: &Path) -> anyhow::Result<()> {
    println!();
    println!("initializing remind-me daemon...");
    println!();
    /*
        let config_dir_name = "config";
        let config_file_name = "Config.toml";
        // TODO:
        // should this be current_exe?
        let current_dir = env::current_dir()?;
        println!("current dir: {current_dir:?}");
        let config_dir = current_dir.join(config_dir_name);

        let file = config_dir.join(config_file_name);
    */
    if !file.exists() {
        println!("config file does not exist, creating...");
        File::create(file)?;
    } else {
        println!("found an existing config file");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;
    #[test]
    fn test_setup_without_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("Test.toml");
        assert!(!path.exists());

        //
        super::setup_file(&path).unwrap();
        assert!(path.exists());
        dir.close().unwrap();
    }
}
