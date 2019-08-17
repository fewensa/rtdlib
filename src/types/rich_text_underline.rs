
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An underlined rich text. 
#[derive(Debug, Serialize, Deserialize)]
pub struct RichTextUnderline {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // richTextUnderline
  /// Text.
  text: Option<Box<RichText>>,
  
}


impl Clone for RichTextUnderline {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for RichTextUnderline {}
impl RObject for RichTextUnderline {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "richTextUnderline" }
  fn td_type(&self) -> RTDType { RTDType::RichTextUnderline }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl RichText for RichTextUnderline {}


impl RichTextUnderline {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "richTextUnderline".to_string(),
      text: None,
      
    }
  }
  
  pub fn text(&self) -> Option<Box<RichText>> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: Box<RichText>) -> &mut Self { self.text = Some(text); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



