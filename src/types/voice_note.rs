
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Describes a voice note. The voice note must be encoded with the Opus codec, and stored inside an OGG container. Voice notes can have only a single audio channel. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // voiceNote
  /// Duration of the voice note, in seconds; as defined by the sender.
  duration: Option<i32>,
  /// A waveform representation of the voice note in 5-bit format.
  waveform: Option<String>,
  /// MIME type of the file; as defined by the sender.
  mime_type: Option<String>,
  /// File containing the voice note.
  voice: Option<File>,
  
}



impl Object for VoiceNote {}
impl RObject for VoiceNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "voiceNote" }
  fn td_type(&self) -> RTDType { RTDType::VoiceNote }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl VoiceNote {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "voiceNote".to_string(),
      duration: None,
      waveform: None,
      mime_type: None,
      voice: None,
      
    }
  }
  
  pub fn duration(&self) -> Option<i32> { self.duration.clone() }
  #[doc(hidden)] pub fn _set_duration(&mut self, duration: i32) -> &mut Self { self.duration = Some(duration); self }
  
  pub fn waveform(&self) -> Option<String> { self.waveform.clone() }
  #[doc(hidden)] pub fn _set_waveform(&mut self, waveform: String) -> &mut Self { self.waveform = Some(waveform); self }
  
  pub fn mime_type(&self) -> Option<String> { self.mime_type.clone() }
  #[doc(hidden)] pub fn _set_mime_type(&mut self, mime_type: String) -> &mut Self { self.mime_type = Some(mime_type); self }
  
  pub fn voice(&self) -> Option<File> { self.voice.clone() }
  #[doc(hidden)] pub fn _set_voice(&mut self, voice: File) -> &mut Self { self.voice = Some(voice); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



