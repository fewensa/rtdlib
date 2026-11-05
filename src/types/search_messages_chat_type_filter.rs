use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Represents a filter for type of the chats in which to search for messages
pub trait TDSearchMessagesChatTypeFilter: Debug + RObject {}

/// Represents a filter for type of the chats in which to search for messages
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum SearchMessagesChatTypeFilter {
  #[doc(hidden)]
  _Default(()),
  /// Returns only messages in channel chats
  Channel(SearchMessagesChatTypeFilterChannel),
  /// Returns only messages in the specified community
  Community(SearchMessagesChatTypeFilterCommunity),
  /// Returns only messages in basic group and supergroup chats
  Group(SearchMessagesChatTypeFilterGroup),
  /// Returns only messages in private chats
  Private(SearchMessagesChatTypeFilterPrivate),
}

impl Default for SearchMessagesChatTypeFilter {
  fn default() -> Self {
    SearchMessagesChatTypeFilter::_Default(())
  }
}

impl<'de> Deserialize<'de> for SearchMessagesChatTypeFilter {
  fn deserialize<D>(deserializer: D) -> Result<SearchMessagesChatTypeFilter, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      SearchMessagesChatTypeFilter,
      (searchMessagesChatTypeFilterChannel, Channel);
      (searchMessagesChatTypeFilterCommunity, Community);
      (searchMessagesChatTypeFilterGroup, Group);
      (searchMessagesChatTypeFilterPrivate, Private);

    )(deserializer)
  }
}

