
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Searches for messages in all chats except secret chats. Returns the results in reverse chronological order (i.e., in order of decreasing (date, chat_id, message_id)). For optimal performance the number of returned messages is chosen by the library.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchMessages
  /// Query to search for.
  query: Option<String>,
  /// The date of the message starting from which the results should be fetched. Use 0 or any date in the future to get results from the last message.
  offset_date: Option<i32>,
  /// The chat identifier of the last found message, or 0 for the first request.
  offset_chat_id: Option<i64>,
  /// The message identifier of the last found message, or 0 for the first request.
  offset_message_id: Option<i64>,
  /// The maximum number of messages to be returned, up to 100. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached.
  limit: Option<i32>,
  
}



impl Object for SearchMessages {}
impl RObject for SearchMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessages" }
  fn td_type(&self) -> RTDType { RTDType::SearchMessages }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SearchMessages {}


impl SearchMessages {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchMessages".to_string(),
      query: None,
      offset_date: None,
      offset_chat_id: None,
      offset_message_id: None,
      limit: None,
      
    }
  }
  
  pub fn query(&self) -> Option<String> { self.query.clone() }
  #[doc(hidden)] pub fn _set_query(&mut self, query: String) -> &mut Self { self.query = Some(query); self }
  
  pub fn offset_date(&self) -> Option<i32> { self.offset_date.clone() }
  #[doc(hidden)] pub fn _set_offset_date(&mut self, offset_date: i32) -> &mut Self { self.offset_date = Some(offset_date); self }
  
  pub fn offset_chat_id(&self) -> Option<i64> { self.offset_chat_id.clone() }
  #[doc(hidden)] pub fn _set_offset_chat_id(&mut self, offset_chat_id: i64) -> &mut Self { self.offset_chat_id = Some(offset_chat_id); self }
  
  pub fn offset_message_id(&self) -> Option<i64> { self.offset_message_id.clone() }
  #[doc(hidden)] pub fn _set_offset_message_id(&mut self, offset_message_id: i64) -> &mut Self { self.offset_message_id = Some(offset_message_id); self }
  
  pub fn limit(&self) -> Option<i32> { self.limit.clone() }
  #[doc(hidden)] pub fn _set_limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



