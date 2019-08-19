
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An HTTP URL. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEntityTypeUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // textEntityTypeUrl
  
}



impl Object for TextEntityTypeUrl {}
impl RObject for TextEntityTypeUrl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypeUrl" }
  fn td_type(&self) -> RTDType { RTDType::TextEntityTypeUrl }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl TextEntityType for TextEntityTypeUrl {}


impl TextEntityTypeUrl {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "textEntityTypeUrl".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



