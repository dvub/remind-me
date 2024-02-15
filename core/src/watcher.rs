use notify::{EventKind, INotifyWatcher};
use notify_debouncer_full::{new_debouncer, DebouncedEvent, Debouncer, FileIdMap};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use tokio::sync::{
    mpsc::{channel, Receiver},
    Mutex,
};

use crate::reminders::{read_all_reminders, Reminder};

// taken from the notify crate example here:
// https://github.com/notify-rs/notify/blob/main/examples/async_monitor.rs

// this is magical

///
/// Generates and returns a tuplet of a file watcher a receiver.
/// The watcher must be configured outside of this function to watch a file.
/// The receiver will receive a message anytime the target file is modified.
///
pub fn gen_watcher_receiver(
    reminders: Arc<Mutex<Vec<Reminder>>>,
    path: PathBuf,
) -> anyhow::Result<Debouncer<INotifyWatcher, FileIdMap>> {
    let handler_closure = move |result: Result<Vec<DebouncedEvent>, _>| match result {
        Ok(debounced_events) => {
            for event in debounced_events {
                if let EventKind::Modify(_) = event.kind {
                    let mut locked = reminders.blocking_lock();
                    *locked = read_all_reminders(&path).unwrap();
                }
            }
        }
        Err(e) => {
            println!("there was an error reading debounced changes: {e:?}")
        }
    };
    // note that the debouncer must be returned
    // so that it's not dropped (and stops sending to the receiver)

    // TODO:
    // find correct timeout
    // maybe set a better tickrate lol
    let debouncer = new_debouncer(
        Duration::from_secs(1),
        Some(Duration::from_millis(500)),
        handler_closure,
    )?;
    Ok(debouncer)
}
