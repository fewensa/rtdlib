
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An object of this type can be returned on every function call, in case of an error. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // error
  /// Error code; subject to future changes. If the error code is 406, the error message must not be processed in any way and must not be displayed to the user.
  code: Option<i32>,
  /// Error message; subject to future changes.
  message: Option<String>,
  
}



impl Object for Error {}
impl RObject for Error {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "error" }
  fn td_type(&self) -> RTDType { RTDType::Error }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Error {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "error".to_string(),
      code: None,
      message: None,
      
    }
  }
  
  pub fn code(&self) -> Option<i32> { self.code.clone() }
  #[doc(hidden)] pub fn _set_code(&mut self, code: i32) -> &mut Self { self.code = Some(code); self }
  
  pub fn message(&self) -> Option<String> { self.message.clone() }
  #[doc(hidden)] pub fn _set_message(&mut self, message: String) -> &mut Self { self.message = Some(message); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



