
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A concatenation of rich texts. 
#[derive(Debug, Serialize, Deserialize)]
pub struct RichTexts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // richTexts
  /// Texts.
  texts: Option<Vec<Box<RichText>>>,
  
}


impl Clone for RichTexts {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for RichTexts {}
impl RObject for RichTexts {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "richTexts" }
  fn td_type(&self) -> RTDType { RTDType::RichTexts }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl RichText for RichTexts {}


impl RichTexts {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "richTexts".to_string(),
      texts: None,
      
    }
  }
  
  pub fn texts(&self) -> Option<Vec<Box<RichText>>> { self.texts.clone() }
  #[doc(hidden)] pub fn _set_texts(&mut self, texts: Vec<Box<RichText>>) -> &mut Self { self.texts = Some(texts); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



