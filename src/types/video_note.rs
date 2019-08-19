
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Describes a video note. The video must be equal in width and height, cropped to a circle, and stored in MPEG4 format. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // videoNote
  /// Duration of the video, in seconds; as defined by the sender.
  duration: Option<i32>,
  /// Video width and height; as defined by the sender.
  length: Option<i32>,
  /// Video thumbnail; as defined by the sender; may be null.
  thumbnail: Option<PhotoSize>,
  /// File containing the video.
  video: Option<File>,
  
}



impl Object for VideoNote {}
impl RObject for VideoNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "videoNote" }
  fn td_type(&self) -> RTDType { RTDType::VideoNote }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl VideoNote {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "videoNote".to_string(),
      duration: None,
      length: None,
      thumbnail: None,
      video: None,
      
    }
  }
  
  pub fn duration(&self) -> Option<i32> { self.duration.clone() }
  #[doc(hidden)] pub fn _set_duration(&mut self, duration: i32) -> &mut Self { self.duration = Some(duration); self }
  
  pub fn length(&self) -> Option<i32> { self.length.clone() }
  #[doc(hidden)] pub fn _set_length(&mut self, length: i32) -> &mut Self { self.length = Some(length); self }
  
  pub fn thumbnail(&self) -> Option<PhotoSize> { self.thumbnail.clone() }
  #[doc(hidden)] pub fn _set_thumbnail(&mut self, thumbnail: PhotoSize) -> &mut Self { self.thumbnail = Some(thumbnail); self }
  
  pub fn video(&self) -> Option<File> { self.video.clone() }
  #[doc(hidden)] pub fn _set_video(&mut self, video: File) -> &mut Self { self.video = Some(video); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



