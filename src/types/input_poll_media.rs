use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | The content of a poll media to send
pub trait TDInputPollMedia: Debug + RObject {}

/// The content of a poll media to send
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputPollMedia {
  #[doc(hidden)]
  _Default(()),
  /// An animation
  Animation(InputPollMediaAnimation),
  /// An audio
  Audio(InputPollMediaAudio),
  /// A document (general file)
  Document(InputPollMediaDocument),
  /// A link
  Link(InputPollMediaLink),
  /// A location
  Location(InputPollMediaLocation),
  /// A photo
  Photo(InputPollMediaPhoto),
  /// A sticker
  Sticker(InputPollMediaSticker),
  /// A venue
  Venue(InputPollMediaVenue),
  /// A video
  Video(InputPollMediaVideo),
}

impl Default for InputPollMedia {
  fn default() -> Self {
    InputPollMedia::_Default(())
  }
}

impl<'de> Deserialize<'de> for InputPollMedia {
  fn deserialize<D>(deserializer: D) -> Result<InputPollMedia, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InputPollMedia,
      (inputPollMediaAnimation, Animation);
      (inputPollMediaAudio, Audio);
      (inputPollMediaDocument, Document);
      (inputPollMediaLink, Link);
      (inputPollMediaLocation, Location);
      (inputPollMediaPhoto, Photo);
      (inputPollMediaSticker, Sticker);
      (inputPollMediaVenue, Venue);
      (inputPollMediaVideo, Video);

    )(deserializer)
  }
}

