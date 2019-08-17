
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A bold text. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEntityTypeBold {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // textEntityTypeBold
  
}



impl Object for TextEntityTypeBold {}
impl RObject for TextEntityTypeBold {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypeBold" }
  fn td_type(&self) -> RTDType { RTDType::TextEntityTypeBold }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl TextEntityType for TextEntityTypeBold {}


impl TextEntityTypeBold {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "textEntityTypeBold".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



