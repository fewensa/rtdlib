use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a media from a link preview album
pub trait TDLinkPreviewAlbumMedia: Debug + RObject {}

/// Describes a media from a link preview album
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum LinkPreviewAlbumMedia {
  #[doc(hidden)]
  _Default(()),
  /// The media is a photo
  Photo(LinkPreviewAlbumMediaPhoto),
  /// The media is a video
  Video(LinkPreviewAlbumMediaVideo),
}

impl Default for LinkPreviewAlbumMedia {
  fn default() -> Self {
    LinkPreviewAlbumMedia::_Default(())
  }
}

impl<'de> Deserialize<'de> for LinkPreviewAlbumMedia {
  fn deserialize<D>(deserializer: D) -> Result<LinkPreviewAlbumMedia, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      LinkPreviewAlbumMedia,
      (linkPreviewAlbumMediaPhoto, Photo);
      (linkPreviewAlbumMediaVideo, Video);

    )(deserializer)
  }
}

impl RObject for LinkPreviewAlbumMedia {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      LinkPreviewAlbumMedia::Photo(t) => t.td_name(),
      LinkPreviewAlbumMedia::Video(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      LinkPreviewAlbumMedia::Photo(t) => t.extra(),
      LinkPreviewAlbumMedia::Video(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl LinkPreviewAlbumMedia {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, LinkPreviewAlbumMedia::_Default(_))
  }

  pub fn is_photo(&self) -> bool {
    matches!(self, LinkPreviewAlbumMedia::Photo(_))
  }
  pub fn is_video(&self) -> bool {
    matches!(self, LinkPreviewAlbumMedia::Video(_))
  }

  pub fn on_photo<F: FnOnce(&LinkPreviewAlbumMediaPhoto)>(&self, fnc: F) -> &Self {
    if let LinkPreviewAlbumMedia::Photo(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_video<F: FnOnce(&LinkPreviewAlbumMediaVideo)>(&self, fnc: F) -> &Self {
    if let LinkPreviewAlbumMedia::Video(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_photo(&self) -> Option<&LinkPreviewAlbumMediaPhoto> {
    if let LinkPreviewAlbumMedia::Photo(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_video(&self) -> Option<&LinkPreviewAlbumMediaVideo> {
    if let LinkPreviewAlbumMedia::Video(t) = self {
      return Some(t);
    }
    None
  }

  pub fn photo<T: AsRef<LinkPreviewAlbumMediaPhoto>>(t: T) -> Self {
    LinkPreviewAlbumMedia::Photo(t.as_ref().clone())
  }

  pub fn video<T: AsRef<LinkPreviewAlbumMediaVideo>>(t: T) -> Self {
    LinkPreviewAlbumMedia::Video(t.as_ref().clone())
  }
}

impl AsRef<LinkPreviewAlbumMedia> for LinkPreviewAlbumMedia {
  fn as_ref(&self) -> &LinkPreviewAlbumMedia {
    self
  }
}

/// The media is a photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewAlbumMediaPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Photo description
  photo: Photo,
}

impl RObject for LinkPreviewAlbumMediaPhoto {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewAlbumMediaPhoto"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewAlbumMedia for LinkPreviewAlbumMediaPhoto {}

impl LinkPreviewAlbumMediaPhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewAlbumMediaPhotoBuilder {
    let inner = LinkPreviewAlbumMediaPhoto {
      td_name: "linkPreviewAlbumMediaPhoto".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewAlbumMediaPhotoBuilder { inner }
  }

  pub fn photo(&self) -> &Photo {
    &self.photo
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewAlbumMediaPhotoBuilder {
  inner: LinkPreviewAlbumMediaPhoto,
}

impl RTDLinkPreviewAlbumMediaPhotoBuilder {
  pub fn build(&self) -> LinkPreviewAlbumMediaPhoto {
    self.inner.clone()
  }

  pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = photo.as_ref().clone();
    self
  }
}

impl AsRef<LinkPreviewAlbumMediaPhoto> for LinkPreviewAlbumMediaPhoto {
  fn as_ref(&self) -> &LinkPreviewAlbumMediaPhoto {
    self
  }
}

impl AsRef<LinkPreviewAlbumMediaPhoto> for RTDLinkPreviewAlbumMediaPhotoBuilder {
  fn as_ref(&self) -> &LinkPreviewAlbumMediaPhoto {
    &self.inner
  }
}

/// The media is a video
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewAlbumMediaVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Video description
  video: Video,
}

impl RObject for LinkPreviewAlbumMediaVideo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewAlbumMediaVideo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewAlbumMedia for LinkPreviewAlbumMediaVideo {}

impl LinkPreviewAlbumMediaVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewAlbumMediaVideoBuilder {
    let inner = LinkPreviewAlbumMediaVideo {
      td_name: "linkPreviewAlbumMediaVideo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewAlbumMediaVideoBuilder { inner }
  }

  pub fn video(&self) -> &Video {
    &self.video
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewAlbumMediaVideoBuilder {
  inner: LinkPreviewAlbumMediaVideo,
}

impl RTDLinkPreviewAlbumMediaVideoBuilder {
  pub fn build(&self) -> LinkPreviewAlbumMediaVideo {
    self.inner.clone()
  }

  pub fn video<T: AsRef<Video>>(&mut self, video: T) -> &mut Self {
    self.inner.video = video.as_ref().clone();
    self
  }
}

impl AsRef<LinkPreviewAlbumMediaVideo> for LinkPreviewAlbumMediaVideo {
  fn as_ref(&self) -> &LinkPreviewAlbumMediaVideo {
    self
  }
}

impl AsRef<LinkPreviewAlbumMediaVideo> for RTDLinkPreviewAlbumMediaVideoBuilder {
  fn as_ref(&self) -> &LinkPreviewAlbumMediaVideo {
    &self.inner
  }
}
