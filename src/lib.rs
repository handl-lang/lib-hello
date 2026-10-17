//! `hello` — Handl stdlib port (`src/index.hnl`).
//!
//! Two forms are exported because consumers want different shapes:
//!
//! - [`GREET_HNL`] — the bare arrow source (`greet.hnl`'s contents)
//! - [`HELLO_HNL`] — the module record `{ greet: (…) }` the table registers

pub const GREET_HNL: &str = include_str!("index.hnl");

/// The module record wrapping [`GREET_HNL`] under `greet`.
pub const HELLO_HNL: &str = concat!("{ greet: (\n", include_str!("index.hnl"), "\n) }");
