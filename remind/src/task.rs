use core::Reminder;

use notify_rust::Notification;
use std::time::Duration;
use tokio::{task::JoinHandle, time::sleep};

pub fn collect_and_run_tasks(
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

/// Sends a desktop notification on the interval specified by `reminder`
pub async fn run_reminder(reminder: Reminder) -> anyhow::Result<()> {
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
