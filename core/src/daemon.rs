use crate::collect_reminders_from_file;
use crate::task::collect_and_run_tasks;
use crate::watcher::gen_watcher_receiver;
use daemonize::Daemonize;
use notify::{RecursiveMode, Watcher};
use std::collections::hash_map::DefaultHasher;

use std::fs::File;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
// thank you kyillingene
// for helping me learn about async rust programming
// this would have taken hours without help

// TODO:
// testing
// unify project dir instead of calling it in individual files

pub mod control {
    use std::{fs::File, io::Read, path::Path};

    use sysinfo::{Pid, System};

    use crate::get_dir;

    use super::{configure_daemon, configure_toml_file, run};

    /// Takes a `Daemonize` and a target config file. Runs the program as a daemon
    /// reading reminders from the target config file.
    pub fn start_daemon() -> anyhow::Result<()> {
        let dir = get_dir()?;
        let path = configure_toml_file(&dir)?;
        let daemon = configure_daemon(&dir)?;
        match daemon.start() {
            Ok(_) => {
                run(&path)?;
            }
            Err(e) => eprintln!("there was an error starting the daemon: {e}"),
        }
        Ok(())
    }
    // TODO:
    // maybe there's a better way to determine if daemon is running?
    // fix error handlling FFS
    pub fn is_daemon_running(dir: &Path) -> bool {
        let path = dir.join("remind.pid");
        let mut file = File::open(path).unwrap();
        // TODO:
        // check if file even exists

        // this feels very scuffed
        let mut str = String::new();
        file.read_to_string(&mut str).unwrap();
        str = str.trim().to_owned();

        let u = str.parse::<u32>().unwrap();
        let system = System::new_all();
        system.process(Pid::from_u32(u)).is_some()
    }
    // TODO: implement stop
}

// important note:
// the actual entry function (main()) cannot be marked by tokio
// or else daemonize will NOT WORK!
// source: https://stackoverflow.com/questions/76042987/having-problem-in-rust-with-tokio-and-daemonize-how-to-get-them-to-work-togethe
// instead, this function contains all the program logic
// and is marked as tokio's entry point

#[tokio::main]
async fn run(file: &Path) -> anyhow::Result<()> {
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
fn configure_daemon(dir: &Path) -> anyhow::Result<Daemonize<()>> {
    println!("configuring daemon...");
    let stdout = File::create(dir.join("daemon.out"))?;
    let stderr = File::create(dir.join("daemon.err"))?;
    let daemonize = Daemonize::new()
        .stdout(stdout)
        .stderr(stderr)
        .pid_file(dir.join("remind.pid"));
    // .working_directory(dir);
    Ok(daemonize)
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
