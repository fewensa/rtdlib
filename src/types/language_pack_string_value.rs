
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Represents the value of a string in a language pack. 
#[typetag::serde(tag = "@struct")]
pub trait LanguagePackStringValue: Object + RObject + Debug {}






impl LanguagePackStringValue {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<LanguagePackStringValue> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDLanguagePackStringValueType {
  LanguagePackStringValueDeleted,
  LanguagePackStringValueOrdinary,
  LanguagePackStringValuePluralized,
  
}
impl RTDLanguagePackStringValueType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDLanguagePackStringValueType)(text.as_ref()) }
}



