//! Tests for `wax_store`.
//!
//! These live in `src/` rather than `tests/` so they can reach private items
//! like `trim_oldest`, `check_expire`, `enforce_limits` and the
//! `#[cfg(test)]` `push_text_at` helper, which the limit tests need.
//!
//! `integrity` checks the relationships *between* tables after any sequence
//! of operations, rather than the behaviour of any single one.

mod common;
mod images;
mod integrity;
mod limits;
mod multipage;
mod pins;
mod store;
