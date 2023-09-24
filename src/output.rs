use serde::Serialize;

pub trait Output {
    fn human_readable(&self) -> String;
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
