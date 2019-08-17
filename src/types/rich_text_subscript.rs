
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A subscript rich text. 
#[derive(Debug, Serialize, Deserialize)]
pub struct RichTextSubscript {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // richTextSubscript
  /// Text.
  text: Option<Box<RichText>>,
  
}


impl Clone for RichTextSubscript {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for RichTextSubscript {}
impl RObject for RichTextSubscript {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "richTextSubscript" }
  fn td_type(&self) -> RTDType { RTDType::RichTextSubscript }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl RichText for RichTextSubscript {}


impl RichTextSubscript {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "richTextSubscript".to_string(),
      text: None,
      
    }
  }
  
  pub fn text(&self) -> Option<Box<RichText>> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: Box<RichText>) -> &mut Self { self.text = Some(text); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