impl RObject for InputPollMedia {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      InputPollMedia::Animation(t) => t.td_name(),
      InputPollMedia::Audio(t) => t.td_name(),
      InputPollMedia::Document(t) => t.td_name(),
      InputPollMedia::Link(t) => t.td_name(),
      InputPollMedia::Location(t) => t.td_name(),
      InputPollMedia::Photo(t) => t.td_name(),
      InputPollMedia::Sticker(t) => t.td_name(),
      InputPollMedia::Venue(t) => t.td_name(),
      InputPollMedia::Video(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      InputPollMedia::Animation(t) => t.extra(),
      InputPollMedia::Audio(t) => t.extra(),
      InputPollMedia::Document(t) => t.extra(),
      InputPollMedia::Link(t) => t.extra(),
      InputPollMedia::Location(t) => t.extra(),
      InputPollMedia::Photo(t) => t.extra(),
      InputPollMedia::Sticker(t) => t.extra(),
      InputPollMedia::Venue(t) => t.extra(),
      InputPollMedia::Video(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputPollMedia {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, InputPollMedia::_Default(_))
  }

  pub fn is_animation(&self) -> bool {
    matches!(self, InputPollMedia::Animation(_))
  }
  pub fn is_audio(&self) -> bool {
    matches!(self, InputPollMedia::Audio(_))
  }
  pub fn is_document(&self) -> bool {
    matches!(self, InputPollMedia::Document(_))
  }
  pub fn is_link(&self) -> bool {
    matches!(self, InputPollMedia::Link(_))
  }
  pub fn is_location(&self) -> bool {
    matches!(self, InputPollMedia::Location(_))
  }
  pub fn is_photo(&self) -> bool {
    matches!(self, InputPollMedia::Photo(_))
  }
  pub fn is_sticker(&self) -> bool {
    matches!(self, InputPollMedia::Sticker(_))
  }
  pub fn is_venue(&self) -> bool {
    matches!(self, InputPollMedia::Venue(_))
  }
  pub fn is_video(&self) -> bool {
    matches!(self, InputPollMedia::Video(_))
  }

  pub fn on_animation<F: FnOnce(&InputPollMediaAnimation)>(&self, fnc: F) -> &Self {
    if let InputPollMedia::Animation(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_audio<F: FnOnce(&InputPollMediaAudio)>(&self, fnc: F) -> &Self {
    if let InputPollMedia::Audio(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_document<F: FnOnce(&InputPollMediaDocument)>(&self, fnc: F) -> &Self {
    if let InputPollMedia::Document(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_link<F: FnOnce(&InputPollMediaLink)>(&self, fnc: F) -> &Self {
    if let InputPollMedia::Link(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_location<F: FnOnce(&InputPollMediaLocation)>(&self, fnc: F) -> &Self {
    if let InputPollMedia::Location(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_photo<F: FnOnce(&InputPollMediaPhoto)>(&self, fnc: F) -> &Self {
    if let InputPollMedia::Photo(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_sticker<F: FnOnce(&InputPollMediaSticker)>(&self, fnc: F) -> &Self {
    if let InputPollMedia::Sticker(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_venue<F: FnOnce(&InputPollMediaVenue)>(&self, fnc: F) -> &Self {
    if let InputPollMedia::Venue(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_video<F: FnOnce(&InputPollMediaVideo)>(&self, fnc: F) -> &Self {
    if let InputPollMedia::Video(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_animation(&self) -> Option<&InputPollMediaAnimation> {
    if let InputPollMedia::Animation(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_audio(&self) -> Option<&InputPollMediaAudio> {
    if let InputPollMedia::Audio(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_document(&self) -> Option<&InputPollMediaDocument> {
    if let InputPollMedia::Document(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_link(&self) -> Option<&InputPollMediaLink> {
    if let InputPollMedia::Link(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_location(&self) -> Option<&InputPollMediaLocation> {
    if let InputPollMedia::Location(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_photo(&self) -> Option<&InputPollMediaPhoto> {
    if let InputPollMedia::Photo(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_sticker(&self) -> Option<&InputPollMediaSticker> {
    if let InputPollMedia::Sticker(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_venue(&self) -> Option<&InputPollMediaVenue> {
    if let InputPollMedia::Venue(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_video(&self) -> Option<&InputPollMediaVideo> {
    if let InputPollMedia::Video(t) = self {
      return Some(t);
    }
    None
  }

  pub fn animation<T: AsRef<InputPollMediaAnimation>>(t: T) -> Self {
    InputPollMedia::Animation(t.as_ref().clone())
  }

  pub fn audio<T: AsRef<InputPollMediaAudio>>(t: T) -> Self {
    InputPollMedia::Audio(t.as_ref().clone())
  }

  pub fn document<T: AsRef<InputPollMediaDocument>>(t: T) -> Self {
    InputPollMedia::Document(t.as_ref().clone())
  }

  pub fn link<T: AsRef<InputPollMediaLink>>(t: T) -> Self {
    InputPollMedia::Link(t.as_ref().clone())
  }

  pub fn location<T: AsRef<InputPollMediaLocation>>(t: T) -> Self {
    InputPollMedia::Location(t.as_ref().clone())
  }

  pub fn photo<T: AsRef<InputPollMediaPhoto>>(t: T) -> Self {
    InputPollMedia::Photo(t.as_ref().clone())
  }

  pub fn sticker<T: AsRef<InputPollMediaSticker>>(t: T) -> Self {
    InputPollMedia::Sticker(t.as_ref().clone())
  }

  pub fn venue<T: AsRef<InputPollMediaVenue>>(t: T) -> Self {
    InputPollMedia::Venue(t.as_ref().clone())
  }

  pub fn video<T: AsRef<InputPollMediaVideo>>(t: T) -> Self {
    InputPollMedia::Video(t.as_ref().clone())
  }
}

impl AsRef<InputPollMedia> for InputPollMedia {
  fn as_ref(&self) -> &InputPollMedia {
    self
  }
}

/// An animation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPollMediaAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The animation to be sent
  animation: InputAnimation,
}

impl RObject for InputPollMediaAnimation {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPollMediaAnimation"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPollMedia for InputPollMediaAnimation {}

impl InputPollMediaAnimation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPollMediaAnimationBuilder {
    let inner = InputPollMediaAnimation {
      td_name: "inputPollMediaAnimation".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPollMediaAnimationBuilder { inner }
  }

  pub fn animation(&self) -> &InputAnimation {
    &self.animation
  }
}

#[doc(hidden)]
pub struct RTDInputPollMediaAnimationBuilder {
  inner: InputPollMediaAnimation,
}

impl RTDInputPollMediaAnimationBuilder {
  pub fn build(&self) -> InputPollMediaAnimation {
    self.inner.clone()
  }

  pub fn animation<T: AsRef<InputAnimation>>(&mut self, animation: T) -> &mut Self {
    self.inner.animation = animation.as_ref().clone();
    self
  }
}

impl AsRef<InputPollMediaAnimation> for InputPollMediaAnimation {
  fn as_ref(&self) -> &InputPollMediaAnimation {
    self
  }
}

impl AsRef<InputPollMediaAnimation> for RTDInputPollMediaAnimationBuilder {
  fn as_ref(&self) -> &InputPollMediaAnimation {
    &self.inner
  }
}

/// An audio
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPollMediaAudio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The audio to be sent
  audio: InputAudio,
}

impl RObject for InputPollMediaAudio {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPollMediaAudio"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPollMedia for InputPollMediaAudio {}

impl InputPollMediaAudio {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPollMediaAudioBuilder {
    let inner = InputPollMediaAudio {
      td_name: "inputPollMediaAudio".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPollMediaAudioBuilder { inner }
  }

  pub fn audio(&self) -> &InputAudio {
    &self.audio
  }
}

#[doc(hidden)]
pub struct RTDInputPollMediaAudioBuilder {
  inner: InputPollMediaAudio,
}

impl RTDInputPollMediaAudioBuilder {
  pub fn build(&self) -> InputPollMediaAudio {
    self.inner.clone()
  }

  pub fn audio<T: AsRef<InputAudio>>(&mut self, audio: T) -> &mut Self {
    self.inner.audio = audio.as_ref().clone();
    self
  }
}

impl AsRef<InputPollMediaAudio> for InputPollMediaAudio {
  fn as_ref(&self) -> &InputPollMediaAudio {
    self
  }
}

impl AsRef<InputPollMediaAudio> for RTDInputPollMediaAudioBuilder {
  fn as_ref(&self) -> &InputPollMediaAudio {
    &self.inner
  }
}

/// A document (general file)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPollMediaDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The document to be sent
  document: InputDocument,
}

impl RObject for InputPollMediaDocument {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPollMediaDocument"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPollMedia for InputPollMediaDocument {}

impl InputPollMediaDocument {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPollMediaDocumentBuilder {
    let inner = InputPollMediaDocument {
      td_name: "inputPollMediaDocument".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPollMediaDocumentBuilder { inner }
  }

  pub fn document(&self) -> &InputDocument {
    &self.document
  }
}

#[doc(hidden)]
pub struct RTDInputPollMediaDocumentBuilder {
  inner: InputPollMediaDocument,
}

impl RTDInputPollMediaDocumentBuilder {
  pub fn build(&self) -> InputPollMediaDocument {
    self.inner.clone()
  }

  pub fn document<T: AsRef<InputDocument>>(&mut self, document: T) -> &mut Self {
    self.inner.document = document.as_ref().clone();
    self
  }
}

impl AsRef<InputPollMediaDocument> for InputPollMediaDocument {
  fn as_ref(&self) -> &InputPollMediaDocument {
    self
  }
}

impl AsRef<InputPollMediaDocument> for RTDInputPollMediaDocumentBuilder {
  fn as_ref(&self) -> &InputPollMediaDocument {
    &self.inner
  }
}

/// A link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPollMediaLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// URL of the link
  url: String,
}

impl RObject for InputPollMediaLink {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPollMediaLink"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPollMedia for InputPollMediaLink {}

impl InputPollMediaLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPollMediaLinkBuilder {
    let inner = InputPollMediaLink {
      td_name: "inputPollMediaLink".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPollMediaLinkBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }
}

#[doc(hidden)]
pub struct RTDInputPollMediaLinkBuilder {
  inner: InputPollMediaLink,
}

impl RTDInputPollMediaLinkBuilder {
  pub fn build(&self) -> InputPollMediaLink {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }
}

impl AsRef<InputPollMediaLink> for InputPollMediaLink {
  fn as_ref(&self) -> &InputPollMediaLink {
    self
  }
}

impl AsRef<InputPollMediaLink> for RTDInputPollMediaLinkBuilder {
  fn as_ref(&self) -> &InputPollMediaLink {
    &self.inner
  }
}

/// A location
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPollMediaLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Location to be sent
  location: Location,
}

impl RObject for InputPollMediaLocation {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPollMediaLocation"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPollMedia for InputPollMediaLocation {}

impl InputPollMediaLocation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPollMediaLocationBuilder {
    let inner = InputPollMediaLocation {
      td_name: "inputPollMediaLocation".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPollMediaLocationBuilder { inner }
  }

  pub fn location(&self) -> &Location {
    &self.location
  }
}

#[doc(hidden)]
pub struct RTDInputPollMediaLocationBuilder {
  inner: InputPollMediaLocation,
}

impl RTDInputPollMediaLocationBuilder {
  pub fn build(&self) -> InputPollMediaLocation {
    self.inner.clone()
  }

  pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
    self.inner.location = location.as_ref().clone();
    self
  }
}

impl AsRef<InputPollMediaLocation> for InputPollMediaLocation {
  fn as_ref(&self) -> &InputPollMediaLocation {
    self
  }
}

impl AsRef<InputPollMediaLocation> for RTDInputPollMediaLocationBuilder {
  fn as_ref(&self) -> &InputPollMediaLocation {
    &self.inner
  }
}

/// A photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPollMediaPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Photo to be sent
  photo: InputPhoto,
}

impl RObject for InputPollMediaPhoto {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPollMediaPhoto"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPollMedia for InputPollMediaPhoto {}

impl InputPollMediaPhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPollMediaPhotoBuilder {
    let inner = InputPollMediaPhoto {
      td_name: "inputPollMediaPhoto".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPollMediaPhotoBuilder { inner }
  }

  pub fn photo(&self) -> &InputPhoto {
    &self.photo
  }
}

#[doc(hidden)]
pub struct RTDInputPollMediaPhotoBuilder {
  inner: InputPollMediaPhoto,
}

impl RTDInputPollMediaPhotoBuilder {
  pub fn build(&self) -> InputPollMediaPhoto {
    self.inner.clone()
  }

  pub fn photo<T: AsRef<InputPhoto>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = photo.as_ref().clone();
    self
  }
}

impl AsRef<InputPollMediaPhoto> for InputPollMediaPhoto {
  fn as_ref(&self) -> &InputPollMediaPhoto {
    self
  }
}

impl AsRef<InputPollMediaPhoto> for RTDInputPollMediaPhotoBuilder {
  fn as_ref(&self) -> &InputPollMediaPhoto {
    &self.inner
  }
}

/// A sticker
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPollMediaSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Sticker to be sent
  sticker: InputSticker,
}

impl RObject for InputPollMediaSticker {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPollMediaSticker"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPollMedia for InputPollMediaSticker {}

impl InputPollMediaSticker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPollMediaStickerBuilder {
    let inner = InputPollMediaSticker {
      td_name: "inputPollMediaSticker".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPollMediaStickerBuilder { inner }
  }

  pub fn sticker(&self) -> &InputSticker {
    &self.sticker
  }
}

#[doc(hidden)]
pub struct RTDInputPollMediaStickerBuilder {
  inner: InputPollMediaSticker,
}

impl RTDInputPollMediaStickerBuilder {
  pub fn build(&self) -> InputPollMediaSticker {
    self.inner.clone()
  }

  pub fn sticker<T: AsRef<InputSticker>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
    self
  }
}

impl AsRef<InputPollMediaSticker> for InputPollMediaSticker {
  fn as_ref(&self) -> &InputPollMediaSticker {
    self
  }
}

impl AsRef<InputPollMediaSticker> for RTDInputPollMediaStickerBuilder {
  fn as_ref(&self) -> &InputPollMediaSticker {
    &self.inner
  }
}

/// A venue
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPollMediaVenue {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Venue to send
  venue: Venue,
}

impl RObject for InputPollMediaVenue {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPollMediaVenue"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPollMedia for InputPollMediaVenue {}

impl InputPollMediaVenue {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPollMediaVenueBuilder {
    let inner = InputPollMediaVenue {
      td_name: "inputPollMediaVenue".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPollMediaVenueBuilder { inner }
  }

  pub fn venue(&self) -> &Venue {
    &self.venue
  }
}

#[doc(hidden)]
pub struct RTDInputPollMediaVenueBuilder {
  inner: InputPollMediaVenue,
}

impl RTDInputPollMediaVenueBuilder {
  pub fn build(&self) -> InputPollMediaVenue {
    self.inner.clone()
  }

  pub fn venue<T: AsRef<Venue>>(&mut self, venue: T) -> &mut Self {
    self.inner.venue = venue.as_ref().clone();
    self
  }
}

impl AsRef<InputPollMediaVenue> for InputPollMediaVenue {
  fn as_ref(&self) -> &InputPollMediaVenue {
    self
  }
}

impl AsRef<InputPollMediaVenue> for RTDInputPollMediaVenueBuilder {
  fn as_ref(&self) -> &InputPollMediaVenue {
    &self.inner
  }
}

/// A video
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPollMediaVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The video to be sent
  video: InputVideo,
}

impl RObject for InputPollMediaVideo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPollMediaVideo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPollMedia for InputPollMediaVideo {}

impl InputPollMediaVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPollMediaVideoBuilder {
    let inner = InputPollMediaVideo {
      td_name: "inputPollMediaVideo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPollMediaVideoBuilder { inner }
  }

  pub fn video(&self) -> &InputVideo {
    &self.video
  }
}

#[doc(hidden)]
pub struct RTDInputPollMediaVideoBuilder {
  inner: InputPollMediaVideo,
}

impl RTDInputPollMediaVideoBuilder {
  pub fn build(&self) -> InputPollMediaVideo {
    self.inner.clone()
  }

  pub fn video<T: AsRef<InputVideo>>(&mut self, video: T) -> &mut Self {
    self.inner.video = video.as_ref().clone();
    self
  }
}

impl AsRef<InputPollMediaVideo> for InputPollMediaVideo {
  fn as_ref(&self) -> &InputPollMediaVideo {
    self
  }
}

impl AsRef<InputPollMediaVideo> for RTDInputPollMediaVideoBuilder {
  fn as_ref(&self) -> &InputPollMediaVideo {
    &self.inner
  }
}
