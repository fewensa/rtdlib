use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes type of clickable area on a story media to be added
pub trait TDInputStoryAreaType: Debug + RObject {}

/// Describes type of clickable area on a story media to be added
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputStoryAreaType {
  #[doc(hidden)]
  _Default(()),
  /// An area pointing to a venue found by the bot getOption("venue_search_bot_username")
  FoundVenue(InputStoryAreaTypeFoundVenue),
  /// An area pointing to an HTTP or tg:// link
  Link(InputStoryAreaTypeLink),
  /// An area pointing to a location
  Location(InputStoryAreaTypeLocation),
  /// An area pointing to a message
  Message(InputStoryAreaTypeMessage),
  /// An area pointing to a venue already added to the story
  PreviousVenue(InputStoryAreaTypePreviousVenue),
  /// An area pointing to a suggested reaction
  SuggestedReaction(InputStoryAreaTypeSuggestedReaction),
  /// An area with an upgraded gift
  UpgradedGift(InputStoryAreaTypeUpgradedGift),
  /// An area with information about weather
  Weather(InputStoryAreaTypeWeather),
}

impl Default for InputStoryAreaType {
  fn default() -> Self {
    InputStoryAreaType::_Default(())
  }
}

impl<'de> Deserialize<'de> for InputStoryAreaType {
  fn deserialize<D>(deserializer: D) -> Result<InputStoryAreaType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InputStoryAreaType,
      (inputStoryAreaTypeFoundVenue, FoundVenue);
      (inputStoryAreaTypeLink, Link);
      (inputStoryAreaTypeLocation, Location);
      (inputStoryAreaTypeMessage, Message);
      (inputStoryAreaTypePreviousVenue, PreviousVenue);
      (inputStoryAreaTypeSuggestedReaction, SuggestedReaction);
      (inputStoryAreaTypeUpgradedGift, UpgradedGift);
      (inputStoryAreaTypeWeather, Weather);

    )(deserializer)
  }
}

