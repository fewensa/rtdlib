
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Creates a new temporary password for processing payments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTemporaryPassword {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // createTemporaryPassword
  /// Persistent user password.
  password: Option<String>,
  /// Time during which the temporary password will be valid, in seconds; should be between 60 and 86400.
  valid_for: Option<i32>,
  
}



impl Object for CreateTemporaryPassword {}
impl RObject for CreateTemporaryPassword {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "createTemporaryPassword" }
  fn td_type(&self) -> RTDType { RTDType::CreateTemporaryPassword }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for CreateTemporaryPassword {}


impl CreateTemporaryPassword {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "createTemporaryPassword".to_string(),
      password: None,
      valid_for: None,
      
    }
  }
  
  pub fn password(&self) -> Option<String> { self.password.clone() }
  #[doc(hidden)] pub fn _set_password(&mut self, password: String) -> &mut Self { self.password = Some(password); self }
  
  pub fn valid_for(&self) -> Option<i32> { self.valid_for.clone() }
  #[doc(hidden)] pub fn _set_valid_for(&mut self, valid_for: i32) -> &mut Self { self.valid_for = Some(valid_for); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



