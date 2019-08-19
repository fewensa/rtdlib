
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Checks the authentication password for correctness. Works only when the current authorization state is 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckAuthenticationPassword {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // checkAuthenticationPassword
  /// The password to check.
  password: Option<String>,
  
}



impl Object for CheckAuthenticationPassword {}
impl RObject for CheckAuthenticationPassword {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkAuthenticationPassword" }
  fn td_type(&self) -> RTDType { RTDType::CheckAuthenticationPassword }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for CheckAuthenticationPassword {}


impl CheckAuthenticationPassword {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "checkAuthenticationPassword".to_string(),
      password: None,
      
    }
  }
  
  pub fn password(&self) -> Option<String> { self.password.clone() }
  #[doc(hidden)] pub fn _set_password(&mut self, password: String) -> &mut Self { self.password = Some(password); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



