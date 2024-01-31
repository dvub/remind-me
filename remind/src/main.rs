use core::{collect_reminders_from_file, Reminder};
use daemonize::Daemonize;
use notify::{
    Config, Event, EventKind, INotifyWatcher, RecommendedWatcher, RecursiveMode, Watcher,
};
use notify_debouncer_full::{new_debouncer, DebouncedEvent, Debouncer, FileIdMap};
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
        let to_abort: Vec<_> = reminders
            .iter()
            .filter(|x| !new_reminders.contains(x))
            .collect();

        println!();
        println!("stopping the following tasks: {:?}", to_abort);
        println!();

        for (handle, task_info) in &tasks {
            if to_abort
                .iter()
                .any(|abort_task_info| task_info == *abort_task_info)
            {
                handle.abort();
                println!("aborted a task");
            }
        }

        let to_start: Vec<_> = new_reminders
            .iter()
            .filter(|x| !reminders.contains(*x))
            .cloned()
            .collect();

        println!();
        println!("starting the following tasks: {:?}", to_start);
        println!();
        //
        tasks = collect_and_run_tasks(to_start);
        reminders = new_reminders;
    }
}

fn collect_and_run_tasks(
    reminders: Vec<Reminder>,
) -> Vec<(JoinHandle<anyhow::Result<()>>, Reminder)> {
    if reminders.is_empty() {
        println!("no reminders were round/read. WARNING: not spawning any tasks");
        return Vec::new();
    }

    println!("(re)starting reminders...");

    reminders
        .into_iter()
        .map(|reminder| (tokio::spawn(run_reminder(reminder.clone())), reminder))
        .collect()
}

// taken from the notify crate example here:
// https://github.com/notify-rs/notify/blob/main/examples/async_monitor.rs

// this is magical
/// Generates and returns a tuplet of a file watcher a receiver.
/// The watcher must be configured outside of this function to watch a file.
/// The receiver will receive a message anytime the target file is modified.
///
fn gen_watcher_receiver() -> anyhow::Result<(
    Debouncer<INotifyWatcher, FileIdMap>,
    Receiver<DebouncedEvent>,
)> {
    let (tx, receiver) = channel(1);
    let debouncer = new_debouncer(
        Duration::from_secs(1),
        None,
        move |result: Result<Vec<DebouncedEvent>, _>| match result {
            Ok(e) => {
                for t in e {
                    match t.kind {
                        EventKind::Modify(_) => {
                            println!("Modification occurred");
                            tx.blocking_send(t).unwrap();
                        }
                        _ => {
                            println!("Something happened that I don't care about")
                        }
                    }
                }
            }
            Err(e) => {
                println!("there was an error reading debounced changes: {e:?}")
            }
        },
    )?;

    Ok((debouncer, receiver))
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
