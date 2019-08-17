
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user is under certain restrictions in the chat. Not supported in basic groups and channels. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberStatusRestricted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatMemberStatusRestricted
  /// True, if the user is a member of the chat.
  is_member: Option<bool>,
  /// Point in time (Unix timestamp) when restrictions will be lifted from the user; 0 if never. If the user is restricted for more than 366 days or for less than 30 seconds from the current time, the user is considered to be restricted forever.
  restricted_until_date: Option<i32>,
  /// True, if the user can send text messages, contacts, locations, and venues.
  can_send_messages: Option<bool>,
  /// True, if the user can send audio files, documents, photos, videos, video notes, and voice notes. Implies can_send_messages permissions.
  can_send_media_messages: Option<bool>,
  /// True, if the user can send animations, games, and stickers and use inline bots. Implies can_send_media_messages permissions.
  can_send_other_messages: Option<bool>,
  /// True, if the user may add a web page preview to his messages. Implies can_send_messages permissions.
  can_add_web_page_previews: Option<bool>,
  
}



impl Object for ChatMemberStatusRestricted {}
impl RObject for ChatMemberStatusRestricted {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatMemberStatusRestricted" }
  fn td_type(&self) -> RTDType { RTDType::ChatMemberStatusRestricted }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatMemberStatus for ChatMemberStatusRestricted {}


impl ChatMemberStatusRestricted {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatMemberStatusRestricted".to_string(),
      is_member: None,
      restricted_until_date: None,
      can_send_messages: None,
      can_send_media_messages: None,
      can_send_other_messages: None,
      can_add_web_page_previews: None,
      
    }
  }
  
  pub fn is_member(&self) -> Option<bool> { self.is_member.clone() }
  #[doc(hidden)] pub fn _set_is_member(&mut self, is_member: bool) -> &mut Self { self.is_member = Some(is_member); self }
  
  pub fn restricted_until_date(&self) -> Option<i32> { self.restricted_until_date.clone() }
  #[doc(hidden)] pub fn _set_restricted_until_date(&mut self, restricted_until_date: i32) -> &mut Self { self.restricted_until_date = Some(restricted_until_date); self }
  
  pub fn can_send_messages(&self) -> Option<bool> { self.can_send_messages.clone() }
  #[doc(hidden)] pub fn _set_can_send_messages(&mut self, can_send_messages: bool) -> &mut Self { self.can_send_messages = Some(can_send_messages); self }
  
  pub fn can_send_media_messages(&self) -> Option<bool> { self.can_send_media_messages.clone() }
  #[doc(hidden)] pub fn _set_can_send_media_messages(&mut self, can_send_media_messages: bool) -> &mut Self { self.can_send_media_messages = Some(can_send_media_messages); self }
  
  pub fn can_send_other_messages(&self) -> Option<bool> { self.can_send_other_messages.clone() }
  #[doc(hidden)] pub fn _set_can_send_other_messages(&mut self, can_send_other_messages: bool) -> &mut Self { self.can_send_other_messages = Some(can_send_other_messages); self }
  
  pub fn can_add_web_page_previews(&self) -> Option<bool> { self.can_add_web_page_previews.clone() }
  #[doc(hidden)] pub fn _set_can_add_web_page_previews(&mut self, can_add_web_page_previews: bool) -> &mut Self { self.can_add_web_page_previews = Some(can_add_web_page_previews); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



