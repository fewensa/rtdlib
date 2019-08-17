
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A video message. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageVideo
  /// Message content.
  video: Option<Video>,
  /// Video caption.
  caption: Option<FormattedText>,
  /// True, if the video thumbnail must be blurred and the video must be shown only while tapped.
  is_secret: Option<bool>,
  
}



impl Object for MessageVideo {}
impl RObject for MessageVideo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageVideo" }
  fn td_type(&self) -> RTDType { RTDType::MessageVideo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageVideo {}


impl MessageVideo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageVideo".to_string(),
      video: None,
      caption: None,
      is_secret: None,
      
    }
  }
  
  pub fn video(&self) -> Option<Video> { self.video.clone() }
  #[doc(hidden)] pub fn _set_video(&mut self, video: Video) -> &mut Self { self.video = Some(video); self }
  
  pub fn caption(&self) -> Option<FormattedText> { self.caption.clone() }
  #[doc(hidden)] pub fn _set_caption(&mut self, caption: FormattedText) -> &mut Self { self.caption = Some(caption); self }
  
  pub fn is_secret(&self) -> Option<bool> { self.is_secret.clone() }
  #[doc(hidden)] pub fn _set_is_secret(&mut self, is_secret: bool) -> &mut Self { self.is_secret = Some(is_secret); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



