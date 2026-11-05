use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes type of clickable area on a story media
pub trait TDStoryAreaType: Debug + RObject {}

/// Describes type of clickable area on a story media
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum StoryAreaType {
  #[doc(hidden)]
  _Default(()),
  /// An area pointing to an HTTP or tg:// link
  Link(StoryAreaTypeLink),
  /// An area pointing to a location
  Location(StoryAreaTypeLocation),
  /// An area pointing to a message
  Message(StoryAreaTypeMessage),
  /// An area pointing to a suggested reaction. App needs to show a clickable reaction on the area and call setStoryReaction when the area is clicked
  SuggestedReaction(StoryAreaTypeSuggestedReaction),
  /// An area with an upgraded gift
  UpgradedGift(StoryAreaTypeUpgradedGift),
  /// An area pointing to a venue
  Venue(StoryAreaTypeVenue),
  /// An area with information about weather
  Weather(StoryAreaTypeWeather),
}

impl Default for StoryAreaType {
  fn default() -> Self {
    StoryAreaType::_Default(())
  }
}

impl<'de> Deserialize<'de> for StoryAreaType {
  fn deserialize<D>(deserializer: D) -> Result<StoryAreaType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      StoryAreaType,
      (storyAreaTypeLink, Link);
      (storyAreaTypeLocation, Location);
      (storyAreaTypeMessage, Message);
      (storyAreaTypeSuggestedReaction, SuggestedReaction);
      (storyAreaTypeUpgradedGift, UpgradedGift);
      (storyAreaTypeVenue, Venue);
      (storyAreaTypeWeather, Weather);

    )(deserializer)
  }
}

