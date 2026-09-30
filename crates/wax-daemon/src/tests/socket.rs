//! The IPC socket must not be reachable by other local users.

use crate::bind_socket;
use std::io::{BufRead, Write};
use std::os::unix::fs::PermissionsExt;
use tempfile::TempDir;

fn mode_of(path: &std::path::Path) -> u32 {
    std::fs::metadata(path).unwrap().permissions().mode() & 0o777
}

#[test]
fn the_socket_is_owner_only() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("wax.sock");

    let listener = bind_socket(&path).unwrap();
    assert_eq!(mode_of(&path), 0o600);
    drop(listener);
}

#[test]
fn no_group_or_other_bits_are_set() {
    // The specific thing that matters: `connect()` needs write permission, so
    // any group or other bit at all is a disclosure of the clipboard history.
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("wax.sock");

    let listener = bind_socket(&path).unwrap();
    let mode = mode_of(&path);
    assert_eq!(mode & 0o077, 0, "socket is reachable by others: {mode:o}");
    drop(listener);
}

#[test]
fn the_socket_is_actually_usable_after_binding() {
    // Guards against the permission change breaking the daemon outright: a
    // socket nobody can reach, including the owner, is not a fix.
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("wax.sock");

    let listener = bind_socket(&path).unwrap();
    let mut client = std::os::unix::net::UnixStream::connect(&path).unwrap();
    let (accepted, _) = listener.accept().unwrap();

    client.write_all(b"hello\n").unwrap();
    let mut reader = std::io::BufReader::new(&accepted);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    assert_eq!(line, "hello\n");
}

#[test]
fn binding_replaces_a_stale_socket() {
    // A leftover socket from a previous run must not block startup, and the
    // permissions must still be applied to the replacement.
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("wax.sock");
    std::fs::write(&path, b"stale").unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o666)).unwrap();

    let listener = bind_socket(&path).unwrap();
    assert_eq!(mode_of(&path), 0o600, "stale 0666 must not survive");
    drop(listener);
}
