
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A rich text anchor. 
#[derive(Debug, Serialize, Deserialize)]
pub struct RichTextAnchor {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // richTextAnchor
  /// Text.
  text: Option<Box<RichText>>,
  /// Anchor name.
  name: Option<String>,
  
}


impl Clone for RichTextAnchor {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for RichTextAnchor {}
impl RObject for RichTextAnchor {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "richTextAnchor" }
  fn td_type(&self) -> RTDType { RTDType::RichTextAnchor }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl RichText for RichTextAnchor {}


impl RichTextAnchor {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "richTextAnchor".to_string(),
      text: None,
      name: None,
      
    }
  }
  
  pub fn text(&self) -> Option<Box<RichText>> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: Box<RichText>) -> &mut Self { self.text = Some(text); self }
  
  pub fn name(&self) -> Option<String> { self.name.clone() }
  #[doc(hidden)] pub fn _set_name(&mut self, name: String) -> &mut Self { self.name = Some(name); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



