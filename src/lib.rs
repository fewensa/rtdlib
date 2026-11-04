//! Rust types for [TDLib](https://github.com/tdlib/td).
//!
//! The default build contains TDLib types only. Enable the `sys` feature to
//! re-export `rtdlib_sys::Tdlib` and call the `tdjson` shared library.

// Generated getters return `&Box<T>`, and several generated enums are large.
// Changing either would break the public API used by telegram-client.
#![allow(clippy::borrowed_box, clippy::large_enum_variant)]

#[macro_use]
extern crate serde_derive;

#[cfg(feature = "sys")]
pub use rtdlib_sys::Tdlib;

pub mod errors;
pub mod types;
