
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An italicized rich text. 
#[derive(Debug, Serialize, Deserialize)]
pub struct RichTextItalic {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // richTextItalic
  /// Text.
  text: Option<Box<RichText>>,
  
}


impl Clone for RichTextItalic {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for RichTextItalic {}
impl RObject for RichTextItalic {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "richTextItalic" }
  fn td_type(&self) -> RTDType { RTDType::RichTextItalic }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl RichText for RichTextItalic {}


impl RichTextItalic {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "richTextItalic".to_string(),
      text: None,
      
    }
  }
  
  pub fn text(&self) -> Option<Box<RichText>> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: Box<RichText>) -> &mut Self { self.text = Some(text); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



