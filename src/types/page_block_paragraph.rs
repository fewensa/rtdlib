
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A text paragraph. 
#[derive(Debug, Serialize, Deserialize)]
pub struct PageBlockParagraph {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockParagraph
  /// Paragraph text.
  text: Option<Box<RichText>>,
  
}


impl Clone for PageBlockParagraph {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for PageBlockParagraph {}
impl RObject for PageBlockParagraph {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockParagraph" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockParagraph }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlock for PageBlockParagraph {}


impl PageBlockParagraph {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockParagraph".to_string(),
      text: None,
      
    }
  }
  
  pub fn text(&self) -> Option<Box<RichText>> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: Box<RichText>) -> &mut Self { self.text = Some(text); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



