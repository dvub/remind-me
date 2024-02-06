pub mod daemon;
mod task;
mod watcher;
// TODO: fix error propagation/handling in general
// its a shitshow right now
// TODO: more documentation
// TODO: testing

use directories::ProjectDirs;
use serde::Deserialize;
use std::fs::{self, create_dir};
use std::path::{Path, PathBuf};

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
// TODO:
// fix PathBuf return
pub fn get_dir() -> anyhow::Result<PathBuf> {
    // TODO:
    // fix this unwrap since its on an Option
    let project_dir = ProjectDirs::from("com", "dvub", "remind-me").unwrap();
    let data_dir = project_dir.data_dir();
    if !data_dir.exists() {
        println!("configuring data directory...");
        create_dir(data_dir)?;
    }
    Ok(data_dir.to_path_buf())
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

// why test no work :(
// #[cfg(test)]
mod tests {
    #[test]
    fn read_from_file() {
        use std::{fs::File, io::Write};
        use tempfile::tempdir;

        let one_reminder = b"[[reminders]]
        name = \"Hello, world!\"
        description = \"...\"
        frequency = 0
        ";
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path().join("Test.toml");
        let mut test_file = File::create(&path).unwrap();
        test_file.write_all(one_reminder).unwrap();
        let res = super::collect_reminders_from_file(&path).unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].name, "Hello, world!");
        drop(test_file);
        temp_dir.close().unwrap();
    }
}
