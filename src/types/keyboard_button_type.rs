
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Describes a keyboard button type. 
#[typetag::serde(tag = "@struct")]
pub trait KeyboardButtonType: Object + RObject + Debug {}






impl KeyboardButtonType {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<KeyboardButtonType> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDKeyboardButtonTypeType {
  KeyboardButtonTypeRequestLocation,
  KeyboardButtonTypeRequestPhoneNumber,
  KeyboardButtonTypeText,
  
}
impl RTDKeyboardButtonTypeType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDKeyboardButtonTypeType)(text.as_ref()) }
}



