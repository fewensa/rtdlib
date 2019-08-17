
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A video note message. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessageContentVideoNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pushMessageContentVideoNote
  /// Message content; may be null.
  video_note: Option<VideoNote>,
  /// True, if the message is a pinned message with the specified content.
  is_pinned: Option<bool>,
  
}



impl Object for PushMessageContentVideoNote {}
impl RObject for PushMessageContentVideoNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentVideoNote" }
  fn td_type(&self) -> RTDType { RTDType::PushMessageContentVideoNote }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PushMessageContent for PushMessageContentVideoNote {}


impl PushMessageContentVideoNote {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pushMessageContentVideoNote".to_string(),
      video_note: None,
      is_pinned: None,
      
    }
  }
  
  pub fn video_note(&self) -> Option<VideoNote> { self.video_note.clone() }
  #[doc(hidden)] pub fn _set_video_note(&mut self, video_note: VideoNote) -> &mut Self { self.video_note = Some(video_note); self }
  
  pub fn is_pinned(&self) -> Option<bool> { self.is_pinned.clone() }
  #[doc(hidden)] pub fn _set_is_pinned(&mut self, is_pinned: bool) -> &mut Self { self.is_pinned = Some(is_pinned); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



