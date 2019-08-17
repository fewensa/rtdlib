
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Describes the photo of a chat. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatPhoto
  /// A small (160x160) chat photo.
  small: Option<File>,
  /// A big (640x640) chat photo.
  big: Option<File>,
  
}



impl Object for ChatPhoto {}
impl RObject for ChatPhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatPhoto" }
  fn td_type(&self) -> RTDType { RTDType::ChatPhoto }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl ChatPhoto {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatPhoto".to_string(),
      small: None,
      big: None,
      
    }
  }
  
  pub fn small(&self) -> Option<File> { self.small.clone() }
  #[doc(hidden)] pub fn _set_small(&mut self, small: File) -> &mut Self { self.small = Some(small); self }
  
  pub fn big(&self) -> Option<File> { self.big.clone() }
  #[doc(hidden)] pub fn _set_big(&mut self, big: File) -> &mut Self { self.big = Some(big); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



