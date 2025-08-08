use crate::get_config_dir;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub token: Option<String>,
}

impl Config {
    pub fn load() -> anyhow::Result<Option<Self>> {
        let config_path = get_config_dir()?.join("config.toml");
        if !config_path.try_exists()? {
            return Ok(None);
        }
        let raw = std::fs::read_to_string(config_path)?;
        let parsed = toml::from_str(&raw)?;

        Ok(Some(parsed))
    }
}
