use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | The content of a story to post
pub trait TDInputStoryContent: Debug + RObject {}

/// The content of a story to post
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputStoryContent {
  #[doc(hidden)]
  _Default(()),
  /// A photo story
  Photo(InputStoryContentPhoto),
  /// A video story
  Video(InputStoryContentVideo),
}

impl Default for InputStoryContent {
  fn default() -> Self {
    InputStoryContent::_Default(())
  }
}

impl<'de> Deserialize<'de> for InputStoryContent {
  fn deserialize<D>(deserializer: D) -> Result<InputStoryContent, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InputStoryContent,
      (inputStoryContentPhoto, Photo);
      (inputStoryContentVideo, Video);

    )(deserializer)
  }
}

impl RObject for InputStoryContent {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      InputStoryContent::Photo(t) => t.td_name(),
      InputStoryContent::Video(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      InputStoryContent::Photo(t) => t.extra(),
      InputStoryContent::Video(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputStoryContent {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, InputStoryContent::_Default(_))
  }

  pub fn is_photo(&self) -> bool {
    matches!(self, InputStoryContent::Photo(_))
  }
  pub fn is_video(&self) -> bool {
    matches!(self, InputStoryContent::Video(_))
  }

  pub fn on_photo<F: FnOnce(&InputStoryContentPhoto)>(&self, fnc: F) -> &Self {
    if let InputStoryContent::Photo(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_video<F: FnOnce(&InputStoryContentVideo)>(&self, fnc: F) -> &Self {
    if let InputStoryContent::Video(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_photo(&self) -> Option<&InputStoryContentPhoto> {
    if let InputStoryContent::Photo(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_video(&self) -> Option<&InputStoryContentVideo> {
    if let InputStoryContent::Video(t) = self {
      return Some(t);
    }
    None
  }

  pub fn photo<T: AsRef<InputStoryContentPhoto>>(t: T) -> Self {
    InputStoryContent::Photo(t.as_ref().clone())
  }

  pub fn video<T: AsRef<InputStoryContentVideo>>(t: T) -> Self {
    InputStoryContent::Video(t.as_ref().clone())
  }
}

impl AsRef<InputStoryContent> for InputStoryContent {
  fn as_ref(&self) -> &InputStoryContent {
    self
  }
}

/// A photo story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputStoryContentPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Photo to send. The photo must be at most 10 MB in size. The photo size must be 1080x1920
  photo: InputFile,
  /// File identifiers of the stickers added to the photo, if applicable
  added_sticker_file_ids: Vec<i64>,
}

impl RObject for InputStoryContentPhoto {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputStoryContentPhoto"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputStoryContent for InputStoryContentPhoto {}

impl InputStoryContentPhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputStoryContentPhotoBuilder {
    let inner = InputStoryContentPhoto {
      td_name: "inputStoryContentPhoto".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputStoryContentPhotoBuilder { inner }
  }

  pub fn photo(&self) -> &InputFile {
    &self.photo
  }

  pub fn added_sticker_file_ids(&self) -> &Vec<i64> {
    &self.added_sticker_file_ids
  }
}

#[doc(hidden)]
pub struct RTDInputStoryContentPhotoBuilder {
  inner: InputStoryContentPhoto,
}

impl RTDInputStoryContentPhotoBuilder {
  pub fn build(&self) -> InputStoryContentPhoto {
    self.inner.clone()
  }

  pub fn photo<T: AsRef<InputFile>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = photo.as_ref().clone();
    self
  }

  pub fn added_sticker_file_ids(&mut self, added_sticker_file_ids: Vec<i64>) -> &mut Self {
    self.inner.added_sticker_file_ids = added_sticker_file_ids;
    self
  }
}

impl AsRef<InputStoryContentPhoto> for InputStoryContentPhoto {
  fn as_ref(&self) -> &InputStoryContentPhoto {
    self
  }
}

impl AsRef<InputStoryContentPhoto> for RTDInputStoryContentPhotoBuilder {
  fn as_ref(&self) -> &InputStoryContentPhoto {
    &self.inner
  }
}

/// A video story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputStoryContentVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Video to be sent. The video size must be 720x1280. The video must be streamable and stored in MPEG4 format, after encoding with H.265 codec and key frames added each second
  video: InputFile,
  /// File identifiers of the stickers added to the video, if applicable
  added_sticker_file_ids: Vec<i64>,
  /// Precise duration of the video, in seconds; 0-60
  duration: f32,
  /// Timestamp of the frame, which will be used as video thumbnail
  cover_frame_timestamp: f32,
  /// True, if the video has no sound
  is_animation: bool,
}

impl RObject for InputStoryContentVideo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputStoryContentVideo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputStoryContent for InputStoryContentVideo {}

impl InputStoryContentVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputStoryContentVideoBuilder {
    let inner = InputStoryContentVideo {
      td_name: "inputStoryContentVideo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputStoryContentVideoBuilder { inner }
  }

  pub fn video(&self) -> &InputFile {
    &self.video
  }

  pub fn added_sticker_file_ids(&self) -> &Vec<i64> {
    &self.added_sticker_file_ids
  }

  pub fn duration(&self) -> f32 {
    self.duration
  }

  pub fn cover_frame_timestamp(&self) -> f32 {
    self.cover_frame_timestamp
  }

  pub fn is_animation(&self) -> bool {
    self.is_animation
  }
}

#[doc(hidden)]
pub struct RTDInputStoryContentVideoBuilder {
  inner: InputStoryContentVideo,
}

impl RTDInputStoryContentVideoBuilder {
  pub fn build(&self) -> InputStoryContentVideo {
    self.inner.clone()
  }

  pub fn video<T: AsRef<InputFile>>(&mut self, video: T) -> &mut Self {
    self.inner.video = video.as_ref().clone();
    self
  }

  pub fn added_sticker_file_ids(&mut self, added_sticker_file_ids: Vec<i64>) -> &mut Self {
    self.inner.added_sticker_file_ids = added_sticker_file_ids;
    self
  }

  pub fn duration(&mut self, duration: f32) -> &mut Self {
    self.inner.duration = duration;
    self
  }

  pub fn cover_frame_timestamp(&mut self, cover_frame_timestamp: f32) -> &mut Self {
    self.inner.cover_frame_timestamp = cover_frame_timestamp;
    self
  }

  pub fn is_animation(&mut self, is_animation: bool) -> &mut Self {
    self.inner.is_animation = is_animation;
    self
  }
}

impl AsRef<InputStoryContentVideo> for InputStoryContentVideo {
  fn as_ref(&self) -> &InputStoryContentVideo {
    self
  }
}

impl AsRef<InputStoryContentVideo> for RTDInputStoryContentVideoBuilder {
  fn as_ref(&self) -> &InputStoryContentVideo {
    &self.inner
  }
}
