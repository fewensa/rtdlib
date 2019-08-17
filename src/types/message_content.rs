
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Contains the content of a message. 
#[typetag::serde(tag = "@struct")]
pub trait MessageContent: Object + RObject + Debug {}






impl MessageContent {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<MessageContent> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDMessageContentType {
  MessageAnimation,
  MessageAudio,
  MessageBasicGroupChatCreate,
  MessageCall,
  MessageChatAddMembers,
  MessageChatChangePhoto,
  MessageChatChangeTitle,
  MessageChatDeleteMember,
  MessageChatDeletePhoto,
  MessageChatJoinByLink,
  MessageChatSetTtl,
  MessageChatUpgradeFrom,
  MessageChatUpgradeTo,
  MessageContact,
  MessageContactRegistered,
  MessageCustomServiceAction,
  MessageDocument,
  MessageExpiredPhoto,
  MessageExpiredVideo,
  MessageGame,
  MessageGameScore,
  MessageInvoice,
  MessageLocation,
  MessagePassportDataReceived,
  MessagePassportDataSent,
  MessagePaymentSuccessful,
  MessagePaymentSuccessfulBot,
  MessagePhoto,
  MessagePinMessage,
  MessagePoll,
  MessageScreenshotTaken,
  MessageSticker,
  MessageSupergroupChatCreate,
  MessageText,
  MessageUnsupported,
  MessageVenue,
  MessageVideo,
  MessageVideoNote,
  MessageVoiceNote,
  MessageWebsiteConnected,
  
}
impl RTDMessageContentType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDMessageContentType)(text.as_ref()) }
}



