
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Text that must be formatted as if inside a code HTML tag. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEntityTypeCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // textEntityTypeCode
  
}



impl Object for TextEntityTypeCode {}
impl RObject for TextEntityTypeCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypeCode" }
  fn td_type(&self) -> RTDType { RTDType::TextEntityTypeCode }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl TextEntityType for TextEntityTypeCode {}


impl TextEntityTypeCode {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "textEntityTypeCode".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



