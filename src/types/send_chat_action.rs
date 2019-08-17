
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Sends a notification about user activity in a chat.
#[derive(Debug, Serialize, Deserialize)]
pub struct SendChatAction {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // sendChatAction
  /// Chat identifier.
  chat_id: Option<i64>,
  /// The action description.
  action: Option<Box<ChatAction>>,
  
}


impl Clone for SendChatAction {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for SendChatAction {}
impl RObject for SendChatAction {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendChatAction" }
  fn td_type(&self) -> RTDType { RTDType::SendChatAction }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SendChatAction {}


impl SendChatAction {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "sendChatAction".to_string(),
      chat_id: None,
      action: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn action(&self) -> Option<Box<ChatAction>> { self.action.clone() }
  #[doc(hidden)] pub fn _set_action(&mut self, action: Box<ChatAction>) -> &mut Self { self.action = Some(action); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



