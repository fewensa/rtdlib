
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Searches for messages in secret chats. Returns the results in reverse chronological order. For optimal performance the number of returned messages is chosen by the library.
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchSecretMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchSecretMessages
  /// Identifier of the chat in which to search. Specify 0 to search in all secret chats.
  chat_id: Option<i64>,
  /// Query to search for. If empty, searchChatMessages should be used instead.
  query: Option<String>,
  /// The identifier from the result of a previous request, use 0 to get results from the last message.
  from_search_id: Option<i64>,
  /// Maximum number of messages to be returned; up to 100. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached.
  limit: Option<i32>,
  /// A filter for the content of messages in the search results.
  filter: Option<Box<SearchMessagesFilter>>,
  
}


impl Clone for SearchSecretMessages {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for SearchSecretMessages {}
impl RObject for SearchSecretMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchSecretMessages" }
  fn td_type(&self) -> RTDType { RTDType::SearchSecretMessages }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SearchSecretMessages {}


impl SearchSecretMessages {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchSecretMessages".to_string(),
      chat_id: None,
      query: None,
      from_search_id: None,
      limit: None,
      filter: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn query(&self) -> Option<String> { self.query.clone() }
  #[doc(hidden)] pub fn _set_query(&mut self, query: String) -> &mut Self { self.query = Some(query); self }
  
  pub fn from_search_id(&self) -> Option<i64> { self.from_search_id.clone() }
  #[doc(hidden)] pub fn _set_from_search_id(&mut self, from_search_id: i64) -> &mut Self { self.from_search_id = Some(from_search_id); self }
  
  pub fn limit(&self) -> Option<i32> { self.limit.clone() }
  #[doc(hidden)] pub fn _set_limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  
  pub fn filter(&self) -> Option<Box<SearchMessagesFilter>> { self.filter.clone() }
  #[doc(hidden)] pub fn _set_filter(&mut self, filter: Box<SearchMessagesFilter>) -> &mut Self { self.filter = Some(filter); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



