use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_max_display_len")]
    pub max_display_len: usize,
    #[serde(default = "default_limit")]
    pub limit: usize,
    #[serde(default = "default_pin_key")]
    pub pin_key: String,
    #[serde(default = "default_pin_icon")]
    pub pin_icon: String,
}

fn default_max_display_len() -> usize {
    50
}

fn default_limit() -> usize {
    50
}

fn default_pin_key() -> String {
    "alt+p".to_string()
}

fn default_pin_icon() -> String {
    "view-pin-symbolic".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_display_len: default_max_display_len(),
            limit: default_limit(),
            pin_key: default_pin_key(),
            pin_icon: default_pin_icon(),
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
