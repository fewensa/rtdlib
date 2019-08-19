
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A simple button, with text that should be sent when the button is pressed. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardButtonTypeText {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // keyboardButtonTypeText
  
}



impl Object for KeyboardButtonTypeText {}
impl RObject for KeyboardButtonTypeText {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "keyboardButtonTypeText" }
  fn td_type(&self) -> RTDType { RTDType::KeyboardButtonTypeText }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl KeyboardButtonType for KeyboardButtonTypeText {}


impl KeyboardButtonTypeText {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "keyboardButtonTypeText".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



