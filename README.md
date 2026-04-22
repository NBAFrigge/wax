# wax

Wayland light clipboard manager for Hyprland. Stores text and images, browse and paste via rofi or wofi.

- Pin entries to keep them at the top
- Image support with thumbnail preview in rofi
- Instant paste — refocuses the previous window and types automatically
- Regex-based exclusion patterns (passwords, secrets)

## Dependencies

- `wl-clipboard`
- `rofi` or `wofi` (picker)

## Install

### Arch Linux (AUR)

```
yay -S wax-clipboard
```

### From source

```bash
git clone https://github.com/NBAFrigge/wax
cd wax
cargo build --release --locked
sudo install -Dm755 target/release/wax-daemon /usr/bin/wax-daemon
sudo install -Dm755 target/release/wax /usr/bin/wax
```

## Setup

Enable the daemon:

```bash
systemctl --user enable --now wax
```

Add to your Hyprland config:

```bash
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

| Key                 | Default             | Description                                               |
| ------------------- | ------------------- | --------------------------------------------------------- |
| `max_db_mb`         | `50`                | Maximum size of the database in MB                        |
| `max_images_mb`     | `100`               | Maximum size of the images folder in MB                   |
| `ttl_secs`          | unset               | Automatically delete entries older than this many seconds |
| `clipboard`         | `true`              | Track the regular clipboard (Ctrl+C)                      |
| `primary_selection` | `false`             | Track the primary selection (mouse highlight)             |
| `excluded_pattern`  | `[]`                | List of regex patterns — matching entries are not saved   |
| `max_display_len`   | `50`                | Max characters shown per entry in the picker              |
| `limit`             | `50`                | Number of entries shown by default in the picker          |
| `pin_key`           | `alt+p`             | Keybind to pin/unpin in rofi                              |
| `pin_icon`          | `view-pin-symbolic` | Icon shown for pinned entries in rofi                     |

When a size limit is exceeded, the oldest entries are removed automatically.

## Usage

```
wax              # open picker (default: last 50 entries)
wax list [N]     # print last N entries (default: 50)
wax pick         # open picker explicitly
wax delete <x>   # delete entry matching <x>
wax pin <x>      # pin entry (always shown at top)
wax unpin <x>    # unpin entry
wax clear        # clear all history (pinned entries are kept)
```

### Pick flags

| Flag                  | Description                                                          |
| --------------------- | -------------------------------------------------------------------- |
| `--limit <N>`         | Show last N entries (overrides `limit` in config)                    |
| `--picker rofi\|wofi` | Force a specific picker (default: auto-detect)                       |
| `--instant-paste`     | After selection, refocus the previous window and paste automatically |