impl RObject for SearchMessagesChatTypeFilter {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      SearchMessagesChatTypeFilter::Channel(t) => t.td_name(),
      SearchMessagesChatTypeFilter::Community(t) => t.td_name(),
      SearchMessagesChatTypeFilter::Group(t) => t.td_name(),
      SearchMessagesChatTypeFilter::Private(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      SearchMessagesChatTypeFilter::Channel(t) => t.extra(),
      SearchMessagesChatTypeFilter::Community(t) => t.extra(),
      SearchMessagesChatTypeFilter::Group(t) => t.extra(),
      SearchMessagesChatTypeFilter::Private(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl SearchMessagesChatTypeFilter {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, SearchMessagesChatTypeFilter::_Default(_))
  }

  pub fn is_channel(&self) -> bool {
    matches!(self, SearchMessagesChatTypeFilter::Channel(_))
  }
  pub fn is_community(&self) -> bool {
    matches!(self, SearchMessagesChatTypeFilter::Community(_))
  }
  pub fn is_group(&self) -> bool {
    matches!(self, SearchMessagesChatTypeFilter::Group(_))
  }
  pub fn is_private(&self) -> bool {
    matches!(self, SearchMessagesChatTypeFilter::Private(_))
  }

  pub fn on_channel<F: FnOnce(&SearchMessagesChatTypeFilterChannel)>(&self, fnc: F) -> &Self {
    if let SearchMessagesChatTypeFilter::Channel(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_community<F: FnOnce(&SearchMessagesChatTypeFilterCommunity)>(&self, fnc: F) -> &Self {
    if let SearchMessagesChatTypeFilter::Community(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_group<F: FnOnce(&SearchMessagesChatTypeFilterGroup)>(&self, fnc: F) -> &Self {
    if let SearchMessagesChatTypeFilter::Group(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_private<F: FnOnce(&SearchMessagesChatTypeFilterPrivate)>(&self, fnc: F) -> &Self {
    if let SearchMessagesChatTypeFilter::Private(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_channel(&self) -> Option<&SearchMessagesChatTypeFilterChannel> {
    if let SearchMessagesChatTypeFilter::Channel(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_community(&self) -> Option<&SearchMessagesChatTypeFilterCommunity> {
    if let SearchMessagesChatTypeFilter::Community(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_group(&self) -> Option<&SearchMessagesChatTypeFilterGroup> {
    if let SearchMessagesChatTypeFilter::Group(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_private(&self) -> Option<&SearchMessagesChatTypeFilterPrivate> {
    if let SearchMessagesChatTypeFilter::Private(t) = self {
      return Some(t);
    }
    None
  }

  pub fn channel<T: AsRef<SearchMessagesChatTypeFilterChannel>>(t: T) -> Self {
    SearchMessagesChatTypeFilter::Channel(t.as_ref().clone())
  }

  pub fn community<T: AsRef<SearchMessagesChatTypeFilterCommunity>>(t: T) -> Self {
    SearchMessagesChatTypeFilter::Community(t.as_ref().clone())
  }

  pub fn group<T: AsRef<SearchMessagesChatTypeFilterGroup>>(t: T) -> Self {
    SearchMessagesChatTypeFilter::Group(t.as_ref().clone())
  }

  pub fn private<T: AsRef<SearchMessagesChatTypeFilterPrivate>>(t: T) -> Self {
    SearchMessagesChatTypeFilter::Private(t.as_ref().clone())
  }
}

impl AsRef<SearchMessagesChatTypeFilter> for SearchMessagesChatTypeFilter {
  fn as_ref(&self) -> &SearchMessagesChatTypeFilter {
    self
  }
}

/// Returns only messages in channel chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesChatTypeFilterChannel {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SearchMessagesChatTypeFilterChannel {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "searchMessagesChatTypeFilterChannel"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSearchMessagesChatTypeFilter for SearchMessagesChatTypeFilterChannel {}

impl SearchMessagesChatTypeFilterChannel {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSearchMessagesChatTypeFilterChannelBuilder {
    let inner = SearchMessagesChatTypeFilterChannel {
      td_name: "searchMessagesChatTypeFilterChannel".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSearchMessagesChatTypeFilterChannelBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSearchMessagesChatTypeFilterChannelBuilder {
  inner: SearchMessagesChatTypeFilterChannel,
}

impl RTDSearchMessagesChatTypeFilterChannelBuilder {
  pub fn build(&self) -> SearchMessagesChatTypeFilterChannel {
    self.inner.clone()
  }
}

impl AsRef<SearchMessagesChatTypeFilterChannel> for SearchMessagesChatTypeFilterChannel {
  fn as_ref(&self) -> &SearchMessagesChatTypeFilterChannel {
    self
  }
}

impl AsRef<SearchMessagesChatTypeFilterChannel> for RTDSearchMessagesChatTypeFilterChannelBuilder {
  fn as_ref(&self) -> &SearchMessagesChatTypeFilterChannel {
    &self.inner
  }
}

/// Returns only messages in the specified community
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesChatTypeFilterCommunity {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the community to search in
  community_id: i64,
}

impl RObject for SearchMessagesChatTypeFilterCommunity {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "searchMessagesChatTypeFilterCommunity"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSearchMessagesChatTypeFilter for SearchMessagesChatTypeFilterCommunity {}

impl SearchMessagesChatTypeFilterCommunity {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSearchMessagesChatTypeFilterCommunityBuilder {
    let inner = SearchMessagesChatTypeFilterCommunity {
      td_name: "searchMessagesChatTypeFilterCommunity".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSearchMessagesChatTypeFilterCommunityBuilder { inner }
  }

  pub fn community_id(&self) -> i64 {
    self.community_id
  }
}

#[doc(hidden)]
pub struct RTDSearchMessagesChatTypeFilterCommunityBuilder {
  inner: SearchMessagesChatTypeFilterCommunity,
}

impl RTDSearchMessagesChatTypeFilterCommunityBuilder {
  pub fn build(&self) -> SearchMessagesChatTypeFilterCommunity {
    self.inner.clone()
  }

  pub fn community_id(&mut self, community_id: i64) -> &mut Self {
    self.inner.community_id = community_id;
    self
  }
}

impl AsRef<SearchMessagesChatTypeFilterCommunity> for SearchMessagesChatTypeFilterCommunity {
  fn as_ref(&self) -> &SearchMessagesChatTypeFilterCommunity {
    self
  }
}

impl AsRef<SearchMessagesChatTypeFilterCommunity>
  for RTDSearchMessagesChatTypeFilterCommunityBuilder
{
  fn as_ref(&self) -> &SearchMessagesChatTypeFilterCommunity {
    &self.inner
  }
}

/// Returns only messages in basic group and supergroup chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesChatTypeFilterGroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SearchMessagesChatTypeFilterGroup {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "searchMessagesChatTypeFilterGroup"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSearchMessagesChatTypeFilter for SearchMessagesChatTypeFilterGroup {}

impl SearchMessagesChatTypeFilterGroup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSearchMessagesChatTypeFilterGroupBuilder {
    let inner = SearchMessagesChatTypeFilterGroup {
      td_name: "searchMessagesChatTypeFilterGroup".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSearchMessagesChatTypeFilterGroupBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSearchMessagesChatTypeFilterGroupBuilder {
  inner: SearchMessagesChatTypeFilterGroup,
}

impl RTDSearchMessagesChatTypeFilterGroupBuilder {
  pub fn build(&self) -> SearchMessagesChatTypeFilterGroup {
    self.inner.clone()
  }
}

impl AsRef<SearchMessagesChatTypeFilterGroup> for SearchMessagesChatTypeFilterGroup {
  fn as_ref(&self) -> &SearchMessagesChatTypeFilterGroup {
    self
  }
}

impl AsRef<SearchMessagesChatTypeFilterGroup> for RTDSearchMessagesChatTypeFilterGroupBuilder {
  fn as_ref(&self) -> &SearchMessagesChatTypeFilterGroup {
    &self.inner
  }
}

/// Returns only messages in private chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesChatTypeFilterPrivate {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SearchMessagesChatTypeFilterPrivate {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "searchMessagesChatTypeFilterPrivate"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSearchMessagesChatTypeFilter for SearchMessagesChatTypeFilterPrivate {}

impl SearchMessagesChatTypeFilterPrivate {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSearchMessagesChatTypeFilterPrivateBuilder {
    let inner = SearchMessagesChatTypeFilterPrivate {
      td_name: "searchMessagesChatTypeFilterPrivate".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSearchMessagesChatTypeFilterPrivateBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSearchMessagesChatTypeFilterPrivateBuilder {
  inner: SearchMessagesChatTypeFilterPrivate,
}

impl RTDSearchMessagesChatTypeFilterPrivateBuilder {
  pub fn build(&self) -> SearchMessagesChatTypeFilterPrivate {
    self.inner.clone()
  }
}

impl AsRef<SearchMessagesChatTypeFilterPrivate> for SearchMessagesChatTypeFilterPrivate {
  fn as_ref(&self) -> &SearchMessagesChatTypeFilterPrivate {
    self
  }
}

impl AsRef<SearchMessagesChatTypeFilterPrivate> for RTDSearchMessagesChatTypeFilterPrivateBuilder {
  fn as_ref(&self) -> &SearchMessagesChatTypeFilterPrivate {
    &self.inner
  }
}
