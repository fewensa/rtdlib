
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A mention of a user by their username. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEntityTypeMention {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // textEntityTypeMention
  
}



impl Object for TextEntityTypeMention {}
impl RObject for TextEntityTypeMention {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypeMention" }
  fn td_type(&self) -> RTDType { RTDType::TextEntityTypeMention }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl TextEntityType for TextEntityTypeMention {}


impl TextEntityTypeMention {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "textEntityTypeMention".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



