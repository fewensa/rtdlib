
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Searches for call messages. Returns the results in reverse chronological order (i. e., in order of decreasing message_id). For optimal performance the number of returned messages is chosen by the library.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCallMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchCallMessages
  /// Identifier of the message from which to search; use 0 to get results from the last message.
  from_message_id: Option<i64>,
  /// The maximum number of messages to be returned; up to 100. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached.
  limit: Option<i32>,
  /// If true, returns only messages with missed calls.
  only_missed: Option<bool>,
  
}



impl Object for SearchCallMessages {}
impl RObject for SearchCallMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchCallMessages" }
  fn td_type(&self) -> RTDType { RTDType::SearchCallMessages }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SearchCallMessages {}


impl SearchCallMessages {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchCallMessages".to_string(),
      from_message_id: None,
      limit: None,
      only_missed: None,
      
    }
  }
  
  pub fn from_message_id(&self) -> Option<i64> { self.from_message_id.clone() }
  #[doc(hidden)] pub fn _set_from_message_id(&mut self, from_message_id: i64) -> &mut Self { self.from_message_id = Some(from_message_id); self }
  
  pub fn limit(&self) -> Option<i32> { self.limit.clone() }
  #[doc(hidden)] pub fn _set_limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  
  pub fn only_missed(&self) -> Option<bool> { self.only_missed.clone() }
  #[doc(hidden)] pub fn _set_only_missed(&mut self, only_missed: bool) -> &mut Self { self.only_missed = Some(only_missed); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



