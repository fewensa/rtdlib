
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Resends phone number confirmation code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResendPhoneNumberConfirmationCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // resendPhoneNumberConfirmationCode
  
}



impl Object for ResendPhoneNumberConfirmationCode {}
impl RObject for ResendPhoneNumberConfirmationCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "resendPhoneNumberConfirmationCode" }
  fn td_type(&self) -> RTDType { RTDType::ResendPhoneNumberConfirmationCode }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ResendPhoneNumberConfirmationCode {}


impl ResendPhoneNumberConfirmationCode {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "resendPhoneNumberConfirmationCode".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



