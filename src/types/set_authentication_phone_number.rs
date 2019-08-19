
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Sets the phone number of the user and sends an authentication code to the user. Works only when the current authorization state is 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetAuthenticationPhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setAuthenticationPhoneNumber
  /// The phone number of the user, in international format.
  phone_number: Option<String>,
  /// Pass true if the authentication code may be sent via flash call to the specified phone number.
  allow_flash_call: Option<bool>,
  /// Pass true if the phone number is used on the current device. Ignored if allow_flash_call is false.
  is_current_phone_number: Option<bool>,
  
}



impl Object for SetAuthenticationPhoneNumber {}
impl RObject for SetAuthenticationPhoneNumber {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setAuthenticationPhoneNumber" }
  fn td_type(&self) -> RTDType { RTDType::SetAuthenticationPhoneNumber }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetAuthenticationPhoneNumber {}


impl SetAuthenticationPhoneNumber {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setAuthenticationPhoneNumber".to_string(),
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



