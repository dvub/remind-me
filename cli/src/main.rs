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
    println!("remind-me CLI");
    println!();

    let path = get_path()?;

    // println!("{:?}", path.display());

    match args.command {
        Commands::Control { action } => match action {
            ControlCommands::Start => {
                run(path)?;
            }
        },
        Commands::Reminders { action } => match action {
            RemindersCommands::Path => {
                println!("Configuration path: {}", path.display());
            }
            RemindersCommands::List => {
                // this could be logged at a more verbose level
                // println!("Printing all current reminders...");
                let all_reminders = read_all_reminders(path)?;
                if all_reminders.is_empty() {
                    println!("No reminders found.");
                    return Ok(());
                }
                for (index, reminder) in all_reminders.iter().enumerate() {
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
                trigger_limit,
            } => {
                let reminder = Reminder {
                    name,
                    description,
                    frequency,
                    icon,
                    trigger_limit, // TODO: fix this
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
                trigger_limit,
            } => {
                let new_data = EditReminder {
                    name: new_name,
                    description,
                    frequency,
                    icon,
                    trigger_limit,
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
