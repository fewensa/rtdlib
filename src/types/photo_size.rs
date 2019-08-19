
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Photo description. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoSize {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // photoSize
  /// Thumbnail type (see https://core.telegram.org/constructor/photoSize).
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: Option<String>,
  /// Information about the photo file.
  photo: Option<File>,
  /// Photo width.
  width: Option<i32>,
  /// Photo height.
  height: Option<i32>,
  
}



impl Object for PhotoSize {}
impl RObject for PhotoSize {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "photoSize" }
  fn td_type(&self) -> RTDType { RTDType::PhotoSize }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl PhotoSize {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "photoSize".to_string(),
      type_: None,
      photo: None,
      width: None,
      height: None,
      
    }
  }
  
  pub fn type_(&self) -> Option<String> { self.type_.clone() }
  #[doc(hidden)] pub fn _set_type_(&mut self, type_: String) -> &mut Self { self.type_ = Some(type_); self }
  
  pub fn photo(&self) -> Option<File> { self.photo.clone() }
  #[doc(hidden)] pub fn _set_photo(&mut self, photo: File) -> &mut Self { self.photo = Some(photo); self }
  
  pub fn width(&self) -> Option<i32> { self.width.clone() }
  #[doc(hidden)] pub fn _set_width(&mut self, width: i32) -> &mut Self { self.width = Some(width); self }
  
  pub fn height(&self) -> Option<i32> { self.height.clone() }
  #[doc(hidden)] pub fn _set_height(&mut self, height: i32) -> &mut Self { self.height = Some(height); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



