
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A strike-through rich text. 
#[derive(Debug, Serialize, Deserialize)]
pub struct RichTextStrikethrough {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // richTextStrikethrough
  /// Text.
  text: Option<Box<RichText>>,
  
}


impl Clone for RichTextStrikethrough {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for RichTextStrikethrough {}
impl RObject for RichTextStrikethrough {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "richTextStrikethrough" }
  fn td_type(&self) -> RTDType { RTDType::RichTextStrikethrough }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl RichText for RichTextStrikethrough {}


impl RichTextStrikethrough {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "richTextStrikethrough".to_string(),
      text: None,
      
    }
  }
  
  pub fn text(&self) -> Option<Box<RichText>> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: Box<RichText>) -> &mut Self { self.text = Some(text); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



