
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Contains a description of a custom keyboard and actions that can be done with it to quickly reply to bots. 
#[typetag::serde(tag = "@struct")]
pub trait ReplyMarkup: Object + RObject + Debug {}






impl ReplyMarkup {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<ReplyMarkup> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDReplyMarkupType {
  ReplyMarkupForceReply,
  ReplyMarkupInlineKeyboard,
  ReplyMarkupRemoveKeyboard,
  ReplyMarkupShowKeyboard,
  
}
impl RTDReplyMarkupType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDReplyMarkupType)(text.as_ref()) }
}



