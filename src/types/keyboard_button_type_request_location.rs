
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A button that sends the user's location when pressed; available only in private chats. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardButtonTypeRequestLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // keyboardButtonTypeRequestLocation
  
}



impl Object for KeyboardButtonTypeRequestLocation {}
impl RObject for KeyboardButtonTypeRequestLocation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "keyboardButtonTypeRequestLocation" }
  fn td_type(&self) -> RTDType { RTDType::KeyboardButtonTypeRequestLocation }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl KeyboardButtonType for KeyboardButtonTypeRequestLocation {}


impl KeyboardButtonTypeRequestLocation {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "keyboardButtonTypeRequestLocation".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



