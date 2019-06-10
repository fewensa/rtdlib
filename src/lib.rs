#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate typetag;

extern crate strum;
#[macro_use]
extern crate strum_macros;

#[macro_use]
extern crate typed_builder;

pub mod tdjson;
pub mod types;
pub mod tdkit;

mod tdsupplement;
