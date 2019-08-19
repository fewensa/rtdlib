
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Informs the server about the number of pending bot updates if they haven't been processed for a long time; for bots only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetBotUpdatesStatus {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setBotUpdatesStatus
  /// The number of pending updates.
  pending_update_count: Option<i32>,
  /// The last error message.
  error_message: Option<String>,
  
}



impl Object for SetBotUpdatesStatus {}
impl RObject for SetBotUpdatesStatus {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setBotUpdatesStatus" }
  fn td_type(&self) -> RTDType { RTDType::SetBotUpdatesStatus }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetBotUpdatesStatus {}


impl SetBotUpdatesStatus {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setBotUpdatesStatus".to_string(),
      pending_update_count: None,
      error_message: None,
      
    }
  }
  
  pub fn pending_update_count(&self) -> Option<i32> { self.pending_update_count.clone() }
  #[doc(hidden)] pub fn _set_pending_update_count(&mut self, pending_update_count: i32) -> &mut Self { self.pending_update_count = Some(pending_update_count); self }
  
  pub fn error_message(&self) -> Option<String> { self.error_message.clone() }
  #[doc(hidden)] pub fn _set_error_message(&mut self, error_message: String) -> &mut Self { self.error_message = Some(error_message); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



