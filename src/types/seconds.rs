
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains a value representing a number of seconds. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Seconds {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // seconds
  /// Number of seconds.
  seconds: Option<f64>,
  
}



impl Object for Seconds {}
impl RObject for Seconds {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "seconds" }
  fn td_type(&self) -> RTDType { RTDType::Seconds }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Seconds {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "seconds".to_string(),
      seconds: None,
      
    }
  }
  
  pub fn seconds(&self) -> Option<f64> { self.seconds.clone() }
  #[doc(hidden)] pub fn _set_seconds(&mut self, seconds: f64) -> &mut Self { self.seconds = Some(seconds); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



