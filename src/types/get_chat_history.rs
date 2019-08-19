
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns messages in a chat. The messages are returned in a reverse chronological order (i.e., in order of decreasing message_id). For optimal performance the number of returned messages is chosen by the library. This is an offline request if only_local is true.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatHistory {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getChatHistory
  /// Chat identifier.
  chat_id: Option<i64>,
  /// Identifier of the message starting from which history must be fetched; use 0 to get results from the last message.
  from_message_id: Option<i64>,
  /// Specify 0 to get results from exactly the from_message_id or a negative offset up to 99 to get additionally some newer messages.
  offset: Option<i32>,
  /// The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, the limit must be greater or equal to -offset. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached.
  limit: Option<i32>,
  /// If true, returns only messages that are available locally without sending network requests.
  only_local: Option<bool>,
  
}



impl Object for GetChatHistory {}
impl RObject for GetChatHistory {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getChatHistory" }
  fn td_type(&self) -> RTDType { RTDType::GetChatHistory }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetChatHistory {}


impl GetChatHistory {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getChatHistory".to_string(),
      chat_id: None,
      from_message_id: None,
      offset: None,
      limit: None,
      only_local: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn from_message_id(&self) -> Option<i64> { self.from_message_id.clone() }
  #[doc(hidden)] pub fn _set_from_message_id(&mut self, from_message_id: i64) -> &mut Self { self.from_message_id = Some(from_message_id); self }
  
  pub fn offset(&self) -> Option<i32> { self.offset.clone() }
  #[doc(hidden)] pub fn _set_offset(&mut self, offset: i32) -> &mut Self { self.offset = Some(offset); self }
  
  pub fn limit(&self) -> Option<i32> { self.limit.clone() }
  #[doc(hidden)] pub fn _set_limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  
  pub fn only_local(&self) -> Option<bool> { self.only_local.clone() }
  #[doc(hidden)] pub fn _set_only_local(&mut self, only_local: bool) -> &mut Self { self.only_local = Some(only_local); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



