
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Describes an animation file. The animation must be encoded in GIF or MPEG4 format. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // animation
  /// Duration of the animation, in seconds; as defined by the sender.
  duration: Option<i32>,
  /// Width of the animation.
  width: Option<i32>,
  /// Height of the animation.
  height: Option<i32>,
  /// Original name of the file; as defined by the sender.
  file_name: Option<String>,
  /// MIME type of the file, usually "image/gif" or "video/mp4".
  mime_type: Option<String>,
  /// Animation thumbnail; may be null.
  thumbnail: Option<PhotoSize>,
  /// File containing the animation.
  animation: Option<File>,
  
}



impl Object for Animation {}
impl RObject for Animation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "animation" }
  fn td_type(&self) -> RTDType { RTDType::Animation }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Animation {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "animation".to_string(),
      duration: None,
      width: None,
      height: None,
      file_name: None,
      mime_type: None,
      thumbnail: None,
      animation: None,
      
    }
  }
  
  pub fn duration(&self) -> Option<i32> { self.duration.clone() }
  #[doc(hidden)] pub fn _set_duration(&mut self, duration: i32) -> &mut Self { self.duration = Some(duration); self }
  
  pub fn width(&self) -> Option<i32> { self.width.clone() }
  #[doc(hidden)] pub fn _set_width(&mut self, width: i32) -> &mut Self { self.width = Some(width); self }
  
  pub fn height(&self) -> Option<i32> { self.height.clone() }
  #[doc(hidden)] pub fn _set_height(&mut self, height: i32) -> &mut Self { self.height = Some(height); self }
  
  pub fn file_name(&self) -> Option<String> { self.file_name.clone() }
  #[doc(hidden)] pub fn _set_file_name(&mut self, file_name: String) -> &mut Self { self.file_name = Some(file_name); self }
  
  pub fn mime_type(&self) -> Option<String> { self.mime_type.clone() }
  #[doc(hidden)] pub fn _set_mime_type(&mut self, mime_type: String) -> &mut Self { self.mime_type = Some(mime_type); self }
  
  pub fn thumbnail(&self) -> Option<PhotoSize> { self.thumbnail.clone() }
  #[doc(hidden)] pub fn _set_thumbnail(&mut self, thumbnail: PhotoSize) -> &mut Self { self.thumbnail = Some(thumbnail); self }
  
  pub fn animation(&self) -> Option<File> { self.animation.clone() }
  #[doc(hidden)] pub fn _set_animation(&mut self, animation: File) -> &mut Self { self.animation = Some(animation); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



