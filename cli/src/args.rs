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
