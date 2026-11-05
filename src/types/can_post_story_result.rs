use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Represents result of checking whether the current user can post a story on behalf of the specific chat
pub trait TDCanPostStoryResult: Debug + RObject {}

/// Represents result of checking whether the current user can post a story on behalf of the specific chat
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CanPostStoryResult {
  #[doc(hidden)]
  _Default(()),
  /// Checks whether the current user can post a story on behalf of a chat; requires can_post_stories administrator right for supergroup and channel chats
  CanPostStory(CanPostStory),
  /// The limit for the number of active stories exceeded. The user can buy Telegram Premium, delete an active story, or wait for the oldest story to expire
  ActiveStoryLimitExceeded(CanPostStoryResultActiveStoryLimitExceeded),
  /// The chat must be boosted first by Telegram Premium subscribers to post more stories. Call getChatBoostStatus to get current boost status of the chat
  BoostNeeded(CanPostStoryResultBoostNeeded),
  /// The user or the chat has an active live story. The live story must be deleted first
  LiveStoryIsActive(CanPostStoryResultLiveStoryIsActive),
  /// The monthly limit for the number of posted stories exceeded. The user needs to buy Telegram Premium or wait specified time
  MonthlyLimitExceeded(CanPostStoryResultMonthlyLimitExceeded),
  /// A story can be sent
  Ok(CanPostStoryResultOk),
  /// The user must subscribe to Telegram Premium to be able to post stories
  PremiumNeeded(CanPostStoryResultPremiumNeeded),
  /// The weekly limit for the number of posted stories exceeded. The user needs to buy Telegram Premium or wait specified time
  WeeklyLimitExceeded(CanPostStoryResultWeeklyLimitExceeded),
}

impl Default for CanPostStoryResult {
  fn default() -> Self {
    CanPostStoryResult::_Default(())
  }
}

impl<'de> Deserialize<'de> for CanPostStoryResult {
  fn deserialize<D>(deserializer: D) -> Result<CanPostStoryResult, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      CanPostStoryResult,
      (canPostStory, CanPostStory);
      (canPostStoryResultActiveStoryLimitExceeded, ActiveStoryLimitExceeded);
      (canPostStoryResultBoostNeeded, BoostNeeded);
      (canPostStoryResultLiveStoryIsActive, LiveStoryIsActive);
      (canPostStoryResultMonthlyLimitExceeded, MonthlyLimitExceeded);
      (canPostStoryResultOk, Ok);
      (canPostStoryResultPremiumNeeded, PremiumNeeded);
      (canPostStoryResultWeeklyLimitExceeded, WeeklyLimitExceeded);

    )(deserializer)
  }
}

impl RObject for CanPostStoryResult {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      CanPostStoryResult::CanPostStory(t) => t.td_name(),
      CanPostStoryResult::ActiveStoryLimitExceeded(t) => t.td_name(),
      CanPostStoryResult::BoostNeeded(t) => t.td_name(),
      CanPostStoryResult::LiveStoryIsActive(t) => t.td_name(),
      CanPostStoryResult::MonthlyLimitExceeded(t) => t.td_name(),
      CanPostStoryResult::Ok(t) => t.td_name(),
      CanPostStoryResult::PremiumNeeded(t) => t.td_name(),
      CanPostStoryResult::WeeklyLimitExceeded(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      CanPostStoryResult::CanPostStory(t) => t.extra(),
      CanPostStoryResult::ActiveStoryLimitExceeded(t) => t.extra(),
      CanPostStoryResult::BoostNeeded(t) => t.extra(),
      CanPostStoryResult::LiveStoryIsActive(t) => t.extra(),
      CanPostStoryResult::MonthlyLimitExceeded(t) => t.extra(),
      CanPostStoryResult::Ok(t) => t.extra(),
      CanPostStoryResult::PremiumNeeded(t) => t.extra(),
      CanPostStoryResult::WeeklyLimitExceeded(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl CanPostStoryResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, CanPostStoryResult::_Default(_))
  }

