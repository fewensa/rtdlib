
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A video message. 
#[derive(Debug, Serialize, Deserialize)]
pub struct InputMessageVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputMessageVideo
  /// Video to be sent.
  video: Option<Box<InputFile>>,
  /// Video thumbnail, if available.
  thumbnail: Option<InputThumbnail>,
  /// File identifiers of the stickers added to the video, if applicable.
  added_sticker_file_ids: Option<Vec<i32>>,
  /// Duration of the video, in seconds.
  duration: Option<i32>,
  /// Video width.
  width: Option<i32>,
  /// Video height.
  height: Option<i32>,
  /// True, if the video should be tried to be streamed.
  supports_streaming: Option<bool>,
  /// Video caption; 0-GetOption("message_caption_length_max") characters.
  caption: Option<FormattedText>,
  /// Video TTL (Time To Live), in seconds (0-60). A non-zero TTL can be specified only in private chats.
  ttl: Option<i32>,
  
}


impl Clone for InputMessageVideo {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for InputMessageVideo {}
impl RObject for InputMessageVideo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageVideo" }
  fn td_type(&self) -> RTDType { RTDType::InputMessageVideo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputMessageContent for InputMessageVideo {}


impl InputMessageVideo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputMessageVideo".to_string(),
      video: None,
      thumbnail: None,
      added_sticker_file_ids: None,
      duration: None,
      width: None,
      height: None,
      supports_streaming: None,
      caption: None,
      ttl: None,
      
    }
  }
  
  pub fn video(&self) -> Option<Box<InputFile>> { self.video.clone() }
  #[doc(hidden)] pub fn _set_video(&mut self, video: Box<InputFile>) -> &mut Self { self.video = Some(video); self }
  
  pub fn thumbnail(&self) -> Option<InputThumbnail> { self.thumbnail.clone() }
  #[doc(hidden)] pub fn _set_thumbnail(&mut self, thumbnail: InputThumbnail) -> &mut Self { self.thumbnail = Some(thumbnail); self }
  
  pub fn added_sticker_file_ids(&self) -> Option<Vec<i32>> { self.added_sticker_file_ids.clone() }
  #[doc(hidden)] pub fn _set_added_sticker_file_ids(&mut self, added_sticker_file_ids: Vec<i32>) -> &mut Self { self.added_sticker_file_ids = Some(added_sticker_file_ids); self }
  
  pub fn duration(&self) -> Option<i32> { self.duration.clone() }
  #[doc(hidden)] pub fn _set_duration(&mut self, duration: i32) -> &mut Self { self.duration = Some(duration); self }
  
  pub fn width(&self) -> Option<i32> { self.width.clone() }
  #[doc(hidden)] pub fn _set_width(&mut self, width: i32) -> &mut Self { self.width = Some(width); self }
  
  pub fn height(&self) -> Option<i32> { self.height.clone() }
  #[doc(hidden)] pub fn _set_height(&mut self, height: i32) -> &mut Self { self.height = Some(height); self }
  
  pub fn supports_streaming(&self) -> Option<bool> { self.supports_streaming.clone() }
  #[doc(hidden)] pub fn _set_supports_streaming(&mut self, supports_streaming: bool) -> &mut Self { self.supports_streaming = Some(supports_streaming); self }
  
  pub fn caption(&self) -> Option<FormattedText> { self.caption.clone() }
  #[doc(hidden)] pub fn _set_caption(&mut self, caption: FormattedText) -> &mut Self { self.caption = Some(caption); self }
  
  pub fn ttl(&self) -> Option<i32> { self.ttl.clone() }
  #[doc(hidden)] pub fn _set_ttl(&mut self, ttl: i32) -> &mut Self { self.ttl = Some(ttl); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



