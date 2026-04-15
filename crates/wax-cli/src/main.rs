use clap::{Parser, Subcommand, ValueEnum};
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::net::UnixStream;
use std::process::{Command, Stdio};
use wax_ipc::{Request, Response};


#[derive(Parser)]
#[command(name = "wax", about = "Clipboard manager for Wayland / Hyprland")]
struct Cli {
    #[command(subcommand)]
    command: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    Pick {
        #[arg(short, long, default_value_t = 50)]
        limit: usize,
        #[arg(long, value_enum)]
        picker: Option<PickerKind>,
    },
    List {
        #[arg(default_value_t = 50)]
        n: usize,
    },
    Delete {
        text: String,
    },
    Clear,
}

#[derive(Clone, ValueEnum)]
enum PickerKind {
    Wofi,
    Rofi,
}

struct PickerEntry {
    display: String,
    icon_path: Option<String>,
    original: String,
}

impl PickerEntry {
    fn from_clip(clip: &str) -> Self {
        if let Some(path) = clip.strip_prefix("[img] ") {
            let dims = match png_dimensions(path) {
                Some((w, h)) => format!("{}×{}", w, h),
                None => String::new(),
            };
            let time = file_time_label(path);
            let display = match (dims.is_empty(), time.is_empty()) {
                (false, false) => format!("[img] {} · {}", dims, time),
                (false, true) => format!("[img] {}", dims),
                (true, false) => format!("[img] {}", time),
                (true, true) => "[img]".to_string(),
            };
            PickerEntry {
                display,
                icon_path: Some(path.to_string()),
                original: clip.to_string(),
            }
        } else {
            let display = clip.replace('\n', "↵");
            PickerEntry {
                display,
                icon_path: None,
                original: clip.to_string(),
            }
        }
    }
}

fn png_dimensions(path: &str) -> Option<(u32, u32)> {
    let mut buf = [0u8; 24];
    std::fs::File::open(path).ok()?.read_exact(&mut buf).ok()?;
    if &buf[0..8] != b"\x89PNG\r\n\x1a\n" {
        return None;
    }
    if &buf[12..16] != b"IHDR" {
        return None;
    }
    let w = u32::from_be_bytes(buf[16..20].try_into().ok()?);
    let h = u32::from_be_bytes(buf[20..24].try_into().ok()?);
    Some((w, h))
}

fn file_time_label(path: &str) -> String {
    file_time_label_inner(path).unwrap_or_default()
}

fn local_utc_offset_secs() -> i64 {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as libc::time_t;
    let mut tm: libc::tm = unsafe { std::mem::zeroed() };
    unsafe { libc::localtime_r(&now, &mut tm) };
    tm.tm_gmtoff
}

fn file_time_label_inner(path: &str) -> Option<String> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let offset = local_utc_offset_secs();
    let mtime = std::fs::metadata(path).and_then(|m| m.modified()).ok()?;
    let mtime_secs = (mtime.duration_since(UNIX_EPOCH).ok()?.as_secs() as i64 + offset) as u64;
    let now_secs =
        (SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs() as i64 + offset) as u64;

    let today_start = now_secs - (now_secs % 86400);
    let mtime_day = mtime_secs - (mtime_secs % 86400);
    let hh = (mtime_secs % 86400) / 3600;
    let mm = (mtime_secs % 3600) / 60;

    if mtime_day == today_start {
        Some(format!("{:02}:{:02}", hh, mm))
    } else {
        let (dd, mo) = epoch_secs_to_date(mtime_secs);
        Some(format!("{:02}/{:02}", dd, mo))
    }
}

