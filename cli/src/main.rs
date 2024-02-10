use clap::Parser;
use colored::Colorize;
use core::{
    daemon::control::{is_daemon_running, start_daemon, stop_daemon},
    get_dir,
};
mod args;

use args::{Args, Commands, ControlCommands};

use crate::args::RemindersCommands;
fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!();
    println!("remind-me CLI - dvub");
    println!();

    match args.command {
        Commands::Control { action } => match action {
            ControlCommands::IsRunning => {
                println!("checking if the remind daemon is running...");
                let is_running = is_daemon_running().unwrap();
                match is_running {
                    true => {
                        println!("the daemon is running.");
                    }
                    false => println!("the daemon is not running."),
                }
            }
            ControlCommands::Start => {
                let is_running = is_daemon_running().unwrap();
                match is_running {
                    true => {
                        println!("error: the daemon is running; multiple instances are not supported at this time. ");
                    }
                    false => {
                        println!("the daemon is not running; starting...");
                        // BOOM SUPER IMPORTANT RIGHT HERE!!
                        start_daemon()?;
                    }
                }
            }
            ControlCommands::Stop => {
                let is_running = is_daemon_running().unwrap();
                match is_running {
                    true => {
                        println!("stopping... ");
                        stop_daemon().unwrap();
                        println!("successfully stopped daemon.");
                    }
                    false => {
                        println!("the daemon is not running; doing nothing...");
                    }
                }
            }
        },
        Commands::Reminders { action } => match action {
            RemindersCommands::Add => todo!(),
            RemindersCommands::Update => todo!(),
            RemindersCommands::Delete => todo!(),
        },
    }
    Ok(())
}
