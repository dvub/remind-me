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
    /// Auth commands. Login, logout, etc.
    Auth {
        #[command(subcommand)]
        action: AuthCommands,
    },
}
#[derive(Subcommand)]
pub enum AuthCommands {
    Status,
    Login,
    Logout,
}

#[derive(Subcommand)]
pub enum RemindersCommands {
    Add,
    Update,
    Delete,
}

#[derive(Subcommand)]
pub enum ControlCommands {
    /// Prints if the daemon is running or not.
    IsRunning,
    /// Starts the daemon if it's not already running. Multiple instances are not supported at this time.
    Start,
    /// Stops the daemon if it is running.
    Stop,
}
