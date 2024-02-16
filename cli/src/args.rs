use clap::{Parser, Subcommand};

// TODO:
// adequately document this shit lol

/// CLI for Remind-me, a blazingly fast
/// desktop reminder app written in Rust.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The type of subcommand to run
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Subcommand)]
pub enum Commands {
    /// Commands for managing the remind-me daemon.
    /// Get info on daemon status, start, and stop daemon.
    Control {
        #[command(subcommand)]
        action: ControlCommands,
    },
    /// Manage reminders through the CLI. Create, edit, or delete reminders. Alternatively, edit the file manually.
    Reminders {
        #[command(subcommand)]
        action: RemindersCommands,
    },
}

#[derive(Subcommand)]
pub enum RemindersCommands {
    Add {
        #[arg(long)]
        name: String,
        #[arg(long)]
        description: String,
        #[arg(long)]
        frequency: i32,
        #[arg(long)]
        icon: Option<String>,
    },
    Update {
        #[arg(long)]
        name: String,
        #[arg(long)]
        new_name: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        frequency: Option<i32>,
        #[arg(long)]
        icon: Option<String>,
    },
    Delete {
        #[arg(long)]
        name: String,
    },
}

#[derive(Subcommand)]
pub enum ControlCommands {
    /// Prints if the daemon is running or not. Note this only works if the program was started as a daemon.
    IsRunning,
    /// Runs the program. Runs as a daemon if the option is enabled.
    Start {
        /// Runs the program as a daemon. NOTE: AFAIK this is only supported for linux.
        #[arg(long, short)]
        daemon: bool,
    },
    /// Stops the daemon if it is running. Note this only works if the program was started as a daemon.
    Stop,
}
