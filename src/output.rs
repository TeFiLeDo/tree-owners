use serde::Serialize;

/// Data that can be printed to the console.
pub trait Output {
    /// A human readable representation of the data.
    fn human_readable(&self) -> String;
    /// A `json` representation of the data.
    fn json(&self) -> Result<String, serde_json::Error>;
}

impl<T> Output for T
where
    T: ToString + Serialize,
{
    fn human_readable(&self) -> String {
        self.to_string()
    }

    fn json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self)
    }
}
