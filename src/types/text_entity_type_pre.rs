
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Text that must be formatted as if inside a pre HTML tag. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEntityTypePre {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // textEntityTypePre
  
}



impl Object for TextEntityTypePre {}
impl RObject for TextEntityTypePre {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypePre" }
  fn td_type(&self) -> RTDType { RTDType::TextEntityTypePre }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl TextEntityType for TextEntityTypePre {}


impl TextEntityTypePre {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "textEntityTypePre".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



