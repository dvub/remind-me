use serde::Deserialize;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};
use toml::Value;

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

pub fn read_all_reminders(file: &Path) -> anyhow::Result<Vec<Reminder>> {
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

// TODO:
// unify create/delete functions to use the same method
// this might be unnecessary

// does reminder param need to be &?
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
    let mut toml_data: Value = toml::from_str(&toml_content)?;
    if let Value::Table(ref mut toml_table) = toml_data {
        // TODO:
        // fix cloning the table
        for (key, value) in toml_table.clone().iter() {
            println!("ASD");
            if let Some(value) = value.as_str() {
                if key == "name" && value == name {
                    // println!("DID IT WORK");
                    toml_table.remove(key);
                }
            }
        }
        //println!("{:?}", toml_table);
        // else ...
    } else {
        // TODO:
        // throw err?
    }

    // Serialize the modified TOML data
    let modified_toml = toml::to_string_pretty(&toml_data)?;

    // Write the modified TOML content back to the file
    fs::write(path, modified_toml)?;

    Ok(())
}

// why test no work :(
// #[cfg(test)]
mod tests {
    use std::io::Read;

    #[test]
    fn read_all_reminders() {
        use std::{fs::File, io::Write};
        use tempfile::tempdir;

        let one_reminder = b"[[reminders]]
        name = \"Hello, world!\"
        description = \"...\"
        frequency = 0
        ";

        let temp_dir = tempdir().unwrap();
        let test_path = temp_dir.path().join("Test.toml");
        let mut test_file = File::create(&test_path).unwrap();
        test_file.write_all(one_reminder).unwrap();
        let result = super::read_all_reminders(&test_path).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "Hello, world!");
        drop(test_file);
        temp_dir.close().unwrap();
    }
    // TODO:
    // fix this
    #[test]
    fn test_add_reminder() {
        use super::{add_reminder, Reminder};
        use std::{fs::File, io::Read};
        use tempfile::tempdir;

        let temp_dir = tempdir().unwrap();
        let test_file_path = temp_dir.path().join("Test.toml");
        File::create(&test_file_path).unwrap();
        //create(&path).unwrap();

        let reminder = Reminder::new(
            String::from("Hello. world."),
            String::from(""),
            0,
            Some("not a real icon".to_owned()),
        );
        add_reminder(&test_file_path, reminder).unwrap();
        // man wtf.
        let mut file_read = File::open(&test_file_path).unwrap();
        let mut string_buffer = String::new();
        file_read.read_to_string(&mut string_buffer).unwrap();

        temp_dir.close().unwrap();
        assert_eq!(
            string_buffer,
            "[[reminder]]\nname = \"Hello. world.\"\ndescription = \"\"\nfrequency = 0\nicon = \"not a real icon\""
        );
    }
    #[test]
    fn test_delete() {
        use std::fs::File;
        use std::io::Write;
        use tempfile::tempdir;

        let one_reminder = b"[[reminders]]
        name = \"Hello, world!\"
        description = \"...\"
        frequency = 0
        ";

        let temp_dir = tempdir().unwrap();
        let test_path = temp_dir.path().join("Test.toml");
        let mut test_file = File::create(&test_path).unwrap();
        test_file.write_all(one_reminder).unwrap();

        super::delete_reminder(&test_path, "Hello, world!").unwrap();

        let mut f = File::open(test_path).unwrap();
        let mut str_buffer = String::new();
        f.read_to_string(&mut str_buffer).unwrap();
        println!("{str_buffer}");
        assert!(str_buffer.is_empty());
    }
}
