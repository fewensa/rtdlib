
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A button that sends the user's phone number when pressed; available only in private chats. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardButtonTypeRequestPhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // keyboardButtonTypeRequestPhoneNumber
  
}



impl Object for KeyboardButtonTypeRequestPhoneNumber {}
impl RObject for KeyboardButtonTypeRequestPhoneNumber {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "keyboardButtonTypeRequestPhoneNumber" }
  fn td_type(&self) -> RTDType { RTDType::KeyboardButtonTypeRequestPhoneNumber }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl KeyboardButtonType for KeyboardButtonTypeRequestPhoneNumber {}


impl KeyboardButtonTypeRequestPhoneNumber {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "keyboardButtonTypeRequestPhoneNumber".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



