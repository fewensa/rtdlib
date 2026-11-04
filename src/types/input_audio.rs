use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// An audio to be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputAudio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Audio file to be sent
  audio: InputFile,
  /// Thumbnail of the cover for the album; pass null to skip thumbnail uploading
  album_cover_thumbnail: InputThumbnail,
  /// Duration of the audio, in seconds; may be replaced by the server
  duration: i64,
  /// Title of the audio; 0-64 characters; may be replaced by the server
  title: String,
  /// Performer of the audio; 0-64 characters, may be replaced by the server
  performer: String,
}

impl RObject for InputAudio {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputAudio"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputAudio {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputAudioBuilder {
    let inner = InputAudio {
      td_name: "inputAudio".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputAudioBuilder { inner }
  }

  pub fn audio(&self) -> &InputFile {
    &self.audio
  }

  pub fn album_cover_thumbnail(&self) -> &InputThumbnail {
    &self.album_cover_thumbnail
  }

  pub fn duration(&self) -> i64 {
    self.duration
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn performer(&self) -> &String {
    &self.performer
  }
}

#[doc(hidden)]
pub struct RTDInputAudioBuilder {
  inner: InputAudio,
}

impl RTDInputAudioBuilder {
  pub fn build(&self) -> InputAudio {
    self.inner.clone()
  }

  pub fn audio<T: AsRef<InputFile>>(&mut self, audio: T) -> &mut Self {
    self.inner.audio = audio.as_ref().clone();
    self
  }

  pub fn album_cover_thumbnail<T: AsRef<InputThumbnail>>(
    &mut self,
    album_cover_thumbnail: T,
  ) -> &mut Self {
    self.inner.album_cover_thumbnail = album_cover_thumbnail.as_ref().clone();
    self
  }

  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }

  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

  pub fn performer<T: AsRef<str>>(&mut self, performer: T) -> &mut Self {
    self.inner.performer = performer.as_ref().to_string();
    self
  }
}

impl AsRef<InputAudio> for InputAudio {
  fn as_ref(&self) -> &InputAudio {
    self
  }
}

impl AsRef<InputAudio> for RTDInputAudioBuilder {
  fn as_ref(&self) -> &InputAudio {
    &self.inner
  }
}
