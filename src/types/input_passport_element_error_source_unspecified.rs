
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The element contains an error in an unspecified place. The error will be considered resolved when new data is added. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPassportElementErrorSourceUnspecified {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputPassportElementErrorSourceUnspecified
  /// Current hash of the entire element.
  element_hash: Option<String>,
  
}



impl Object for InputPassportElementErrorSourceUnspecified {}
impl RObject for InputPassportElementErrorSourceUnspecified {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementErrorSourceUnspecified" }
  fn td_type(&self) -> RTDType { RTDType::InputPassportElementErrorSourceUnspecified }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputPassportElementErrorSource for InputPassportElementErrorSourceUnspecified {}


impl InputPassportElementErrorSourceUnspecified {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputPassportElementErrorSourceUnspecified".to_string(),
      element_hash: None,
      
    }
  }
  
  pub fn element_hash(&self) -> Option<String> { self.element_hash.clone() }
  #[doc(hidden)] pub fn _set_element_hash(&mut self, element_hash: String) -> &mut Self { self.element_hash = Some(element_hash); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



