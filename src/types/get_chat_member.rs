
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns information about a single member of a chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatMember {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getChatMember
  /// Chat identifier.
  chat_id: Option<i64>,
  /// User identifier.
  user_id: Option<i32>,
  
}



impl Object for GetChatMember {}
impl RObject for GetChatMember {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getChatMember" }
  fn td_type(&self) -> RTDType { RTDType::GetChatMember }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetChatMember {}


impl GetChatMember {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getChatMember".to_string(),
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



