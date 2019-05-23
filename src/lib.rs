#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate typetag;

pub use rtdjson::RTDLib;

mod tdjson;
mod rtdjson;

pub mod types;
