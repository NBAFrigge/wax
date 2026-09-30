use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Config {
    #[serde(default = "default_max_entries")]
    pub max_entries: u64,
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

fn default_max_entries() -> u64 {
    1000
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_entries: default_max_entries(),
            ttl_secs: None,
            excluded_pattern: Vec::new(),
            clipboard: true,
            primary_selection: false,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        match config_path() {
            Some(p) => Self::load_from(&p),
            None => Self::default(),
        }
    }

    /// Load from an explicit path, creating it from defaults when absent.
    ///
    /// Split out from [`Config::load`] so the resolution can be tested without
    /// depending on the process environment, which is global and racy.
    pub(crate) fn load_from(path: &Path) -> Self {
        if !path.exists() {
            Self::default().save(path);
        }
        std::fs::read_to_string(path)
            .ok()
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub(crate) fn save(&self, path: &Path) {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        let excluded = if self.excluded_pattern.is_empty() {
            "# excluded_pattern = [\"password\", \"secret.*\"]".to_string()
        } else {
            format!(
                "excluded_pattern = [{}]",
                self.excluded_pattern
                    .iter()
                    .map(|p| format!("\"{}\"", p))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        let ttl = match self.ttl_secs {
            Some(s) => format!("ttl_secs = {}", s),
            None => "# ttl_secs = 604800  # 7 days".to_string(),
        };
        let content = format!(
            "max_entries = {}\n{}\n\nclipboard = {}\nprimary_selection = {}\n\n{}\n",
            self.max_entries, ttl, self.clipboard, self.primary_selection, excluded
        );
        std::fs::write(path, content).ok();
    }
}

fn config_path() -> Option<PathBuf> {
    Some(dirs::config_dir()?.join("wax/config.toml"))
}
