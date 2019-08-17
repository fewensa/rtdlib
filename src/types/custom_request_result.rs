
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains the result of a custom request. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomRequestResult {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // customRequestResult
  /// A JSON-serialized result.
  result: Option<String>,
  
}



impl Object for CustomRequestResult {}
impl RObject for CustomRequestResult {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "customRequestResult" }
  fn td_type(&self) -> RTDType { RTDType::CustomRequestResult }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl CustomRequestResult {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "customRequestResult".to_string(),
      result: None,
      
    }
  }
  
  pub fn result(&self) -> Option<String> { self.result.clone() }
  #[doc(hidden)] pub fn _set_result(&mut self, result: String) -> &mut Self { self.result = Some(result); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



