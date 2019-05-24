#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate typetag;

// not use now
pub use client::Client;

// tmp pub
pub mod tdjson;

mod client;

pub mod types;
