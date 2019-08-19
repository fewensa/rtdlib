
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns a list of service actions taken by chat members and administrators in the last 48 hours. Available only in supergroups and channels. Requires administrator rights. Returns results in reverse chronological order (i. e., in order of decreasing event_id).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatEventLog {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getChatEventLog
  /// Chat identifier.
  chat_id: Option<i64>,
  /// Search query by which to filter events.
  query: Option<String>,
  /// Identifier of an event from which to return results. Use 0 to get results from the latest events.
  from_event_id: Option<i64>,
  /// Maximum number of events to return; up to 100.
  limit: Option<i32>,
  /// The types of events to return. By default, all types will be returned.
  filters: Option<ChatEventLogFilters>,
  /// User identifiers by which to filter events. By default, events relating to all users will be returned.
  user_ids: Option<Vec<i32>>,
  
}



impl Object for GetChatEventLog {}
impl RObject for GetChatEventLog {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getChatEventLog" }
  fn td_type(&self) -> RTDType { RTDType::GetChatEventLog }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetChatEventLog {}


impl GetChatEventLog {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getChatEventLog".to_string(),
      chat_id: None,
      query: None,
      from_event_id: None,
      limit: None,
      filters: None,
      user_ids: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn query(&self) -> Option<String> { self.query.clone() }
  #[doc(hidden)] pub fn _set_query(&mut self, query: String) -> &mut Self { self.query = Some(query); self }
  
  pub fn from_event_id(&self) -> Option<i64> { self.from_event_id.clone() }
  #[doc(hidden)] pub fn _set_from_event_id(&mut self, from_event_id: i64) -> &mut Self { self.from_event_id = Some(from_event_id); self }
  
  pub fn limit(&self) -> Option<i32> { self.limit.clone() }
  #[doc(hidden)] pub fn _set_limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  
  pub fn filters(&self) -> Option<ChatEventLogFilters> { self.filters.clone() }
  #[doc(hidden)] pub fn _set_filters(&mut self, filters: ChatEventLogFilters) -> &mut Self { self.filters = Some(filters); self }
  
  pub fn user_ids(&self) -> Option<Vec<i32>> { self.user_ids.clone() }
  #[doc(hidden)] pub fn _set_user_ids(&mut self, user_ids: Vec<i32>) -> &mut Self { self.user_ids = Some(user_ids); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



