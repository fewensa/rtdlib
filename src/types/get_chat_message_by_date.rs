
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns the last message sent in a chat no later than the specified date.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatMessageByDate {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getChatMessageByDate
  /// Chat identifier.
  chat_id: Option<i64>,
  /// Point in time (Unix timestamp) relative to which to search for messages.
  date: Option<i32>,
  
}



impl Object for GetChatMessageByDate {}
impl RObject for GetChatMessageByDate {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getChatMessageByDate" }
  fn td_type(&self) -> RTDType { RTDType::GetChatMessageByDate }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetChatMessageByDate {}


impl GetChatMessageByDate {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getChatMessageByDate".to_string(),
      chat_id: None,
      date: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn date(&self) -> Option<i32> { self.date.clone() }
  #[doc(hidden)] pub fn _set_date(&mut self, date: i32) -> &mut Self { self.date = Some(date); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



