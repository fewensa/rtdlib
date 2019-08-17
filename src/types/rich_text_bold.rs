
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A bold rich text. 
#[derive(Debug, Serialize, Deserialize)]
pub struct RichTextBold {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // richTextBold
  /// Text.
  text: Option<Box<RichText>>,
  
}


impl Clone for RichTextBold {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for RichTextBold {}
impl RObject for RichTextBold {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "richTextBold" }
  fn td_type(&self) -> RTDType { RTDType::RichTextBold }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl RichText for RichTextBold {}


impl RichTextBold {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "richTextBold".to_string(),
      text: None,
      
    }
  }
  
  pub fn text(&self) -> Option<Box<RichText>> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: Box<RichText>) -> &mut Self { self.text = Some(text); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



