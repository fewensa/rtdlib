
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An animation message (GIF-style). 
#[derive(Debug, Serialize, Deserialize)]
pub struct InputMessageAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputMessageAnimation
  /// Animation file to be sent.
  animation: Option<Box<InputFile>>,
  /// Animation thumbnail, if available.
  thumbnail: Option<InputThumbnail>,
  /// Duration of the animation, in seconds.
  duration: Option<i32>,
  /// Width of the animation; may be replaced by the server.
  width: Option<i32>,
  /// Height of the animation; may be replaced by the server.
  height: Option<i32>,
  /// Animation caption; 0-GetOption("message_caption_length_max") characters.
  caption: Option<FormattedText>,
  
}


impl Clone for InputMessageAnimation {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for InputMessageAnimation {}
impl RObject for InputMessageAnimation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageAnimation" }
  fn td_type(&self) -> RTDType { RTDType::InputMessageAnimation }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputMessageContent for InputMessageAnimation {}


impl InputMessageAnimation {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputMessageAnimation".to_string(),
      animation: None,
      thumbnail: None,
      duration: None,
      width: None,
      height: None,
      caption: None,
      
    }
  }
  
  pub fn animation(&self) -> Option<Box<InputFile>> { self.animation.clone() }
  #[doc(hidden)] pub fn _set_animation(&mut self, animation: Box<InputFile>) -> &mut Self { self.animation = Some(animation); self }
  
  pub fn thumbnail(&self) -> Option<InputThumbnail> { self.thumbnail.clone() }
  #[doc(hidden)] pub fn _set_thumbnail(&mut self, thumbnail: InputThumbnail) -> &mut Self { self.thumbnail = Some(thumbnail); self }
  
  pub fn duration(&self) -> Option<i32> { self.duration.clone() }
  #[doc(hidden)] pub fn _set_duration(&mut self, duration: i32) -> &mut Self { self.duration = Some(duration); self }
  
  pub fn width(&self) -> Option<i32> { self.width.clone() }
  #[doc(hidden)] pub fn _set_width(&mut self, width: i32) -> &mut Self { self.width = Some(width); self }
  
  pub fn height(&self) -> Option<i32> { self.height.clone() }
  #[doc(hidden)] pub fn _set_height(&mut self, height: i32) -> &mut Self { self.height = Some(height); self }
  
  pub fn caption(&self) -> Option<FormattedText> { self.caption.clone() }
  #[doc(hidden)] pub fn _set_caption(&mut self, caption: FormattedText) -> &mut Self { self.caption = Some(caption); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



