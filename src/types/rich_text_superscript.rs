
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A superscript rich text. 
#[derive(Debug, Serialize, Deserialize)]
pub struct RichTextSuperscript {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // richTextSuperscript
  /// Text.
  text: Option<Box<RichText>>,
  
}


impl Clone for RichTextSuperscript {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for RichTextSuperscript {}
impl RObject for RichTextSuperscript {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "richTextSuperscript" }
  fn td_type(&self) -> RTDType { RTDType::RichTextSuperscript }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl RichText for RichTextSuperscript {}


impl RichTextSuperscript {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "richTextSuperscript".to_string(),
      text: None,
      
    }
  }
  
  pub fn text(&self) -> Option<Box<RichText>> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: Box<RichText>) -> &mut Self { self.text = Some(text); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