  pub fn is_can_post_story(&self) -> bool {
    matches!(self, CanPostStoryResult::CanPostStory(_))
  }
  pub fn is_active_story_limit_exceeded(&self) -> bool {
    matches!(self, CanPostStoryResult::ActiveStoryLimitExceeded(_))
  }
  pub fn is_boost_needed(&self) -> bool {
    matches!(self, CanPostStoryResult::BoostNeeded(_))
  }
  pub fn is_live_story_is_active(&self) -> bool {
    matches!(self, CanPostStoryResult::LiveStoryIsActive(_))
  }
  pub fn is_monthly_limit_exceeded(&self) -> bool {
    matches!(self, CanPostStoryResult::MonthlyLimitExceeded(_))
  }
  pub fn is_ok(&self) -> bool {
    matches!(self, CanPostStoryResult::Ok(_))
  }
  pub fn is_premium_needed(&self) -> bool {
    matches!(self, CanPostStoryResult::PremiumNeeded(_))
  }
  pub fn is_weekly_limit_exceeded(&self) -> bool {
    matches!(self, CanPostStoryResult::WeeklyLimitExceeded(_))
  }

  pub fn on_can_post_story<F: FnOnce(&CanPostStory)>(&self, fnc: F) -> &Self {
    if let CanPostStoryResult::CanPostStory(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_active_story_limit_exceeded<F: FnOnce(&CanPostStoryResultActiveStoryLimitExceeded)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let CanPostStoryResult::ActiveStoryLimitExceeded(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_boost_needed<F: FnOnce(&CanPostStoryResultBoostNeeded)>(&self, fnc: F) -> &Self {
    if let CanPostStoryResult::BoostNeeded(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_live_story_is_active<F: FnOnce(&CanPostStoryResultLiveStoryIsActive)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let CanPostStoryResult::LiveStoryIsActive(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_monthly_limit_exceeded<F: FnOnce(&CanPostStoryResultMonthlyLimitExceeded)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let CanPostStoryResult::MonthlyLimitExceeded(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ok<F: FnOnce(&CanPostStoryResultOk)>(&self, fnc: F) -> &Self {
    if let CanPostStoryResult::Ok(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_premium_needed<F: FnOnce(&CanPostStoryResultPremiumNeeded)>(&self, fnc: F) -> &Self {
    if let CanPostStoryResult::PremiumNeeded(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_weekly_limit_exceeded<F: FnOnce(&CanPostStoryResultWeeklyLimitExceeded)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let CanPostStoryResult::WeeklyLimitExceeded(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_can_post_story(&self) -> Option<&CanPostStory> {
    if let CanPostStoryResult::CanPostStory(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_active_story_limit_exceeded(
    &self,
  ) -> Option<&CanPostStoryResultActiveStoryLimitExceeded> {
    if let CanPostStoryResult::ActiveStoryLimitExceeded(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_boost_needed(&self) -> Option<&CanPostStoryResultBoostNeeded> {
    if let CanPostStoryResult::BoostNeeded(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_live_story_is_active(&self) -> Option<&CanPostStoryResultLiveStoryIsActive> {
    if let CanPostStoryResult::LiveStoryIsActive(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_monthly_limit_exceeded(&self) -> Option<&CanPostStoryResultMonthlyLimitExceeded> {
    if let CanPostStoryResult::MonthlyLimitExceeded(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ok(&self) -> Option<&CanPostStoryResultOk> {
    if let CanPostStoryResult::Ok(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_premium_needed(&self) -> Option<&CanPostStoryResultPremiumNeeded> {
    if let CanPostStoryResult::PremiumNeeded(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_weekly_limit_exceeded(&self) -> Option<&CanPostStoryResultWeeklyLimitExceeded> {
    if let CanPostStoryResult::WeeklyLimitExceeded(t) = self {
      return Some(t);
    }
    None
  }

  pub fn can_post_story<T: AsRef<CanPostStory>>(t: T) -> Self {
    CanPostStoryResult::CanPostStory(t.as_ref().clone())
  }

  pub fn active_story_limit_exceeded<T: AsRef<CanPostStoryResultActiveStoryLimitExceeded>>(
    t: T,
  ) -> Self {
    CanPostStoryResult::ActiveStoryLimitExceeded(t.as_ref().clone())
  }

  pub fn boost_needed<T: AsRef<CanPostStoryResultBoostNeeded>>(t: T) -> Self {
    CanPostStoryResult::BoostNeeded(t.as_ref().clone())
  }

  pub fn live_story_is_active<T: AsRef<CanPostStoryResultLiveStoryIsActive>>(t: T) -> Self {
    CanPostStoryResult::LiveStoryIsActive(t.as_ref().clone())
  }

  pub fn monthly_limit_exceeded<T: AsRef<CanPostStoryResultMonthlyLimitExceeded>>(t: T) -> Self {
    CanPostStoryResult::MonthlyLimitExceeded(t.as_ref().clone())
  }

  pub fn ok<T: AsRef<CanPostStoryResultOk>>(t: T) -> Self {
    CanPostStoryResult::Ok(t.as_ref().clone())
  }

  pub fn premium_needed<T: AsRef<CanPostStoryResultPremiumNeeded>>(t: T) -> Self {
    CanPostStoryResult::PremiumNeeded(t.as_ref().clone())
  }

  pub fn weekly_limit_exceeded<T: AsRef<CanPostStoryResultWeeklyLimitExceeded>>(t: T) -> Self {
    CanPostStoryResult::WeeklyLimitExceeded(t.as_ref().clone())
  }
}

impl AsRef<CanPostStoryResult> for CanPostStoryResult {
  fn as_ref(&self) -> &CanPostStoryResult {
    self
  }
}

/// The limit for the number of active stories exceeded. The user can buy Telegram Premium, delete an active story, or wait for the oldest story to expire
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanPostStoryResultActiveStoryLimitExceeded {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for CanPostStoryResultActiveStoryLimitExceeded {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "canPostStoryResultActiveStoryLimitExceeded"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCanPostStoryResult for CanPostStoryResultActiveStoryLimitExceeded {}

impl CanPostStoryResultActiveStoryLimitExceeded {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCanPostStoryResultActiveStoryLimitExceededBuilder {
    let inner = CanPostStoryResultActiveStoryLimitExceeded {
      td_name: "canPostStoryResultActiveStoryLimitExceeded".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDCanPostStoryResultActiveStoryLimitExceededBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDCanPostStoryResultActiveStoryLimitExceededBuilder {
  inner: CanPostStoryResultActiveStoryLimitExceeded,
}

impl RTDCanPostStoryResultActiveStoryLimitExceededBuilder {
  pub fn build(&self) -> CanPostStoryResultActiveStoryLimitExceeded {
    self.inner.clone()
  }
}

impl AsRef<CanPostStoryResultActiveStoryLimitExceeded>
  for CanPostStoryResultActiveStoryLimitExceeded
{
  fn as_ref(&self) -> &CanPostStoryResultActiveStoryLimitExceeded {
    self
  }
}

impl AsRef<CanPostStoryResultActiveStoryLimitExceeded>
  for RTDCanPostStoryResultActiveStoryLimitExceededBuilder
{
  fn as_ref(&self) -> &CanPostStoryResultActiveStoryLimitExceeded {
    &self.inner
  }
}

/// The chat must be boosted first by Telegram Premium subscribers to post more stories. Call getChatBoostStatus to get current boost status of the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanPostStoryResultBoostNeeded {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for CanPostStoryResultBoostNeeded {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "canPostStoryResultBoostNeeded"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCanPostStoryResult for CanPostStoryResultBoostNeeded {}

impl CanPostStoryResultBoostNeeded {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCanPostStoryResultBoostNeededBuilder {
    let inner = CanPostStoryResultBoostNeeded {
      td_name: "canPostStoryResultBoostNeeded".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDCanPostStoryResultBoostNeededBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDCanPostStoryResultBoostNeededBuilder {
  inner: CanPostStoryResultBoostNeeded,
}

impl RTDCanPostStoryResultBoostNeededBuilder {
  pub fn build(&self) -> CanPostStoryResultBoostNeeded {
    self.inner.clone()
  }
}

impl AsRef<CanPostStoryResultBoostNeeded> for CanPostStoryResultBoostNeeded {
  fn as_ref(&self) -> &CanPostStoryResultBoostNeeded {
    self
  }
}

impl AsRef<CanPostStoryResultBoostNeeded> for RTDCanPostStoryResultBoostNeededBuilder {
  fn as_ref(&self) -> &CanPostStoryResultBoostNeeded {
    &self.inner
  }
}

/// The user or the chat has an active live story. The live story must be deleted first
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanPostStoryResultLiveStoryIsActive {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the active live story
  story_id: i64,
}

impl RObject for CanPostStoryResultLiveStoryIsActive {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "canPostStoryResultLiveStoryIsActive"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCanPostStoryResult for CanPostStoryResultLiveStoryIsActive {}

impl CanPostStoryResultLiveStoryIsActive {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCanPostStoryResultLiveStoryIsActiveBuilder {
    let inner = CanPostStoryResultLiveStoryIsActive {
      td_name: "canPostStoryResultLiveStoryIsActive".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCanPostStoryResultLiveStoryIsActiveBuilder { inner }
  }

  pub fn story_id(&self) -> i64 {
    self.story_id
  }
}

#[doc(hidden)]
pub struct RTDCanPostStoryResultLiveStoryIsActiveBuilder {
  inner: CanPostStoryResultLiveStoryIsActive,
}

impl RTDCanPostStoryResultLiveStoryIsActiveBuilder {
  pub fn build(&self) -> CanPostStoryResultLiveStoryIsActive {
    self.inner.clone()
  }

  pub fn story_id(&mut self, story_id: i64) -> &mut Self {
    self.inner.story_id = story_id;
    self
  }
}

impl AsRef<CanPostStoryResultLiveStoryIsActive> for CanPostStoryResultLiveStoryIsActive {
  fn as_ref(&self) -> &CanPostStoryResultLiveStoryIsActive {
    self
  }
}

impl AsRef<CanPostStoryResultLiveStoryIsActive> for RTDCanPostStoryResultLiveStoryIsActiveBuilder {
  fn as_ref(&self) -> &CanPostStoryResultLiveStoryIsActive {
    &self.inner
  }
}

/// The monthly limit for the number of posted stories exceeded. The user needs to buy Telegram Premium or wait specified time
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanPostStoryResultMonthlyLimitExceeded {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Time left before the user can post the next story, in seconds
  retry_after: i64,
}

impl RObject for CanPostStoryResultMonthlyLimitExceeded {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "canPostStoryResultMonthlyLimitExceeded"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCanPostStoryResult for CanPostStoryResultMonthlyLimitExceeded {}

impl CanPostStoryResultMonthlyLimitExceeded {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCanPostStoryResultMonthlyLimitExceededBuilder {
    let inner = CanPostStoryResultMonthlyLimitExceeded {
      td_name: "canPostStoryResultMonthlyLimitExceeded".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCanPostStoryResultMonthlyLimitExceededBuilder { inner }
  }

  pub fn retry_after(&self) -> i64 {
    self.retry_after
  }
}

#[doc(hidden)]
pub struct RTDCanPostStoryResultMonthlyLimitExceededBuilder {
  inner: CanPostStoryResultMonthlyLimitExceeded,
}

impl RTDCanPostStoryResultMonthlyLimitExceededBuilder {
  pub fn build(&self) -> CanPostStoryResultMonthlyLimitExceeded {
    self.inner.clone()
  }

  pub fn retry_after(&mut self, retry_after: i64) -> &mut Self {
    self.inner.retry_after = retry_after;
    self
  }
}

impl AsRef<CanPostStoryResultMonthlyLimitExceeded> for CanPostStoryResultMonthlyLimitExceeded {
  fn as_ref(&self) -> &CanPostStoryResultMonthlyLimitExceeded {
    self
  }
}

impl AsRef<CanPostStoryResultMonthlyLimitExceeded>
  for RTDCanPostStoryResultMonthlyLimitExceededBuilder
{
  fn as_ref(&self) -> &CanPostStoryResultMonthlyLimitExceeded {
    &self.inner
  }
}

/// A story can be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanPostStoryResultOk {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Number of stories that can be posted by the user
  story_count: i64,
}

impl RObject for CanPostStoryResultOk {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "canPostStoryResultOk"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCanPostStoryResult for CanPostStoryResultOk {}

impl CanPostStoryResultOk {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCanPostStoryResultOkBuilder {
    let inner = CanPostStoryResultOk {
      td_name: "canPostStoryResultOk".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCanPostStoryResultOkBuilder { inner }
  }

  pub fn story_count(&self) -> i64 {
    self.story_count
  }
}

#[doc(hidden)]
pub struct RTDCanPostStoryResultOkBuilder {
  inner: CanPostStoryResultOk,
}

impl RTDCanPostStoryResultOkBuilder {
  pub fn build(&self) -> CanPostStoryResultOk {
    self.inner.clone()
  }

  pub fn story_count(&mut self, story_count: i64) -> &mut Self {
    self.inner.story_count = story_count;
    self
  }
}

impl AsRef<CanPostStoryResultOk> for CanPostStoryResultOk {
  fn as_ref(&self) -> &CanPostStoryResultOk {
    self
  }
}

impl AsRef<CanPostStoryResultOk> for RTDCanPostStoryResultOkBuilder {
  fn as_ref(&self) -> &CanPostStoryResultOk {
    &self.inner
  }
}

/// The user must subscribe to Telegram Premium to be able to post stories
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanPostStoryResultPremiumNeeded {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for CanPostStoryResultPremiumNeeded {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "canPostStoryResultPremiumNeeded"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCanPostStoryResult for CanPostStoryResultPremiumNeeded {}

impl CanPostStoryResultPremiumNeeded {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCanPostStoryResultPremiumNeededBuilder {
    let inner = CanPostStoryResultPremiumNeeded {
      td_name: "canPostStoryResultPremiumNeeded".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDCanPostStoryResultPremiumNeededBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDCanPostStoryResultPremiumNeededBuilder {
  inner: CanPostStoryResultPremiumNeeded,
}

impl RTDCanPostStoryResultPremiumNeededBuilder {
  pub fn build(&self) -> CanPostStoryResultPremiumNeeded {
    self.inner.clone()
  }
}

impl AsRef<CanPostStoryResultPremiumNeeded> for CanPostStoryResultPremiumNeeded {
  fn as_ref(&self) -> &CanPostStoryResultPremiumNeeded {
    self
  }
}

impl AsRef<CanPostStoryResultPremiumNeeded> for RTDCanPostStoryResultPremiumNeededBuilder {
  fn as_ref(&self) -> &CanPostStoryResultPremiumNeeded {
    &self.inner
  }
}

/// The weekly limit for the number of posted stories exceeded. The user needs to buy Telegram Premium or wait specified time
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanPostStoryResultWeeklyLimitExceeded {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Time left before the user can post the next story, in seconds
  retry_after: i64,
}

impl RObject for CanPostStoryResultWeeklyLimitExceeded {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "canPostStoryResultWeeklyLimitExceeded"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCanPostStoryResult for CanPostStoryResultWeeklyLimitExceeded {}

impl CanPostStoryResultWeeklyLimitExceeded {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCanPostStoryResultWeeklyLimitExceededBuilder {
    let inner = CanPostStoryResultWeeklyLimitExceeded {
      td_name: "canPostStoryResultWeeklyLimitExceeded".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCanPostStoryResultWeeklyLimitExceededBuilder { inner }
  }

  pub fn retry_after(&self) -> i64 {
    self.retry_after
  }
}

#[doc(hidden)]
pub struct RTDCanPostStoryResultWeeklyLimitExceededBuilder {
  inner: CanPostStoryResultWeeklyLimitExceeded,
}

impl RTDCanPostStoryResultWeeklyLimitExceededBuilder {
  pub fn build(&self) -> CanPostStoryResultWeeklyLimitExceeded {
    self.inner.clone()
  }

  pub fn retry_after(&mut self, retry_after: i64) -> &mut Self {
    self.inner.retry_after = retry_after;
    self
  }
}

impl AsRef<CanPostStoryResultWeeklyLimitExceeded> for CanPostStoryResultWeeklyLimitExceeded {
  fn as_ref(&self) -> &CanPostStoryResultWeeklyLimitExceeded {
    self
  }
}

impl AsRef<CanPostStoryResultWeeklyLimitExceeded>
  for RTDCanPostStoryResultWeeklyLimitExceededBuilder
{
  fn as_ref(&self) -> &CanPostStoryResultWeeklyLimitExceeded {
    &self.inner
  }
}
