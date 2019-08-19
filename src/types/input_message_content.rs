
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. The content of a message to send. 
#[typetag::serde(tag = "@struct")]
pub trait InputMessageContent: Object + RObject + Debug {}






impl InputMessageContent {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<InputMessageContent> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDInputMessageContentType {
  InputMessageAnimation,
  InputMessageAudio,
  InputMessageContact,
  InputMessageDocument,
  InputMessageForwarded,
  InputMessageGame,
  InputMessageInvoice,
  InputMessageLocation,
  InputMessagePhoto,
  InputMessagePoll,
  InputMessageSticker,
  InputMessageText,
  InputMessageVenue,
  InputMessageVideo,
  InputMessageVideoNote,
  InputMessageVoiceNote,
  
}
impl RTDInputMessageContentType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDInputMessageContentType)(text.as_ref()) }
}



