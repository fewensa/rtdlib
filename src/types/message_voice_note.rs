
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A voice note message. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageVoiceNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageVoiceNote
  /// Message content.
  voice_note: Option<VoiceNote>,
  /// Voice note caption.
  caption: Option<FormattedText>,
  /// True, if at least one of the recipients has listened to the voice note.
  is_listened: Option<bool>,
  
}



impl Object for MessageVoiceNote {}
impl RObject for MessageVoiceNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageVoiceNote" }
  fn td_type(&self) -> RTDType { RTDType::MessageVoiceNote }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageVoiceNote {}


impl MessageVoiceNote {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageVoiceNote".to_string(),
      voice_note: None,
      caption: None,
      is_listened: None,
      
    }
  }
  
  pub fn voice_note(&self) -> Option<VoiceNote> { self.voice_note.clone() }
  #[doc(hidden)] pub fn _set_voice_note(&mut self, voice_note: VoiceNote) -> &mut Self { self.voice_note = Some(voice_note); self }
  
  pub fn caption(&self) -> Option<FormattedText> { self.caption.clone() }
  #[doc(hidden)] pub fn _set_caption(&mut self, caption: FormattedText) -> &mut Self { self.caption = Some(caption); self }
  
  pub fn is_listened(&self) -> Option<bool> { self.is_listened.clone() }
  #[doc(hidden)] pub fn _set_is_listened(&mut self, is_listened: bool) -> &mut Self { self.is_listened = Some(is_listened); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



