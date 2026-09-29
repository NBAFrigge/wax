//! Tests for `wax_store`.
//!
//! These live in `src/` rather than `tests/` so they can reach private items
//! like `trim_oldest`, `check_expire` and the `#[cfg(test)]` `push_text_at`
//! helper, which several of the limit tests need.

mod common;
mod limits;
mod store;
