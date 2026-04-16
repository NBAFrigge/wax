# wax

Fast Wayland clipboard manager. Stores text and images, picked via rofi or wofi.

```
Benchmark 1: wax list
  Time (mean ± σ):   2.0 ms ±   0.7 ms    [User: 0.9 ms, System: 0.9 ms]

Benchmark 2: cliphist list
  Time (mean ± σ):   7.5 ms ±   1.5 ms    [User: 4.5 ms, System: 3.5 ms]

Summary: wax list ran 3.79x faster than cliphist list
```

## Dependencies

- `wl-clipboard`
- `rofi` or `wofi` (picker)

## Install

### Arch Linux (AUR)

```
yay -S wax-clipboard
```

### From source

```
git clone https://github.com/NBAFrigge/wax
cd wax
cargo build --release --locked
sudo install -Dm755 target/release/wax-daemon /usr/bin/wax-daemon
sudo install -Dm755 target/release/wax /usr/bin/wax
```

## Setup

Enable the daemon:

```
systemctl --user enable --now wax
```

Add to your Hyprland config:

```
bind = $mod, V, exec, /usr/bin/wax
```

## Configuration

The daemon creates `~/.config/wax/config.toml` on first run:

```toml
max_db_mb = 50
max_images_mb = 100
# ttl_secs = 604800  # 7 days

clipboard = true
primary_selection = false

# excluded_pattern = ["password", "secret.*"]
```

| Key | Default | Description |
|---|---|---|
| `max_db_mb` | `50` | Maximum size of the database in MB |
| `max_images_mb` | `100` | Maximum size of the images folder in MB |
| `ttl_secs` | unset | Automatically delete entries older than this many seconds |
| `clipboard` | `true` | Track the regular clipboard (Ctrl+C) |
| `primary_selection` | `false` | Track the primary selection (mouse highlight) |
| `excluded_pattern` | `[]` | List of regex patterns — matching entries are not saved |
| `max_display_len` | `50` | Max characters shown per entry in the picker |
| `limit` | `50` | Number of entries shown by default in the picker |

When a size limit is exceeded, the oldest entries are removed automatically.

## Usage

```
wax              # open picker (default: last 50 entries)
wax list [N]     # print last N entries (default: 50)
wax pick         # open picker explicitly
wax delete <x>   # delete entry matching <x>
wax clear        # clear all history
```

### Pick flags

| Flag | Description |
|---|---|
| `--limit <N>` | Show last N entries (overrides `limit` in config) |
| `--picker rofi\|wofi` | Force a specific picker (default: auto-detect) |
| `--instant-paste` | After selection, refocus the previous window and paste automatically |
