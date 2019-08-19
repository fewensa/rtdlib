
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about a Telegram Passport elements and corresponding errors. 
#[derive(Debug, Serialize, Deserialize)]
pub struct PassportElementsWithErrors {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementsWithErrors
  /// Telegram Passport elements.
  elements: Option<Vec<Box<PassportElement>>>,
  /// Errors in the elements that are already available.
  errors: Option<Vec<PassportElementError>>,
  
}


impl Clone for PassportElementsWithErrors {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for PassportElementsWithErrors {}
impl RObject for PassportElementsWithErrors {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementsWithErrors" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementsWithErrors }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl PassportElementsWithErrors {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementsWithErrors".to_string(),
      elements: None,
      errors: None,
      
    }
  }
  
  pub fn elements(&self) -> Option<Vec<Box<PassportElement>>> { self.elements.clone() }
  #[doc(hidden)] pub fn _set_elements(&mut self, elements: Vec<Box<PassportElement>>) -> &mut Self { self.elements = Some(elements); self }
  
  pub fn errors(&self) -> Option<Vec<PassportElementError>> { self.errors.clone() }
  #[doc(hidden)] pub fn _set_errors(&mut self, errors: Vec<PassportElementError>) -> &mut Self { self.errors = Some(errors); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



