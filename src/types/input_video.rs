use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A video to be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Video file to be sent. The video is expected to be re-encoded to MPEG4 format with H.264 codec by the sender
  video: InputFile,
  /// Video thumbnail; pass null to skip thumbnail uploading
  thumbnail: InputThumbnail,
  /// Cover of the video; pass null to skip cover uploading; not supported in secret chats and for self-destructing messages
  cover: InputFile,
  /// Timestamp from which the video playing must start, in seconds
  start_timestamp: i64,
  /// File identifiers of the stickers added to the video, if applicable
  added_sticker_file_ids: Vec<i64>,
  /// Duration of the video, in seconds
  duration: i64,
  /// Video width
  width: i64,
  /// Video height
  height: i64,
  /// True, if the video is expected to be streamed
  supports_streaming: bool,
}

impl RObject for InputVideo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputVideo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputVideoBuilder {
    let inner = InputVideo {
      td_name: "inputVideo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputVideoBuilder { inner }
  }

  pub fn video(&self) -> &InputFile {
    &self.video
  }

  pub fn thumbnail(&self) -> &InputThumbnail {
    &self.thumbnail
  }

  pub fn cover(&self) -> &InputFile {
    &self.cover
  }

  pub fn start_timestamp(&self) -> i64 {
    self.start_timestamp
  }

  pub fn added_sticker_file_ids(&self) -> &Vec<i64> {
    &self.added_sticker_file_ids
  }

  pub fn duration(&self) -> i64 {
    self.duration
  }

  pub fn width(&self) -> i64 {
    self.width
  }

  pub fn height(&self) -> i64 {
    self.height
  }

  pub fn supports_streaming(&self) -> bool {
    self.supports_streaming
  }
}

#[doc(hidden)]
pub struct RTDInputVideoBuilder {
  inner: InputVideo,
}

impl RTDInputVideoBuilder {
  pub fn build(&self) -> InputVideo {
    self.inner.clone()
  }

  pub fn video<T: AsRef<InputFile>>(&mut self, video: T) -> &mut Self {
    self.inner.video = video.as_ref().clone();
    self
  }

  pub fn thumbnail<T: AsRef<InputThumbnail>>(&mut self, thumbnail: T) -> &mut Self {
    self.inner.thumbnail = thumbnail.as_ref().clone();
    self
  }

  pub fn cover<T: AsRef<InputFile>>(&mut self, cover: T) -> &mut Self {
    self.inner.cover = cover.as_ref().clone();
    self
  }

  pub fn start_timestamp(&mut self, start_timestamp: i64) -> &mut Self {
    self.inner.start_timestamp = start_timestamp;
    self
  }

  pub fn added_sticker_file_ids(&mut self, added_sticker_file_ids: Vec<i64>) -> &mut Self {
    self.inner.added_sticker_file_ids = added_sticker_file_ids;
    self
  }

  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }

  pub fn width(&mut self, width: i64) -> &mut Self {
    self.inner.width = width;
    self
  }

  pub fn height(&mut self, height: i64) -> &mut Self {
    self.inner.height = height;
    self
  }

  pub fn supports_streaming(&mut self, supports_streaming: bool) -> &mut Self {
    self.inner.supports_streaming = supports_streaming;
    self
  }
}

impl AsRef<InputVideo> for InputVideo {
  fn as_ref(&self) -> &InputVideo {
    self
  }
}

impl AsRef<InputVideo> for RTDInputVideoBuilder {
  fn as_ref(&self) -> &InputVideo {
    &self.inner
  }
}
