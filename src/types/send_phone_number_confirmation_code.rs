
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Sends phone number confirmation code. Should be called when user presses "
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendPhoneNumberConfirmationCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // sendPhoneNumberConfirmationCode
  /// Value of the "hash" parameter from the link.
  hash: Option<String>,
  /// Value of the "phone" parameter from the link.
  phone_number: Option<String>,
  /// Pass true if the authentication code may be sent via flash call to the specified phone number.
  allow_flash_call: Option<bool>,
  /// Pass true if the phone number is used on the current device. Ignored if allow_flash_call is false.
  is_current_phone_number: Option<bool>,
  
}



impl Object for SendPhoneNumberConfirmationCode {}
impl RObject for SendPhoneNumberConfirmationCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendPhoneNumberConfirmationCode" }
  fn td_type(&self) -> RTDType { RTDType::SendPhoneNumberConfirmationCode }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SendPhoneNumberConfirmationCode {}


impl SendPhoneNumberConfirmationCode {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "sendPhoneNumberConfirmationCode".to_string(),
      hash: None,
      phone_number: None,
      allow_flash_call: None,
      is_current_phone_number: None,
      
    }
  }
  
  pub fn hash(&self) -> Option<String> { self.hash.clone() }
  #[doc(hidden)] pub fn _set_hash(&mut self, hash: String) -> &mut Self { self.hash = Some(hash); self }
  
  pub fn phone_number(&self) -> Option<String> { self.phone_number.clone() }
  #[doc(hidden)] pub fn _set_phone_number(&mut self, phone_number: String) -> &mut Self { self.phone_number = Some(phone_number); self }
  
  pub fn allow_flash_call(&self) -> Option<bool> { self.allow_flash_call.clone() }
  #[doc(hidden)] pub fn _set_allow_flash_call(&mut self, allow_flash_call: bool) -> &mut Self { self.allow_flash_call = Some(allow_flash_call); self }
  
  pub fn is_current_phone_number(&self) -> Option<bool> { self.is_current_phone_number.clone() }
  #[doc(hidden)] pub fn _set_is_current_phone_number(&mut self, is_current_phone_number: bool) -> &mut Self { self.is_current_phone_number = Some(is_current_phone_number); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



