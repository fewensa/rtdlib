
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A block quote. 
#[derive(Debug, Serialize, Deserialize)]
pub struct PageBlockBlockQuote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockBlockQuote
  /// Quote text.
  text: Option<Box<RichText>>,
  /// Quote credit.
  credit: Option<Box<RichText>>,
  
}


impl Clone for PageBlockBlockQuote {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for PageBlockBlockQuote {}
impl RObject for PageBlockBlockQuote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockBlockQuote" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockBlockQuote }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlock for PageBlockBlockQuote {}


impl PageBlockBlockQuote {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockBlockQuote".to_string(),
      text: None,
      credit: None,
      
    }
  }
  
  pub fn text(&self) -> Option<Box<RichText>> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: Box<RichText>) -> &mut Self { self.text = Some(text); self }
  
  pub fn credit(&self) -> Option<Box<RichText>> { self.credit.clone() }
  #[doc(hidden)] pub fn _set_credit(&mut self, credit: Box<RichText>) -> &mut Self { self.credit = Some(credit); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



