
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A voice note message. 
#[derive(Debug, Serialize, Deserialize)]
pub struct InputMessageVoiceNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputMessageVoiceNote
  /// Voice note to be sent.
  voice_note: Option<Box<InputFile>>,
  /// Duration of the voice note, in seconds.
  duration: Option<i32>,
  /// Waveform representation of the voice note, in 5-bit format.
  waveform: Option<String>,
  /// Voice note caption; 0-GetOption("message_caption_length_max") characters.
  caption: Option<FormattedText>,
  
}


impl Clone for InputMessageVoiceNote {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for InputMessageVoiceNote {}
impl RObject for InputMessageVoiceNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageVoiceNote" }
  fn td_type(&self) -> RTDType { RTDType::InputMessageVoiceNote }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputMessageContent for InputMessageVoiceNote {}


impl InputMessageVoiceNote {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputMessageVoiceNote".to_string(),
      voice_note: None,
      duration: None,
      waveform: None,
      caption: None,
      
    }
  }
  
  pub fn voice_note(&self) -> Option<Box<InputFile>> { self.voice_note.clone() }
  #[doc(hidden)] pub fn _set_voice_note(&mut self, voice_note: Box<InputFile>) -> &mut Self { self.voice_note = Some(voice_note); self }
  
  pub fn duration(&self) -> Option<i32> { self.duration.clone() }
  #[doc(hidden)] pub fn _set_duration(&mut self, duration: i32) -> &mut Self { self.duration = Some(duration); self }
  
  pub fn waveform(&self) -> Option<String> { self.waveform.clone() }
  #[doc(hidden)] pub fn _set_waveform(&mut self, waveform: String) -> &mut Self { self.waveform = Some(waveform); self }
  
  pub fn caption(&self) -> Option<FormattedText> { self.caption.clone() }
  #[doc(hidden)] pub fn _set_caption(&mut self, caption: FormattedText) -> &mut Self { self.caption = Some(caption); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



