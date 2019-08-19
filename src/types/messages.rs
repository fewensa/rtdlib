
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains a list of messages. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Messages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messages
  /// Approximate total count of messages found.
  total_count: Option<i32>,
  /// List of messages; messages may be null.
  messages: Option<Vec<Message>>,
  
}



impl Object for Messages {}
impl RObject for Messages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messages" }
  fn td_type(&self) -> RTDType { RTDType::Messages }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Messages {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messages".to_string(),
      total_count: None,
      messages: None,
      
    }
  }
  
  pub fn total_count(&self) -> Option<i32> { self.total_count.clone() }
  #[doc(hidden)] pub fn _set_total_count(&mut self, total_count: i32) -> &mut Self { self.total_count = Some(total_count); self }
  
  pub fn messages(&self) -> Option<Vec<Message>> { self.messages.clone() }
  #[doc(hidden)] pub fn _set_messages(&mut self, messages: Vec<Message>) -> &mut Self { self.messages = Some(messages); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



