
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An audio message. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAudio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageAudio
  /// Message content.
  audio: Option<Audio>,
  /// Audio caption.
  caption: Option<FormattedText>,
  
}



impl Object for MessageAudio {}
impl RObject for MessageAudio {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageAudio" }
  fn td_type(&self) -> RTDType { RTDType::MessageAudio }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageAudio {}


impl MessageAudio {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageAudio".to_string(),
      audio: None,
      caption: None,
      
    }
  }
  
  pub fn audio(&self) -> Option<Audio> { self.audio.clone() }
  #[doc(hidden)] pub fn _set_audio(&mut self, audio: Audio) -> &mut Self { self.audio = Some(audio); self }
  
  pub fn caption(&self) -> Option<FormattedText> { self.caption.clone() }
  #[doc(hidden)] pub fn _set_caption(&mut self, caption: FormattedText) -> &mut Self { self.caption = Some(caption); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



