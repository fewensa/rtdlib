
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A text description shown instead of a raw URL. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEntityTypeTextUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // textEntityTypeTextUrl
  /// HTTP or tg:// URL to be opened when the link is clicked.
  url: Option<String>,
  
}



impl Object for TextEntityTypeTextUrl {}
impl RObject for TextEntityTypeTextUrl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypeTextUrl" }
  fn td_type(&self) -> RTDType { RTDType::TextEntityTypeTextUrl }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl TextEntityType for TextEntityTypeTextUrl {}


impl TextEntityTypeTextUrl {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "textEntityTypeTextUrl".to_string(),
      url: None,
      
    }
  }
  
  pub fn url(&self) -> Option<String> { self.url.clone() }
  #[doc(hidden)] pub fn _set_url(&mut self, url: String) -> &mut Self { self.url = Some(url); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



