
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Changes the phone number of the user and sends an authentication code to the user's new phone number. On success, returns information about the sent code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // changePhoneNumber
  /// The new phone number of the user in international format.
  phone_number: Option<String>,
  /// Pass true if the code can be sent via flash call to the specified phone number.
  allow_flash_call: Option<bool>,
  /// Pass true if the phone number is used on the current device. Ignored if allow_flash_call is false.
  is_current_phone_number: Option<bool>,
  
}



impl Object for ChangePhoneNumber {}
impl RObject for ChangePhoneNumber {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "changePhoneNumber" }
  fn td_type(&self) -> RTDType { RTDType::ChangePhoneNumber }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ChangePhoneNumber {}


impl ChangePhoneNumber {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "changePhoneNumber".to_string(),
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



