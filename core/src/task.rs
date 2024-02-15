use notify_rust::Notification;
use std::{sync::Arc, time::Duration};
use tokio::{sync::Mutex, time::sleep};

use crate::reminders::Reminder;

pub fn collect_and_run_tasks(reminders: Arc<Mutex<Vec<Reminder>>>) -> anyhow::Result<()> {
    reminders
        .blocking_lock()
        .iter()
        .enumerate()
        .for_each(|(i, _)| {
            tokio::spawn(start_reminder_task(reminders.clone(), i));
        });

    Ok(())
}

/// Sends a desktop notification on the interval specified by `reminder`
pub async fn start_reminder_task(
    reminders: Arc<Mutex<Vec<Reminder>>>,
    index: usize,
) -> anyhow::Result<()> {
    loop {
        let r = &reminders.lock().await[index];
        sleep(Duration::from_secs(r.frequency as u64)).await;
        let reminder = &reminders.lock().await[index];
        let icon = reminder.icon.clone().unwrap_or_default();
        println!("displaying reminder: {}", &reminder.name);
        Notification::new()
            .summary(&format!("{} Reminder: {}", icon, &reminder.name))
            .body(&reminder.description)
            .show()?;
    }
}
