
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A new incoming event; for bots only. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNewCustomEvent {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateNewCustomEvent
  /// A JSON-serialized event.
  event: Option<String>,
  
}



impl Object for UpdateNewCustomEvent {}
impl RObject for UpdateNewCustomEvent {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateNewCustomEvent" }
  fn td_type(&self) -> RTDType { RTDType::UpdateNewCustomEvent }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateNewCustomEvent {}


impl UpdateNewCustomEvent {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateNewCustomEvent".to_string(),
      event: None,
      
    }
  }
  
  pub fn event(&self) -> Option<String> { self.event.clone() }
  #[doc(hidden)] pub fn _set_event(&mut self, event: String) -> &mut Self { self.event = Some(event); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



