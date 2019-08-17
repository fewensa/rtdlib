
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Checks whether a username can be set for a chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckChatUsername {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // checkChatUsername
  /// Chat identifier; should be identifier of a supergroup chat, or a channel chat, or a private chat with self, or zero if chat is being created.
  chat_id: Option<i64>,
  /// Username to be checked.
  username: Option<String>,
  
}



impl Object for CheckChatUsername {}
impl RObject for CheckChatUsername {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkChatUsername" }
  fn td_type(&self) -> RTDType { RTDType::CheckChatUsername }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for CheckChatUsername {}


impl CheckChatUsername {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "checkChatUsername".to_string(),
      chat_id: None,
      username: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn username(&self) -> Option<String> { self.username.clone() }
  #[doc(hidden)] pub fn _set_username(&mut self, username: String) -> &mut Self { self.username = Some(username); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



