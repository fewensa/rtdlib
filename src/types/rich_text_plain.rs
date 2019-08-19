
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A plain text. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextPlain {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // richTextPlain
  /// Text.
  text: Option<String>,
  
}



impl Object for RichTextPlain {}
impl RObject for RichTextPlain {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "richTextPlain" }
  fn td_type(&self) -> RTDType { RTDType::RichTextPlain }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl RichText for RichTextPlain {}


impl RichTextPlain {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "richTextPlain".to_string(),
      text: None,
      
    }
  }
  
  pub fn text(&self) -> Option<String> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: String) -> &mut Self { self.text = Some(text); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



