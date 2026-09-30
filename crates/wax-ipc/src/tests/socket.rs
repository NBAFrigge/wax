//! Socket path resolution.

use crate::socket_path_in;
use std::path::PathBuf;

#[test]
fn uses_xdg_runtime_dir_when_set() {
    assert_eq!(
        socket_path_in(Some("/run/user/1000")),
        PathBuf::from("/run/user/1000/wax.sock")
    );
}

#[test]
fn falls_back_to_tmp_when_unset() {
    assert_eq!(socket_path_in(None), PathBuf::from("/tmp/wax.sock"));
}

#[test]
fn empty_runtime_dir_falls_back_instead_of_producing_a_relative_path() {
    // An empty XDG_RUNTIME_DIR is "set" as far as env::var is concerned. Without
    // the filter, PathBuf::from("").join("wax.sock") is the bare relative path
    // "wax.sock", so the daemon would bind a socket in its working directory.
    assert_eq!(socket_path_in(Some("")), PathBuf::from("/tmp/wax.sock"));
}

#[test]
fn resulting_path_is_absolute() {
    for dir in [None, Some(""), Some("/run/user/1000"), Some("/tmp")] {
        let path = socket_path_in(dir);
        assert!(
            path.is_absolute(),
            "socket path should be absolute, got {:?}",
            path
        );
    }
}

#[test]
fn socket_name_is_preserved_under_a_trailing_slash() {
    assert_eq!(
        socket_path_in(Some("/run/user/1000/")),
        PathBuf::from("/run/user/1000/wax.sock")
    );
    assert_eq!(
        socket_path_in(Some("/run/user/1000"))
            .file_name()
            .and_then(|n| n.to_str()),
        Some("wax.sock")
    );
}
