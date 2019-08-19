
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A video note message. 
#[derive(Debug, Serialize, Deserialize)]
pub struct InputMessageVideoNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputMessageVideoNote
  /// Video note to be sent.
  video_note: Option<Box<InputFile>>,
  /// Video thumbnail, if available.
  thumbnail: Option<InputThumbnail>,
  /// Duration of the video, in seconds.
  duration: Option<i32>,
  /// Video width and height; must be positive and not greater than 640.
  length: Option<i32>,
  
}


impl Clone for InputMessageVideoNote {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for InputMessageVideoNote {}
impl RObject for InputMessageVideoNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageVideoNote" }
  fn td_type(&self) -> RTDType { RTDType::InputMessageVideoNote }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputMessageContent for InputMessageVideoNote {}


impl InputMessageVideoNote {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputMessageVideoNote".to_string(),
      video_note: None,
      thumbnail: None,
      duration: None,
      length: None,
      
    }
  }
  
  pub fn video_note(&self) -> Option<Box<InputFile>> { self.video_note.clone() }
  #[doc(hidden)] pub fn _set_video_note(&mut self, video_note: Box<InputFile>) -> &mut Self { self.video_note = Some(video_note); self }
  
  pub fn thumbnail(&self) -> Option<InputThumbnail> { self.thumbnail.clone() }
  #[doc(hidden)] pub fn _set_thumbnail(&mut self, thumbnail: InputThumbnail) -> &mut Self { self.thumbnail = Some(thumbnail); self }
  
  pub fn duration(&self) -> Option<i32> { self.duration.clone() }
  #[doc(hidden)] pub fn _set_duration(&mut self, duration: i32) -> &mut Self { self.duration = Some(duration); self }
  
  pub fn length(&self) -> Option<i32> { self.length.clone() }
  #[doc(hidden)] pub fn _set_length(&mut self, length: i32) -> &mut Self { self.length = Some(length); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



