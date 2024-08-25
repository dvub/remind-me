use notify_rust::Notification;
use std::time::Duration;
use tokio::{task::JoinHandle, time::sleep};

use crate::{get_hashes, reminders::Reminder};

/// Takes a `Vec<Reminder>` and spawns a task to run each Reminder.
/// returns a `Vec` of tuples of handles to the spawned tasks and hashes of each Reminder
/// (so that it can be determined which handles to abort, etc.)
pub fn collect_and_run_tasks(
    reminders: Vec<Reminder>,
) -> Vec<(JoinHandle<anyhow::Result<()>>, u64)> {
    if reminders.is_empty() {
        println!("no reminders were round/read. WARNING: not spawning any tasks");
        return Vec::new();
    }

    println!("(re)starting reminders...");
    // TODO: fix this LOL

    let hashes: Vec<u64> = get_hashes(reminders.iter().collect());
    reminders
        .into_iter()
        .map(|reminder| tokio::spawn(start_reminder_task(reminder)))
        .zip(hashes.iter().copied())
        .collect()
}

// this is the function that actually sends desktop notifications!!

// TODO: should this take a &Reminder?

/// Sends a desktop notification on the interval specified by `Reminder`
pub async fn start_reminder_task(reminder: Reminder) -> anyhow::Result<()> {
    println!("starting a new reminder: {}", &reminder.name);

    // if there's a trigger limit specified, we need a for loop
    if let Some(max_n) = reminder.trigger_limit {
        for _ in 0..max_n {
            sleep(Duration::from_secs(reminder.frequency as u64)).await;
            send_notification(&reminder)?;
        }
        // TODO: decide whether to delete reminder after completion
    } else {
        // if no trigger limit is specified, just use an infinite loop
        loop {
            sleep(Duration::from_secs(reminder.frequency as u64)).await;
            send_notification(&reminder)?;
        }
    };
    Ok(())
}
// this abstraction is stupid lol
fn send_notification(reminder: &Reminder) -> anyhow::Result<()> {
    let icon = reminder.icon.clone().unwrap_or_default();
    println!("displaying reminder: {}", &reminder.name);
    Notification::new()
        .summary(&format!("{} Reminder: {}", icon, &reminder.name))
        .body(&reminder.description)
        .show()?;

    // TODO figure out what to do with this
    Ok(())
}
