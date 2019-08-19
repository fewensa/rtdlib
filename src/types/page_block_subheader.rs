
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A subheader. 
#[derive(Debug, Serialize, Deserialize)]
pub struct PageBlockSubheader {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockSubheader
  /// Subheader.
  subheader: Option<Box<RichText>>,
  
}


impl Clone for PageBlockSubheader {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for PageBlockSubheader {}
impl RObject for PageBlockSubheader {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockSubheader" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockSubheader }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlock for PageBlockSubheader {}


impl PageBlockSubheader {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockSubheader".to_string(),
      subheader: None,
      
    }
  }
  
  pub fn subheader(&self) -> Option<Box<RichText>> { self.subheader.clone() }
  #[doc(hidden)] pub fn _set_subheader(&mut self, subheader: Box<RichText>) -> &mut Self { self.subheader = Some(subheader); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



