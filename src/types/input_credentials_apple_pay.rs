
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Applies if a user enters new credentials using Apple Pay. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputCredentialsApplePay {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputCredentialsApplePay
  /// JSON-encoded data with the credential identifier.
  data: Option<String>,
  
}



impl Object for InputCredentialsApplePay {}
impl RObject for InputCredentialsApplePay {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputCredentialsApplePay" }
  fn td_type(&self) -> RTDType { RTDType::InputCredentialsApplePay }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputCredentials for InputCredentialsApplePay {}


impl InputCredentialsApplePay {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputCredentialsApplePay".to_string(),
      data: None,
      
    }
  }
  
  pub fn data(&self) -> Option<String> { self.data.clone() }
  #[doc(hidden)] pub fn _set_data(&mut self, data: String) -> &mut Self { self.data = Some(data); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