fn epoch_secs_to_date(secs: u64) -> (u64, u64) {
    let mut remaining = secs / 86400;
    let mut y = 1970u64;
    loop {
        let days_in_year = if (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 {
            366
        } else {
            365
        };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        y += 1;
    }
    let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    let months = if leap {
        [31u64, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31u64, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut m = 0usize;
    for &dim in &months {
        if remaining < dim {
            break;
        }
        remaining -= dim;
        m += 1;
    }
    (remaining + 1, m as u64 + 1)
}

enum Picker {
    Wofi,
    Rofi,
}

impl Picker {
    fn detect() -> Result<Self, Box<dyn std::error::Error>> {
        if is_in_path("rofi") {
            Ok(Picker::Rofi)
        } else if is_in_path("wofi") {
            Ok(Picker::Wofi)
        } else {
            Err("no supported picker found; install rofi or wofi".into())
        }
    }

    fn from_kind(kind: PickerKind) -> Self {
        match kind {
            PickerKind::Wofi => Picker::Wofi,
            PickerKind::Rofi => Picker::Rofi,
        }
    }

    fn format_entry(&self, entry: &PickerEntry) -> String {
        match (&self, &entry.icon_path) {
            (Picker::Wofi, Some(path)) => format!("img:{}\t{}", path, entry.display),
            (Picker::Rofi, Some(path)) => format!("{}\0icon\x1f{}", entry.display, path),
            _ => entry.display.clone(),
        }
    }

    fn spawn(&self, entries: &[PickerEntry]) -> Option<String> {
        let input = entries
            .iter()
            .map(|e| self.format_entry(e))
            .collect::<Vec<_>>()
            .join("\n");

        let mut child = match self {
            Picker::Wofi => Command::new("wofi")
                .args([
                    "--show",
                    "dmenu",
                    "--prompt",
                    "clipboard",
                    "--no-markup",
                    "--allow-images",
                ])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .ok()?,

            Picker::Rofi => {
                let mut cmd = Command::new("rofi");
                cmd.args(["-dmenu", "-p", "clipboard", "-show-icons"]);

                let image_indices: Vec<String> = entries
                    .iter()
                    .enumerate()
                    .filter(|(_, e)| e.icon_path.is_some())
                    .map(|(i, _)| i.to_string())
                    .collect();
                if !image_indices.is_empty() {
                    cmd.arg("-a").arg(image_indices.join(","));
                }

                if let Some(theme) = dirs::config_dir()
                    .map(|d| d.join("rofi/clipboard.rasi"))
                    .filter(|p| p.exists())
                {
                    cmd.arg("-theme").arg(theme);
                }

                cmd.stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()
                    .ok()?
            }
        };

        child.stdin.as_mut()?.write_all(input.as_bytes()).ok()?;

        let output = child.wait_with_output().ok()?;
        let raw = String::from_utf8(output.stdout).ok()?;
        let selected = raw.trim();
        if selected.is_empty() {
            return None;
        }

        let display = if let Some(pos) = selected.rfind('\t') {
            &selected[pos + 1..]
        } else if let Some(pos) = selected.find('\0') {
            &selected[..pos]
        } else {
            selected
        };

        entries
            .iter()
            .find(|e| e.display == display)
            .map(|e| e.original.clone())
    }
}

fn is_in_path(cmd: &str) -> bool {
    std::env::var("PATH")
        .unwrap_or_default()
        .split(':')
        .any(|dir| std::path::Path::new(dir).join(cmd).is_file())
}

fn send(req: &Request) -> Result<Response, Box<dyn std::error::Error>> {
    let socket = wax_ipc::socket_path();
    let stream = UnixStream::connect(&socket).map_err(|e| {
        format!(
            "cannot connect to wax daemon at {}: {}",
            socket.display(),
            e
        )
    })?;
    let mut writer = &stream;
    let mut reader = BufReader::new(&stream);

    let mut line = serde_json::to_string(req)?;
    line.push('\n');
    writer.write_all(line.as_bytes())?;

    let mut resp = String::new();
    reader.read_line(&mut resp)?;
    Ok(serde_json::from_str(&resp)?)
}

fn set_clipboard(clip: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = clip.strip_prefix("[img] ") {
        let data = std::fs::read(path)?;
        let mut child = Command::new("wl-copy")
            .args(["--type", "image/png"])
            .stdin(Stdio::piped())
            .spawn()?;
        child
            .stdin
            .as_mut()
            .ok_or("wl-copy stdin unavailable")?
            .write_all(&data)?;
        child.wait()?;
    } else {
        let mut child = Command::new("wl-copy").stdin(Stdio::piped()).spawn()?;
        child
            .stdin
            .as_mut()
            .ok_or("wl-copy stdin unavailable")?
            .write_all(clip.as_bytes())?;
        child.wait()?;
    }
    Ok(())
}

fn get_clips(n: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    if let Some(clips) = wax_store::read_cache(n) {
        return Ok(clips);
    }
    match send(&Request::Get { n })? {
        Response::Clips(c) => Ok(c),
        Response::Error(e) => Err(e.into()),
        _ => Err("unexpected response".into()),
    }
}

fn pick(
    limit: usize,
    picker_override: Option<PickerKind>,
) -> Result<(), Box<dyn std::error::Error>> {
    let clips = get_clips(limit)?;

    if clips.is_empty() {
        return Ok(());
    }

    let entries: Vec<PickerEntry> = clips.iter().map(|c| PickerEntry::from_clip(c)).collect();
    let picker = match picker_override {
        Some(kind) => Picker::from_kind(kind),
        None => Picker::detect()?,
    };

    let Some(original) = picker.spawn(&entries) else {
        return Ok(());
    };

    set_clipboard(&original)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command.unwrap_or(Cmd::Pick {
        limit: 50,
        picker: None,
    }) {
        Cmd::Pick { limit, picker } => pick(limit, picker)?,

        Cmd::List { n } => {
            let clips = get_clips(n)?;
            clips.into_iter().for_each(|c| println!("{}", c));
        }

        Cmd::Delete { text } => match send(&Request::Delete { text })? {
            Response::Ok => {}
            Response::Error(e) => return Err(e.into()),
            _ => return Err("unexpected response".into()),
        },

        Cmd::Clear => match send(&Request::Clear)? {
            Response::Ok => {}
            Response::Error(e) => return Err(e.into()),
            _ => return Err("unexpected response".into()),
        },
    }

    Ok(())
}
