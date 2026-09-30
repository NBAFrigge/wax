//! Daemon configuration: defaults, parsing, and the generated config file.

use crate::config::Config;
use std::fs;
use tempfile::TempDir;

fn path_in(dir: &TempDir) -> std::path::PathBuf {
    dir.path().join("wax/config.toml")
}

fn write_config(dir: &TempDir, contents: &str) -> std::path::PathBuf {
    let path = path_in(dir);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, contents).unwrap();
    path
}

#[test]
fn defaults_match_the_documented_values() {
    let config = Config::default();
    assert_eq!(config.max_db_mb, 50);
    assert_eq!(config.max_images_mb, 100);
}

#[test]
fn default_config_is_the_documented_one() {
    let config = Config::default();
    assert_eq!(config.max_db_mb, 50);
    assert_eq!(config.max_images_mb, 100);
    assert_eq!(config.ttl_secs, None);
    assert!(config.excluded_pattern.is_empty());
    assert!(config.clipboard);
    assert!(!config.primary_selection);
}

#[test]
fn an_empty_file_yields_all_defaults() {
    let dir = TempDir::new().unwrap();
    let path = write_config(&dir, "");
    assert_eq!(Config::load_from(&path), Config::default());
}

#[test]
fn omitted_fields_fall_back_to_their_defaults() {
    let dir = TempDir::new().unwrap();
    let path = write_config(&dir, "max_db_mb = 10\n");
    let config = Config::load_from(&path);
    assert_eq!(config.max_db_mb, 10);
    assert_eq!(config.max_images_mb, 100);
    assert!(config.clipboard);
}

#[test]
fn every_field_is_read_when_present() {
    let dir = TempDir::new().unwrap();
    let path = write_config(
        &dir,
        "max_db_mb = 1\nmax_images_mb = 2\nttl_secs = 3\nexcluded_pattern = [\"a\", \"b\"]\nclipboard = false\nprimary_selection = true\n",
    );
    let config = Config::load_from(&path);
    assert_eq!(config.max_db_mb, 1);
    assert_eq!(config.max_images_mb, 2);
    assert_eq!(config.ttl_secs, Some(3));
    assert_eq!(config.excluded_pattern, vec!["a", "b"]);
    assert!(!config.clipboard);
    assert!(config.primary_selection);
}

#[test]
fn a_malformed_file_silently_yields_defaults_and_is_not_rewritten() {
    let dir = TempDir::new().unwrap();
    let broken = "max_db_mb = not-a-number\n";
    let path = write_config(&dir, broken);
    assert_eq!(Config::load_from(&path), Config::default());
    // The bad file is left in place, so the user keeps seeing their broken
    // config on every start and gets defaults each time with no warning.
    assert_eq!(fs::read_to_string(&path).unwrap(), broken);
}

#[test]
fn loading_a_missing_file_creates_it_from_defaults() {
    let dir = TempDir::new().unwrap();
    let path = path_in(&dir);
    assert!(!path.exists());

    let config = Config::load_from(&path);
    assert_eq!(config, Config::default());
    assert!(path.exists(), "load should write a starter config");
}

#[test]
fn save_creates_missing_parent_directories() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("deeply/nested/wax/config.toml");
    Config::default().save(&path);
    assert!(path.exists());
}

#[test]
fn a_generated_config_reloads_to_the_same_values() {
    let dir = TempDir::new().unwrap();
    let path = path_in(&dir);
    let original = Config {
        max_db_mb: 7,
        max_images_mb: 8,
        ttl_secs: Some(604800),
        excluded_pattern: vec!["password".into(), "secret.*".into()],
        clipboard: false,
        primary_selection: true,
    };
    original.save(&path);
    assert_eq!(Config::load_from(&path), original);
}

#[test]
fn the_generated_default_config_reloads_to_the_defaults() {
    let dir = TempDir::new().unwrap();
    let path = path_in(&dir);
    Config::default().save(&path);

    let text = fs::read_to_string(&path).unwrap();
    assert!(text.contains("max_db_mb = 50"));
    assert!(text.contains("max_images_mb = 100"));
    // Unset options are emitted as commented examples so users can discover
    // them without them being active.
    assert!(text.contains("# ttl_secs = 604800"));
    assert!(text.contains("# excluded_pattern = [\"password\", \"secret.*\"]"));

    assert_eq!(Config::load_from(&path), Config::default());
}

#[test]
fn unset_options_are_written_as_comments_not_values() {
    let dir = TempDir::new().unwrap();
    let path = path_in(&dir);
    Config::default().save(&path);
    let text = fs::read_to_string(&path).unwrap();
    assert!(!text.contains("\nttl_secs ="));
    assert!(!text.contains("\nexcluded_pattern ="));
}

#[test]
fn a_pattern_containing_a_quote_produces_a_file_that_will_not_reload() {
    // `save` interpolates patterns with no escaping, so a regex containing a
    // double quote writes invalid TOML and the value is silently lost on the
    // next load.
    let dir = TempDir::new().unwrap();
    let path = path_in(&dir);
    let config = Config {
        excluded_pattern: vec!["say \"hi\"".into()],
        ..Config::default()
    };
    config.save(&path);

    let text = fs::read_to_string(&path).unwrap();
    assert!(text.contains("excluded_pattern = [\"say \"hi\"\"]"));

    let reloaded = Config::load_from(&path);
    assert!(reloaded.excluded_pattern.is_empty(), "pattern was lost");
}
