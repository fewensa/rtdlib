use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a feature available to Business user accounts
pub trait TDBusinessFeature: Debug + RObject {}

/// Describes a feature available to Business user accounts
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum BusinessFeature {
  #[doc(hidden)]
  _Default(()),
  /// The ability to create links to the business account with predefined message text
  AccountLinks(BusinessFeatureAccountLinks),
  /// The ability to set up an away message
  AwayMessage(BusinessFeatureAwayMessage),
  /// The ability to connect a bot to the account
  Bots(BusinessFeatureBots),
  /// The ability to display folder names for each chat in the chat list
  ChatFolderTags(BusinessFeatureChatFolderTags),
  /// The ability to show an emoji status along with the business name
  EmojiStatus(BusinessFeatureEmojiStatus),
  /// The ability to set up a greeting message
  GreetingMessage(BusinessFeatureGreetingMessage),
  /// The ability to set location
  Location(BusinessFeatureLocation),
  /// The ability to set opening hours
  OpeningHours(BusinessFeatureOpeningHours),
  /// The ability to use quick replies
  QuickReplies(BusinessFeatureQuickReplies),
  /// The ability to customize start page
  StartPage(BusinessFeatureStartPage),
  /// Allowed to use many additional features for stories
  UpgradedStories(BusinessFeatureUpgradedStories),
}

impl Default for BusinessFeature {
  fn default() -> Self {
    BusinessFeature::_Default(())
  }
}

impl<'de> Deserialize<'de> for BusinessFeature {
  fn deserialize<D>(deserializer: D) -> Result<BusinessFeature, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      BusinessFeature,
      (businessFeatureAccountLinks, AccountLinks);
      (businessFeatureAwayMessage, AwayMessage);
      (businessFeatureBots, Bots);
      (businessFeatureChatFolderTags, ChatFolderTags);
      (businessFeatureEmojiStatus, EmojiStatus);
      (businessFeatureGreetingMessage, GreetingMessage);
      (businessFeatureLocation, Location);
      (businessFeatureOpeningHours, OpeningHours);
      (businessFeatureQuickReplies, QuickReplies);
      (businessFeatureStartPage, StartPage);
      (businessFeatureUpgradedStories, UpgradedStories);

    )(deserializer)
  }
}

