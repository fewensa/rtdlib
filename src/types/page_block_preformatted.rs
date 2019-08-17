
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A preformatted text paragraph. 
#[derive(Debug, Serialize, Deserialize)]
pub struct PageBlockPreformatted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockPreformatted
  /// Paragraph text.
  text: Option<Box<RichText>>,
  /// Programming language for which the text should be formatted.
  language: Option<String>,
  
}


impl Clone for PageBlockPreformatted {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for PageBlockPreformatted {}
impl RObject for PageBlockPreformatted {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockPreformatted" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockPreformatted }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlock for PageBlockPreformatted {}


impl PageBlockPreformatted {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockPreformatted".to_string(),
      text: None,
      language: None,
      
    }
  }
  
  pub fn text(&self) -> Option<Box<RichText>> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: Box<RichText>) -> &mut Self { self.text = Some(text); self }
  
  pub fn language(&self) -> Option<String> { self.language.clone() }
  #[doc(hidden)] pub fn _set_language(&mut self, language: String) -> &mut Self { self.language = Some(language); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