impl RObject for StoryAreaType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      StoryAreaType::Link(t) => t.td_name(),
      StoryAreaType::Location(t) => t.td_name(),
      StoryAreaType::Message(t) => t.td_name(),
      StoryAreaType::SuggestedReaction(t) => t.td_name(),
      StoryAreaType::UpgradedGift(t) => t.td_name(),
      StoryAreaType::Venue(t) => t.td_name(),
      StoryAreaType::Weather(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      StoryAreaType::Link(t) => t.extra(),
      StoryAreaType::Location(t) => t.extra(),
      StoryAreaType::Message(t) => t.extra(),
      StoryAreaType::SuggestedReaction(t) => t.extra(),
      StoryAreaType::UpgradedGift(t) => t.extra(),
      StoryAreaType::Venue(t) => t.extra(),
      StoryAreaType::Weather(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StoryAreaType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, StoryAreaType::_Default(_))
  }

  pub fn is_link(&self) -> bool {
    matches!(self, StoryAreaType::Link(_))
  }
  pub fn is_location(&self) -> bool {
    matches!(self, StoryAreaType::Location(_))
  }
  pub fn is_message(&self) -> bool {
    matches!(self, StoryAreaType::Message(_))
  }
  pub fn is_suggested_reaction(&self) -> bool {
    matches!(self, StoryAreaType::SuggestedReaction(_))
  }
  pub fn is_upgraded_gift(&self) -> bool {
    matches!(self, StoryAreaType::UpgradedGift(_))
  }
  pub fn is_venue(&self) -> bool {
    matches!(self, StoryAreaType::Venue(_))
  }
  pub fn is_weather(&self) -> bool {
    matches!(self, StoryAreaType::Weather(_))
  }

  pub fn on_link<F: FnOnce(&StoryAreaTypeLink)>(&self, fnc: F) -> &Self {
    if let StoryAreaType::Link(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_location<F: FnOnce(&StoryAreaTypeLocation)>(&self, fnc: F) -> &Self {
    if let StoryAreaType::Location(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message<F: FnOnce(&StoryAreaTypeMessage)>(&self, fnc: F) -> &Self {
    if let StoryAreaType::Message(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_suggested_reaction<F: FnOnce(&StoryAreaTypeSuggestedReaction)>(&self, fnc: F) -> &Self {
    if let StoryAreaType::SuggestedReaction(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_upgraded_gift<F: FnOnce(&StoryAreaTypeUpgradedGift)>(&self, fnc: F) -> &Self {
    if let StoryAreaType::UpgradedGift(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_venue<F: FnOnce(&StoryAreaTypeVenue)>(&self, fnc: F) -> &Self {
    if let StoryAreaType::Venue(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_weather<F: FnOnce(&StoryAreaTypeWeather)>(&self, fnc: F) -> &Self {
    if let StoryAreaType::Weather(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_link(&self) -> Option<&StoryAreaTypeLink> {
    if let StoryAreaType::Link(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_location(&self) -> Option<&StoryAreaTypeLocation> {
    if let StoryAreaType::Location(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message(&self) -> Option<&StoryAreaTypeMessage> {
    if let StoryAreaType::Message(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_suggested_reaction(&self) -> Option<&StoryAreaTypeSuggestedReaction> {
    if let StoryAreaType::SuggestedReaction(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_upgraded_gift(&self) -> Option<&StoryAreaTypeUpgradedGift> {
    if let StoryAreaType::UpgradedGift(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_venue(&self) -> Option<&StoryAreaTypeVenue> {
    if let StoryAreaType::Venue(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_weather(&self) -> Option<&StoryAreaTypeWeather> {
    if let StoryAreaType::Weather(t) = self {
      return Some(t);
    }
    None
  }

  pub fn link<T: AsRef<StoryAreaTypeLink>>(t: T) -> Self {
    StoryAreaType::Link(t.as_ref().clone())
  }

  pub fn location<T: AsRef<StoryAreaTypeLocation>>(t: T) -> Self {
    StoryAreaType::Location(t.as_ref().clone())
  }

  pub fn message<T: AsRef<StoryAreaTypeMessage>>(t: T) -> Self {
    StoryAreaType::Message(t.as_ref().clone())
  }

  pub fn suggested_reaction<T: AsRef<StoryAreaTypeSuggestedReaction>>(t: T) -> Self {
    StoryAreaType::SuggestedReaction(t.as_ref().clone())
  }

  pub fn upgraded_gift<T: AsRef<StoryAreaTypeUpgradedGift>>(t: T) -> Self {
    StoryAreaType::UpgradedGift(t.as_ref().clone())
  }

  pub fn venue<T: AsRef<StoryAreaTypeVenue>>(t: T) -> Self {
    StoryAreaType::Venue(t.as_ref().clone())
  }

  pub fn weather<T: AsRef<StoryAreaTypeWeather>>(t: T) -> Self {
    StoryAreaType::Weather(t.as_ref().clone())
  }
}

impl AsRef<StoryAreaType> for StoryAreaType {
  fn as_ref(&self) -> &StoryAreaType {
    self
  }
}

/// An area pointing to an HTTP or tg:// link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryAreaTypeLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// HTTP or tg:// URL to be opened when the area is clicked
  url: String,
}

impl RObject for StoryAreaTypeLink {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyAreaTypeLink"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoryAreaType for StoryAreaTypeLink {}

impl StoryAreaTypeLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryAreaTypeLinkBuilder {
    let inner = StoryAreaTypeLink {
      td_name: "storyAreaTypeLink".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryAreaTypeLinkBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }
}

#[doc(hidden)]
pub struct RTDStoryAreaTypeLinkBuilder {
  inner: StoryAreaTypeLink,
}

impl RTDStoryAreaTypeLinkBuilder {
  pub fn build(&self) -> StoryAreaTypeLink {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }
}

impl AsRef<StoryAreaTypeLink> for StoryAreaTypeLink {
  fn as_ref(&self) -> &StoryAreaTypeLink {
    self
  }
}

impl AsRef<StoryAreaTypeLink> for RTDStoryAreaTypeLinkBuilder {
  fn as_ref(&self) -> &StoryAreaTypeLink {
    &self.inner
  }
}

/// An area pointing to a location
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryAreaTypeLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The location
  location: Location,
  /// Address of the location; may be null if unknown
  address: Option<LocationAddress>,
}

impl RObject for StoryAreaTypeLocation {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyAreaTypeLocation"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoryAreaType for StoryAreaTypeLocation {}

impl StoryAreaTypeLocation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryAreaTypeLocationBuilder {
    let inner = StoryAreaTypeLocation {
      td_name: "storyAreaTypeLocation".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryAreaTypeLocationBuilder { inner }
  }

  pub fn location(&self) -> &Location {
    &self.location
  }

  pub fn address(&self) -> &Option<LocationAddress> {
    &self.address
  }
}

#[doc(hidden)]
pub struct RTDStoryAreaTypeLocationBuilder {
  inner: StoryAreaTypeLocation,
}

impl RTDStoryAreaTypeLocationBuilder {
  pub fn build(&self) -> StoryAreaTypeLocation {
    self.inner.clone()
  }

  pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
    self.inner.location = location.as_ref().clone();
    self
  }

  pub fn address<T: AsRef<LocationAddress>>(&mut self, address: T) -> &mut Self {
    self.inner.address = Some(address.as_ref().clone());
    self
  }
}

impl AsRef<StoryAreaTypeLocation> for StoryAreaTypeLocation {
  fn as_ref(&self) -> &StoryAreaTypeLocation {
    self
  }
}

impl AsRef<StoryAreaTypeLocation> for RTDStoryAreaTypeLocationBuilder {
  fn as_ref(&self) -> &StoryAreaTypeLocation {
    &self.inner
  }
}

/// An area pointing to a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryAreaTypeMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat with the message
  chat_id: i64,
  /// Identifier of the message
  message_id: i64,
}

impl RObject for StoryAreaTypeMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyAreaTypeMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoryAreaType for StoryAreaTypeMessage {}

impl StoryAreaTypeMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryAreaTypeMessageBuilder {
    let inner = StoryAreaTypeMessage {
      td_name: "storyAreaTypeMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryAreaTypeMessageBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }
}

#[doc(hidden)]
pub struct RTDStoryAreaTypeMessageBuilder {
  inner: StoryAreaTypeMessage,
}

impl RTDStoryAreaTypeMessageBuilder {
  pub fn build(&self) -> StoryAreaTypeMessage {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }
}

impl AsRef<StoryAreaTypeMessage> for StoryAreaTypeMessage {
  fn as_ref(&self) -> &StoryAreaTypeMessage {
    self
  }
}

impl AsRef<StoryAreaTypeMessage> for RTDStoryAreaTypeMessageBuilder {
  fn as_ref(&self) -> &StoryAreaTypeMessage {
    &self.inner
  }
}

/// An area pointing to a suggested reaction. App needs to show a clickable reaction on the area and call setStoryReaction when the area is clicked
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryAreaTypeSuggestedReaction {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Type of the reaction
  reaction_type: ReactionType,
  /// Number of times the reaction was added
  total_count: i64,
  /// True, if reaction has a dark background
  is_dark: bool,
  /// True, if reaction corner is flipped
  is_flipped: bool,
}

impl RObject for StoryAreaTypeSuggestedReaction {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyAreaTypeSuggestedReaction"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoryAreaType for StoryAreaTypeSuggestedReaction {}

impl StoryAreaTypeSuggestedReaction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryAreaTypeSuggestedReactionBuilder {
    let inner = StoryAreaTypeSuggestedReaction {
      td_name: "storyAreaTypeSuggestedReaction".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryAreaTypeSuggestedReactionBuilder { inner }
  }

  pub fn reaction_type(&self) -> &ReactionType {
    &self.reaction_type
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }

  pub fn is_dark(&self) -> bool {
    self.is_dark
  }

  pub fn is_flipped(&self) -> bool {
    self.is_flipped
  }
}

#[doc(hidden)]
pub struct RTDStoryAreaTypeSuggestedReactionBuilder {
  inner: StoryAreaTypeSuggestedReaction,
}

impl RTDStoryAreaTypeSuggestedReactionBuilder {
  pub fn build(&self) -> StoryAreaTypeSuggestedReaction {
    self.inner.clone()
  }

  pub fn reaction_type<T: AsRef<ReactionType>>(&mut self, reaction_type: T) -> &mut Self {
    self.inner.reaction_type = reaction_type.as_ref().clone();
    self
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

  pub fn is_dark(&mut self, is_dark: bool) -> &mut Self {
    self.inner.is_dark = is_dark;
    self
  }

  pub fn is_flipped(&mut self, is_flipped: bool) -> &mut Self {
    self.inner.is_flipped = is_flipped;
    self
  }
}

impl AsRef<StoryAreaTypeSuggestedReaction> for StoryAreaTypeSuggestedReaction {
  fn as_ref(&self) -> &StoryAreaTypeSuggestedReaction {
    self
  }
}

impl AsRef<StoryAreaTypeSuggestedReaction> for RTDStoryAreaTypeSuggestedReactionBuilder {
  fn as_ref(&self) -> &StoryAreaTypeSuggestedReaction {
    &self.inner
  }
}

/// An area with an upgraded gift
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryAreaTypeUpgradedGift {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique name of the upgraded gift
  gift_name: String,
}

impl RObject for StoryAreaTypeUpgradedGift {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyAreaTypeUpgradedGift"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoryAreaType for StoryAreaTypeUpgradedGift {}

impl StoryAreaTypeUpgradedGift {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryAreaTypeUpgradedGiftBuilder {
    let inner = StoryAreaTypeUpgradedGift {
      td_name: "storyAreaTypeUpgradedGift".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryAreaTypeUpgradedGiftBuilder { inner }
  }

  pub fn gift_name(&self) -> &String {
    &self.gift_name
  }
}

#[doc(hidden)]
pub struct RTDStoryAreaTypeUpgradedGiftBuilder {
  inner: StoryAreaTypeUpgradedGift,
}

impl RTDStoryAreaTypeUpgradedGiftBuilder {
  pub fn build(&self) -> StoryAreaTypeUpgradedGift {
    self.inner.clone()
  }

  pub fn gift_name<T: AsRef<str>>(&mut self, gift_name: T) -> &mut Self {
    self.inner.gift_name = gift_name.as_ref().to_string();
    self
  }
}

impl AsRef<StoryAreaTypeUpgradedGift> for StoryAreaTypeUpgradedGift {
  fn as_ref(&self) -> &StoryAreaTypeUpgradedGift {
    self
  }
}

impl AsRef<StoryAreaTypeUpgradedGift> for RTDStoryAreaTypeUpgradedGiftBuilder {
  fn as_ref(&self) -> &StoryAreaTypeUpgradedGift {
    &self.inner
  }
}

/// An area pointing to a venue
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryAreaTypeVenue {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Information about the venue
  venue: Venue,
}

impl RObject for StoryAreaTypeVenue {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyAreaTypeVenue"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoryAreaType for StoryAreaTypeVenue {}

impl StoryAreaTypeVenue {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryAreaTypeVenueBuilder {
    let inner = StoryAreaTypeVenue {
      td_name: "storyAreaTypeVenue".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryAreaTypeVenueBuilder { inner }
  }

  pub fn venue(&self) -> &Venue {
    &self.venue
  }
}

#[doc(hidden)]
pub struct RTDStoryAreaTypeVenueBuilder {
  inner: StoryAreaTypeVenue,
}

impl RTDStoryAreaTypeVenueBuilder {
  pub fn build(&self) -> StoryAreaTypeVenue {
    self.inner.clone()
  }

  pub fn venue<T: AsRef<Venue>>(&mut self, venue: T) -> &mut Self {
    self.inner.venue = venue.as_ref().clone();
    self
  }
}

impl AsRef<StoryAreaTypeVenue> for StoryAreaTypeVenue {
  fn as_ref(&self) -> &StoryAreaTypeVenue {
    self
  }
}

impl AsRef<StoryAreaTypeVenue> for RTDStoryAreaTypeVenueBuilder {
  fn as_ref(&self) -> &StoryAreaTypeVenue {
    &self.inner
  }
}

/// An area with information about weather
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryAreaTypeWeather {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Temperature, in degree Celsius
  temperature: f32,
  /// Emoji representing the weather
  emoji: String,
  /// A color of the area background in the ARGB format
  background_color: i64,
}

impl RObject for StoryAreaTypeWeather {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyAreaTypeWeather"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoryAreaType for StoryAreaTypeWeather {}

impl StoryAreaTypeWeather {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryAreaTypeWeatherBuilder {
    let inner = StoryAreaTypeWeather {
      td_name: "storyAreaTypeWeather".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryAreaTypeWeatherBuilder { inner }
  }

  pub fn temperature(&self) -> f32 {
    self.temperature
  }

  pub fn emoji(&self) -> &String {
    &self.emoji
  }

  pub fn background_color(&self) -> i64 {
    self.background_color
  }
}

#[doc(hidden)]
pub struct RTDStoryAreaTypeWeatherBuilder {
  inner: StoryAreaTypeWeather,
}

impl RTDStoryAreaTypeWeatherBuilder {
  pub fn build(&self) -> StoryAreaTypeWeather {
    self.inner.clone()
  }

  pub fn temperature(&mut self, temperature: f32) -> &mut Self {
    self.inner.temperature = temperature;
    self
  }

  pub fn emoji<T: AsRef<str>>(&mut self, emoji: T) -> &mut Self {
    self.inner.emoji = emoji.as_ref().to_string();
    self
  }

  pub fn background_color(&mut self, background_color: i64) -> &mut Self {
    self.inner.background_color = background_color;
    self
  }
}

impl AsRef<StoryAreaTypeWeather> for StoryAreaTypeWeather {
  fn as_ref(&self) -> &StoryAreaTypeWeather {
    self
  }
}

impl AsRef<StoryAreaTypeWeather> for RTDStoryAreaTypeWeatherBuilder {
  fn as_ref(&self) -> &StoryAreaTypeWeather {
    &self.inner
  }
}
