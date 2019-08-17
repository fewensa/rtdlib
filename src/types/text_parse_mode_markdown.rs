
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The text should be parsed in markdown-style. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextParseModeMarkdown {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // textParseModeMarkdown
  
}



impl Object for TextParseModeMarkdown {}
impl RObject for TextParseModeMarkdown {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textParseModeMarkdown" }
  fn td_type(&self) -> RTDType { RTDType::TextParseModeMarkdown }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl TextParseMode for TextParseModeMarkdown {}


impl TextParseModeMarkdown {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "textParseModeMarkdown".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



