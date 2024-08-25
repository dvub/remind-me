use clap::{Parser, Subcommand};

// half of this file is just documentation
// but the CLI is very well-explained now, at least. hahah

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
    /// Commands for managing the remind-me backend.
    Control {
        #[command(subcommand)]
        action: ControlCommands,
    },
    /// Manage reminders through the CLI. Create, edit, or delete reminders. (Alternatively, edit the file manually.)
    Reminders {
        #[command(subcommand)]
        action: RemindersCommands,
    },
}

#[derive(Subcommand)]
pub enum RemindersCommands {
    /// Add a new reminder. All arguments except for `icon` and `trigger_limit` are required.
    Add {
        /// The name of the new reminder. This will be the name you see in notifications, etc. - so make it human-friendly.
        #[arg(long)]
        name: String,
        /// The reminder's description.
        #[arg(long)]
        description: String,
        /// The frequency, *in seconds*, that this reminder will trigger. Make sure to calculate out hours/minutes to seconds!
        #[arg(long)]
        frequency: i32,
        /// An optional string which could be an emoji, etc. to help represent the reminder.
        /// In desktop notifications, this is prepended to the name in the notification title.
        #[arg(long)]
        icon: Option<String>,
        /// Optionally, supply a limit for how many times this reminder will trigger. If nothing is supplied, the reminder will trigger indefinitely.
        #[arg(long)]
        trigger_limit: Option<i32>,
    },
    /// Update a reminder. Only the name of the existing reminder is required; every other argument is optional.
    Update {
        /// The name of the *existing* reminder to update.
        #[arg(long)]
        name: String,
        /// Optionally, you may update the reminder's name.
        #[arg(long)]
        new_name: Option<String>,
        /// The reminder's new description.
        #[arg(long)]
        description: Option<String>,
        /// The reminder's new frequency (in seconds).
        #[arg(long)]
        frequency: Option<i32>,
        /// The reminder's new icon.
        #[arg(long)]
        icon: Option<String>,
        /// The new max number of times this reminder may trigger.
        #[arg(long)]
        trigger_limit: Option<i32>,
    },
    /// Deletes a reminder, removing its entry from the TOML file specified by the `path` subcommand.
    Delete {
        #[arg(long)]
        name: String,
    },
    /// Prints all reminders. A specific message will be displayed if no reminders were found. (i.e, if the TOML file specified by the `path` subcommand is empty)
    List,
    /// Displays the path to the TOML file containing reminders.
    Path,
}

#[derive(Subcommand)]
pub enum ControlCommands {
    /// Runs the program.
    Start,
}