impl RObject for BusinessFeature {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      BusinessFeature::AccountLinks(t) => t.td_name(),
      BusinessFeature::AwayMessage(t) => t.td_name(),
      BusinessFeature::Bots(t) => t.td_name(),
      BusinessFeature::ChatFolderTags(t) => t.td_name(),
      BusinessFeature::EmojiStatus(t) => t.td_name(),
      BusinessFeature::GreetingMessage(t) => t.td_name(),
      BusinessFeature::Location(t) => t.td_name(),
      BusinessFeature::OpeningHours(t) => t.td_name(),
      BusinessFeature::QuickReplies(t) => t.td_name(),
      BusinessFeature::StartPage(t) => t.td_name(),
      BusinessFeature::UpgradedStories(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      BusinessFeature::AccountLinks(t) => t.extra(),
      BusinessFeature::AwayMessage(t) => t.extra(),
      BusinessFeature::Bots(t) => t.extra(),
      BusinessFeature::ChatFolderTags(t) => t.extra(),
      BusinessFeature::EmojiStatus(t) => t.extra(),
      BusinessFeature::GreetingMessage(t) => t.extra(),
      BusinessFeature::Location(t) => t.extra(),
      BusinessFeature::OpeningHours(t) => t.extra(),
      BusinessFeature::QuickReplies(t) => t.extra(),
      BusinessFeature::StartPage(t) => t.extra(),
      BusinessFeature::UpgradedStories(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BusinessFeature {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, BusinessFeature::_Default(_))
  }

  pub fn is_account_links(&self) -> bool {
    matches!(self, BusinessFeature::AccountLinks(_))
  }
  pub fn is_away_message(&self) -> bool {
    matches!(self, BusinessFeature::AwayMessage(_))
  }
  pub fn is_bots(&self) -> bool {
    matches!(self, BusinessFeature::Bots(_))
  }
  pub fn is_chat_folder_tags(&self) -> bool {
    matches!(self, BusinessFeature::ChatFolderTags(_))
  }
  pub fn is_emoji_status(&self) -> bool {
    matches!(self, BusinessFeature::EmojiStatus(_))
  }
  pub fn is_greeting_message(&self) -> bool {
    matches!(self, BusinessFeature::GreetingMessage(_))
  }
  pub fn is_location(&self) -> bool {
    matches!(self, BusinessFeature::Location(_))
  }
  pub fn is_opening_hours(&self) -> bool {
    matches!(self, BusinessFeature::OpeningHours(_))
  }
  pub fn is_quick_replies(&self) -> bool {
    matches!(self, BusinessFeature::QuickReplies(_))
  }
  pub fn is_start_page(&self) -> bool {
    matches!(self, BusinessFeature::StartPage(_))
  }
  pub fn is_upgraded_stories(&self) -> bool {
    matches!(self, BusinessFeature::UpgradedStories(_))
  }

  pub fn on_account_links<F: FnOnce(&BusinessFeatureAccountLinks)>(&self, fnc: F) -> &Self {
    if let BusinessFeature::AccountLinks(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_away_message<F: FnOnce(&BusinessFeatureAwayMessage)>(&self, fnc: F) -> &Self {
    if let BusinessFeature::AwayMessage(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_bots<F: FnOnce(&BusinessFeatureBots)>(&self, fnc: F) -> &Self {
    if let BusinessFeature::Bots(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_folder_tags<F: FnOnce(&BusinessFeatureChatFolderTags)>(&self, fnc: F) -> &Self {
    if let BusinessFeature::ChatFolderTags(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_emoji_status<F: FnOnce(&BusinessFeatureEmojiStatus)>(&self, fnc: F) -> &Self {
    if let BusinessFeature::EmojiStatus(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_greeting_message<F: FnOnce(&BusinessFeatureGreetingMessage)>(&self, fnc: F) -> &Self {
    if let BusinessFeature::GreetingMessage(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_location<F: FnOnce(&BusinessFeatureLocation)>(&self, fnc: F) -> &Self {
    if let BusinessFeature::Location(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_opening_hours<F: FnOnce(&BusinessFeatureOpeningHours)>(&self, fnc: F) -> &Self {
    if let BusinessFeature::OpeningHours(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_quick_replies<F: FnOnce(&BusinessFeatureQuickReplies)>(&self, fnc: F) -> &Self {
    if let BusinessFeature::QuickReplies(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_start_page<F: FnOnce(&BusinessFeatureStartPage)>(&self, fnc: F) -> &Self {
    if let BusinessFeature::StartPage(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_upgraded_stories<F: FnOnce(&BusinessFeatureUpgradedStories)>(&self, fnc: F) -> &Self {
    if let BusinessFeature::UpgradedStories(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_account_links(&self) -> Option<&BusinessFeatureAccountLinks> {
    if let BusinessFeature::AccountLinks(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_away_message(&self) -> Option<&BusinessFeatureAwayMessage> {
    if let BusinessFeature::AwayMessage(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_bots(&self) -> Option<&BusinessFeatureBots> {
    if let BusinessFeature::Bots(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_folder_tags(&self) -> Option<&BusinessFeatureChatFolderTags> {
    if let BusinessFeature::ChatFolderTags(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_emoji_status(&self) -> Option<&BusinessFeatureEmojiStatus> {
    if let BusinessFeature::EmojiStatus(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_greeting_message(&self) -> Option<&BusinessFeatureGreetingMessage> {
    if let BusinessFeature::GreetingMessage(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_location(&self) -> Option<&BusinessFeatureLocation> {
    if let BusinessFeature::Location(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_opening_hours(&self) -> Option<&BusinessFeatureOpeningHours> {
    if let BusinessFeature::OpeningHours(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_quick_replies(&self) -> Option<&BusinessFeatureQuickReplies> {
    if let BusinessFeature::QuickReplies(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_start_page(&self) -> Option<&BusinessFeatureStartPage> {
    if let BusinessFeature::StartPage(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_upgraded_stories(&self) -> Option<&BusinessFeatureUpgradedStories> {
    if let BusinessFeature::UpgradedStories(t) = self {
      return Some(t);
    }
    None
  }

  pub fn account_links<T: AsRef<BusinessFeatureAccountLinks>>(t: T) -> Self {
    BusinessFeature::AccountLinks(t.as_ref().clone())
  }

  pub fn away_message<T: AsRef<BusinessFeatureAwayMessage>>(t: T) -> Self {
    BusinessFeature::AwayMessage(t.as_ref().clone())
  }

  pub fn bots<T: AsRef<BusinessFeatureBots>>(t: T) -> Self {
    BusinessFeature::Bots(t.as_ref().clone())
  }

  pub fn chat_folder_tags<T: AsRef<BusinessFeatureChatFolderTags>>(t: T) -> Self {
    BusinessFeature::ChatFolderTags(t.as_ref().clone())
  }

  pub fn emoji_status<T: AsRef<BusinessFeatureEmojiStatus>>(t: T) -> Self {
    BusinessFeature::EmojiStatus(t.as_ref().clone())
  }

  pub fn greeting_message<T: AsRef<BusinessFeatureGreetingMessage>>(t: T) -> Self {
    BusinessFeature::GreetingMessage(t.as_ref().clone())
  }

  pub fn location<T: AsRef<BusinessFeatureLocation>>(t: T) -> Self {
    BusinessFeature::Location(t.as_ref().clone())
  }

  pub fn opening_hours<T: AsRef<BusinessFeatureOpeningHours>>(t: T) -> Self {
    BusinessFeature::OpeningHours(t.as_ref().clone())
  }

  pub fn quick_replies<T: AsRef<BusinessFeatureQuickReplies>>(t: T) -> Self {
    BusinessFeature::QuickReplies(t.as_ref().clone())
  }

  pub fn start_page<T: AsRef<BusinessFeatureStartPage>>(t: T) -> Self {
    BusinessFeature::StartPage(t.as_ref().clone())
  }

  pub fn upgraded_stories<T: AsRef<BusinessFeatureUpgradedStories>>(t: T) -> Self {
    BusinessFeature::UpgradedStories(t.as_ref().clone())
  }
}

impl AsRef<BusinessFeature> for BusinessFeature {
  fn as_ref(&self) -> &BusinessFeature {
    self
  }
}

/// The ability to create links to the business account with predefined message text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessFeatureAccountLinks {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for BusinessFeatureAccountLinks {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessFeatureAccountLinks"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDBusinessFeature for BusinessFeatureAccountLinks {}

impl BusinessFeatureAccountLinks {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessFeatureAccountLinksBuilder {
    let inner = BusinessFeatureAccountLinks {
      td_name: "businessFeatureAccountLinks".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDBusinessFeatureAccountLinksBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDBusinessFeatureAccountLinksBuilder {
  inner: BusinessFeatureAccountLinks,
}

impl RTDBusinessFeatureAccountLinksBuilder {
  pub fn build(&self) -> BusinessFeatureAccountLinks {
    self.inner.clone()
  }
}

impl AsRef<BusinessFeatureAccountLinks> for BusinessFeatureAccountLinks {
  fn as_ref(&self) -> &BusinessFeatureAccountLinks {
    self
  }
}

impl AsRef<BusinessFeatureAccountLinks> for RTDBusinessFeatureAccountLinksBuilder {
  fn as_ref(&self) -> &BusinessFeatureAccountLinks {
    &self.inner
  }
}

/// The ability to set up an away message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessFeatureAwayMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for BusinessFeatureAwayMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessFeatureAwayMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDBusinessFeature for BusinessFeatureAwayMessage {}

impl BusinessFeatureAwayMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessFeatureAwayMessageBuilder {
    let inner = BusinessFeatureAwayMessage {
      td_name: "businessFeatureAwayMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDBusinessFeatureAwayMessageBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDBusinessFeatureAwayMessageBuilder {
  inner: BusinessFeatureAwayMessage,
}

impl RTDBusinessFeatureAwayMessageBuilder {
  pub fn build(&self) -> BusinessFeatureAwayMessage {
    self.inner.clone()
  }
}

impl AsRef<BusinessFeatureAwayMessage> for BusinessFeatureAwayMessage {
  fn as_ref(&self) -> &BusinessFeatureAwayMessage {
    self
  }
}

impl AsRef<BusinessFeatureAwayMessage> for RTDBusinessFeatureAwayMessageBuilder {
  fn as_ref(&self) -> &BusinessFeatureAwayMessage {
    &self.inner
  }
}

/// The ability to connect a bot to the account
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessFeatureBots {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for BusinessFeatureBots {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessFeatureBots"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDBusinessFeature for BusinessFeatureBots {}

impl BusinessFeatureBots {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessFeatureBotsBuilder {
    let inner = BusinessFeatureBots {
      td_name: "businessFeatureBots".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDBusinessFeatureBotsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDBusinessFeatureBotsBuilder {
  inner: BusinessFeatureBots,
}

impl RTDBusinessFeatureBotsBuilder {
  pub fn build(&self) -> BusinessFeatureBots {
    self.inner.clone()
  }
}

impl AsRef<BusinessFeatureBots> for BusinessFeatureBots {
  fn as_ref(&self) -> &BusinessFeatureBots {
    self
  }
}

impl AsRef<BusinessFeatureBots> for RTDBusinessFeatureBotsBuilder {
  fn as_ref(&self) -> &BusinessFeatureBots {
    &self.inner
  }
}

/// The ability to display folder names for each chat in the chat list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessFeatureChatFolderTags {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for BusinessFeatureChatFolderTags {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessFeatureChatFolderTags"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDBusinessFeature for BusinessFeatureChatFolderTags {}

impl BusinessFeatureChatFolderTags {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessFeatureChatFolderTagsBuilder {
    let inner = BusinessFeatureChatFolderTags {
      td_name: "businessFeatureChatFolderTags".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDBusinessFeatureChatFolderTagsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDBusinessFeatureChatFolderTagsBuilder {
  inner: BusinessFeatureChatFolderTags,
}

impl RTDBusinessFeatureChatFolderTagsBuilder {
  pub fn build(&self) -> BusinessFeatureChatFolderTags {
    self.inner.clone()
  }
}

impl AsRef<BusinessFeatureChatFolderTags> for BusinessFeatureChatFolderTags {
  fn as_ref(&self) -> &BusinessFeatureChatFolderTags {
    self
  }
}

impl AsRef<BusinessFeatureChatFolderTags> for RTDBusinessFeatureChatFolderTagsBuilder {
  fn as_ref(&self) -> &BusinessFeatureChatFolderTags {
    &self.inner
  }
}

/// The ability to show an emoji status along with the business name
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessFeatureEmojiStatus {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for BusinessFeatureEmojiStatus {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessFeatureEmojiStatus"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDBusinessFeature for BusinessFeatureEmojiStatus {}

impl BusinessFeatureEmojiStatus {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessFeatureEmojiStatusBuilder {
    let inner = BusinessFeatureEmojiStatus {
      td_name: "businessFeatureEmojiStatus".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDBusinessFeatureEmojiStatusBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDBusinessFeatureEmojiStatusBuilder {
  inner: BusinessFeatureEmojiStatus,
}

impl RTDBusinessFeatureEmojiStatusBuilder {
  pub fn build(&self) -> BusinessFeatureEmojiStatus {
    self.inner.clone()
  }
}

impl AsRef<BusinessFeatureEmojiStatus> for BusinessFeatureEmojiStatus {
  fn as_ref(&self) -> &BusinessFeatureEmojiStatus {
    self
  }
}

impl AsRef<BusinessFeatureEmojiStatus> for RTDBusinessFeatureEmojiStatusBuilder {
  fn as_ref(&self) -> &BusinessFeatureEmojiStatus {
    &self.inner
  }
}

/// The ability to set up a greeting message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessFeatureGreetingMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for BusinessFeatureGreetingMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessFeatureGreetingMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDBusinessFeature for BusinessFeatureGreetingMessage {}

impl BusinessFeatureGreetingMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessFeatureGreetingMessageBuilder {
    let inner = BusinessFeatureGreetingMessage {
      td_name: "businessFeatureGreetingMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDBusinessFeatureGreetingMessageBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDBusinessFeatureGreetingMessageBuilder {
  inner: BusinessFeatureGreetingMessage,
}

impl RTDBusinessFeatureGreetingMessageBuilder {
  pub fn build(&self) -> BusinessFeatureGreetingMessage {
    self.inner.clone()
  }
}

impl AsRef<BusinessFeatureGreetingMessage> for BusinessFeatureGreetingMessage {
  fn as_ref(&self) -> &BusinessFeatureGreetingMessage {
    self
  }
}

impl AsRef<BusinessFeatureGreetingMessage> for RTDBusinessFeatureGreetingMessageBuilder {
  fn as_ref(&self) -> &BusinessFeatureGreetingMessage {
    &self.inner
  }
}

/// The ability to set location
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessFeatureLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for BusinessFeatureLocation {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessFeatureLocation"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDBusinessFeature for BusinessFeatureLocation {}

impl BusinessFeatureLocation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessFeatureLocationBuilder {
    let inner = BusinessFeatureLocation {
      td_name: "businessFeatureLocation".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDBusinessFeatureLocationBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDBusinessFeatureLocationBuilder {
  inner: BusinessFeatureLocation,
}

impl RTDBusinessFeatureLocationBuilder {
  pub fn build(&self) -> BusinessFeatureLocation {
    self.inner.clone()
  }
}

impl AsRef<BusinessFeatureLocation> for BusinessFeatureLocation {
  fn as_ref(&self) -> &BusinessFeatureLocation {
    self
  }
}

impl AsRef<BusinessFeatureLocation> for RTDBusinessFeatureLocationBuilder {
  fn as_ref(&self) -> &BusinessFeatureLocation {
    &self.inner
  }
}

/// The ability to set opening hours
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessFeatureOpeningHours {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for BusinessFeatureOpeningHours {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessFeatureOpeningHours"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDBusinessFeature for BusinessFeatureOpeningHours {}

impl BusinessFeatureOpeningHours {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessFeatureOpeningHoursBuilder {
    let inner = BusinessFeatureOpeningHours {
      td_name: "businessFeatureOpeningHours".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDBusinessFeatureOpeningHoursBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDBusinessFeatureOpeningHoursBuilder {
  inner: BusinessFeatureOpeningHours,
}

impl RTDBusinessFeatureOpeningHoursBuilder {
  pub fn build(&self) -> BusinessFeatureOpeningHours {
    self.inner.clone()
  }
}

impl AsRef<BusinessFeatureOpeningHours> for BusinessFeatureOpeningHours {
  fn as_ref(&self) -> &BusinessFeatureOpeningHours {
    self
  }
}

impl AsRef<BusinessFeatureOpeningHours> for RTDBusinessFeatureOpeningHoursBuilder {
  fn as_ref(&self) -> &BusinessFeatureOpeningHours {
    &self.inner
  }
}

/// The ability to use quick replies
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessFeatureQuickReplies {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for BusinessFeatureQuickReplies {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessFeatureQuickReplies"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDBusinessFeature for BusinessFeatureQuickReplies {}

impl BusinessFeatureQuickReplies {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessFeatureQuickRepliesBuilder {
    let inner = BusinessFeatureQuickReplies {
      td_name: "businessFeatureQuickReplies".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDBusinessFeatureQuickRepliesBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDBusinessFeatureQuickRepliesBuilder {
  inner: BusinessFeatureQuickReplies,
}

impl RTDBusinessFeatureQuickRepliesBuilder {
  pub fn build(&self) -> BusinessFeatureQuickReplies {
    self.inner.clone()
  }
}

impl AsRef<BusinessFeatureQuickReplies> for BusinessFeatureQuickReplies {
  fn as_ref(&self) -> &BusinessFeatureQuickReplies {
    self
  }
}

impl AsRef<BusinessFeatureQuickReplies> for RTDBusinessFeatureQuickRepliesBuilder {
  fn as_ref(&self) -> &BusinessFeatureQuickReplies {
    &self.inner
  }
}

/// The ability to customize start page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessFeatureStartPage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for BusinessFeatureStartPage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessFeatureStartPage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDBusinessFeature for BusinessFeatureStartPage {}

impl BusinessFeatureStartPage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessFeatureStartPageBuilder {
    let inner = BusinessFeatureStartPage {
      td_name: "businessFeatureStartPage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDBusinessFeatureStartPageBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDBusinessFeatureStartPageBuilder {
  inner: BusinessFeatureStartPage,
}

impl RTDBusinessFeatureStartPageBuilder {
  pub fn build(&self) -> BusinessFeatureStartPage {
    self.inner.clone()
  }
}

impl AsRef<BusinessFeatureStartPage> for BusinessFeatureStartPage {
  fn as_ref(&self) -> &BusinessFeatureStartPage {
    self
  }
}

impl AsRef<BusinessFeatureStartPage> for RTDBusinessFeatureStartPageBuilder {
  fn as_ref(&self) -> &BusinessFeatureStartPage {
    &self.inner
  }
}

/// Allowed to use many additional features for stories
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessFeatureUpgradedStories {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for BusinessFeatureUpgradedStories {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessFeatureUpgradedStories"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDBusinessFeature for BusinessFeatureUpgradedStories {}

impl BusinessFeatureUpgradedStories {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessFeatureUpgradedStoriesBuilder {
    let inner = BusinessFeatureUpgradedStories {
      td_name: "businessFeatureUpgradedStories".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDBusinessFeatureUpgradedStoriesBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDBusinessFeatureUpgradedStoriesBuilder {
  inner: BusinessFeatureUpgradedStories,
}

impl RTDBusinessFeatureUpgradedStoriesBuilder {
  pub fn build(&self) -> BusinessFeatureUpgradedStories {
    self.inner.clone()
  }
}

impl AsRef<BusinessFeatureUpgradedStories> for BusinessFeatureUpgradedStories {
  fn as_ref(&self) -> &BusinessFeatureUpgradedStories {
    self
  }
}

impl AsRef<BusinessFeatureUpgradedStories> for RTDBusinessFeatureUpgradedStoriesBuilder {
  fn as_ref(&self) -> &BusinessFeatureUpgradedStories {
    &self.inner
  }
}
