use notify::{EventKind, INotifyWatcher};
use notify_debouncer_full::{new_debouncer, DebouncedEvent, Debouncer, FileIdMap};
use std::{
    env,
    fs::{create_dir, File},
    path::PathBuf,
    time::Duration,
};
use tokio::sync::mpsc::{channel, Receiver};

// taken from the notify crate example here:
// https://github.com/notify-rs/notify/blob/main/examples/async_monitor.rs

// this is magical
/// Generates and returns a tuplet of a file watcher a receiver.
/// The watcher must be configured outside of this function to watch a file.
/// The receiver will receive a message anytime the target file is modified.
///
pub fn gen_watcher_receiver() -> anyhow::Result<(
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
