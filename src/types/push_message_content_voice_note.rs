
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A voice note message. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessageContentVoiceNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pushMessageContentVoiceNote
  /// Message content; may be null.
  voice_note: Option<VoiceNote>,
  /// True, if the message is a pinned message with the specified content.
  is_pinned: Option<bool>,
  
}



impl Object for PushMessageContentVoiceNote {}
impl RObject for PushMessageContentVoiceNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentVoiceNote" }
  fn td_type(&self) -> RTDType { RTDType::PushMessageContentVoiceNote }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PushMessageContent for PushMessageContentVoiceNote {}


impl PushMessageContentVoiceNote {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pushMessageContentVoiceNote".to_string(),
      voice_note: None,
      is_pinned: None,
      
    }
  }
  
  pub fn voice_note(&self) -> Option<VoiceNote> { self.voice_note.clone() }
  #[doc(hidden)] pub fn _set_voice_note(&mut self, voice_note: VoiceNote) -> &mut Self { self.voice_note = Some(voice_note); self }
  
  pub fn is_pinned(&self) -> Option<bool> { self.is_pinned.clone() }
  #[doc(hidden)] pub fn _set_is_pinned(&mut self, is_pinned: bool) -> &mut Self { self.is_pinned = Some(is_pinned); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



