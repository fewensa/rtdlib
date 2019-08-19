
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Deletes all messages in the chat. Use Chat.can_be_deleted_only_for_self and Chat.can_be_deleted_for_all_users fields to find whether and how the method can be applied to the chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteChatHistory {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // deleteChatHistory
  /// Chat identifier.
  chat_id: Option<i64>,
  /// Pass true if the chat should be removed from the chat list.
  remove_from_chat_list: Option<bool>,
  /// Pass true to try to delete chat history for all users.
  revoke: Option<bool>,
  
}



impl Object for DeleteChatHistory {}
impl RObject for DeleteChatHistory {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deleteChatHistory" }
  fn td_type(&self) -> RTDType { RTDType::DeleteChatHistory }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for DeleteChatHistory {}


impl DeleteChatHistory {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "deleteChatHistory".to_string(),
      chat_id: None,
      remove_from_chat_list: None,
      revoke: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn remove_from_chat_list(&self) -> Option<bool> { self.remove_from_chat_list.clone() }
  #[doc(hidden)] pub fn _set_remove_from_chat_list(&mut self, remove_from_chat_list: bool) -> &mut Self { self.remove_from_chat_list = Some(remove_from_chat_list); self }
  
  pub fn revoke(&self) -> Option<bool> { self.revoke.clone() }
  #[doc(hidden)] pub fn _set_revoke(&mut self, revoke: bool) -> &mut Self { self.revoke = Some(revoke); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



