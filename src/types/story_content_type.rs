use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains the type of the content of a story
pub trait TDStoryContentType: Debug + RObject {}

/// Contains the type of the content of a story
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum StoryContentType {
  #[doc(hidden)]
  _Default(()),
  /// A live story
  Live(StoryContentTypeLive),
  /// A photo story
  Photo(StoryContentTypePhoto),
  /// A story of unknown content type
  Unsupported(StoryContentTypeUnsupported),
  /// A video story
  Video(StoryContentTypeVideo),
}

impl Default for StoryContentType {
  fn default() -> Self {
    StoryContentType::_Default(())
  }
}

impl<'de> Deserialize<'de> for StoryContentType {
  fn deserialize<D>(deserializer: D) -> Result<StoryContentType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      StoryContentType,
      (storyContentTypeLive, Live);
      (storyContentTypePhoto, Photo);
      (storyContentTypeUnsupported, Unsupported);
      (storyContentTypeVideo, Video);

    )(deserializer)
  }
}

impl RObject for StoryContentType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      StoryContentType::Live(t) => t.td_name(),
      StoryContentType::Photo(t) => t.td_name(),
      StoryContentType::Unsupported(t) => t.td_name(),
      StoryContentType::Video(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      StoryContentType::Live(t) => t.extra(),
      StoryContentType::Photo(t) => t.extra(),
      StoryContentType::Unsupported(t) => t.extra(),
      StoryContentType::Video(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StoryContentType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, StoryContentType::_Default(_))
  }

  pub fn is_live(&self) -> bool {
    matches!(self, StoryContentType::Live(_))
  }
  pub fn is_photo(&self) -> bool {
    matches!(self, StoryContentType::Photo(_))
  }
  pub fn is_unsupported(&self) -> bool {
    matches!(self, StoryContentType::Unsupported(_))
  }
  pub fn is_video(&self) -> bool {
    matches!(self, StoryContentType::Video(_))
  }

  pub fn on_live<F: FnOnce(&StoryContentTypeLive)>(&self, fnc: F) -> &Self {
    if let StoryContentType::Live(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_photo<F: FnOnce(&StoryContentTypePhoto)>(&self, fnc: F) -> &Self {
    if let StoryContentType::Photo(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_unsupported<F: FnOnce(&StoryContentTypeUnsupported)>(&self, fnc: F) -> &Self {
    if let StoryContentType::Unsupported(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_video<F: FnOnce(&StoryContentTypeVideo)>(&self, fnc: F) -> &Self {
    if let StoryContentType::Video(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_live(&self) -> Option<&StoryContentTypeLive> {
    if let StoryContentType::Live(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_photo(&self) -> Option<&StoryContentTypePhoto> {
    if let StoryContentType::Photo(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_unsupported(&self) -> Option<&StoryContentTypeUnsupported> {
    if let StoryContentType::Unsupported(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_video(&self) -> Option<&StoryContentTypeVideo> {
    if let StoryContentType::Video(t) = self {
      return Some(t);
    }
    None
  }

  pub fn live<T: AsRef<StoryContentTypeLive>>(t: T) -> Self {
    StoryContentType::Live(t.as_ref().clone())
  }

  pub fn photo<T: AsRef<StoryContentTypePhoto>>(t: T) -> Self {
    StoryContentType::Photo(t.as_ref().clone())
  }

  pub fn unsupported<T: AsRef<StoryContentTypeUnsupported>>(t: T) -> Self {
    StoryContentType::Unsupported(t.as_ref().clone())
  }

  pub fn video<T: AsRef<StoryContentTypeVideo>>(t: T) -> Self {
    StoryContentType::Video(t.as_ref().clone())
  }
}

impl AsRef<StoryContentType> for StoryContentType {
  fn as_ref(&self) -> &StoryContentType {
    self
  }
}

/// A live story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryContentTypeLive {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for StoryContentTypeLive {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyContentTypeLive"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoryContentType for StoryContentTypeLive {}

impl StoryContentTypeLive {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryContentTypeLiveBuilder {
    let inner = StoryContentTypeLive {
      td_name: "storyContentTypeLive".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDStoryContentTypeLiveBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDStoryContentTypeLiveBuilder {
  inner: StoryContentTypeLive,
}

impl RTDStoryContentTypeLiveBuilder {
  pub fn build(&self) -> StoryContentTypeLive {
    self.inner.clone()
  }
}

impl AsRef<StoryContentTypeLive> for StoryContentTypeLive {
  fn as_ref(&self) -> &StoryContentTypeLive {
    self
  }
}

impl AsRef<StoryContentTypeLive> for RTDStoryContentTypeLiveBuilder {
  fn as_ref(&self) -> &StoryContentTypeLive {
    &self.inner
  }
}

/// A photo story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryContentTypePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for StoryContentTypePhoto {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyContentTypePhoto"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoryContentType for StoryContentTypePhoto {}

impl StoryContentTypePhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryContentTypePhotoBuilder {
    let inner = StoryContentTypePhoto {
      td_name: "storyContentTypePhoto".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDStoryContentTypePhotoBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDStoryContentTypePhotoBuilder {
  inner: StoryContentTypePhoto,
}

impl RTDStoryContentTypePhotoBuilder {
  pub fn build(&self) -> StoryContentTypePhoto {
    self.inner.clone()
  }
}

impl AsRef<StoryContentTypePhoto> for StoryContentTypePhoto {
  fn as_ref(&self) -> &StoryContentTypePhoto {
    self
  }
}

impl AsRef<StoryContentTypePhoto> for RTDStoryContentTypePhotoBuilder {
  fn as_ref(&self) -> &StoryContentTypePhoto {
    &self.inner
  }
}

/// A story of unknown content type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryContentTypeUnsupported {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for StoryContentTypeUnsupported {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyContentTypeUnsupported"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoryContentType for StoryContentTypeUnsupported {}

impl StoryContentTypeUnsupported {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryContentTypeUnsupportedBuilder {
    let inner = StoryContentTypeUnsupported {
      td_name: "storyContentTypeUnsupported".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDStoryContentTypeUnsupportedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDStoryContentTypeUnsupportedBuilder {
  inner: StoryContentTypeUnsupported,
}

impl RTDStoryContentTypeUnsupportedBuilder {
  pub fn build(&self) -> StoryContentTypeUnsupported {
    self.inner.clone()
  }
}

impl AsRef<StoryContentTypeUnsupported> for StoryContentTypeUnsupported {
  fn as_ref(&self) -> &StoryContentTypeUnsupported {
    self
  }
}

impl AsRef<StoryContentTypeUnsupported> for RTDStoryContentTypeUnsupportedBuilder {
  fn as_ref(&self) -> &StoryContentTypeUnsupported {
    &self.inner
  }
}

/// A video story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryContentTypeVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for StoryContentTypeVideo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyContentTypeVideo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoryContentType for StoryContentTypeVideo {}

impl StoryContentTypeVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryContentTypeVideoBuilder {
    let inner = StoryContentTypeVideo {
      td_name: "storyContentTypeVideo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDStoryContentTypeVideoBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDStoryContentTypeVideoBuilder {
  inner: StoryContentTypeVideo,
}

impl RTDStoryContentTypeVideoBuilder {
  pub fn build(&self) -> StoryContentTypeVideo {
    self.inner.clone()
  }
}

impl AsRef<StoryContentTypeVideo> for StoryContentTypeVideo {
  fn as_ref(&self) -> &StoryContentTypeVideo {
    self
  }
}

impl AsRef<StoryContentTypeVideo> for RTDStoryContentTypeVideoBuilder {
  fn as_ref(&self) -> &StoryContentTypeVideo {
    &self.inner
  }
}
