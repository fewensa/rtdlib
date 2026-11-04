use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains the content of a story
pub trait TDStoryContent: Debug + RObject {}

/// Contains the content of a story
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum StoryContent {
  #[doc(hidden)]
  _Default(()),
  /// A live story
  Live(StoryContentLive),
  /// A photo story
  Photo(StoryContentPhoto),
  /// A story content that is not supported in the current TDLib version
  Unsupported(StoryContentUnsupported),
  /// A video story
  Video(StoryContentVideo),
}

impl Default for StoryContent {
  fn default() -> Self {
    StoryContent::_Default(())
  }
}

impl<'de> Deserialize<'de> for StoryContent {
  fn deserialize<D>(deserializer: D) -> Result<StoryContent, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      StoryContent,
      (storyContentLive, Live);
      (storyContentPhoto, Photo);
      (storyContentUnsupported, Unsupported);
      (storyContentVideo, Video);

    )(deserializer)
  }
}

impl RObject for StoryContent {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      StoryContent::Live(t) => t.td_name(),
      StoryContent::Photo(t) => t.td_name(),
      StoryContent::Unsupported(t) => t.td_name(),
      StoryContent::Video(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      StoryContent::Live(t) => t.extra(),
      StoryContent::Photo(t) => t.extra(),
      StoryContent::Unsupported(t) => t.extra(),
      StoryContent::Video(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StoryContent {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, StoryContent::_Default(_))
  }

  pub fn is_live(&self) -> bool {
    matches!(self, StoryContent::Live(_))
  }
  pub fn is_photo(&self) -> bool {
    matches!(self, StoryContent::Photo(_))
  }
  pub fn is_unsupported(&self) -> bool {
    matches!(self, StoryContent::Unsupported(_))
  }
  pub fn is_video(&self) -> bool {
    matches!(self, StoryContent::Video(_))
  }

  pub fn on_live<F: FnOnce(&StoryContentLive)>(&self, fnc: F) -> &Self {
    if let StoryContent::Live(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_photo<F: FnOnce(&StoryContentPhoto)>(&self, fnc: F) -> &Self {
    if let StoryContent::Photo(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_unsupported<F: FnOnce(&StoryContentUnsupported)>(&self, fnc: F) -> &Self {
    if let StoryContent::Unsupported(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_video<F: FnOnce(&StoryContentVideo)>(&self, fnc: F) -> &Self {
    if let StoryContent::Video(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_live(&self) -> Option<&StoryContentLive> {
    if let StoryContent::Live(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_photo(&self) -> Option<&StoryContentPhoto> {
    if let StoryContent::Photo(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_unsupported(&self) -> Option<&StoryContentUnsupported> {
    if let StoryContent::Unsupported(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_video(&self) -> Option<&StoryContentVideo> {
    if let StoryContent::Video(t) = self {
      return Some(t);
    }
    None
  }

  pub fn live<T: AsRef<StoryContentLive>>(t: T) -> Self {
    StoryContent::Live(t.as_ref().clone())
  }

  pub fn photo<T: AsRef<StoryContentPhoto>>(t: T) -> Self {
    StoryContent::Photo(t.as_ref().clone())
  }

  pub fn unsupported<T: AsRef<StoryContentUnsupported>>(t: T) -> Self {
    StoryContent::Unsupported(t.as_ref().clone())
  }

  pub fn video<T: AsRef<StoryContentVideo>>(t: T) -> Self {
    StoryContent::Video(t.as_ref().clone())
  }
}

impl AsRef<StoryContent> for StoryContent {
  fn as_ref(&self) -> &StoryContent {
    self
  }
}

/// A live story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryContentLive {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the corresponding group call. The group call can be received through the method getGroupCall
  group_call_id: i64,
  /// True, if the call is an RTMP stream instead of an ordinary group call
  is_rtmp_stream: bool,
}

impl RObject for StoryContentLive {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyContentLive"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoryContent for StoryContentLive {}

impl StoryContentLive {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryContentLiveBuilder {
    let inner = StoryContentLive {
      td_name: "storyContentLive".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryContentLiveBuilder { inner }
  }

  pub fn group_call_id(&self) -> i64 {
    self.group_call_id
  }

  pub fn is_rtmp_stream(&self) -> bool {
    self.is_rtmp_stream
  }
}

#[doc(hidden)]
pub struct RTDStoryContentLiveBuilder {
  inner: StoryContentLive,
}

impl RTDStoryContentLiveBuilder {
  pub fn build(&self) -> StoryContentLive {
    self.inner.clone()
  }

  pub fn group_call_id(&mut self, group_call_id: i64) -> &mut Self {
    self.inner.group_call_id = group_call_id;
    self
  }

  pub fn is_rtmp_stream(&mut self, is_rtmp_stream: bool) -> &mut Self {
    self.inner.is_rtmp_stream = is_rtmp_stream;
    self
  }
}

impl AsRef<StoryContentLive> for StoryContentLive {
  fn as_ref(&self) -> &StoryContentLive {
    self
  }
}

impl AsRef<StoryContentLive> for RTDStoryContentLiveBuilder {
  fn as_ref(&self) -> &StoryContentLive {
    &self.inner
  }
}

/// A photo story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryContentPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The photo
  photo: Photo,
}

impl RObject for StoryContentPhoto {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyContentPhoto"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoryContent for StoryContentPhoto {}

impl StoryContentPhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryContentPhotoBuilder {
    let inner = StoryContentPhoto {
      td_name: "storyContentPhoto".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryContentPhotoBuilder { inner }
  }

  pub fn photo(&self) -> &Photo {
    &self.photo
  }
}

#[doc(hidden)]
pub struct RTDStoryContentPhotoBuilder {
  inner: StoryContentPhoto,
}

impl RTDStoryContentPhotoBuilder {
  pub fn build(&self) -> StoryContentPhoto {
    self.inner.clone()
  }

  pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = photo.as_ref().clone();
    self
  }
}

impl AsRef<StoryContentPhoto> for StoryContentPhoto {
  fn as_ref(&self) -> &StoryContentPhoto {
    self
  }
}

impl AsRef<StoryContentPhoto> for RTDStoryContentPhotoBuilder {
  fn as_ref(&self) -> &StoryContentPhoto {
    &self.inner
  }
}

/// A story content that is not supported in the current TDLib version
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryContentUnsupported {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for StoryContentUnsupported {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyContentUnsupported"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoryContent for StoryContentUnsupported {}

impl StoryContentUnsupported {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryContentUnsupportedBuilder {
    let inner = StoryContentUnsupported {
      td_name: "storyContentUnsupported".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDStoryContentUnsupportedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDStoryContentUnsupportedBuilder {
  inner: StoryContentUnsupported,
}

impl RTDStoryContentUnsupportedBuilder {
  pub fn build(&self) -> StoryContentUnsupported {
    self.inner.clone()
  }
}

impl AsRef<StoryContentUnsupported> for StoryContentUnsupported {
  fn as_ref(&self) -> &StoryContentUnsupported {
    self
  }
}

impl AsRef<StoryContentUnsupported> for RTDStoryContentUnsupportedBuilder {
  fn as_ref(&self) -> &StoryContentUnsupported {
    &self.inner
  }
}

/// A video story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryContentVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The video in MPEG4 format
  video: StoryVideo,
  /// Alternative version of the video in MPEG4 format, encoded with H.264 codec; may be null
  alternative_video: Option<StoryVideo>,
}

impl RObject for StoryContentVideo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyContentVideo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoryContent for StoryContentVideo {}

impl StoryContentVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryContentVideoBuilder {
    let inner = StoryContentVideo {
      td_name: "storyContentVideo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryContentVideoBuilder { inner }
  }

  pub fn video(&self) -> &StoryVideo {
    &self.video
  }

  pub fn alternative_video(&self) -> &Option<StoryVideo> {
    &self.alternative_video
  }
}

#[doc(hidden)]
pub struct RTDStoryContentVideoBuilder {
  inner: StoryContentVideo,
}

impl RTDStoryContentVideoBuilder {
  pub fn build(&self) -> StoryContentVideo {
    self.inner.clone()
  }

  pub fn video<T: AsRef<StoryVideo>>(&mut self, video: T) -> &mut Self {
    self.inner.video = video.as_ref().clone();
    self
  }

  pub fn alternative_video<T: AsRef<StoryVideo>>(&mut self, alternative_video: T) -> &mut Self {
    self.inner.alternative_video = Some(alternative_video.as_ref().clone());
    self
  }
}

impl AsRef<StoryContentVideo> for StoryContentVideo {
  fn as_ref(&self) -> &StoryContentVideo {
    self
  }
}

impl AsRef<StoryContentVideo> for RTDStoryContentVideoBuilder {
  fn as_ref(&self) -> &StoryContentVideo {
    &self.inner
  }
}
