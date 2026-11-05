use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains the media in a poll
pub trait TDPollMedia: Debug + RObject {}

/// Contains the media in a poll
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PollMedia {
  #[doc(hidden)]
  _Default(()),
  /// An animation
  Animation(PollMediaAnimation),
  /// An audio
  Audio(PollMediaAudio),
  /// A document (general file)
  Document(PollMediaDocument),
  /// A link
  Link(PollMediaLink),
  /// A location
  Location(PollMediaLocation),
  /// A photo
  Photo(PollMediaPhoto),
  /// A sticker
  Sticker(PollMediaSticker),
  /// A venue
  Venue(PollMediaVenue),
  /// A video
  Video(PollMediaVideo),
}

impl Default for PollMedia {
  fn default() -> Self {
    PollMedia::_Default(())
  }
}

impl<'de> Deserialize<'de> for PollMedia {
  fn deserialize<D>(deserializer: D) -> Result<PollMedia, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      PollMedia,
      (pollMediaAnimation, Animation);
      (pollMediaAudio, Audio);
      (pollMediaDocument, Document);
      (pollMediaLink, Link);
      (pollMediaLocation, Location);
      (pollMediaPhoto, Photo);
      (pollMediaSticker, Sticker);
      (pollMediaVenue, Venue);
      (pollMediaVideo, Video);

    )(deserializer)
  }
}

