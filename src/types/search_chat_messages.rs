
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Searches for messages with given words in the chat. Returns the results in reverse chronological order, i.e. in order of decreasing message_id. Cannot be used in secret chats with a non-empty query (
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchChatMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchChatMessages
  /// Identifier of the chat in which to search messages.
  chat_id: Option<i64>,
  /// Query to search for.
  query: Option<String>,
  /// If not 0, only messages sent by the specified user will be returned. Not supported in secret chats.
  sender_user_id: Option<i32>,
  /// Identifier of the message starting from which history must be fetched; use 0 to get results from the last message.
  from_message_id: Option<i64>,
  /// Specify 0 to get results from exactly the from_message_id or a negative offset to get the specified message and some newer messages.
  offset: Option<i32>,
  /// The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, the limit must be greater than -offset. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached.
  limit: Option<i32>,
  /// Filter for message content in the search results.
  filter: Option<Box<SearchMessagesFilter>>,
  
}


impl Clone for SearchChatMessages {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for SearchChatMessages {}
impl RObject for SearchChatMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchChatMessages" }
  fn td_type(&self) -> RTDType { RTDType::SearchChatMessages }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SearchChatMessages {}


impl SearchChatMessages {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchChatMessages".to_string(),
      chat_id: None,
      query: None,
      sender_user_id: None,
      from_message_id: None,
      offset: None,
      limit: None,
      filter: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn query(&self) -> Option<String> { self.query.clone() }
  #[doc(hidden)] pub fn _set_query(&mut self, query: String) -> &mut Self { self.query = Some(query); self }
  
  pub fn sender_user_id(&self) -> Option<i32> { self.sender_user_id.clone() }
  #[doc(hidden)] pub fn _set_sender_user_id(&mut self, sender_user_id: i32) -> &mut Self { self.sender_user_id = Some(sender_user_id); self }
  
  pub fn from_message_id(&self) -> Option<i64> { self.from_message_id.clone() }
  #[doc(hidden)] pub fn _set_from_message_id(&mut self, from_message_id: i64) -> &mut Self { self.from_message_id = Some(from_message_id); self }
  
  pub fn offset(&self) -> Option<i32> { self.offset.clone() }
  #[doc(hidden)] pub fn _set_offset(&mut self, offset: i32) -> &mut Self { self.offset = Some(offset); self }
  
  pub fn limit(&self) -> Option<i32> { self.limit.clone() }
  #[doc(hidden)] pub fn _set_limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  
  pub fn filter(&self) -> Option<Box<SearchMessagesFilter>> { self.filter.clone() }
  #[doc(hidden)] pub fn _set_filter(&mut self, filter: Box<SearchMessagesFilter>) -> &mut Self { self.filter = Some(filter); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



