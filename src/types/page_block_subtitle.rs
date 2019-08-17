
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The subtitle of a page. 
#[derive(Debug, Serialize, Deserialize)]
pub struct PageBlockSubtitle {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockSubtitle
  /// Subtitle.
  subtitle: Option<Box<RichText>>,
  
}


impl Clone for PageBlockSubtitle {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for PageBlockSubtitle {}
impl RObject for PageBlockSubtitle {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockSubtitle" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockSubtitle }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlock for PageBlockSubtitle {}


impl PageBlockSubtitle {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockSubtitle".to_string(),
      subtitle: None,
      
    }
  }
  
  pub fn subtitle(&self) -> Option<Box<RichText>> { self.subtitle.clone() }
  #[doc(hidden)] pub fn _set_subtitle(&mut self, subtitle: Box<RichText>) -> &mut Self { self.subtitle = Some(subtitle); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



