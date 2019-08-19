
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An italic text. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEntityTypeItalic {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // textEntityTypeItalic
  
}



impl Object for TextEntityTypeItalic {}
impl RObject for TextEntityTypeItalic {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypeItalic" }
  fn td_type(&self) -> RTDType { RTDType::TextEntityTypeItalic }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl TextEntityType for TextEntityTypeItalic {}


impl TextEntityTypeItalic {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "textEntityTypeItalic".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