impl RObject for PollMedia {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      PollMedia::Animation(t) => t.td_name(),
      PollMedia::Audio(t) => t.td_name(),
      PollMedia::Document(t) => t.td_name(),
      PollMedia::Link(t) => t.td_name(),
      PollMedia::Location(t) => t.td_name(),
      PollMedia::Photo(t) => t.td_name(),
      PollMedia::Sticker(t) => t.td_name(),
      PollMedia::Venue(t) => t.td_name(),
      PollMedia::Video(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      PollMedia::Animation(t) => t.extra(),
      PollMedia::Audio(t) => t.extra(),
      PollMedia::Document(t) => t.extra(),
      PollMedia::Link(t) => t.extra(),
      PollMedia::Location(t) => t.extra(),
      PollMedia::Photo(t) => t.extra(),
      PollMedia::Sticker(t) => t.extra(),
      PollMedia::Venue(t) => t.extra(),
      PollMedia::Video(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PollMedia {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, PollMedia::_Default(_))
  }

  pub fn is_animation(&self) -> bool {
    matches!(self, PollMedia::Animation(_))
  }
  pub fn is_audio(&self) -> bool {
    matches!(self, PollMedia::Audio(_))
  }
  pub fn is_document(&self) -> bool {
    matches!(self, PollMedia::Document(_))
  }
  pub fn is_link(&self) -> bool {
    matches!(self, PollMedia::Link(_))
  }
  pub fn is_location(&self) -> bool {
    matches!(self, PollMedia::Location(_))
  }
  pub fn is_photo(&self) -> bool {
    matches!(self, PollMedia::Photo(_))
  }
  pub fn is_sticker(&self) -> bool {
    matches!(self, PollMedia::Sticker(_))
  }
  pub fn is_venue(&self) -> bool {
    matches!(self, PollMedia::Venue(_))
  }
  pub fn is_video(&self) -> bool {
    matches!(self, PollMedia::Video(_))
  }

  pub fn on_animation<F: FnOnce(&PollMediaAnimation)>(&self, fnc: F) -> &Self {
    if let PollMedia::Animation(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_audio<F: FnOnce(&PollMediaAudio)>(&self, fnc: F) -> &Self {
    if let PollMedia::Audio(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_document<F: FnOnce(&PollMediaDocument)>(&self, fnc: F) -> &Self {
    if let PollMedia::Document(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_link<F: FnOnce(&PollMediaLink)>(&self, fnc: F) -> &Self {
    if let PollMedia::Link(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_location<F: FnOnce(&PollMediaLocation)>(&self, fnc: F) -> &Self {
    if let PollMedia::Location(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_photo<F: FnOnce(&PollMediaPhoto)>(&self, fnc: F) -> &Self {
    if let PollMedia::Photo(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_sticker<F: FnOnce(&PollMediaSticker)>(&self, fnc: F) -> &Self {
    if let PollMedia::Sticker(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_venue<F: FnOnce(&PollMediaVenue)>(&self, fnc: F) -> &Self {
    if let PollMedia::Venue(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_video<F: FnOnce(&PollMediaVideo)>(&self, fnc: F) -> &Self {
    if let PollMedia::Video(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_animation(&self) -> Option<&PollMediaAnimation> {
    if let PollMedia::Animation(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_audio(&self) -> Option<&PollMediaAudio> {
    if let PollMedia::Audio(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_document(&self) -> Option<&PollMediaDocument> {
    if let PollMedia::Document(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_link(&self) -> Option<&PollMediaLink> {
    if let PollMedia::Link(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_location(&self) -> Option<&PollMediaLocation> {
    if let PollMedia::Location(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_photo(&self) -> Option<&PollMediaPhoto> {
    if let PollMedia::Photo(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_sticker(&self) -> Option<&PollMediaSticker> {
    if let PollMedia::Sticker(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_venue(&self) -> Option<&PollMediaVenue> {
    if let PollMedia::Venue(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_video(&self) -> Option<&PollMediaVideo> {
    if let PollMedia::Video(t) = self {
      return Some(t);
    }
    None
  }

  pub fn animation<T: AsRef<PollMediaAnimation>>(t: T) -> Self {
    PollMedia::Animation(t.as_ref().clone())
  }

  pub fn audio<T: AsRef<PollMediaAudio>>(t: T) -> Self {
    PollMedia::Audio(t.as_ref().clone())
  }

  pub fn document<T: AsRef<PollMediaDocument>>(t: T) -> Self {
    PollMedia::Document(t.as_ref().clone())
  }

  pub fn link<T: AsRef<PollMediaLink>>(t: T) -> Self {
    PollMedia::Link(t.as_ref().clone())
  }

  pub fn location<T: AsRef<PollMediaLocation>>(t: T) -> Self {
    PollMedia::Location(t.as_ref().clone())
  }

  pub fn photo<T: AsRef<PollMediaPhoto>>(t: T) -> Self {
    PollMedia::Photo(t.as_ref().clone())
  }

  pub fn sticker<T: AsRef<PollMediaSticker>>(t: T) -> Self {
    PollMedia::Sticker(t.as_ref().clone())
  }

  pub fn venue<T: AsRef<PollMediaVenue>>(t: T) -> Self {
    PollMedia::Venue(t.as_ref().clone())
  }

  pub fn video<T: AsRef<PollMediaVideo>>(t: T) -> Self {
    PollMedia::Video(t.as_ref().clone())
  }
}

impl AsRef<PollMedia> for PollMedia {
  fn as_ref(&self) -> &PollMedia {
    self
  }
}

/// An animation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollMediaAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The animation
  animation: Animation,
}

impl RObject for PollMediaAnimation {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pollMediaAnimation"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPollMedia for PollMediaAnimation {}

impl PollMediaAnimation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPollMediaAnimationBuilder {
    let inner = PollMediaAnimation {
      td_name: "pollMediaAnimation".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPollMediaAnimationBuilder { inner }
  }

  pub fn animation(&self) -> &Animation {
    &self.animation
  }
}

#[doc(hidden)]
pub struct RTDPollMediaAnimationBuilder {
  inner: PollMediaAnimation,
}

impl RTDPollMediaAnimationBuilder {
  pub fn build(&self) -> PollMediaAnimation {
    self.inner.clone()
  }

  pub fn animation<T: AsRef<Animation>>(&mut self, animation: T) -> &mut Self {
    self.inner.animation = animation.as_ref().clone();
    self
  }
}

impl AsRef<PollMediaAnimation> for PollMediaAnimation {
  fn as_ref(&self) -> &PollMediaAnimation {
    self
  }
}

impl AsRef<PollMediaAnimation> for RTDPollMediaAnimationBuilder {
  fn as_ref(&self) -> &PollMediaAnimation {
    &self.inner
  }
}

/// An audio
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollMediaAudio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The audio
  audio: Audio,
}

impl RObject for PollMediaAudio {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pollMediaAudio"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPollMedia for PollMediaAudio {}

impl PollMediaAudio {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPollMediaAudioBuilder {
    let inner = PollMediaAudio {
      td_name: "pollMediaAudio".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPollMediaAudioBuilder { inner }
  }

  pub fn audio(&self) -> &Audio {
    &self.audio
  }
}

#[doc(hidden)]
pub struct RTDPollMediaAudioBuilder {
  inner: PollMediaAudio,
}

impl RTDPollMediaAudioBuilder {
  pub fn build(&self) -> PollMediaAudio {
    self.inner.clone()
  }

  pub fn audio<T: AsRef<Audio>>(&mut self, audio: T) -> &mut Self {
    self.inner.audio = audio.as_ref().clone();
    self
  }
}

impl AsRef<PollMediaAudio> for PollMediaAudio {
  fn as_ref(&self) -> &PollMediaAudio {
    self
  }
}

impl AsRef<PollMediaAudio> for RTDPollMediaAudioBuilder {
  fn as_ref(&self) -> &PollMediaAudio {
    &self.inner
  }
}

/// A document (general file)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollMediaDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The document
  document: Document,
}

impl RObject for PollMediaDocument {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pollMediaDocument"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPollMedia for PollMediaDocument {}

impl PollMediaDocument {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPollMediaDocumentBuilder {
    let inner = PollMediaDocument {
      td_name: "pollMediaDocument".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPollMediaDocumentBuilder { inner }
  }

  pub fn document(&self) -> &Document {
    &self.document
  }
}

#[doc(hidden)]
pub struct RTDPollMediaDocumentBuilder {
  inner: PollMediaDocument,
}

impl RTDPollMediaDocumentBuilder {
  pub fn build(&self) -> PollMediaDocument {
    self.inner.clone()
  }

  pub fn document<T: AsRef<Document>>(&mut self, document: T) -> &mut Self {
    self.inner.document = document.as_ref().clone();
    self
  }
}

impl AsRef<PollMediaDocument> for PollMediaDocument {
  fn as_ref(&self) -> &PollMediaDocument {
    self
  }
}

impl AsRef<PollMediaDocument> for RTDPollMediaDocumentBuilder {
  fn as_ref(&self) -> &PollMediaDocument {
    &self.inner
  }
}

/// A link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollMediaLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// URL of the link
  url: String,
  /// Preview of the link; may be null if unknown
  link_preview: Option<LinkPreview>,
}

impl RObject for PollMediaLink {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pollMediaLink"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPollMedia for PollMediaLink {}

impl PollMediaLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPollMediaLinkBuilder {
    let inner = PollMediaLink {
      td_name: "pollMediaLink".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPollMediaLinkBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }

  pub fn link_preview(&self) -> &Option<LinkPreview> {
    &self.link_preview
  }
}

#[doc(hidden)]
pub struct RTDPollMediaLinkBuilder {
  inner: PollMediaLink,
}

impl RTDPollMediaLinkBuilder {
  pub fn build(&self) -> PollMediaLink {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

  pub fn link_preview<T: AsRef<LinkPreview>>(&mut self, link_preview: T) -> &mut Self {
    self.inner.link_preview = Some(link_preview.as_ref().clone());
    self
  }
}

impl AsRef<PollMediaLink> for PollMediaLink {
  fn as_ref(&self) -> &PollMediaLink {
    self
  }
}

impl AsRef<PollMediaLink> for RTDPollMediaLinkBuilder {
  fn as_ref(&self) -> &PollMediaLink {
    &self.inner
  }
}

/// A location
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollMediaLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The location
  location: Location,
}

impl RObject for PollMediaLocation {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pollMediaLocation"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPollMedia for PollMediaLocation {}

impl PollMediaLocation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPollMediaLocationBuilder {
    let inner = PollMediaLocation {
      td_name: "pollMediaLocation".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPollMediaLocationBuilder { inner }
  }

  pub fn location(&self) -> &Location {
    &self.location
  }
}

#[doc(hidden)]
pub struct RTDPollMediaLocationBuilder {
  inner: PollMediaLocation,
}

impl RTDPollMediaLocationBuilder {
  pub fn build(&self) -> PollMediaLocation {
    self.inner.clone()
  }

  pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
    self.inner.location = location.as_ref().clone();
    self
  }
}

impl AsRef<PollMediaLocation> for PollMediaLocation {
  fn as_ref(&self) -> &PollMediaLocation {
    self
  }
}

impl AsRef<PollMediaLocation> for RTDPollMediaLocationBuilder {
  fn as_ref(&self) -> &PollMediaLocation {
    &self.inner
  }
}

/// A photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollMediaPhoto {
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

impl RObject for PollMediaPhoto {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pollMediaPhoto"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPollMedia for PollMediaPhoto {}

impl PollMediaPhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPollMediaPhotoBuilder {
    let inner = PollMediaPhoto {
      td_name: "pollMediaPhoto".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPollMediaPhotoBuilder { inner }
  }

  pub fn photo(&self) -> &Photo {
    &self.photo
  }

  pub fn video(&self) -> &Option<Video> {
    &self.video
  }
}

#[doc(hidden)]
pub struct RTDPollMediaPhotoBuilder {
  inner: PollMediaPhoto,
}

impl RTDPollMediaPhotoBuilder {
  pub fn build(&self) -> PollMediaPhoto {
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

impl AsRef<PollMediaPhoto> for PollMediaPhoto {
  fn as_ref(&self) -> &PollMediaPhoto {
    self
  }
}

impl AsRef<PollMediaPhoto> for RTDPollMediaPhotoBuilder {
  fn as_ref(&self) -> &PollMediaPhoto {
    &self.inner
  }
}

/// A sticker
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollMediaSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The sticker
  sticker: Sticker,
}

impl RObject for PollMediaSticker {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pollMediaSticker"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPollMedia for PollMediaSticker {}

impl PollMediaSticker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPollMediaStickerBuilder {
    let inner = PollMediaSticker {
      td_name: "pollMediaSticker".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPollMediaStickerBuilder { inner }
  }

  pub fn sticker(&self) -> &Sticker {
    &self.sticker
  }
}

#[doc(hidden)]
pub struct RTDPollMediaStickerBuilder {
  inner: PollMediaSticker,
}

impl RTDPollMediaStickerBuilder {
  pub fn build(&self) -> PollMediaSticker {
    self.inner.clone()
  }

  pub fn sticker<T: AsRef<Sticker>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
    self
  }
}

impl AsRef<PollMediaSticker> for PollMediaSticker {
  fn as_ref(&self) -> &PollMediaSticker {
    self
  }
}

impl AsRef<PollMediaSticker> for RTDPollMediaStickerBuilder {
  fn as_ref(&self) -> &PollMediaSticker {
    &self.inner
  }
}

/// A venue
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollMediaVenue {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The venue
  venue: Venue,
}

impl RObject for PollMediaVenue {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pollMediaVenue"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPollMedia for PollMediaVenue {}

impl PollMediaVenue {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPollMediaVenueBuilder {
    let inner = PollMediaVenue {
      td_name: "pollMediaVenue".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPollMediaVenueBuilder { inner }
  }

  pub fn venue(&self) -> &Venue {
    &self.venue
  }
}

#[doc(hidden)]
pub struct RTDPollMediaVenueBuilder {
  inner: PollMediaVenue,
}

impl RTDPollMediaVenueBuilder {
  pub fn build(&self) -> PollMediaVenue {
    self.inner.clone()
  }

  pub fn venue<T: AsRef<Venue>>(&mut self, venue: T) -> &mut Self {
    self.inner.venue = venue.as_ref().clone();
    self
  }
}

impl AsRef<PollMediaVenue> for PollMediaVenue {
  fn as_ref(&self) -> &PollMediaVenue {
    self
  }
}

impl AsRef<PollMediaVenue> for RTDPollMediaVenueBuilder {
  fn as_ref(&self) -> &PollMediaVenue {
    &self.inner
  }
}

/// A video
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollMediaVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The video description
  video: Video,
  /// Alternative qualities of the video
  alternative_videos: Vec<AlternativeVideo>,
  /// Available storyboards for the video
  storyboards: Vec<VideoStoryboard>,
  /// Cover of the video; may be null if none
  cover: Option<Photo>,
  /// Timestamp from which the video playing must start, in seconds
  start_timestamp: i64,
}

impl RObject for PollMediaVideo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pollMediaVideo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPollMedia for PollMediaVideo {}

impl PollMediaVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPollMediaVideoBuilder {
    let inner = PollMediaVideo {
      td_name: "pollMediaVideo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPollMediaVideoBuilder { inner }
  }

  pub fn video(&self) -> &Video {
    &self.video
  }

  pub fn alternative_videos(&self) -> &Vec<AlternativeVideo> {
    &self.alternative_videos
  }

  pub fn storyboards(&self) -> &Vec<VideoStoryboard> {
    &self.storyboards
  }

  pub fn cover(&self) -> &Option<Photo> {
    &self.cover
  }

  pub fn start_timestamp(&self) -> i64 {
    self.start_timestamp
  }
}

#[doc(hidden)]
pub struct RTDPollMediaVideoBuilder {
  inner: PollMediaVideo,
}

impl RTDPollMediaVideoBuilder {
  pub fn build(&self) -> PollMediaVideo {
    self.inner.clone()
  }

  pub fn video<T: AsRef<Video>>(&mut self, video: T) -> &mut Self {
    self.inner.video = video.as_ref().clone();
    self
  }

  pub fn alternative_videos(&mut self, alternative_videos: Vec<AlternativeVideo>) -> &mut Self {
    self.inner.alternative_videos = alternative_videos;
    self
  }

  pub fn storyboards(&mut self, storyboards: Vec<VideoStoryboard>) -> &mut Self {
    self.inner.storyboards = storyboards;
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

impl AsRef<PollMediaVideo> for PollMediaVideo {
  fn as_ref(&self) -> &PollMediaVideo {
    self
  }
}

impl AsRef<PollMediaVideo> for RTDPollMediaVideoBuilder {
  fn as_ref(&self) -> &PollMediaVideo {
    &self.inner
  }
}
