
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains a caption of an instant view web page block, consisting of a text and a trailing credit. 
#[derive(Debug, Serialize, Deserialize)]
pub struct PageBlockCaption {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockCaption
  /// Content of the caption.
  text: Option<Box<RichText>>,
  /// Block credit (like HTML tag <cite>).
  credit: Option<Box<RichText>>,
  
}


impl Clone for PageBlockCaption {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for PageBlockCaption {}
impl RObject for PageBlockCaption {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockCaption" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockCaption }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl PageBlockCaption {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockCaption".to_string(),
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



