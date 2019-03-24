use crate::clients::google::GoogleAuthConfig;
use crate::clients::hackernews::Hackernews;
use serde_json;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub google_auth: GoogleAuthConfig,
    pub tokens_path: String,
    pub calendars: Vec<String>,
    pub hackernews: Hackernews,
}

impl Config {
    pub fn load() -> Self {
        let data = fs::read_to_string("config.json").expect("Missing 'config.json'");
        serde_json::from_str(&data).expect("Error reading configuration :(")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reading_config() {
        let cfg = dbg!(Config::load());
    }
}
