
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Describes a video file. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // video
  /// Duration of the video, in seconds; as defined by the sender.
  duration: Option<i32>,
  /// Video width; as defined by the sender.
  width: Option<i32>,
  /// Video height; as defined by the sender.
  height: Option<i32>,
  /// Original name of the file; as defined by the sender.
  file_name: Option<String>,
  /// MIME type of the file; as defined by the sender.
  mime_type: Option<String>,
  /// True, if stickers were added to the photo.
  has_stickers: Option<bool>,
  /// True, if the video should be tried to be streamed.
  supports_streaming: Option<bool>,
  /// Video thumbnail; as defined by the sender; may be null.
  thumbnail: Option<PhotoSize>,
  /// File containing the video.
  video: Option<File>,
  
}



impl Object for Video {}
impl RObject for Video {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "video" }
  fn td_type(&self) -> RTDType { RTDType::Video }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Video {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "video".to_string(),
      duration: None,
      width: None,
      height: None,
      file_name: None,
      mime_type: None,
      has_stickers: None,
      supports_streaming: None,
      thumbnail: None,
      video: None,
      
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
  
  pub fn has_stickers(&self) -> Option<bool> { self.has_stickers.clone() }
  #[doc(hidden)] pub fn _set_has_stickers(&mut self, has_stickers: bool) -> &mut Self { self.has_stickers = Some(has_stickers); self }
  
  pub fn supports_streaming(&self) -> Option<bool> { self.supports_streaming.clone() }
  #[doc(hidden)] pub fn _set_supports_streaming(&mut self, supports_streaming: bool) -> &mut Self { self.supports_streaming = Some(supports_streaming); self }
  
  pub fn thumbnail(&self) -> Option<PhotoSize> { self.thumbnail.clone() }
  #[doc(hidden)] pub fn _set_thumbnail(&mut self, thumbnail: PhotoSize) -> &mut Self { self.thumbnail = Some(thumbnail); self }
  
  pub fn video(&self) -> Option<File> { self.video.clone() }
  #[doc(hidden)] pub fn _set_video(&mut self, video: File) -> &mut Self { self.video = Some(video); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



