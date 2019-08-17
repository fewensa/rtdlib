
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Describes the reason why a call was discarded. 
#[typetag::serde(tag = "@struct")]
pub trait CallDiscardReason: Object + RObject + Debug {}






impl CallDiscardReason {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<CallDiscardReason> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDCallDiscardReasonType {
  CallDiscardReasonDeclined,
  CallDiscardReasonDisconnected,
  CallDiscardReasonEmpty,
  CallDiscardReasonHungUp,
  CallDiscardReasonMissed,
  
}
impl RTDCallDiscardReasonType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDCallDiscardReasonType)(text.as_ref()) }
}



