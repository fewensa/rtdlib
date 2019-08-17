
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Sends a custom request; for bots only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendCustomRequest {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // sendCustomRequest
  /// The method name.
  method: Option<String>,
  /// JSON-serialized method parameters.
  parameters: Option<String>,
  
}



impl Object for SendCustomRequest {}
impl RObject for SendCustomRequest {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendCustomRequest" }
  fn td_type(&self) -> RTDType { RTDType::SendCustomRequest }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SendCustomRequest {}


impl SendCustomRequest {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "sendCustomRequest".to_string(),
      method: None,
      parameters: None,
      
    }
  }
  
  pub fn method(&self) -> Option<String> { self.method.clone() }
  #[doc(hidden)] pub fn _set_method(&mut self, method: String) -> &mut Self { self.method = Some(method); self }
  
  pub fn parameters(&self) -> Option<String> { self.parameters.clone() }
  #[doc(hidden)] pub fn _set_parameters(&mut self, parameters: String) -> &mut Self { self.parameters = Some(parameters); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



