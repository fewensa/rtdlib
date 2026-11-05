use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Represents a filter for type of the chats to search for
pub trait TDSearchChatTypeFilter: Debug + RObject {}

/// Represents a filter for type of the chats to search for
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum SearchChatTypeFilter {
  #[doc(hidden)]
  _Default(()),
  /// Returns only private chats with bots
  Bot(SearchChatTypeFilterBot),
  /// Returns only channel chats
  Channel(SearchChatTypeFilterChannel),
}

impl Default for SearchChatTypeFilter {
  fn default() -> Self {
    SearchChatTypeFilter::_Default(())
  }
}

impl<'de> Deserialize<'de> for SearchChatTypeFilter {
  fn deserialize<D>(deserializer: D) -> Result<SearchChatTypeFilter, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      SearchChatTypeFilter,
      (searchChatTypeFilterBot, Bot);
      (searchChatTypeFilterChannel, Channel);

    )(deserializer)
  }
}

impl RObject for SearchChatTypeFilter {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      SearchChatTypeFilter::Bot(t) => t.td_name(),
      SearchChatTypeFilter::Channel(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      SearchChatTypeFilter::Bot(t) => t.extra(),
      SearchChatTypeFilter::Channel(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl SearchChatTypeFilter {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, SearchChatTypeFilter::_Default(_))
  }

  pub fn is_bot(&self) -> bool {
    matches!(self, SearchChatTypeFilter::Bot(_))
  }
  pub fn is_channel(&self) -> bool {
    matches!(self, SearchChatTypeFilter::Channel(_))
  }

  pub fn on_bot<F: FnOnce(&SearchChatTypeFilterBot)>(&self, fnc: F) -> &Self {
    if let SearchChatTypeFilter::Bot(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_channel<F: FnOnce(&SearchChatTypeFilterChannel)>(&self, fnc: F) -> &Self {
    if let SearchChatTypeFilter::Channel(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_bot(&self) -> Option<&SearchChatTypeFilterBot> {
    if let SearchChatTypeFilter::Bot(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_channel(&self) -> Option<&SearchChatTypeFilterChannel> {
    if let SearchChatTypeFilter::Channel(t) = self {
      return Some(t);
    }
    None
  }

  pub fn bot<T: AsRef<SearchChatTypeFilterBot>>(t: T) -> Self {
    SearchChatTypeFilter::Bot(t.as_ref().clone())
  }

  pub fn channel<T: AsRef<SearchChatTypeFilterChannel>>(t: T) -> Self {
    SearchChatTypeFilter::Channel(t.as_ref().clone())
  }
}

impl AsRef<SearchChatTypeFilter> for SearchChatTypeFilter {
  fn as_ref(&self) -> &SearchChatTypeFilter {
    self
  }
}

/// Returns only private chats with bots
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchChatTypeFilterBot {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SearchChatTypeFilterBot {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "searchChatTypeFilterBot"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSearchChatTypeFilter for SearchChatTypeFilterBot {}

impl SearchChatTypeFilterBot {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSearchChatTypeFilterBotBuilder {
    let inner = SearchChatTypeFilterBot {
      td_name: "searchChatTypeFilterBot".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSearchChatTypeFilterBotBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSearchChatTypeFilterBotBuilder {
  inner: SearchChatTypeFilterBot,
}

impl RTDSearchChatTypeFilterBotBuilder {
  pub fn build(&self) -> SearchChatTypeFilterBot {
    self.inner.clone()
  }
}

impl AsRef<SearchChatTypeFilterBot> for SearchChatTypeFilterBot {
  fn as_ref(&self) -> &SearchChatTypeFilterBot {
    self
  }
}

impl AsRef<SearchChatTypeFilterBot> for RTDSearchChatTypeFilterBotBuilder {
  fn as_ref(&self) -> &SearchChatTypeFilterBot {
    &self.inner
  }
}

/// Returns only channel chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchChatTypeFilterChannel {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SearchChatTypeFilterChannel {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "searchChatTypeFilterChannel"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSearchChatTypeFilter for SearchChatTypeFilterChannel {}

impl SearchChatTypeFilterChannel {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSearchChatTypeFilterChannelBuilder {
    let inner = SearchChatTypeFilterChannel {
      td_name: "searchChatTypeFilterChannel".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSearchChatTypeFilterChannelBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSearchChatTypeFilterChannelBuilder {
  inner: SearchChatTypeFilterChannel,
}

impl RTDSearchChatTypeFilterChannelBuilder {
  pub fn build(&self) -> SearchChatTypeFilterChannel {
    self.inner.clone()
  }
}

impl AsRef<SearchChatTypeFilterChannel> for SearchChatTypeFilterChannel {
  fn as_ref(&self) -> &SearchChatTypeFilterChannel {
    self
  }
}

impl AsRef<SearchChatTypeFilterChannel> for RTDSearchChatTypeFilterChannelBuilder {
  fn as_ref(&self) -> &SearchChatTypeFilterChannel {
    &self.inner
  }
}
