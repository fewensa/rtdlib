
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Succeeds after a specified amount of time has passed. Can be called before authorization. Can be called before initialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetAlarm {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setAlarm
  /// Number of seconds before the function returns.
  seconds: Option<f64>,
  
}



impl Object for SetAlarm {}
impl RObject for SetAlarm {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setAlarm" }
  fn td_type(&self) -> RTDType { RTDType::SetAlarm }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetAlarm {}


impl SetAlarm {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setAlarm".to_string(),
      seconds: None,
      
    }
  }
  
  pub fn seconds(&self) -> Option<f64> { self.seconds.clone() }
  #[doc(hidden)] pub fn _set_seconds(&mut self, seconds: f64) -> &mut Self { self.seconds = Some(seconds); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



