
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Describes the type of an inline keyboard button. 
#[typetag::serde(tag = "@struct")]
pub trait InlineKeyboardButtonType: Object + RObject + Debug {}






impl InlineKeyboardButtonType {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<InlineKeyboardButtonType> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDInlineKeyboardButtonTypeType {
  InlineKeyboardButtonTypeBuy,
  InlineKeyboardButtonTypeCallback,
  InlineKeyboardButtonTypeCallbackGame,
  InlineKeyboardButtonTypeSwitchInline,
  InlineKeyboardButtonTypeUrl,
  
}
impl RTDInlineKeyboardButtonTypeType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDInlineKeyboardButtonTypeType)(text.as_ref()) }
}



