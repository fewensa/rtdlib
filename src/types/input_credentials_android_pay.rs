
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Applies if a user enters new credentials using Android Pay. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputCredentialsAndroidPay {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputCredentialsAndroidPay
  /// JSON-encoded data with the credential identifier.
  data: Option<String>,
  
}



impl Object for InputCredentialsAndroidPay {}
impl RObject for InputCredentialsAndroidPay {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputCredentialsAndroidPay" }
  fn td_type(&self) -> RTDType { RTDType::InputCredentialsAndroidPay }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputCredentials for InputCredentialsAndroidPay {}


impl InputCredentialsAndroidPay {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputCredentialsAndroidPay".to_string(),
      data: None,
      
    }
  }
  
  pub fn data(&self) -> Option<String> { self.data.clone() }
  #[doc(hidden)] pub fn _set_data(&mut self, data: String) -> &mut Self { self.data = Some(data); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



