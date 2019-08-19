
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An email address. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEntityTypeEmailAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // textEntityTypeEmailAddress
  
}



impl Object for TextEntityTypeEmailAddress {}
impl RObject for TextEntityTypeEmailAddress {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypeEmailAddress" }
  fn td_type(&self) -> RTDType { RTDType::TextEntityTypeEmailAddress }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl TextEntityType for TextEntityTypeEmailAddress {}


impl TextEntityTypeEmailAddress {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "textEntityTypeEmailAddress".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



