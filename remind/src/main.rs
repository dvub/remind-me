use core::{collect_reminders_from_file, Reminder};
use daemonize::Daemonize;
use notify::{
    Config, Event, EventKind, INotifyWatcher, RecommendedWatcher, RecursiveMode, Watcher,
};
use notify_rust::Notification;
use std::{
    env,
    fs::{create_dir, File},
    path::{Path, PathBuf},
    time::Duration,
};
use tokio::{
    sync::mpsc::{channel, Receiver},
    task::JoinHandle,
    time::sleep,
};

// thank you kyillingene
// for helping me learn about async rust programming
// this would have taken hours without help

fn main() -> anyhow::Result<()> {
    let file = setup_config()?;
    let daemon = configure_daemon(&env::current_dir()?)?;
    //start_daemon(daemon, &file)?;
    run(&file);
    Ok(())
}
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
/// Uses the directory from `env::current_dir()`
/// to check for (or create) a configuration directory
/// which contains the toml file to read from.
/// This function returns a path to the toml file
pub fn setup_config() -> anyhow::Result<PathBuf> {
    println!();
    println!("initializing remind-me daemon...");
    println!();
    let config_dir_name = "config";
    let config_file_name = "Config.toml";
    // TODO:
    // should this be current_exe?
    let current_dir = env::current_dir()?;
    println!("current dir: {current_dir:?}");
    let config_dir = current_dir.join(config_dir_name);

    let file = config_dir.join(config_file_name);

    if !config_dir.exists() {
        println!("config directory does not exist, creating dir and config file");
        create_dir(&config_dir)?;
        File::create(&file)?;
    } else {
        println!("found an existing config directory.");
    }
    Ok(file)
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

// important note:
// the actual entry function (main()) cannot be marked by tokio
// or else daemonize will NOT WORK!
// source: https://stackoverflow.com/questions/76042987/having-problem-in-rust-with-tokio-and-daemonize-how-to-get-them-to-work-togethe
// instead, this function contains all the program logic
// and is marked as tokio's entry point

#[tokio::main]
async fn run(file: &Path) -> anyhow::Result<()> {
    let (mut watcher, mut rx) = gen_watcher_receiver()?;
    watcher.watch(file, RecursiveMode::NonRecursive)?;

    // let mut matching: Vec<Reminder> = Vec::new();
    loop {
        let reminders = collect_reminders_from_file(file)?;
        // TODO: don't clone here!
        let tasks = collect_and_run_tasks(reminders.clone());

        // at the moment, we don't care about what the message is
        // we just need to wait for a change to happen
        rx.recv().await.unwrap();
        // TODO: fix this
        // currently, the receiver can detect a file change and trigger a reload of all tasks
        // *before* the file has been rewritten (i think?)
        // so the collect function will read an empty file
        // i added this delay to (hopefully) finish modifying the file before reading
        // it seems to work for now
        sleep(Duration::from_millis(1000)).await;
        // now that we know there's been a change, restart tasks
        let new_reminders = collect_reminders_from_file(file)?;

        /*
        let added: Vec<_> = new_reminders
            .iter()
            .filter(|&item| !reminders.contains(item))
            .cloned()
            .collect();

        let removed: Vec<_> = reminders
            .iter()
            .filter(|&item| !new_reminders.contains(item))
            .cloned()
            .collect();
        */
        let changed: Vec<_> = reminders
            .iter()
            .zip(new_reminders.iter())
            .filter(|(old, new)| old != new)
            .map(|(_old, new)| new) // return new because we dont care about the old data
            .collect();

        //println!("added: {added:?}");
        //println!("rmd: {removed:?}");
        println!("changed: {:?}", changed);
        if !tasks.is_empty() {
            for task in &tasks {
                task.abort();
            }
        }

        // loop will restart, so tasks will restart
    }
}

fn collect_and_run_tasks(reminders: Vec<Reminder>) -> Vec<JoinHandle<anyhow::Result<()>>> {
    if reminders.is_empty() {
        println!("no reminders were round/read. WARNING: not spawning any tasks");
        return Vec::new();
    }

    println!("(re)starting reminders...");

    reminders
        .into_iter()
        .map(|reminder| tokio::spawn(async { run_reminder(reminder).await }))
        .collect()
}

// taken from the notify crate example here:
// https://github.com/notify-rs/notify/blob/main/examples/async_monitor.rs

// this is magical
/// Generates and returns a tuplet of a file watcher a receiver.
/// The watcher must be configured outside of this function to watch a file.
/// The receiver will receive a message anytime the target file is modified.
///
fn gen_watcher_receiver() -> anyhow::Result<(INotifyWatcher, Receiver<Event>)> {
    // buffer capacity of 1 should usually be enough
    let (tx, receiver) = channel(1);
    let watcher = RecommendedWatcher::new(
        move |result: Result<Event, notify::Error>| {
            // how can this error?
            match result {
                // we only want to send messages if the target was modified
                // this syntax is magical
                Ok(
                    event @ notify::Event {
                        kind: EventKind::Modify(_),
                        ..
                    },
                ) => {
                    println!("a file was modified, sending a message...");

                    tx.blocking_send(event).unwrap();
                }
                Ok(_) => {} //println!("another operation occurred, ignoring..."),
                Err(e) => println!("there was an error watching the file: {e}"),
            }
        },
        Config::default(),
    )?;

    Ok((watcher, receiver))
}
/// Sends a desktop notification on the interval specified by `reminder`
async fn run_reminder(reminder: Reminder) -> anyhow::Result<()> {
    println!("starting a new reminder: {}", &reminder.name);
    loop {
        sleep(Duration::from_secs(reminder.frequency as u64)).await;
        let icon = reminder.icon.clone().unwrap_or_default();
        println!("displaying reminder: {}", &reminder.name);
        Notification::new()
            .summary(&format!("{} Reminder: {}", icon, &reminder.name))
            .body(&reminder.description)
            .show()?;
    }
}
