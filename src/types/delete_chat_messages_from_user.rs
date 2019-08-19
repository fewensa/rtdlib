
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Deletes all messages sent by the specified user to a chat. Supported only in supergroups; requires can_delete_messages administrator privileges.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteChatMessagesFromUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // deleteChatMessagesFromUser
  /// Chat identifier.
  chat_id: Option<i64>,
  /// User identifier.
  user_id: Option<i32>,
  
}



impl Object for DeleteChatMessagesFromUser {}
impl RObject for DeleteChatMessagesFromUser {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deleteChatMessagesFromUser" }
  fn td_type(&self) -> RTDType { RTDType::DeleteChatMessagesFromUser }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for DeleteChatMessagesFromUser {}


impl DeleteChatMessagesFromUser {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "deleteChatMessagesFromUser".to_string(),
      chat_id: None,
      user_id: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



