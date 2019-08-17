
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Describes the different types of activity in a chat. 
#[typetag::serde(tag = "@struct")]
pub trait ChatAction: Object + RObject + Debug {}






impl ChatAction {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<ChatAction> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDChatActionType {
  ChatActionCancel,
  ChatActionChoosingContact,
  ChatActionChoosingLocation,
  ChatActionRecordingVideo,
  ChatActionRecordingVideoNote,
  ChatActionRecordingVoiceNote,
  ChatActionStartPlayingGame,
  ChatActionTyping,
  ChatActionUploadingDocument,
  ChatActionUploadingPhoto,
  ChatActionUploadingVideo,
  ChatActionUploadingVideoNote,
  ChatActionUploadingVoiceNote,
  
}
impl RTDChatActionType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDChatActionType)(text.as_ref()) }
}



