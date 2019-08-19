
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Contains content of a push message notification. 
#[typetag::serde(tag = "@struct")]
pub trait PushMessageContent: Object + RObject + Debug {}






impl PushMessageContent {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<PushMessageContent> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDPushMessageContentType {
  PushMessageContentAnimation,
  PushMessageContentAudio,
  PushMessageContentBasicGroupChatCreate,
  PushMessageContentChatAddMembers,
  PushMessageContentChatChangePhoto,
  PushMessageContentChatChangeTitle,
  PushMessageContentChatDeleteMember,
  PushMessageContentChatJoinByLink,
  PushMessageContentContact,
  PushMessageContentContactRegistered,
  PushMessageContentDocument,
  PushMessageContentGame,
  PushMessageContentGameScore,
  PushMessageContentHidden,
  PushMessageContentInvoice,
  PushMessageContentLocation,
  PushMessageContentMediaAlbum,
  PushMessageContentMessageForwards,
  PushMessageContentPhoto,
  PushMessageContentPoll,
  PushMessageContentScreenshotTaken,
  PushMessageContentSticker,
  PushMessageContentText,
  PushMessageContentVideo,
  PushMessageContentVideoNote,
  PushMessageContentVoiceNote,
  
}
impl RTDPushMessageContentType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDPushMessageContentType)(text.as_ref()) }
}



