use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_max_display_len")]
    pub max_display_len: usize,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_max_display_len() -> usize {
    50
}

fn default_limit() -> usize {
    50
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_display_len: default_max_display_len(),
            limit: default_limit(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let path = match dirs::config_dir() {
            Some(d) => d.join("wax/config.toml"),
            None => return Self::default(),
        };
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or_default()
    }
}
