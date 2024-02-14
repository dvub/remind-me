use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
    ops::Index,
    path::Path,
};
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
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

pub struct EditReminder {
    pub name: Option<String>,
    pub description: Option<String>,
    pub frequency: Option<i32>,
    pub icon: Option<String>,
}

// this struct may be used for any other configuration
// if needed in the future

/// Wrapper struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AllReminders {
    pub reminders: Vec<Reminder>,
}

/// Attempts to read a vector of Reminders from the specified path
pub fn read_all_reminders(path: &Path) -> anyhow::Result<Vec<Reminder>> {
    // read the target file and parse them into a data structure
    println!("reading configuration file for reminders...");
    let toml_str = fs::read_to_string(path)?;
    // println!("File content: {:?}", toml_str);

    if toml_str.is_empty() {
        return Ok(Vec::new());
    }
    let res: AllReminders = toml::from_str(&toml_str)?;
    let reminders = res.reminders;

    println!("successfully read file into memory...");
    Ok(reminders)
}

// TODO:
// make sure its a toml file/dont just directly write (ser/deser first)
// unify create/delete functions to use the same method
// this might be unnecessary
// does reminder param need to be &?

/// Attempts to add a reminder to the specified path by writing directly to the file.
pub fn add_reminder(path: &Path, reminder: Reminder) -> anyhow::Result<()> {
    // TODO: improve this
    let icon_str = {
        if let Some(icon) = reminder.icon {
            format!("icon = \"{}\"", icon)
        } else {
            String::new()
        }
    };

    let toml_str = format!(
        "[[reminder]]\nname = \"{}\"\ndescription = \"{}\"\nfrequency = {}\n{}",
        reminder.name,
        reminder.description,
        reminder.frequency,
        icon_str // reminder.icon.unwrap_or_default()
    );
    let mut file = File::options().write(true).open(path)?;
    file.write_all(toml_str.as_bytes())?;
    Ok(())
}

/// Attempts to remove a reminder from the toml file at the specified path by name.
pub fn delete_reminder(path: &Path, name: &str) -> anyhow::Result<()> {
    let toml_content = fs::read_to_string(path)?;
    // i luv turbofish syntax
    let mut reminders = toml::from_str::<AllReminders>(&toml_content)?.reminders;
    // modify
    reminders.retain(|r| r.name != name);
    // if empty dont bother
    // TODO:
    // rework this
    if reminders.is_empty() {
        fs::write(path, "")?;
        return Ok(());
    }

    // we need to make use of our wrapper struct
    let ar = AllReminders { reminders };
    let modified_toml = toml::to_string(&ar)?;
    fs::write(path, modified_toml)?;
    Ok(())
}

pub fn read_reminder(path: &Path, name: &str) -> anyhow::Result<Option<Reminder>> {
    let toml_content = fs::read_to_string(path)?;
    let reminders = toml::from_str::<AllReminders>(&toml_content)?.reminders;
    // TODO: rm cloned
    Ok(reminders.iter().find(|r| r.name == name).cloned())
}

pub fn edit_reminder(path: &Path, name: &str, new_data: EditReminder) -> anyhow::Result<()> {
    let toml_content = fs::read_to_string(path)?;
    let mut reminders = toml::from_str::<AllReminders>(&toml_content)?.reminders;
    let index = reminders.iter().position(|r| r.name == name);
    if let Some(idx) = index {
        if let Some(new_name) = new_data.name {
            reminders[idx].name = new_name;
        }
        if let Some(new_description) = new_data.description {
            reminders[idx].description = new_description;
        }
        if let Some(new_frequency) = new_data.frequency {
            reminders[idx].frequency = new_frequency;
        }
        // since the icon is already optional we don't need to check for Some
        reminders[idx].icon = new_data.icon;
    }
    let ar = AllReminders { reminders };
    let modified_toml = toml::to_string(&ar)?;
    fs::write(path, modified_toml)?;
    Ok(())
}

// why test no work :(
// #[cfg(test)]
mod tests {
    use std::{
        io::{Read, Write},
        path::PathBuf,
    };

