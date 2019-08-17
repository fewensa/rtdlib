
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A chat was marked as unread or was read. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChatIsMarkedAsUnread {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateChatIsMarkedAsUnread
  /// Chat identifier.
  chat_id: Option<i64>,
  /// New value of is_marked_as_unread.
  is_marked_as_unread: Option<bool>,
  
}



impl Object for UpdateChatIsMarkedAsUnread {}
impl RObject for UpdateChatIsMarkedAsUnread {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatIsMarkedAsUnread" }
  fn td_type(&self) -> RTDType { RTDType::UpdateChatIsMarkedAsUnread }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateChatIsMarkedAsUnread {}


impl UpdateChatIsMarkedAsUnread {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateChatIsMarkedAsUnread".to_string(),
      chat_id: None,
      is_marked_as_unread: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn is_marked_as_unread(&self) -> Option<bool> { self.is_marked_as_unread.clone() }
  #[doc(hidden)] pub fn _set_is_marked_as_unread(&mut self, is_marked_as_unread: bool) -> &mut Self { self.is_marked_as_unread = Some(is_marked_as_unread); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



