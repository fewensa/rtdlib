
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A cashtag text, beginning with "$" and consisting of capital english letters (i.e. "$USD"). 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEntityTypeCashtag {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // textEntityTypeCashtag
  
}



impl Object for TextEntityTypeCashtag {}
impl RObject for TextEntityTypeCashtag {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypeCashtag" }
  fn td_type(&self) -> RTDType { RTDType::TextEntityTypeCashtag }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl TextEntityType for TextEntityTypeCashtag {}


impl TextEntityTypeCashtag {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "textEntityTypeCashtag".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



