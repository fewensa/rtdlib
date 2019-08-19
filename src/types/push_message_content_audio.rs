
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An audio message. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessageContentAudio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pushMessageContentAudio
  /// Message content; may be null.
  audio: Option<Audio>,
  /// True, if the message is a pinned message with the specified content.
  is_pinned: Option<bool>,
  
}



impl Object for PushMessageContentAudio {}
impl RObject for PushMessageContentAudio {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentAudio" }
  fn td_type(&self) -> RTDType { RTDType::PushMessageContentAudio }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PushMessageContent for PushMessageContentAudio {}


impl PushMessageContentAudio {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pushMessageContentAudio".to_string(),
      audio: None,
      is_pinned: None,
      
    }
  }
  
  pub fn audio(&self) -> Option<Audio> { self.audio.clone() }
  #[doc(hidden)] pub fn _set_audio(&mut self, audio: Audio) -> &mut Self { self.audio = Some(audio); self }
  
  pub fn is_pinned(&self) -> Option<bool> { self.is_pinned.clone() }
  #[doc(hidden)] pub fn _set_is_pinned(&mut self, is_pinned: bool) -> &mut Self { self.is_pinned = Some(is_pinned); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



