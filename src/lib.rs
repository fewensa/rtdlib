#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate typetag;
extern crate strum;
#[macro_use]
extern crate strum_macros;


// not use now
pub use self::client::Client;

// tmp pub
pub mod tdjson;

mod client;

pub mod types;
