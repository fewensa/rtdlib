
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns information about the recent locations of chat members that were sent to the chat. Returns up to 1 location message per user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchChatRecentLocationMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchChatRecentLocationMessages
  /// Chat identifier.
  chat_id: Option<i64>,
  /// Maximum number of messages to be returned.
  limit: Option<i32>,
  
}



impl Object for SearchChatRecentLocationMessages {}
impl RObject for SearchChatRecentLocationMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchChatRecentLocationMessages" }
  fn td_type(&self) -> RTDType { RTDType::SearchChatRecentLocationMessages }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SearchChatRecentLocationMessages {}


impl SearchChatRecentLocationMessages {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchChatRecentLocationMessages".to_string(),
      chat_id: None,
      limit: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn limit(&self) -> Option<i32> { self.limit.clone() }
  #[doc(hidden)] pub fn _set_limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



