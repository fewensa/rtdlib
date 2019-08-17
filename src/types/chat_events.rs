
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains a list of chat events. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEvents {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatEvents
  /// List of events.
  events: Option<Vec<ChatEvent>>,
  
}



impl Object for ChatEvents {}
impl RObject for ChatEvents {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEvents" }
  fn td_type(&self) -> RTDType { RTDType::ChatEvents }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl ChatEvents {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatEvents".to_string(),
      events: None,
      
    }
  }
  
  pub fn events(&self) -> Option<Vec<ChatEvent>> { self.events.clone() }
  #[doc(hidden)] pub fn _set_events(&mut self, events: Vec<ChatEvent>) -> &mut Self { self.events = Some(events); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



