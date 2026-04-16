use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_max_db_mb")]
    pub max_db_mb: u64,
    #[serde(default = "default_max_images_mb")]
    pub max_images_mb: u64,
    #[serde(default)]
    pub ttl_secs: Option<u64>,
    #[serde(default)]
    pub excluded_pattern: Vec<String>,
    #[serde(default = "default_true")]
    pub clipboard: bool,
    #[serde(default)]
    pub primary_selection: bool,
}

fn default_true() -> bool {
    true
}

fn default_max_db_mb() -> u64 {
    50
}

fn default_max_images_mb() -> u64 {
    100
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_db_mb: default_max_db_mb(),
            max_images_mb: default_max_images_mb(),
            ttl_secs: None,
            excluded_pattern: Vec::new(),
            clipboard: true,
            primary_selection: false,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let path = match config_path() {
            Some(p) => p,
            None => return Self::default(),
        };
        if !path.exists() {
            Self::default().save(&path);
        }
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or_default()
    }

    fn save(&self, path: &std::path::Path) {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        let excluded = if self.excluded_pattern.is_empty() {
            "# excluded_pattern = [\"password\", \"secret.*\"]".to_string()
        } else {
            format!(
                "excluded_pattern = [{}]",
                self.excluded_pattern.iter().map(|p| format!("\"{}\"", p)).collect::<Vec<_>>().join(", ")
            )
        };
        let ttl = match self.ttl_secs {
            Some(s) => format!("ttl_secs = {}", s),
            None => "# ttl_secs = 604800  # 7 days".to_string(),
        };
        let content = format!(
            "max_db_mb = {}\nmax_images_mb = {}\n{}\n\nclipboard = {}\nprimary_selection = {}\n\n{}\n",
            self.max_db_mb, self.max_images_mb, ttl, self.clipboard, self.primary_selection, excluded
        );
        std::fs::write(path, content).ok();
    }
}

fn config_path() -> Option<PathBuf> {
    Some(dirs::config_dir()?.join("wax/config.toml"))
}
