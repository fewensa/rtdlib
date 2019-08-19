
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Adds multiple new members to a chat. Currently this option is only available for supergroups and channels. This option can't be used to join a chat. Members can't be added to a channel if it has more than 200 members. Members will not be added until the chat state has been synchronized with the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddChatMembers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // addChatMembers
  /// Chat identifier.
  chat_id: Option<i64>,
  /// Identifiers of the users to be added to the chat.
  user_ids: Option<Vec<i32>>,
  
}



impl Object for AddChatMembers {}
impl RObject for AddChatMembers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "addChatMembers" }
  fn td_type(&self) -> RTDType { RTDType::AddChatMembers }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for AddChatMembers {}


impl AddChatMembers {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "addChatMembers".to_string(),
      chat_id: None,
      user_ids: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn user_ids(&self) -> Option<Vec<i32>> { self.user_ids.clone() }
  #[doc(hidden)] pub fn _set_user_ids(&mut self, user_ids: Vec<i32>) -> &mut Self { self.user_ids = Some(user_ids); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



