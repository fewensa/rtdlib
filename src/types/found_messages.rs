
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains a list of messages found by a search. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoundMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // foundMessages
  /// List of messages.
  messages: Option<Vec<Message>>,
  /// Value to pass as from_search_id to get more results.
  next_from_search_id: Option<i64>,
  
}



impl Object for FoundMessages {}
impl RObject for FoundMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "foundMessages" }
  fn td_type(&self) -> RTDType { RTDType::FoundMessages }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl FoundMessages {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "foundMessages".to_string(),
      messages: None,
      next_from_search_id: None,
      
    }
  }
  
  pub fn messages(&self) -> Option<Vec<Message>> { self.messages.clone() }
  #[doc(hidden)] pub fn _set_messages(&mut self, messages: Vec<Message>) -> &mut Self { self.messages = Some(messages); self }
  
  pub fn next_from_search_id(&self) -> Option<i64> { self.next_from_search_id.clone() }
  #[doc(hidden)] pub fn _set_next_from_search_id(&mut self, next_from_search_id: i64) -> &mut Self { self.next_from_search_id = Some(next_from_search_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



