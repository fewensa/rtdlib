
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Represents a payload of a callback query. 
#[typetag::serde(tag = "@struct")]
pub trait CallbackQueryPayload: Object + RObject + Debug {}






impl CallbackQueryPayload {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<CallbackQueryPayload> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDCallbackQueryPayloadType {
  CallbackQueryPayloadData,
  CallbackQueryPayloadGame,
  
}
impl RTDCallbackQueryPayloadType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDCallbackQueryPayloadType)(text.as_ref()) }
}



