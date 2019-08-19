
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Adds an element to the user's Telegram Passport. May return an error with a message "PHONE_VERIFICATION_NEEDED" or "EMAIL_VERIFICATION_NEEDED" if the chosen phone number or the chosen email address must be verified first.
#[derive(Debug, Serialize, Deserialize)]
pub struct SetPassportElement {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setPassportElement
  /// Input Telegram Passport element.
  element: Option<Box<InputPassportElement>>,
  /// Password of the current user.
  password: Option<String>,
  
}


impl Clone for SetPassportElement {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for SetPassportElement {}
impl RObject for SetPassportElement {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setPassportElement" }
  fn td_type(&self) -> RTDType { RTDType::SetPassportElement }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetPassportElement {}


impl SetPassportElement {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setPassportElement".to_string(),
      element: None,
      password: None,
      
    }
  }
  
  pub fn element(&self) -> Option<Box<InputPassportElement>> { self.element.clone() }
  #[doc(hidden)] pub fn _set_element(&mut self, element: Box<InputPassportElement>) -> &mut Self { self.element = Some(element); self }
  
  pub fn password(&self) -> Option<String> { self.password.clone() }
  #[doc(hidden)] pub fn _set_password(&mut self, password: String) -> &mut Self { self.password = Some(password); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



