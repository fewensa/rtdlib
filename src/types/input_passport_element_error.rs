
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains the description of an error in a Telegram Passport element; for bots only. 
#[derive(Debug, Serialize, Deserialize)]
pub struct InputPassportElementError {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputPassportElementError
  /// Type of Telegram Passport element that has the error.
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: Option<Box<PassportElementType>>,
  /// Error message.
  message: Option<String>,
  /// Error source.
  source: Option<Box<InputPassportElementErrorSource>>,
  
}


impl Clone for InputPassportElementError {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for InputPassportElementError {}
impl RObject for InputPassportElementError {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementError" }
  fn td_type(&self) -> RTDType { RTDType::InputPassportElementError }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl InputPassportElementError {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputPassportElementError".to_string(),
      type_: None,
      message: None,
      source: None,
      
    }
  }
  
  pub fn type_(&self) -> Option<Box<PassportElementType>> { self.type_.clone() }
  #[doc(hidden)] pub fn _set_type_(&mut self, type_: Box<PassportElementType>) -> &mut Self { self.type_ = Some(type_); self }
  
  pub fn message(&self) -> Option<String> { self.message.clone() }
  #[doc(hidden)] pub fn _set_message(&mut self, message: String) -> &mut Self { self.message = Some(message); self }
  
  pub fn source(&self) -> Option<Box<InputPassportElementErrorSource>> { self.source.clone() }
  #[doc(hidden)] pub fn _set_source(&mut self, source: Box<InputPassportElementErrorSource>) -> &mut Self { self.source = Some(source); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



