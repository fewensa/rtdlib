
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Text that must be formatted as if inside pre, and code HTML tags. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEntityTypePreCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // textEntityTypePreCode
  /// Programming language of the code; as defined by the sender.
  language: Option<String>,
  
}



impl Object for TextEntityTypePreCode {}
impl RObject for TextEntityTypePreCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypePreCode" }
  fn td_type(&self) -> RTDType { RTDType::TextEntityTypePreCode }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl TextEntityType for TextEntityTypePreCode {}


impl TextEntityTypePreCode {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "textEntityTypePreCode".to_string(),
      language: None,
      
    }
  }
  
  pub fn language(&self) -> Option<String> { self.language.clone() }
  #[doc(hidden)] pub fn _set_language(&mut self, language: String) -> &mut Self { self.language = Some(language); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



