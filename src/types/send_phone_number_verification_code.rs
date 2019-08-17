
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Sends a code to verify a phone number to be added to a user's Telegram Passport.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendPhoneNumberVerificationCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // sendPhoneNumberVerificationCode
  /// The phone number of the user, in international format.
  phone_number: Option<String>,
  /// Pass true if the authentication code may be sent via flash call to the specified phone number.
  allow_flash_call: Option<bool>,
  /// Pass true if the phone number is used on the current device. Ignored if allow_flash_call is false.
  is_current_phone_number: Option<bool>,
  
}



impl Object for SendPhoneNumberVerificationCode {}
impl RObject for SendPhoneNumberVerificationCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendPhoneNumberVerificationCode" }
  fn td_type(&self) -> RTDType { RTDType::SendPhoneNumberVerificationCode }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SendPhoneNumberVerificationCode {}


impl SendPhoneNumberVerificationCode {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "sendPhoneNumberVerificationCode".to_string(),
      phone_number: None,
      allow_flash_call: None,
      is_current_phone_number: None,
      
    }
  }
  
  pub fn phone_number(&self) -> Option<String> { self.phone_number.clone() }
  #[doc(hidden)] pub fn _set_phone_number(&mut self, phone_number: String) -> &mut Self { self.phone_number = Some(phone_number); self }
  
  pub fn allow_flash_call(&self) -> Option<bool> { self.allow_flash_call.clone() }
  #[doc(hidden)] pub fn _set_allow_flash_call(&mut self, allow_flash_call: bool) -> &mut Self { self.allow_flash_call = Some(allow_flash_call); self }
  
  pub fn is_current_phone_number(&self) -> Option<bool> { self.is_current_phone_number.clone() }
  #[doc(hidden)] pub fn _set_is_current_phone_number(&mut self, is_current_phone_number: bool) -> &mut Self { self.is_current_phone_number = Some(is_current_phone_number); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



