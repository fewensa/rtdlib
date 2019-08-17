
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Describes a photo. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Photo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // photo
  /// True, if stickers were added to the photo.
  has_stickers: Option<bool>,
  /// Available variants of the photo, in different sizes.
  sizes: Option<Vec<PhotoSize>>,
  
}



impl Object for Photo {}
impl RObject for Photo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "photo" }
  fn td_type(&self) -> RTDType { RTDType::Photo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Photo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "photo".to_string(),
      has_stickers: None,
      sizes: None,
      
    }
  }
  
  pub fn has_stickers(&self) -> Option<bool> { self.has_stickers.clone() }
  #[doc(hidden)] pub fn _set_has_stickers(&mut self, has_stickers: bool) -> &mut Self { self.has_stickers = Some(has_stickers); self }
  
  pub fn sizes(&self) -> Option<Vec<PhotoSize>> { self.sizes.clone() }
  #[doc(hidden)] pub fn _set_sizes(&mut self, sizes: Vec<PhotoSize>) -> &mut Self { self.sizes = Some(sizes); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



