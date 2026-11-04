use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a paid media
pub trait TDPaidMedia: Debug + RObject {}

/// Describes a paid media
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PaidMedia {
  #[doc(hidden)]
  _Default(()),
  /// The media is a photo
  Photo(PaidMediaPhoto),
  /// The media is hidden until the invoice is paid
  Preview(PaidMediaPreview),
  /// The media is unsupported
  Unsupported(PaidMediaUnsupported),
  /// The media is a video
  Video(PaidMediaVideo),
}

impl Default for PaidMedia {
  fn default() -> Self {
    PaidMedia::_Default(())
  }
}

impl<'de> Deserialize<'de> for PaidMedia {
  fn deserialize<D>(deserializer: D) -> Result<PaidMedia, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      PaidMedia,
      (paidMediaPhoto, Photo);
      (paidMediaPreview, Preview);
      (paidMediaUnsupported, Unsupported);
      (paidMediaVideo, Video);

    )(deserializer)
  }
}

impl RObject for PaidMedia {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      PaidMedia::Photo(t) => t.td_name(),
      PaidMedia::Preview(t) => t.td_name(),
      PaidMedia::Unsupported(t) => t.td_name(),
      PaidMedia::Video(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      PaidMedia::Photo(t) => t.extra(),
      PaidMedia::Preview(t) => t.extra(),
      PaidMedia::Unsupported(t) => t.extra(),
      PaidMedia::Video(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PaidMedia {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, PaidMedia::_Default(_))
  }

  pub fn is_photo(&self) -> bool {
    matches!(self, PaidMedia::Photo(_))
  }
  pub fn is_preview(&self) -> bool {
    matches!(self, PaidMedia::Preview(_))
  }
  pub fn is_unsupported(&self) -> bool {
    matches!(self, PaidMedia::Unsupported(_))
  }
  pub fn is_video(&self) -> bool {
    matches!(self, PaidMedia::Video(_))
  }

  pub fn on_photo<F: FnOnce(&PaidMediaPhoto)>(&self, fnc: F) -> &Self {
    if let PaidMedia::Photo(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_preview<F: FnOnce(&PaidMediaPreview)>(&self, fnc: F) -> &Self {
    if let PaidMedia::Preview(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_unsupported<F: FnOnce(&PaidMediaUnsupported)>(&self, fnc: F) -> &Self {
    if let PaidMedia::Unsupported(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_video<F: FnOnce(&PaidMediaVideo)>(&self, fnc: F) -> &Self {
    if let PaidMedia::Video(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_photo(&self) -> Option<&PaidMediaPhoto> {
    if let PaidMedia::Photo(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_preview(&self) -> Option<&PaidMediaPreview> {
    if let PaidMedia::Preview(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_unsupported(&self) -> Option<&PaidMediaUnsupported> {
    if let PaidMedia::Unsupported(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_video(&self) -> Option<&PaidMediaVideo> {
    if let PaidMedia::Video(t) = self {
      return Some(t);
    }
    None
  }

  pub fn photo<T: AsRef<PaidMediaPhoto>>(t: T) -> Self {
    PaidMedia::Photo(t.as_ref().clone())
  }

  pub fn preview<T: AsRef<PaidMediaPreview>>(t: T) -> Self {
    PaidMedia::Preview(t.as_ref().clone())
  }

  pub fn unsupported<T: AsRef<PaidMediaUnsupported>>(t: T) -> Self {
    PaidMedia::Unsupported(t.as_ref().clone())
  }

  pub fn video<T: AsRef<PaidMediaVideo>>(t: T) -> Self {
    PaidMedia::Video(t.as_ref().clone())
  }
}

impl AsRef<PaidMedia> for PaidMedia {
  fn as_ref(&self) -> &PaidMedia {
    self
  }
}

/// The media is a photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaidMediaPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The photo
  photo: Photo,
  /// The video representing the live photo; may be null if the photo is static
  video: Option<Video>,
}

impl RObject for PaidMediaPhoto {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "paidMediaPhoto"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPaidMedia for PaidMediaPhoto {}

impl PaidMediaPhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPaidMediaPhotoBuilder {
    let inner = PaidMediaPhoto {
      td_name: "paidMediaPhoto".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPaidMediaPhotoBuilder { inner }
  }

  pub fn photo(&self) -> &Photo {
    &self.photo
  }

  pub fn video(&self) -> &Option<Video> {
    &self.video
  }
}

#[doc(hidden)]
pub struct RTDPaidMediaPhotoBuilder {
  inner: PaidMediaPhoto,
}

impl RTDPaidMediaPhotoBuilder {
  pub fn build(&self) -> PaidMediaPhoto {
    self.inner.clone()
  }

  pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = photo.as_ref().clone();
    self
  }

  pub fn video<T: AsRef<Video>>(&mut self, video: T) -> &mut Self {
    self.inner.video = Some(video.as_ref().clone());
    self
  }
}

impl AsRef<PaidMediaPhoto> for PaidMediaPhoto {
  fn as_ref(&self) -> &PaidMediaPhoto {
    self
  }
}

impl AsRef<PaidMediaPhoto> for RTDPaidMediaPhotoBuilder {
  fn as_ref(&self) -> &PaidMediaPhoto {
    &self.inner
  }
}

/// The media is hidden until the invoice is paid
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaidMediaPreview {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Media width; 0 if unknown
  width: i64,
  /// Media height; 0 if unknown
  height: i64,
  /// Media duration, in seconds; 0 if unknown
  duration: i64,
  /// Media minithumbnail; may be null
  minithumbnail: Option<Minithumbnail>,
}

impl RObject for PaidMediaPreview {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "paidMediaPreview"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPaidMedia for PaidMediaPreview {}

impl PaidMediaPreview {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPaidMediaPreviewBuilder {
    let inner = PaidMediaPreview {
      td_name: "paidMediaPreview".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPaidMediaPreviewBuilder { inner }
  }

  pub fn width(&self) -> i64 {
    self.width
  }

  pub fn height(&self) -> i64 {
    self.height
  }

  pub fn duration(&self) -> i64 {
    self.duration
  }

  pub fn minithumbnail(&self) -> &Option<Minithumbnail> {
    &self.minithumbnail
  }
}

#[doc(hidden)]
pub struct RTDPaidMediaPreviewBuilder {
  inner: PaidMediaPreview,
}

impl RTDPaidMediaPreviewBuilder {
  pub fn build(&self) -> PaidMediaPreview {
    self.inner.clone()
  }

  pub fn width(&mut self, width: i64) -> &mut Self {
    self.inner.width = width;
    self
  }

  pub fn height(&mut self, height: i64) -> &mut Self {
    self.inner.height = height;
    self
  }

  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }

  pub fn minithumbnail<T: AsRef<Minithumbnail>>(&mut self, minithumbnail: T) -> &mut Self {
    self.inner.minithumbnail = Some(minithumbnail.as_ref().clone());
    self
  }
}

impl AsRef<PaidMediaPreview> for PaidMediaPreview {
  fn as_ref(&self) -> &PaidMediaPreview {
    self
  }
}

impl AsRef<PaidMediaPreview> for RTDPaidMediaPreviewBuilder {
  fn as_ref(&self) -> &PaidMediaPreview {
    &self.inner
  }
}

/// The media is unsupported
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaidMediaUnsupported {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PaidMediaUnsupported {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "paidMediaUnsupported"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPaidMedia for PaidMediaUnsupported {}

impl PaidMediaUnsupported {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPaidMediaUnsupportedBuilder {
    let inner = PaidMediaUnsupported {
      td_name: "paidMediaUnsupported".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPaidMediaUnsupportedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPaidMediaUnsupportedBuilder {
  inner: PaidMediaUnsupported,
}

impl RTDPaidMediaUnsupportedBuilder {
  pub fn build(&self) -> PaidMediaUnsupported {
    self.inner.clone()
  }
}

impl AsRef<PaidMediaUnsupported> for PaidMediaUnsupported {
  fn as_ref(&self) -> &PaidMediaUnsupported {
    self
  }
}

impl AsRef<PaidMediaUnsupported> for RTDPaidMediaUnsupportedBuilder {
  fn as_ref(&self) -> &PaidMediaUnsupported {
    &self.inner
  }
}

/// The media is a video
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaidMediaVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The video
  video: Video,
  /// Cover of the video; may be null if none
  cover: Option<Photo>,
  /// Timestamp from which the video playing must start, in seconds
  start_timestamp: i64,
}

impl RObject for PaidMediaVideo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "paidMediaVideo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPaidMedia for PaidMediaVideo {}

impl PaidMediaVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPaidMediaVideoBuilder {
    let inner = PaidMediaVideo {
      td_name: "paidMediaVideo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPaidMediaVideoBuilder { inner }
  }

  pub fn video(&self) -> &Video {
    &self.video
  }

  pub fn cover(&self) -> &Option<Photo> {
    &self.cover
  }

  pub fn start_timestamp(&self) -> i64 {
    self.start_timestamp
  }
}

#[doc(hidden)]
pub struct RTDPaidMediaVideoBuilder {
  inner: PaidMediaVideo,
}

impl RTDPaidMediaVideoBuilder {
  pub fn build(&self) -> PaidMediaVideo {
    self.inner.clone()
  }

  pub fn video<T: AsRef<Video>>(&mut self, video: T) -> &mut Self {
    self.inner.video = video.as_ref().clone();
    self
  }

  pub fn cover<T: AsRef<Photo>>(&mut self, cover: T) -> &mut Self {
    self.inner.cover = Some(cover.as_ref().clone());
    self
  }

  pub fn start_timestamp(&mut self, start_timestamp: i64) -> &mut Self {
    self.inner.start_timestamp = start_timestamp;
    self
  }
}

impl AsRef<PaidMediaVideo> for PaidMediaVideo {
  fn as_ref(&self) -> &PaidMediaVideo {
    self
  }
}

impl AsRef<PaidMediaVideo> for RTDPaidMediaVideoBuilder {
  fn as_ref(&self) -> &PaidMediaVideo {
    &self.inner
  }
}
