
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Checks phone number confirmation code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckPhoneNumberConfirmationCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // checkPhoneNumberConfirmationCode
  /// The phone number confirmation code.
  code: Option<String>,
  
}



impl Object for CheckPhoneNumberConfirmationCode {}
impl RObject for CheckPhoneNumberConfirmationCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkPhoneNumberConfirmationCode" }
  fn td_type(&self) -> RTDType { RTDType::CheckPhoneNumberConfirmationCode }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for CheckPhoneNumberConfirmationCode {}


impl CheckPhoneNumberConfirmationCode {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "checkPhoneNumberConfirmationCode".to_string(),
      code: None,
      
    }
  }
  
  pub fn code(&self) -> Option<String> { self.code.clone() }
  #[doc(hidden)] pub fn _set_code(&mut self, code: String) -> &mut Self { self.code = Some(code); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



