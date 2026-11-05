//! Shared support for the per-problem binaries in `src/bin/`.
//!
//! Each file under `src/bin/` is compiled as its own binary target, so one
//! broken solution can never break the rest of the repo. Cargo discovers them
//! automatically; there is no `mod` list to keep in sync.

#![allow(dead_code)]

pub mod prelude;
