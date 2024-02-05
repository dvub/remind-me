use clap::{Parser, Subcommand};

// TODO:
// adequately document this shit lol

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Subcommand)]
pub enum Commands {
    Control {
        #[command(subcommand)]
        action: ControlCommands,
    },
    Reminders {
        #[command(subcommand)]
        action: RemindersCommands,
    },
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
    IsRunning,
    Start {
        #[arg(short, long)]
        force: bool,
    },
    Stop,
}
