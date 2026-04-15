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
```

| Key | Default | Description |
|---|---|---|
| `max_db_mb` | `50` | Maximum size of the database in MB |
| `max_images_mb` | `100` | Maximum size of the images folder in MB |

When a limit is exceeded, the oldest entries are removed automatically.

## Usage

```
wax              # open picker (default: last 50 entries)
wax list         # print history
wax pick         # open picker explicitly
wax delete <x>   # delete entry
wax clear        # clear all
```

### Picker options

```
wax pick --limit 100          # show last 100 entries
wax pick --picker wofi        # force wofi
wax pick --picker rofi        # force rofi
```
