//! Tests for the pure logic in the `wax` CLI.
//!
//! The bin target has no library, so these live in `src/` and use the
//! crate-private items directly. Anything that shells out to rofi, wofi,
//! hyprctl, wtype or wl-copy is not covered here.

mod dates;
mod entries;
mod format;
mod png;
