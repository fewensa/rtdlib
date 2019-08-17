
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns a 2-step verification recovery email address that was previously set up. This method can be used to verify a password provided by the user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRecoveryEmailAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getRecoveryEmailAddress
  /// The password for the current user.
  password: Option<String>,
  
}



impl Object for GetRecoveryEmailAddress {}
impl RObject for GetRecoveryEmailAddress {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getRecoveryEmailAddress" }
  fn td_type(&self) -> RTDType { RTDType::GetRecoveryEmailAddress }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetRecoveryEmailAddress {}


impl GetRecoveryEmailAddress {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getRecoveryEmailAddress".to_string(),
      password: None,
      
    }
  }
  
  pub fn password(&self) -> Option<String> { self.password.clone() }
  #[doc(hidden)] pub fn _set_password(&mut self, password: String) -> &mut Self { self.password = Some(password); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



