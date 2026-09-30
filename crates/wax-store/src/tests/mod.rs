//! Tests for `wax_store`.
//!
//! These live in `src/` rather than `tests/` so they can reach private items
//! like `trim_oldest`, `check_expire`, `enforce_limits` and the
//! `#[cfg(test)]` `push_text_at` helper, which the limit tests need.

mod common;
mod images;
mod limits;
mod multipage;
mod store;
