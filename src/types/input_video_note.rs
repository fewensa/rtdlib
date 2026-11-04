use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A video note to be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputVideoNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Video note file to be sent. The video is expected to be encoded to MPEG4 format with H.264 codec and have no data outside of the visible circle
  video_note: InputFile,
  /// Video thumbnail; may be null if empty; pass null to skip thumbnail uploading
  thumbnail: Option<InputThumbnail>,
  /// Duration of the video, in seconds; 0-60
  duration: i64,
  /// Video width and height; must be positive and not greater than 640
  length: i64,
}

impl RObject for InputVideoNote {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputVideoNote"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputVideoNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputVideoNoteBuilder {
    let inner = InputVideoNote {
      td_name: "inputVideoNote".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputVideoNoteBuilder { inner }
  }

  pub fn video_note(&self) -> &InputFile {
    &self.video_note
  }

  pub fn thumbnail(&self) -> &Option<InputThumbnail> {
    &self.thumbnail
  }

  pub fn duration(&self) -> i64 {
    self.duration
  }

  pub fn length(&self) -> i64 {
    self.length
  }
}

#[doc(hidden)]
pub struct RTDInputVideoNoteBuilder {
  inner: InputVideoNote,
}

impl RTDInputVideoNoteBuilder {
  pub fn build(&self) -> InputVideoNote {
    self.inner.clone()
  }

  pub fn video_note<T: AsRef<InputFile>>(&mut self, video_note: T) -> &mut Self {
    self.inner.video_note = video_note.as_ref().clone();
    self
  }

  pub fn thumbnail<T: AsRef<InputThumbnail>>(&mut self, thumbnail: T) -> &mut Self {
    self.inner.thumbnail = Some(thumbnail.as_ref().clone());
    self
  }

  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }

  pub fn length(&mut self, length: i64) -> &mut Self {
    self.inner.length = length;
    self
  }
}

impl AsRef<InputVideoNote> for InputVideoNote {
  fn as_ref(&self) -> &InputVideoNote {
    self
  }
}

impl AsRef<InputVideoNote> for RTDInputVideoNoteBuilder {
  fn as_ref(&self) -> &InputVideoNote {
    &self.inner
  }
}