    fn gen_test_file() -> anyhow::Result<PathBuf> {
        use std::fs::File;
        use tempfile::tempdir;
        let temp_dir = tempdir()?;
        let test_path = temp_dir.path().join("Test.toml");
        File::create(&test_path)?;
        Ok(test_path)
    }
    fn write_into_test_file(data: &[u8]) -> anyhow::Result<PathBuf> {
        use std::fs::File;
        use tempfile::tempdir;
        let temp_dir = tempdir()?;
        let test_path = temp_dir.path().join("Test.toml");
        let mut f = File::create(&test_path)?;
        f.write_all(data).unwrap();
        Ok(test_path)
    }

    // TODO:
    // should this take a Reminder instead of a &str?

    /// Testing wrapper function that takes a string of TOML and a reminder name,
    /// writes it to a file, performs the deletion, returning what remains in the file.
    /// The output of this function is intended to be used for assertions
    fn read_remaining(reminder_str: &str, name: &str) -> String {
        use std::fs::File;
        let test_path = write_into_test_file(reminder_str.as_bytes()).unwrap();
        super::delete_reminder(&test_path, name).unwrap();

        let mut f = File::open(test_path).unwrap();
        let mut str_buffer = String::new();
        f.read_to_string(&mut str_buffer).unwrap();
        str_buffer
    }

    #[test]
    fn read_no_reminders() {
        let test_path = gen_test_file().unwrap();
        let result = super::read_all_reminders(&test_path).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn read_all_reminders() {
        let test_path = write_into_test_file(b"[[reminders]]\nname = \"Find me!\"\ndescription = \"...\"\nfrequency = 0\n[[reminders]]\nname = \"Dont find me\"\ndescription = \"You found me...\"\nfrequency = 1").unwrap();
        let result = super::read_all_reminders(&test_path).unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "Find me!");
    }

    #[test]
    fn read_two_rem() {
        let test_path = write_into_test_file(b"[[reminders]]\nname = \"Find me!\"\ndescription = \"...\"\nfrequency = 0\n[[reminders]]\nname = \"Dont find me\"\ndescription = \"You found me...\"\nfrequency = 1")
            .unwrap();
        let first_reminder = super::read_reminder(&test_path, "Find me!").unwrap();
        let second_reminder = super::read_reminder(&test_path, "Dont find me").unwrap();
        assert_eq!(first_reminder.unwrap().description, "...");
        assert_eq!(second_reminder.unwrap().description, "Dont find me");
    }

    // TODO:
    // fix this
    #[test]
    fn test_add_reminder() {
        use super::Reminder;
        use std::{fs::File, io::Read};

        let test_path = gen_test_file().unwrap();
        //create(&path).unwrap();

        let reminder = Reminder::new(
            String::from("Hello. world."),
            String::from(""),
            0,
            Some("not a real icon".to_owned()),
        );

        super::add_reminder(&test_path, reminder).unwrap();
        // man wtf.

        let mut file_read = File::open(&test_path).unwrap();
        let mut string_buffer = String::new();
        file_read.read_to_string(&mut string_buffer).unwrap();
        assert_eq!(
            string_buffer,
            "[[reminder]]\nname = \"Hello. world.\"\ndescription = \"\"\nfrequency = 0\nicon = \"not a real icon\""
        );
    }

    #[test]
    fn edit_reminder() {
        use std::fs::File;

        let test_path = write_into_test_file(
            b"[[reminders]]\nname = \"Find me!\"\ndescription = \"...\"\nfrequency = 0",
        )
        .unwrap();

        super::edit_reminder(
            &test_path,
            "Find me!",
            super::EditReminder {
                name: None,
                description: Some(String::from("New description!!")),
                frequency: None,
                icon: None,
            },
        )
        .unwrap();

        let mut f = File::open(test_path).unwrap();
        let mut str_buffer = String::new();
        f.read_to_string(&mut str_buffer).unwrap();

        assert_eq!(str_buffer.trim(), "[[reminders]]\nname = \"Find me!\"\ndescription = \"New description!!\"\nfrequency = 0");
    }
}
