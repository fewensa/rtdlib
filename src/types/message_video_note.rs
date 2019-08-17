
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A video note message. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageVideoNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageVideoNote
  /// Message content.
  video_note: Option<VideoNote>,
  /// True, if at least one of the recipients has viewed the video note.
  is_viewed: Option<bool>,
  /// True, if the video note thumbnail must be blurred and the video note must be shown only while tapped.
  is_secret: Option<bool>,
  
}



impl Object for MessageVideoNote {}
impl RObject for MessageVideoNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageVideoNote" }
  fn td_type(&self) -> RTDType { RTDType::MessageVideoNote }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageVideoNote {}


impl MessageVideoNote {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageVideoNote".to_string(),
      video_note: None,
      is_viewed: None,
      is_secret: None,
      
    }
  }
  
  pub fn video_note(&self) -> Option<VideoNote> { self.video_note.clone() }
  #[doc(hidden)] pub fn _set_video_note(&mut self, video_note: VideoNote) -> &mut Self { self.video_note = Some(video_note); self }
  
  pub fn is_viewed(&self) -> Option<bool> { self.is_viewed.clone() }
  #[doc(hidden)] pub fn _set_is_viewed(&mut self, is_viewed: bool) -> &mut Self { self.is_viewed = Some(is_viewed); self }
  
  pub fn is_secret(&self) -> Option<bool> { self.is_secret.clone() }
  #[doc(hidden)] pub fn _set_is_secret(&mut self, is_secret: bool) -> &mut Self { self.is_secret = Some(is_secret); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



