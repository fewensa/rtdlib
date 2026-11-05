use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A voice note to be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputVoiceNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Voice note file to be sent. The voice note must be encoded with the Opus codec and stored inside an OGG container with a single audio channel, or be in MP3 or M4A format as regular audio
  voice_note: InputFile,
  /// Duration of the voice note, in seconds
  duration: i64,
  /// Waveform representation of the voice note in 5-bit format
  waveform: String,
}

impl RObject for InputVoiceNote {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputVoiceNote"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputVoiceNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputVoiceNoteBuilder {
    let inner = InputVoiceNote {
      td_name: "inputVoiceNote".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputVoiceNoteBuilder { inner }
  }

  pub fn voice_note(&self) -> &InputFile {
    &self.voice_note
  }

  pub fn duration(&self) -> i64 {
    self.duration
  }

  pub fn waveform(&self) -> &String {
    &self.waveform
  }
}

#[doc(hidden)]
pub struct RTDInputVoiceNoteBuilder {
  inner: InputVoiceNote,
}

impl RTDInputVoiceNoteBuilder {
  pub fn build(&self) -> InputVoiceNote {
    self.inner.clone()
  }

  pub fn voice_note<T: AsRef<InputFile>>(&mut self, voice_note: T) -> &mut Self {
    self.inner.voice_note = voice_note.as_ref().clone();
    self
  }

  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }

  pub fn waveform<T: AsRef<str>>(&mut self, waveform: T) -> &mut Self {
    self.inner.waveform = waveform.as_ref().to_string();
    self
  }
}

impl AsRef<InputVoiceNote> for InputVoiceNote {
  fn as_ref(&self) -> &InputVoiceNote {
    self
  }
}

impl AsRef<InputVoiceNote> for RTDInputVoiceNoteBuilder {
  fn as_ref(&self) -> &InputVoiceNote {
    &self.inner
  }
}
