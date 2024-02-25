use clap::Parser;
use remind::{
    commands::get_path,
    reminders::{
        commands::{add_reminder, delete_reminder, edit_reminder, read_all_reminders},
        EditReminder, Reminder,
    },
    run,
};
mod args;

use args::{Args, Commands, ControlCommands};

use crate::args::RemindersCommands;
fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!();
    println!("remind-me CLI - dvub");
    println!();
    let path = get_path()?;
    println!("{:?}", path.display());

    match args.command {
        Commands::Control { action } => match action {
            ControlCommands::Start => {
                run(path)?;
            }
        },
        Commands::Reminders { action } => match action {
            RemindersCommands::Path => {
                println!("{}", path.display());
            }
            RemindersCommands::List => {
                println!("Printing all current reminders...");
                let all = read_all_reminders(path)?;
                for (index, reminder) in all.iter().enumerate() {
                    println!();
                    println!("{}. {}", index + 1, reminder.name);
                    println!("Description: {}", reminder.description);
                    println!("Frequency: {} seconds", reminder.frequency);
                }
            }
            RemindersCommands::Add {
                name,
                description,
                frequency,
                icon,
            } => {
                let reminder = Reminder {
                    name,
                    description,
                    frequency,
                    icon,
                };
                println!("Adding a reminder...");
                add_reminder(path, reminder)?;
                println!("sucessfully added a new reminder.");
            }
            RemindersCommands::Update {
                name,
                new_name,
                description,
                frequency,
                icon,
            } => {
                let new_data = EditReminder {
                    name: new_name,
                    description,
                    frequency,
                    icon,
                };
                let res = edit_reminder(path.clone(), &name, new_data)?;
                match res {
                    0 => {
                        println!("didn't find a reminder with that name. Nothing was changed");
                    }
                    n => {
                        println!("Modified {n} reminder(s)");
                    }
                }
            }
            // TODO:
            // confirmation
            RemindersCommands::Delete { name } => {
                println!("Deleting reminder {}", name);
                let res = delete_reminder(path, &name)?;
                match res {
                    0 => {
                        println!("didn't find a reminder with that name. Nothing was deleted");
                    }
                    n => {
                        println!("Deleted {n} reminder(s)");
                    }
                }
            }
        },
    }
    Ok(())
}
