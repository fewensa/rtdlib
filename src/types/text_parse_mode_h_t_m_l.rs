
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The text should be parsed in HTML-style. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextParseModeHTML {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // textParseModeHTML
  
}



impl Object for TextParseModeHTML {}
impl RObject for TextParseModeHTML {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textParseModeHTML" }
  fn td_type(&self) -> RTDType { RTDType::TextParseModeHTML }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl TextParseMode for TextParseModeHTML {}


impl TextParseModeHTML {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "textParseModeHTML".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



