use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::process::{Command, Stdio};
use wax_ipc::{Request, Response, SOCKET_PATH};

fn get_clips(n: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let stream = UnixStream::connect(SOCKET_PATH)?;
    let mut writer = &stream;
    let mut reader = BufReader::new(&stream);
    let mut req = serde_json::to_string(&Request::Get { n })?;
    req.push('\n');
    writer.write_all(req.as_bytes())?;
    let mut line = String::new();
    reader.read_line(&mut line)?;
    match serde_json::from_str::<Response>(&line)? {
        Response::Clips(clips) => Ok(clips),
        Response::Error(e) => Err(e.into()),
        _ => Err("unexpected response".into()),
    }
}

fn spawn_wofi(items: &[String]) -> Option<String> {
    let mut wofi = Command::new("wofi")
        .args(["--show", "dmenu", "--prompt", "clipboard", "--no-markup"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .ok()?;

    wofi.stdin
        .as_mut()?
        .write_all(items.join("\n").as_bytes())
        .ok()?;

    let output = wofi.wait_with_output().ok()?;
    let selected = String::from_utf8(output.stdout).ok()?.trim().to_string();

    if selected.is_empty() {
        None
    } else {
        Some(selected)
    }
}

fn set_clipboard(selected: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = selected.strip_prefix("[img] ") {
        let data = std::fs::read(path)?;
        let mut child = Command::new("wl-copy")
            .args(["--type", "image/png"])
            .stdin(Stdio::piped())
            .spawn()?;
        child.stdin.as_mut().unwrap().write_all(&data)?;
        child.wait()?;
    } else {
        let mut child = Command::new("wl-copy").stdin(Stdio::piped()).spawn()?;
        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(selected.as_bytes())?;
        child.wait()?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let clips = get_clips(50)?;

    if clips.is_empty() {
        return Ok(());
    }

    if let Some(selected) = spawn_wofi(&clips) {
        set_clipboard(&selected)?;
    }

    Ok(())
}
