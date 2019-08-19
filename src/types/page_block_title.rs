
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The title of a page. 
#[derive(Debug, Serialize, Deserialize)]
pub struct PageBlockTitle {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockTitle
  /// Title.
  title: Option<Box<RichText>>,
  
}


impl Clone for PageBlockTitle {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for PageBlockTitle {}
impl RObject for PageBlockTitle {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockTitle" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockTitle }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlock for PageBlockTitle {}


impl PageBlockTitle {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockTitle".to_string(),
      title: None,
      
    }
  }
  
  pub fn title(&self) -> Option<Box<RichText>> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: Box<RichText>) -> &mut Self { self.title = Some(title); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



