use std::{
    fs::{self, create_dir, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use serde::Deserialize;

use crate::get_dir;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct Reminder {
    pub name: String,
    pub description: String,
    pub frequency: i32,
    pub icon: Option<String>,
}

impl Reminder {
    pub fn new(name: String, description: String, frequency: i32, icon: Option<String>) -> Self {
        Self {
            name,
            description,
            frequency,
            icon,
        }
    }
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

pub fn add_reminder(mut file: &File, reminder: Reminder) {
    // let mut file = OpenOptions::new().write(true).open(path).unwrap();
    let asd = format!(
        "
[[reminder]]
name = \"{}\"
description = \"{}\"
frequency = {}
{}
",
        reminder.name,
        reminder.description,
        reminder.frequency,
        reminder.icon.unwrap_or_default()
    );
    file.write_all(asd.as_bytes()).unwrap();
}

// why test no work :(
// #[cfg(test)]
mod tests {
    use std::{
        fs::{File, OpenOptions},
        io::{Read, Seek, Write},
    };

    use tempfile::tempdir;

    use super::{add_reminder, Reminder};

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
    #[test]
    fn test_add_reminder() {
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path().join("Test.toml");
        let mut f = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(true)
            .open(path)
            .unwrap();
        //create(&path).unwrap();

        let reminder = Reminder::new(String::from("Hello, world!"), String::from(""), 0, None);
        add_reminder(&mut f, reminder);
        f.seek(std::io::SeekFrom::Start(0)).unwrap();
        let mut str = String::new();
        f.read_to_string(&mut str).unwrap();

        temp_dir.close().unwrap();
        assert_eq!(
            str,
            "[[reminder]]\nname = \"Hello, world!\"\ndescription = \"\"\nfrequency = 0"
        );
    }
}