impl RObject for InputStoryAreaType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      InputStoryAreaType::FoundVenue(t) => t.td_name(),
      InputStoryAreaType::Link(t) => t.td_name(),
      InputStoryAreaType::Location(t) => t.td_name(),
      InputStoryAreaType::Message(t) => t.td_name(),
      InputStoryAreaType::PreviousVenue(t) => t.td_name(),
      InputStoryAreaType::SuggestedReaction(t) => t.td_name(),
      InputStoryAreaType::UpgradedGift(t) => t.td_name(),
      InputStoryAreaType::Weather(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      InputStoryAreaType::FoundVenue(t) => t.extra(),
      InputStoryAreaType::Link(t) => t.extra(),
      InputStoryAreaType::Location(t) => t.extra(),
      InputStoryAreaType::Message(t) => t.extra(),
      InputStoryAreaType::PreviousVenue(t) => t.extra(),
      InputStoryAreaType::SuggestedReaction(t) => t.extra(),
      InputStoryAreaType::UpgradedGift(t) => t.extra(),
      InputStoryAreaType::Weather(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputStoryAreaType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, InputStoryAreaType::_Default(_))
  }

  pub fn is_found_venue(&self) -> bool {
    matches!(self, InputStoryAreaType::FoundVenue(_))
  }
  pub fn is_link(&self) -> bool {
    matches!(self, InputStoryAreaType::Link(_))
  }
  pub fn is_location(&self) -> bool {
    matches!(self, InputStoryAreaType::Location(_))
  }
  pub fn is_message(&self) -> bool {
    matches!(self, InputStoryAreaType::Message(_))
  }
  pub fn is_previous_venue(&self) -> bool {
    matches!(self, InputStoryAreaType::PreviousVenue(_))
  }
  pub fn is_suggested_reaction(&self) -> bool {
    matches!(self, InputStoryAreaType::SuggestedReaction(_))
  }
  pub fn is_upgraded_gift(&self) -> bool {
    matches!(self, InputStoryAreaType::UpgradedGift(_))
  }
  pub fn is_weather(&self) -> bool {
    matches!(self, InputStoryAreaType::Weather(_))
  }

  pub fn on_found_venue<F: FnOnce(&InputStoryAreaTypeFoundVenue)>(&self, fnc: F) -> &Self {
    if let InputStoryAreaType::FoundVenue(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_link<F: FnOnce(&InputStoryAreaTypeLink)>(&self, fnc: F) -> &Self {
    if let InputStoryAreaType::Link(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_location<F: FnOnce(&InputStoryAreaTypeLocation)>(&self, fnc: F) -> &Self {
    if let InputStoryAreaType::Location(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message<F: FnOnce(&InputStoryAreaTypeMessage)>(&self, fnc: F) -> &Self {
    if let InputStoryAreaType::Message(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_previous_venue<F: FnOnce(&InputStoryAreaTypePreviousVenue)>(&self, fnc: F) -> &Self {
    if let InputStoryAreaType::PreviousVenue(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_suggested_reaction<F: FnOnce(&InputStoryAreaTypeSuggestedReaction)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InputStoryAreaType::SuggestedReaction(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_upgraded_gift<F: FnOnce(&InputStoryAreaTypeUpgradedGift)>(&self, fnc: F) -> &Self {
    if let InputStoryAreaType::UpgradedGift(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_weather<F: FnOnce(&InputStoryAreaTypeWeather)>(&self, fnc: F) -> &Self {
    if let InputStoryAreaType::Weather(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_found_venue(&self) -> Option<&InputStoryAreaTypeFoundVenue> {
    if let InputStoryAreaType::FoundVenue(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_link(&self) -> Option<&InputStoryAreaTypeLink> {
    if let InputStoryAreaType::Link(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_location(&self) -> Option<&InputStoryAreaTypeLocation> {
    if let InputStoryAreaType::Location(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message(&self) -> Option<&InputStoryAreaTypeMessage> {
    if let InputStoryAreaType::Message(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_previous_venue(&self) -> Option<&InputStoryAreaTypePreviousVenue> {
    if let InputStoryAreaType::PreviousVenue(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_suggested_reaction(&self) -> Option<&InputStoryAreaTypeSuggestedReaction> {
    if let InputStoryAreaType::SuggestedReaction(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_upgraded_gift(&self) -> Option<&InputStoryAreaTypeUpgradedGift> {
    if let InputStoryAreaType::UpgradedGift(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_weather(&self) -> Option<&InputStoryAreaTypeWeather> {
    if let InputStoryAreaType::Weather(t) = self {
      return Some(t);
    }
    None
  }

  pub fn found_venue<T: AsRef<InputStoryAreaTypeFoundVenue>>(t: T) -> Self {
    InputStoryAreaType::FoundVenue(t.as_ref().clone())
  }

  pub fn link<T: AsRef<InputStoryAreaTypeLink>>(t: T) -> Self {
    InputStoryAreaType::Link(t.as_ref().clone())
  }

  pub fn location<T: AsRef<InputStoryAreaTypeLocation>>(t: T) -> Self {
    InputStoryAreaType::Location(t.as_ref().clone())
  }

  pub fn message<T: AsRef<InputStoryAreaTypeMessage>>(t: T) -> Self {
    InputStoryAreaType::Message(t.as_ref().clone())
  }

  pub fn previous_venue<T: AsRef<InputStoryAreaTypePreviousVenue>>(t: T) -> Self {
    InputStoryAreaType::PreviousVenue(t.as_ref().clone())
  }

  pub fn suggested_reaction<T: AsRef<InputStoryAreaTypeSuggestedReaction>>(t: T) -> Self {
    InputStoryAreaType::SuggestedReaction(t.as_ref().clone())
  }

  pub fn upgraded_gift<T: AsRef<InputStoryAreaTypeUpgradedGift>>(t: T) -> Self {
    InputStoryAreaType::UpgradedGift(t.as_ref().clone())
  }

  pub fn weather<T: AsRef<InputStoryAreaTypeWeather>>(t: T) -> Self {
    InputStoryAreaType::Weather(t.as_ref().clone())
  }
}

impl AsRef<InputStoryAreaType> for InputStoryAreaType {
  fn as_ref(&self) -> &InputStoryAreaType {
    self
  }
}

/// An area pointing to a venue found by the bot getOption("venue_search_bot_username")
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputStoryAreaTypeFoundVenue {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the inline query, used to find the venue
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  query_id: isize,
  /// Identifier of the inline query result
  result_id: String,
}

impl RObject for InputStoryAreaTypeFoundVenue {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputStoryAreaTypeFoundVenue"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputStoryAreaType for InputStoryAreaTypeFoundVenue {}

impl InputStoryAreaTypeFoundVenue {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputStoryAreaTypeFoundVenueBuilder {
    let inner = InputStoryAreaTypeFoundVenue {
      td_name: "inputStoryAreaTypeFoundVenue".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputStoryAreaTypeFoundVenueBuilder { inner }
  }

  pub fn query_id(&self) -> isize {
    self.query_id
  }

  pub fn result_id(&self) -> &String {
    &self.result_id
  }
}

#[doc(hidden)]
pub struct RTDInputStoryAreaTypeFoundVenueBuilder {
  inner: InputStoryAreaTypeFoundVenue,
}

impl RTDInputStoryAreaTypeFoundVenueBuilder {
  pub fn build(&self) -> InputStoryAreaTypeFoundVenue {
    self.inner.clone()
  }

  pub fn query_id(&mut self, query_id: isize) -> &mut Self {
    self.inner.query_id = query_id;
    self
  }

  pub fn result_id<T: AsRef<str>>(&mut self, result_id: T) -> &mut Self {
    self.inner.result_id = result_id.as_ref().to_string();
    self
  }
}

impl AsRef<InputStoryAreaTypeFoundVenue> for InputStoryAreaTypeFoundVenue {
  fn as_ref(&self) -> &InputStoryAreaTypeFoundVenue {
    self
  }
}

impl AsRef<InputStoryAreaTypeFoundVenue> for RTDInputStoryAreaTypeFoundVenueBuilder {
  fn as_ref(&self) -> &InputStoryAreaTypeFoundVenue {
    &self.inner
  }
}

/// An area pointing to an HTTP or tg:// link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputStoryAreaTypeLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// HTTP or tg:// URL to be opened when the area is clicked
  url: String,
}

impl RObject for InputStoryAreaTypeLink {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputStoryAreaTypeLink"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputStoryAreaType for InputStoryAreaTypeLink {}

impl InputStoryAreaTypeLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputStoryAreaTypeLinkBuilder {
    let inner = InputStoryAreaTypeLink {
      td_name: "inputStoryAreaTypeLink".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputStoryAreaTypeLinkBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }
}

#[doc(hidden)]
pub struct RTDInputStoryAreaTypeLinkBuilder {
  inner: InputStoryAreaTypeLink,
}

impl RTDInputStoryAreaTypeLinkBuilder {
  pub fn build(&self) -> InputStoryAreaTypeLink {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }
}

impl AsRef<InputStoryAreaTypeLink> for InputStoryAreaTypeLink {
  fn as_ref(&self) -> &InputStoryAreaTypeLink {
    self
  }
}

impl AsRef<InputStoryAreaTypeLink> for RTDInputStoryAreaTypeLinkBuilder {
  fn as_ref(&self) -> &InputStoryAreaTypeLink {
    &self.inner
  }
}

/// An area pointing to a location
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputStoryAreaTypeLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The location
  location: Location,
  /// Address of the location; pass null if unknown
  address: LocationAddress,
}

impl RObject for InputStoryAreaTypeLocation {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputStoryAreaTypeLocation"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputStoryAreaType for InputStoryAreaTypeLocation {}

impl InputStoryAreaTypeLocation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputStoryAreaTypeLocationBuilder {
    let inner = InputStoryAreaTypeLocation {
      td_name: "inputStoryAreaTypeLocation".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputStoryAreaTypeLocationBuilder { inner }
  }

  pub fn location(&self) -> &Location {
    &self.location
  }

  pub fn address(&self) -> &LocationAddress {
    &self.address
  }
}

#[doc(hidden)]
pub struct RTDInputStoryAreaTypeLocationBuilder {
  inner: InputStoryAreaTypeLocation,
}

impl RTDInputStoryAreaTypeLocationBuilder {
  pub fn build(&self) -> InputStoryAreaTypeLocation {
    self.inner.clone()
  }

  pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
    self.inner.location = location.as_ref().clone();
    self
  }

  pub fn address<T: AsRef<LocationAddress>>(&mut self, address: T) -> &mut Self {
    self.inner.address = address.as_ref().clone();
    self
  }
}

impl AsRef<InputStoryAreaTypeLocation> for InputStoryAreaTypeLocation {
  fn as_ref(&self) -> &InputStoryAreaTypeLocation {
    self
  }
}

impl AsRef<InputStoryAreaTypeLocation> for RTDInputStoryAreaTypeLocationBuilder {
  fn as_ref(&self) -> &InputStoryAreaTypeLocation {
    &self.inner
  }
}

/// An area pointing to a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputStoryAreaTypeMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat with the message. Currently, the chat must be a supergroup or a channel chat
  chat_id: i64,
  /// Identifier of the message. Use messageProperties.can_be_shared_in_story to check whether the message is suitable
  message_id: i64,
}

impl RObject for InputStoryAreaTypeMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputStoryAreaTypeMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputStoryAreaType for InputStoryAreaTypeMessage {}

impl InputStoryAreaTypeMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputStoryAreaTypeMessageBuilder {
    let inner = InputStoryAreaTypeMessage {
      td_name: "inputStoryAreaTypeMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputStoryAreaTypeMessageBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }
}

#[doc(hidden)]
pub struct RTDInputStoryAreaTypeMessageBuilder {
  inner: InputStoryAreaTypeMessage,
}

impl RTDInputStoryAreaTypeMessageBuilder {
  pub fn build(&self) -> InputStoryAreaTypeMessage {
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

impl AsRef<InputStoryAreaTypeMessage> for InputStoryAreaTypeMessage {
  fn as_ref(&self) -> &InputStoryAreaTypeMessage {
    self
  }
}

impl AsRef<InputStoryAreaTypeMessage> for RTDInputStoryAreaTypeMessageBuilder {
  fn as_ref(&self) -> &InputStoryAreaTypeMessage {
    &self.inner
  }
}

/// An area pointing to a venue already added to the story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputStoryAreaTypePreviousVenue {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Provider of the venue
  venue_provider: String,
  /// Identifier of the venue in the provider database
  venue_id: String,
}

impl RObject for InputStoryAreaTypePreviousVenue {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputStoryAreaTypePreviousVenue"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputStoryAreaType for InputStoryAreaTypePreviousVenue {}

impl InputStoryAreaTypePreviousVenue {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputStoryAreaTypePreviousVenueBuilder {
    let inner = InputStoryAreaTypePreviousVenue {
      td_name: "inputStoryAreaTypePreviousVenue".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputStoryAreaTypePreviousVenueBuilder { inner }
  }

  pub fn venue_provider(&self) -> &String {
    &self.venue_provider
  }

  pub fn venue_id(&self) -> &String {
    &self.venue_id
  }
}

#[doc(hidden)]
pub struct RTDInputStoryAreaTypePreviousVenueBuilder {
  inner: InputStoryAreaTypePreviousVenue,
}

impl RTDInputStoryAreaTypePreviousVenueBuilder {
  pub fn build(&self) -> InputStoryAreaTypePreviousVenue {
    self.inner.clone()
  }

  pub fn venue_provider<T: AsRef<str>>(&mut self, venue_provider: T) -> &mut Self {
    self.inner.venue_provider = venue_provider.as_ref().to_string();
    self
  }

  pub fn venue_id<T: AsRef<str>>(&mut self, venue_id: T) -> &mut Self {
    self.inner.venue_id = venue_id.as_ref().to_string();
    self
  }
}

impl AsRef<InputStoryAreaTypePreviousVenue> for InputStoryAreaTypePreviousVenue {
  fn as_ref(&self) -> &InputStoryAreaTypePreviousVenue {
    self
  }
}

impl AsRef<InputStoryAreaTypePreviousVenue> for RTDInputStoryAreaTypePreviousVenueBuilder {
  fn as_ref(&self) -> &InputStoryAreaTypePreviousVenue {
    &self.inner
  }
}

/// An area pointing to a suggested reaction
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputStoryAreaTypeSuggestedReaction {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Type of the reaction
  reaction_type: ReactionType,
  /// True, if reaction has a dark background
  is_dark: bool,
  /// True, if reaction corner is flipped
  is_flipped: bool,
}

impl RObject for InputStoryAreaTypeSuggestedReaction {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputStoryAreaTypeSuggestedReaction"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputStoryAreaType for InputStoryAreaTypeSuggestedReaction {}

impl InputStoryAreaTypeSuggestedReaction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputStoryAreaTypeSuggestedReactionBuilder {
    let inner = InputStoryAreaTypeSuggestedReaction {
      td_name: "inputStoryAreaTypeSuggestedReaction".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputStoryAreaTypeSuggestedReactionBuilder { inner }
  }

  pub fn reaction_type(&self) -> &ReactionType {
    &self.reaction_type
  }

  pub fn is_dark(&self) -> bool {
    self.is_dark
  }

  pub fn is_flipped(&self) -> bool {
    self.is_flipped
  }
}

#[doc(hidden)]
pub struct RTDInputStoryAreaTypeSuggestedReactionBuilder {
  inner: InputStoryAreaTypeSuggestedReaction,
}

impl RTDInputStoryAreaTypeSuggestedReactionBuilder {
  pub fn build(&self) -> InputStoryAreaTypeSuggestedReaction {
    self.inner.clone()
  }

  pub fn reaction_type<T: AsRef<ReactionType>>(&mut self, reaction_type: T) -> &mut Self {
    self.inner.reaction_type = reaction_type.as_ref().clone();
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

impl AsRef<InputStoryAreaTypeSuggestedReaction> for InputStoryAreaTypeSuggestedReaction {
  fn as_ref(&self) -> &InputStoryAreaTypeSuggestedReaction {
    self
  }
}

impl AsRef<InputStoryAreaTypeSuggestedReaction> for RTDInputStoryAreaTypeSuggestedReactionBuilder {
  fn as_ref(&self) -> &InputStoryAreaTypeSuggestedReaction {
    &self.inner
  }
}

/// An area with an upgraded gift
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputStoryAreaTypeUpgradedGift {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique name of the upgraded gift
  gift_name: String,
}

impl RObject for InputStoryAreaTypeUpgradedGift {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputStoryAreaTypeUpgradedGift"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputStoryAreaType for InputStoryAreaTypeUpgradedGift {}

impl InputStoryAreaTypeUpgradedGift {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputStoryAreaTypeUpgradedGiftBuilder {
    let inner = InputStoryAreaTypeUpgradedGift {
      td_name: "inputStoryAreaTypeUpgradedGift".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputStoryAreaTypeUpgradedGiftBuilder { inner }
  }

  pub fn gift_name(&self) -> &String {
    &self.gift_name
  }
}

#[doc(hidden)]
pub struct RTDInputStoryAreaTypeUpgradedGiftBuilder {
  inner: InputStoryAreaTypeUpgradedGift,
}

impl RTDInputStoryAreaTypeUpgradedGiftBuilder {
  pub fn build(&self) -> InputStoryAreaTypeUpgradedGift {
    self.inner.clone()
  }

  pub fn gift_name<T: AsRef<str>>(&mut self, gift_name: T) -> &mut Self {
    self.inner.gift_name = gift_name.as_ref().to_string();
    self
  }
}

impl AsRef<InputStoryAreaTypeUpgradedGift> for InputStoryAreaTypeUpgradedGift {
  fn as_ref(&self) -> &InputStoryAreaTypeUpgradedGift {
    self
  }
}

impl AsRef<InputStoryAreaTypeUpgradedGift> for RTDInputStoryAreaTypeUpgradedGiftBuilder {
  fn as_ref(&self) -> &InputStoryAreaTypeUpgradedGift {
    &self.inner
  }
}

/// An area with information about weather
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputStoryAreaTypeWeather {
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

impl RObject for InputStoryAreaTypeWeather {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputStoryAreaTypeWeather"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputStoryAreaType for InputStoryAreaTypeWeather {}

impl InputStoryAreaTypeWeather {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputStoryAreaTypeWeatherBuilder {
    let inner = InputStoryAreaTypeWeather {
      td_name: "inputStoryAreaTypeWeather".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputStoryAreaTypeWeatherBuilder { inner }
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
pub struct RTDInputStoryAreaTypeWeatherBuilder {
  inner: InputStoryAreaTypeWeather,
}

impl RTDInputStoryAreaTypeWeatherBuilder {
  pub fn build(&self) -> InputStoryAreaTypeWeather {
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

impl AsRef<InputStoryAreaTypeWeather> for InputStoryAreaTypeWeather {
  fn as_ref(&self) -> &InputStoryAreaTypeWeather {
    self
  }
}

impl AsRef<InputStoryAreaTypeWeather> for RTDInputStoryAreaTypeWeatherBuilder {
  fn as_ref(&self) -> &InputStoryAreaTypeWeather {
    &self.inner
  }
}
