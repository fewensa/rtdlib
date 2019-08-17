
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains the description of an error in a Telegram Passport element. 
#[derive(Debug, Serialize, Deserialize)]
pub struct PassportElementError {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementError
  /// Type of the Telegram Passport element which has the error.
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: Option<Box<PassportElementType>>,
  /// Error message.
  message: Option<String>,
  /// Error source.
  source: Option<Box<PassportElementErrorSource>>,
  
}


impl Clone for PassportElementError {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for PassportElementError {}
impl RObject for PassportElementError {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementError" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementError }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl PassportElementError {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementError".to_string(),
      type_: None,
      message: None,
      source: None,
      
    }
  }
  
  pub fn type_(&self) -> Option<Box<PassportElementType>> { self.type_.clone() }
  #[doc(hidden)] pub fn _set_type_(&mut self, type_: Box<PassportElementType>) -> &mut Self { self.type_ = Some(type_); self }
  
  pub fn message(&self) -> Option<String> { self.message.clone() }
  #[doc(hidden)] pub fn _set_message(&mut self, message: String) -> &mut Self { self.message = Some(message); self }
  
  pub fn source(&self) -> Option<Box<PassportElementErrorSource>> { self.source.clone() }
  #[doc(hidden)] pub fn _set_source(&mut self, source: Box<PassportElementErrorSource>) -> &mut Self { self.source = Some(source); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



