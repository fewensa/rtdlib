
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Adds a message to TDLib internal log. This is an offline method. Can be called before authorization. Can be called synchronously.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddLogMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // addLogMessage
  /// Minimum verbosity level needed for the message to be logged, 0-1023.
  verbosity_level: Option<i32>,
  /// Text of a message to log.
  text: Option<String>,
  
}



impl Object for AddLogMessage {}
impl RObject for AddLogMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "addLogMessage" }
  fn td_type(&self) -> RTDType { RTDType::AddLogMessage }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for AddLogMessage {}


impl AddLogMessage {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "addLogMessage".to_string(),
      verbosity_level: None,
      text: None,
      
    }
  }
  
  pub fn verbosity_level(&self) -> Option<i32> { self.verbosity_level.clone() }
  #[doc(hidden)] pub fn _set_verbosity_level(&mut self, verbosity_level: i32) -> &mut Self { self.verbosity_level = Some(verbosity_level); self }
  
  pub fn text(&self) -> Option<String> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: String) -> &mut Self { self.text = Some(text); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



