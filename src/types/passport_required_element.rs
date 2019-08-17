
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains a description of the required Telegram Passport element that was requested by a service. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportRequiredElement {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportRequiredElement
  /// List of Telegram Passport elements any of which is enough to provide.
  suitable_elements: Option<Vec<PassportSuitableElement>>,
  
}



impl Object for PassportRequiredElement {}
impl RObject for PassportRequiredElement {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportRequiredElement" }
  fn td_type(&self) -> RTDType { RTDType::PassportRequiredElement }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl PassportRequiredElement {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportRequiredElement".to_string(),
      suitable_elements: None,
      
    }
  }
  
  pub fn suitable_elements(&self) -> Option<Vec<PassportSuitableElement>> { self.suitable_elements.clone() }
  #[doc(hidden)] pub fn _set_suitable_elements(&mut self, suitable_elements: Vec<PassportSuitableElement>) -> &mut Self { self.suitable_elements = Some(suitable_elements); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



