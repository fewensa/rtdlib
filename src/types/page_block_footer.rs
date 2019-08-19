
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The footer of a page. 
#[derive(Debug, Serialize, Deserialize)]
pub struct PageBlockFooter {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockFooter
  /// Footer.
  footer: Option<Box<RichText>>,
  
}


impl Clone for PageBlockFooter {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for PageBlockFooter {}
impl RObject for PageBlockFooter {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockFooter" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockFooter }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlock for PageBlockFooter {}


impl PageBlockFooter {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockFooter".to_string(),
      footer: None,
      
    }
  }
  
  pub fn footer(&self) -> Option<Box<RichText>> { self.footer.clone() }
  #[doc(hidden)] pub fn _set_footer(&mut self, footer: Box<RichText>) -> &mut Self { self.footer = Some(footer); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



