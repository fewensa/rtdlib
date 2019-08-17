
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Applies if a user enters new credentials on a payment provider website. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputCredentialsNew {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputCredentialsNew
  /// Contains JSON-encoded data with a credential identifier from the payment provider.
  data: Option<String>,
  /// True, if the credential identifier can be saved on the server side.
  allow_save: Option<bool>,
  
}



impl Object for InputCredentialsNew {}
impl RObject for InputCredentialsNew {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputCredentialsNew" }
  fn td_type(&self) -> RTDType { RTDType::InputCredentialsNew }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputCredentials for InputCredentialsNew {}


impl InputCredentialsNew {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputCredentialsNew".to_string(),
      data: None,
      allow_save: None,
      
    }
  }
  
  pub fn data(&self) -> Option<String> { self.data.clone() }
  #[doc(hidden)] pub fn _set_data(&mut self, data: String) -> &mut Self { self.data = Some(data); self }
  
  pub fn allow_save(&self) -> Option<bool> { self.allow_save.clone() }
  #[doc(hidden)] pub fn _set_allow_save(&mut self, allow_save: bool) -> &mut Self { self.allow_save = Some(allow_save); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



