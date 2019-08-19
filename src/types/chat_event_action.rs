
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Represents a chat event. 
#[typetag::serde(tag = "@struct")]
pub trait ChatEventAction: Object + RObject + Debug {}






impl ChatEventAction {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<ChatEventAction> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDChatEventActionType {
  ChatEventDescriptionChanged,
  ChatEventInvitesToggled,
  ChatEventIsAllHistoryAvailableToggled,
  ChatEventMemberInvited,
  ChatEventMemberJoined,
  ChatEventMemberLeft,
  ChatEventMemberPromoted,
  ChatEventMemberRestricted,
  ChatEventMessageDeleted,
  ChatEventMessageEdited,
  ChatEventMessagePinned,
  ChatEventMessageUnpinned,
  ChatEventPhotoChanged,
  ChatEventSignMessagesToggled,
  ChatEventStickerSetChanged,
  ChatEventTitleChanged,
  ChatEventUsernameChanged,
  
}
impl RTDChatEventActionType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDChatEventActionType)(text.as_ref()) }
}



