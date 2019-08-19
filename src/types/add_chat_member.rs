
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Adds a new member to a chat. Members can't be added to private or secret chats. Members will not be added until the chat state has been synchronized with the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddChatMember {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // addChatMember
  /// Chat identifier.
  chat_id: Option<i64>,
  /// Identifier of the user.
  user_id: Option<i32>,
  /// The number of earlier messages from the chat to be forwarded to the new member; up to 100. Ignored for supergroups and channels.
  forward_limit: Option<i32>,
  
}



impl Object for AddChatMember {}
impl RObject for AddChatMember {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "addChatMember" }
  fn td_type(&self) -> RTDType { RTDType::AddChatMember }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for AddChatMember {}


impl AddChatMember {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "addChatMember".to_string(),
      chat_id: None,
      user_id: None,
      forward_limit: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn forward_limit(&self) -> Option<i32> { self.forward_limit.clone() }
  #[doc(hidden)] pub fn _set_forward_limit(&mut self, forward_limit: i32) -> &mut Self { self.forward_limit = Some(forward_limit); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



