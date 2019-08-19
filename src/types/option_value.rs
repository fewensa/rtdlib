
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Represents the value of an option. 
#[typetag::serde(tag = "@struct")]
pub trait OptionValue: Object + RObject + Debug {}






impl OptionValue {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<OptionValue> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDOptionValueType {
  OptionValueBoolean,
  OptionValueEmpty,
  OptionValueInteger,
  OptionValueString,
  
}
impl RTDOptionValueType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDOptionValueType)(text.as_ref()) }
}



