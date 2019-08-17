
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// User activity in the chat has changed. 
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserChatAction {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateUserChatAction
  /// Chat identifier.
  chat_id: Option<i64>,
  /// Identifier of a user performing an action.
  user_id: Option<i32>,
  /// The action description.
  action: Option<Box<ChatAction>>,
  
}


impl Clone for UpdateUserChatAction {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for UpdateUserChatAction {}
impl RObject for UpdateUserChatAction {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateUserChatAction" }
  fn td_type(&self) -> RTDType { RTDType::UpdateUserChatAction }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateUserChatAction {}


impl UpdateUserChatAction {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateUserChatAction".to_string(),
      chat_id: None,
      user_id: None,
      action: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn action(&self) -> Option<Box<ChatAction>> { self.action.clone() }
  #[doc(hidden)] pub fn _set_action(&mut self, action: Box<ChatAction>) -> &mut Self { self.action = Some(action); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



