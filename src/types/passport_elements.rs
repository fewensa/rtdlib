
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about saved Telegram Passport elements. 
#[derive(Debug, Serialize, Deserialize)]
pub struct PassportElements {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElements
  /// Telegram Passport elements.
  elements: Option<Vec<Box<PassportElement>>>,
  
}


impl Clone for PassportElements {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for PassportElements {}
impl RObject for PassportElements {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElements" }
  fn td_type(&self) -> RTDType { RTDType::PassportElements }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl PassportElements {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElements".to_string(),
      elements: None,
      
    }
  }
  
  pub fn elements(&self) -> Option<Vec<Box<PassportElement>>> { self.elements.clone() }
  #[doc(hidden)] pub fn _set_elements(&mut self, elements: Vec<Box<PassportElement>>) -> &mut Self { self.elements = Some(elements); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



