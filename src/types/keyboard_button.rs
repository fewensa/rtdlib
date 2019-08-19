
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a single button in a bot keyboard. 
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyboardButton {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // keyboardButton
  /// Text of the button.
  text: Option<String>,
  /// Type of the button.
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: Option<Box<KeyboardButtonType>>,
  
}


impl Clone for KeyboardButton {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for KeyboardButton {}
impl RObject for KeyboardButton {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "keyboardButton" }
  fn td_type(&self) -> RTDType { RTDType::KeyboardButton }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl KeyboardButton {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "keyboardButton".to_string(),
      text: None,
      type_: None,
      
    }
  }
  
  pub fn text(&self) -> Option<String> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: String) -> &mut Self { self.text = Some(text); self }
  
  pub fn type_(&self) -> Option<Box<KeyboardButtonType>> { self.type_.clone() }
  #[doc(hidden)] pub fn _set_type_(&mut self, type_: Box<KeyboardButtonType>) -> &mut Self { self.type_ = Some(type_); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



