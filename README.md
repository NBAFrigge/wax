# wax

Fast Wayland clipboard manager. Stores text and images, picked via rofi or wofi.

```
Benchmark 1: wax list
  Time (mean ± σ):   2.2 ms ±   1.1 ms    [User: 0.6 ms, System: 1.1 ms]

Benchmark 2: cliphist list
  Time (mean ± σ):   5.2 ms ±   1.3 ms    [User: 3.1 ms, System: 2.4 ms]

Summary: wax list ran 2.36x faster than cliphist list
```

## Dependencies

- `wl-clipboard`
- `rofi` or `wofi` (picker)

## Install

### Manual

```
cargo install --path crates/wax-daemon --locked
cargo install --path crates/wax-cli --locked
```

## Setup

Enable the daemon:

```
systemctl --user enable --now wax
```

Add to your Hyprland config:

```
bind = $mod, V, exec, wax
```

## Usage

```
wax              # open picker (default: last 50 entries)
wax list         # print history
wax pick         # open picker explicitly
wax delete <x>   # delete entry
wax clear        # clear all
```
