use notify_rust::Notification;
use std::time::Duration;
use tokio::{task::JoinHandle, time::sleep};

use crate::{get_hashes, reminders::Reminder};

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

/// Sends a desktop notification on the interval specified by `reminder`
pub async fn start_reminder_task(reminder: Reminder) -> anyhow::Result<()> {
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
