
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A hashtag text, beginning with "#". 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEntityTypeHashtag {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // textEntityTypeHashtag
  
}



impl Object for TextEntityTypeHashtag {}
impl RObject for TextEntityTypeHashtag {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypeHashtag" }
  fn td_type(&self) -> RTDType { RTDType::TextEntityTypeHashtag }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl TextEntityType for TextEntityTypeHashtag {}


impl TextEntityTypeHashtag {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "textEntityTypeHashtag".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



