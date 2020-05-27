#[macro_use]
extern crate serde_derive;

#[cfg(feature = "sys")]
pub use rtdlib_sys::Tdlib;

pub mod types;
pub mod errors;
