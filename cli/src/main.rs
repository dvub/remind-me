use clap::Parser;
use core::daemon::{is_daemon_running, start_daemon};
mod args;

use args::{Args, Commands, ControlCommands};
fn main() -> anyhow::Result<()> {
    println!();
    println!("remind-me CLI - dvub");
    println!();
    let args = Args::parse();

    match args.command {
        Commands::Control { action } => match action {
            ControlCommands::IsRunning => {
                println!("Checking if the remind daemon is running...");
                // TODO:
                // fix the process name
                // i.e. determine that dynamically
                let is_running = is_daemon_running();
                match is_running {
                    true => {
                        println!("The daemon is running.");
                        println!("note: attempting to start the daemon will do nothing unless the force option is used");
                    }
                    false => println!("The daemon is not running."),
                }
            }
            ControlCommands::Start { force } => {
                println!("Checking if the remind daemon is running...");
                // TODO:
                // fix the process name
                // i.e. determine that dynamically
                let is_running = is_daemon_running();
                match is_running {
                    true => {
                        println!("the daemon is running");
                        if force {
                            println!("force option is enabled, starting another instance...");
                            start_daemon()?;
                        } else {
                            println!("exiting...")
                        }
                    }
                    false => {
                        println!("The daemon is not running, starting...");
                        start_daemon()?;
                    }
                }
            }
            ControlCommands::Stop => todo!(),
        },
        Commands::Auth { action: _ } => todo!(),
        Commands::Reminders { action: _ } => todo!(),
    }
    Ok(())
}
