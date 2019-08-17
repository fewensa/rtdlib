
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Describes a stream to which TDLib internal log is written. 
#[typetag::serde(tag = "@struct")]
pub trait LogStream: Object + RObject + Debug {}






impl LogStream {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<LogStream> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDLogStreamType {
  LogStreamDefault,
  LogStreamEmpty,
  LogStreamFile,
  
}
impl RTDLogStreamType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDLogStreamType)(text.as_ref()) }
}



