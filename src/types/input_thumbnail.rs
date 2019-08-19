
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A thumbnail to be sent along with a file; should be in JPEG or WEBP format for stickers, and less than 200 kB in size. 
#[derive(Debug, Serialize, Deserialize)]
pub struct InputThumbnail {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputThumbnail
  /// Thumbnail file to send. Sending thumbnails by file_id is currently not supported.
  thumbnail: Option<Box<InputFile>>,
  /// Thumbnail width, usually shouldn't exceed 320. Use 0 if unknown.
  width: Option<i32>,
  /// Thumbnail height, usually shouldn't exceed 320. Use 0 if unknown.
  height: Option<i32>,
  
}


impl Clone for InputThumbnail {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for InputThumbnail {}
impl RObject for InputThumbnail {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputThumbnail" }
  fn td_type(&self) -> RTDType { RTDType::InputThumbnail }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl InputThumbnail {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputThumbnail".to_string(),
      thumbnail: None,
      width: None,
      height: None,
      
    }
  }
  
  pub fn thumbnail(&self) -> Option<Box<InputFile>> { self.thumbnail.clone() }
  #[doc(hidden)] pub fn _set_thumbnail(&mut self, thumbnail: Box<InputFile>) -> &mut Self { self.thumbnail = Some(thumbnail); self }
  
  pub fn width(&self) -> Option<i32> { self.width.clone() }
  #[doc(hidden)] pub fn _set_width(&mut self, width: i32) -> &mut Self { self.width = Some(width); self }
  
  pub fn height(&self) -> Option<i32> { self.height.clone() }
  #[doc(hidden)] pub fn _set_height(&mut self, height: i32) -> &mut Self { self.height = Some(height); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



