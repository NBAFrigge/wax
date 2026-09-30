//! Tests for the `wax` daemon and CLI.
//!
//! Both are bin targets with no library, so these live in `src/` and reach
//! crate-private items directly.

mod config;
mod socket;
mod state;
