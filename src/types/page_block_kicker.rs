
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A kicker. 
#[derive(Debug, Serialize, Deserialize)]
pub struct PageBlockKicker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockKicker
  /// Kicker.
  kicker: Option<Box<RichText>>,
  
}


impl Clone for PageBlockKicker {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for PageBlockKicker {}
impl RObject for PageBlockKicker {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockKicker" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockKicker }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlock for PageBlockKicker {}


impl PageBlockKicker {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockKicker".to_string(),
      kicker: None,
      
    }
  }
  
  pub fn kicker(&self) -> Option<Box<RichText>> { self.kicker.clone() }
  #[doc(hidden)] pub fn _set_kicker(&mut self, kicker: Box<RichText>) -> &mut Self { self.kicker = Some(kicker); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



