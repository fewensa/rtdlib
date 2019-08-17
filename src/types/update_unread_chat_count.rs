
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Number of unread chats, i.e. with unread messages or marked as unread, has changed. This update is sent only if a message database is used. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUnreadChatCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateUnreadChatCount
  /// Total number of unread chats.
  unread_count: Option<i32>,
  /// Total number of unread unmuted chats.
  unread_unmuted_count: Option<i32>,
  /// Total number of chats marked as unread.
  marked_as_unread_count: Option<i32>,
  /// Total number of unmuted chats marked as unread.
  marked_as_unread_unmuted_count: Option<i32>,
  
}



impl Object for UpdateUnreadChatCount {}
impl RObject for UpdateUnreadChatCount {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateUnreadChatCount" }
  fn td_type(&self) -> RTDType { RTDType::UpdateUnreadChatCount }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateUnreadChatCount {}


impl UpdateUnreadChatCount {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateUnreadChatCount".to_string(),
      unread_count: None,
      unread_unmuted_count: None,
      marked_as_unread_count: None,
      marked_as_unread_unmuted_count: None,
      
    }
  }
  
  pub fn unread_count(&self) -> Option<i32> { self.unread_count.clone() }
  #[doc(hidden)] pub fn _set_unread_count(&mut self, unread_count: i32) -> &mut Self { self.unread_count = Some(unread_count); self }
  
  pub fn unread_unmuted_count(&self) -> Option<i32> { self.unread_unmuted_count.clone() }
  #[doc(hidden)] pub fn _set_unread_unmuted_count(&mut self, unread_unmuted_count: i32) -> &mut Self { self.unread_unmuted_count = Some(unread_unmuted_count); self }
  
  pub fn marked_as_unread_count(&self) -> Option<i32> { self.marked_as_unread_count.clone() }
  #[doc(hidden)] pub fn _set_marked_as_unread_count(&mut self, marked_as_unread_count: i32) -> &mut Self { self.marked_as_unread_count = Some(marked_as_unread_count); self }
  
  pub fn marked_as_unread_unmuted_count(&self) -> Option<i32> { self.marked_as_unread_unmuted_count.clone() }
  #[doc(hidden)] pub fn _set_marked_as_unread_unmuted_count(&mut self, marked_as_unread_unmuted_count: i32) -> &mut Self { self.marked_as_unread_unmuted_count = Some(marked_as_unread_unmuted_count); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



