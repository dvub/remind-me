use clap::Parser;
use core::{
    daemon::control::{is_daemon_running, start_daemon},
    get_dir,
};
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
                println!("checking if the remind daemon is running...");
                let is_running = is_daemon_running(&get_dir().unwrap());
                match is_running {
                    true => {
                        println!("the daemon is running.");
                    }
                    false => println!("the daemon is not running."),
                }
            }
            ControlCommands::Start => {
                let is_running = is_daemon_running(&get_dir().unwrap());
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
            ControlCommands::Stop => todo!(),
        },
        Commands::Auth { action: _ } => todo!(),
        Commands::Reminders { action: _ } => todo!(),
    }
    Ok(())
}
