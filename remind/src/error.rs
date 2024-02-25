use serde::Serialize;

/// This struct uses `thiserror` to wrap all of the possible errors that Tauri commands can return.
/// This struct implements `Serialize` so that these errors can be sent to the frontend.
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("There was an IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("There was an error deserializing some data (probably in the TOML file): {0}")]
    DeserializationError(#[from] toml::de::Error),
    #[error("There was an error serializing data: {0}")]
    SerializationError(#[from] toml::ser::Error),
}

impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
