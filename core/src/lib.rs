// TODO: more documentation
// TODO: testing

use serde::Deserialize;
use std::fs;
use std::path::Path;
use sysinfo::{System, IS_SUPPORTED_SYSTEM};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct Reminder {
    pub name: String,
    pub description: String,
    pub frequency: i32,
    pub icon: Option<String>,
}

// this struct may be used for any other configuration
// if needed in the future
#[derive(Debug, Deserialize, Clone)]
pub struct AllReminders {
    pub reminders: Vec<Reminder>,
}

pub fn collect_reminders_from_file(file: &Path) -> anyhow::Result<Vec<Reminder>> {
    // read the target file and parse them into a data structure
    println!("reading configuration file for reminders...");
    let toml_str = fs::read_to_string(file)?;
    // println!("File content: {:?}", toml_str);

    if toml_str.is_empty() {
        return Ok(Vec::new());
    }
    let res: AllReminders = toml::from_str(&toml_str)?;
    let reminders = res.reminders;

    println!("successfully read file into memory...");
    Ok(reminders)
}

pub fn is_daemon_running(process_name: &str) -> bool {
    let system = System::new_all();
    !system
        .processes_by_name(process_name)
        .collect::<Vec<_>>()
        .is_empty()
}

// #[cfg(test)]
mod tests {
    use std::{env, process::Command};

    use super::Reminder;

    #[test]
    fn read_from_file() {
        let dir = env::current_dir().unwrap();
        let res = super::collect_reminders_from_file(&dir.join("Test.toml")).unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].name, "Hello, world!");
    }

    #[test]
    fn is_daemon_running() {
        let process = "htop";
        let mut handle = Command::new(process).spawn().unwrap();
        assert!(super::is_daemon_running(process));
        handle.kill().unwrap();
    }
}
