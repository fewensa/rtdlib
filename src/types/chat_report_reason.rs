
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Describes the reason why a chat is reported. 
#[typetag::serde(tag = "@struct")]
pub trait ChatReportReason: Object + RObject + Debug {}






impl ChatReportReason {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<ChatReportReason> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDChatReportReasonType {
  ChatReportReasonChildAbuse,
  ChatReportReasonCopyright,
  ChatReportReasonCustom,
  ChatReportReasonPornography,
  ChatReportReasonSpam,
  ChatReportReasonViolence,
  
}
impl RTDChatReportReasonType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDChatReportReasonType)(text.as_ref()) }
}



