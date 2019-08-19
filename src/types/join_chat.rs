
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Adds current user as a new member to a chat. Private and secret chats can't be joined using this method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // joinChat
  /// Chat identifier.
  chat_id: Option<i64>,
  
}



impl Object for JoinChat {}
impl RObject for JoinChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "joinChat" }
  fn td_type(&self) -> RTDType { RTDType::JoinChat }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for JoinChat {}


impl JoinChat {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "joinChat".to_string(),
      chat_id: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



