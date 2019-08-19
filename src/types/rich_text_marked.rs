
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A marked rich text. 
#[derive(Debug, Serialize, Deserialize)]
pub struct RichTextMarked {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // richTextMarked
  /// Text.
  text: Option<Box<RichText>>,
  
}


impl Clone for RichTextMarked {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for RichTextMarked {}
impl RObject for RichTextMarked {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "richTextMarked" }
  fn td_type(&self) -> RTDType { RTDType::RichTextMarked }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl RichText for RichTextMarked {}


impl RichTextMarked {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "richTextMarked".to_string(),
      text: None,
      
    }
  }
  
  pub fn text(&self) -> Option<Box<RichText>> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: Box<RichText>) -> &mut Self { self.text = Some(text); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



