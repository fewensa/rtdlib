
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Number of unread messages has changed. This update is sent only if a message database is used. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUnreadMessageCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateUnreadMessageCount
  /// Total number of unread messages.
  unread_count: Option<i32>,
  /// Total number of unread messages in unmuted chats.
  unread_unmuted_count: Option<i32>,
  
}



impl Object for UpdateUnreadMessageCount {}
impl RObject for UpdateUnreadMessageCount {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateUnreadMessageCount" }
  fn td_type(&self) -> RTDType { RTDType::UpdateUnreadMessageCount }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateUnreadMessageCount {}


impl UpdateUnreadMessageCount {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateUnreadMessageCount".to_string(),
      unread_count: None,
      unread_unmuted_count: None,
      
    }
  }
  
  pub fn unread_count(&self) -> Option<i32> { self.unread_count.clone() }
  #[doc(hidden)] pub fn _set_unread_count(&mut self, unread_count: i32) -> &mut Self { self.unread_count = Some(unread_count); self }
  
  pub fn unread_unmuted_count(&self) -> Option<i32> { self.unread_unmuted_count.clone() }
  #[doc(hidden)] pub fn _set_unread_unmuted_count(&mut self, unread_unmuted_count: i32) -> &mut Self { self.unread_unmuted_count = Some(unread_unmuted_count); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



