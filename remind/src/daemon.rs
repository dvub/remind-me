use core::collect_reminders_from_file;
use daemonize::Daemonize;
use notify::{RecursiveMode, Watcher};
use std::fs::File;
use std::path::Path;

use crate::task::collect_and_run_tasks;
use crate::watcher::gen_watcher_receiver;

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
