use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes type of paid media to send
pub trait TDInputPaidMediaType: Debug + RObject {}

/// Describes type of paid media to send
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputPaidMediaType {
  #[doc(hidden)]
  _Default(()),
  /// The media is a photo. The photo must be at most 10 MB in size. The photo's width and height must not exceed 10000 in total. Width and height ratio must be at most 20
  Photo(InputPaidMediaTypePhoto),
  /// The media is a video
  Video(InputPaidMediaTypeVideo),
}

impl Default for InputPaidMediaType {
  fn default() -> Self {
    InputPaidMediaType::_Default(())
  }
}

impl<'de> Deserialize<'de> for InputPaidMediaType {
  fn deserialize<D>(deserializer: D) -> Result<InputPaidMediaType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InputPaidMediaType,
      (inputPaidMediaTypePhoto, Photo);
      (inputPaidMediaTypeVideo, Video);

    )(deserializer)
  }
}

impl RObject for InputPaidMediaType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      InputPaidMediaType::Photo(t) => t.td_name(),
      InputPaidMediaType::Video(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      InputPaidMediaType::Photo(t) => t.extra(),
      InputPaidMediaType::Video(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputPaidMediaType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, InputPaidMediaType::_Default(_))
  }

  pub fn is_photo(&self) -> bool {
    matches!(self, InputPaidMediaType::Photo(_))
  }
  pub fn is_video(&self) -> bool {
    matches!(self, InputPaidMediaType::Video(_))
  }

  pub fn on_photo<F: FnOnce(&InputPaidMediaTypePhoto)>(&self, fnc: F) -> &Self {
    if let InputPaidMediaType::Photo(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_video<F: FnOnce(&InputPaidMediaTypeVideo)>(&self, fnc: F) -> &Self {
    if let InputPaidMediaType::Video(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_photo(&self) -> Option<&InputPaidMediaTypePhoto> {
    if let InputPaidMediaType::Photo(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_video(&self) -> Option<&InputPaidMediaTypeVideo> {
    if let InputPaidMediaType::Video(t) = self {
      return Some(t);
    }
    None
  }

  pub fn photo<T: AsRef<InputPaidMediaTypePhoto>>(t: T) -> Self {
    InputPaidMediaType::Photo(t.as_ref().clone())
  }

  pub fn video<T: AsRef<InputPaidMediaTypeVideo>>(t: T) -> Self {
    InputPaidMediaType::Video(t.as_ref().clone())
  }
}

impl AsRef<InputPaidMediaType> for InputPaidMediaType {
  fn as_ref(&self) -> &InputPaidMediaType {
    self
  }
}

/// The media is a photo. The photo must be at most 10 MB in size. The photo's width and height must not exceed 10000 in total. Width and height ratio must be at most 20
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPaidMediaTypePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Video of the live photo; pass null if the photo isn't a live photo
  video: InputFile,
}

impl RObject for InputPaidMediaTypePhoto {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPaidMediaTypePhoto"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPaidMediaType for InputPaidMediaTypePhoto {}

impl InputPaidMediaTypePhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPaidMediaTypePhotoBuilder {
    let inner = InputPaidMediaTypePhoto {
      td_name: "inputPaidMediaTypePhoto".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPaidMediaTypePhotoBuilder { inner }
  }

  pub fn video(&self) -> &InputFile {
    &self.video
  }
}

#[doc(hidden)]
pub struct RTDInputPaidMediaTypePhotoBuilder {
  inner: InputPaidMediaTypePhoto,
}

impl RTDInputPaidMediaTypePhotoBuilder {
  pub fn build(&self) -> InputPaidMediaTypePhoto {
    self.inner.clone()
  }

  pub fn video<T: AsRef<InputFile>>(&mut self, video: T) -> &mut Self {
    self.inner.video = video.as_ref().clone();
    self
  }
}

impl AsRef<InputPaidMediaTypePhoto> for InputPaidMediaTypePhoto {
  fn as_ref(&self) -> &InputPaidMediaTypePhoto {
    self
  }
}

impl AsRef<InputPaidMediaTypePhoto> for RTDInputPaidMediaTypePhotoBuilder {
  fn as_ref(&self) -> &InputPaidMediaTypePhoto {
    &self.inner
  }
}

/// The media is a video
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPaidMediaTypeVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Cover of the video; pass null to skip cover uploading
  cover: InputFile,
  /// Timestamp from which the video playing must start, in seconds
  start_timestamp: i64,
  /// Duration of the video, in seconds
  duration: i64,
  /// True, if the video is expected to be streamed
  supports_streaming: bool,
}

impl RObject for InputPaidMediaTypeVideo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPaidMediaTypeVideo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPaidMediaType for InputPaidMediaTypeVideo {}

impl InputPaidMediaTypeVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPaidMediaTypeVideoBuilder {
    let inner = InputPaidMediaTypeVideo {
      td_name: "inputPaidMediaTypeVideo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPaidMediaTypeVideoBuilder { inner }
  }

  pub fn cover(&self) -> &InputFile {
    &self.cover
  }

  pub fn start_timestamp(&self) -> i64 {
    self.start_timestamp
  }

  pub fn duration(&self) -> i64 {
    self.duration
  }

  pub fn supports_streaming(&self) -> bool {
    self.supports_streaming
  }
}

#[doc(hidden)]
pub struct RTDInputPaidMediaTypeVideoBuilder {
  inner: InputPaidMediaTypeVideo,
}

impl RTDInputPaidMediaTypeVideoBuilder {
  pub fn build(&self) -> InputPaidMediaTypeVideo {
    self.inner.clone()
  }

  pub fn cover<T: AsRef<InputFile>>(&mut self, cover: T) -> &mut Self {
    self.inner.cover = cover.as_ref().clone();
    self
  }

  pub fn start_timestamp(&mut self, start_timestamp: i64) -> &mut Self {
    self.inner.start_timestamp = start_timestamp;
    self
  }

  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }

  pub fn supports_streaming(&mut self, supports_streaming: bool) -> &mut Self {
    self.inner.supports_streaming = supports_streaming;
    self
  }
}

impl AsRef<InputPaidMediaTypeVideo> for InputPaidMediaTypeVideo {
  fn as_ref(&self) -> &InputPaidMediaTypeVideo {
    self
  }
}

impl AsRef<InputPaidMediaTypeVideo> for RTDInputPaidMediaTypeVideoBuilder {
  fn as_ref(&self) -> &InputPaidMediaTypeVideo {
    &self.inner
  }
}
