use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes conditions for sending of away messages by a Telegram Business account
pub trait TDBusinessAwayMessageSchedule: Debug + RObject {}

/// Describes conditions for sending of away messages by a Telegram Business account
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum BusinessAwayMessageSchedule {
  #[doc(hidden)]
  _Default(()),
  /// Send away messages always
  Always(BusinessAwayMessageScheduleAlways),
  /// Send away messages only in the specified time span
  Custom(BusinessAwayMessageScheduleCustom),
  /// Send away messages outside of the business opening hours
  OutsideOfOpeningHours(BusinessAwayMessageScheduleOutsideOfOpeningHours),
}

impl Default for BusinessAwayMessageSchedule {
  fn default() -> Self {
    BusinessAwayMessageSchedule::_Default(())
  }
}

impl<'de> Deserialize<'de> for BusinessAwayMessageSchedule {
  fn deserialize<D>(deserializer: D) -> Result<BusinessAwayMessageSchedule, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      BusinessAwayMessageSchedule,
      (businessAwayMessageScheduleAlways, Always);
      (businessAwayMessageScheduleCustom, Custom);
      (businessAwayMessageScheduleOutsideOfOpeningHours, OutsideOfOpeningHours);

    )(deserializer)
  }
}

impl RObject for BusinessAwayMessageSchedule {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      BusinessAwayMessageSchedule::Always(t) => t.td_name(),
      BusinessAwayMessageSchedule::Custom(t) => t.td_name(),
      BusinessAwayMessageSchedule::OutsideOfOpeningHours(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      BusinessAwayMessageSchedule::Always(t) => t.extra(),
      BusinessAwayMessageSchedule::Custom(t) => t.extra(),
      BusinessAwayMessageSchedule::OutsideOfOpeningHours(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BusinessAwayMessageSchedule {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, BusinessAwayMessageSchedule::_Default(_))
  }

  pub fn is_always(&self) -> bool {
    matches!(self, BusinessAwayMessageSchedule::Always(_))
  }
  pub fn is_custom(&self) -> bool {
    matches!(self, BusinessAwayMessageSchedule::Custom(_))
  }
  pub fn is_outside_of_opening_hours(&self) -> bool {
    matches!(self, BusinessAwayMessageSchedule::OutsideOfOpeningHours(_))
  }

  pub fn on_always<F: FnOnce(&BusinessAwayMessageScheduleAlways)>(&self, fnc: F) -> &Self {
    if let BusinessAwayMessageSchedule::Always(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_custom<F: FnOnce(&BusinessAwayMessageScheduleCustom)>(&self, fnc: F) -> &Self {
    if let BusinessAwayMessageSchedule::Custom(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_outside_of_opening_hours<
    F: FnOnce(&BusinessAwayMessageScheduleOutsideOfOpeningHours),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let BusinessAwayMessageSchedule::OutsideOfOpeningHours(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_always(&self) -> Option<&BusinessAwayMessageScheduleAlways> {
    if let BusinessAwayMessageSchedule::Always(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_custom(&self) -> Option<&BusinessAwayMessageScheduleCustom> {
    if let BusinessAwayMessageSchedule::Custom(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_outside_of_opening_hours(
    &self,
  ) -> Option<&BusinessAwayMessageScheduleOutsideOfOpeningHours> {
    if let BusinessAwayMessageSchedule::OutsideOfOpeningHours(t) = self {
      return Some(t);
    }
    None
  }

  pub fn always<T: AsRef<BusinessAwayMessageScheduleAlways>>(t: T) -> Self {
    BusinessAwayMessageSchedule::Always(t.as_ref().clone())
  }

  pub fn custom<T: AsRef<BusinessAwayMessageScheduleCustom>>(t: T) -> Self {
    BusinessAwayMessageSchedule::Custom(t.as_ref().clone())
  }

  pub fn outside_of_opening_hours<T: AsRef<BusinessAwayMessageScheduleOutsideOfOpeningHours>>(
    t: T,
  ) -> Self {
    BusinessAwayMessageSchedule::OutsideOfOpeningHours(t.as_ref().clone())
  }
}

impl AsRef<BusinessAwayMessageSchedule> for BusinessAwayMessageSchedule {
  fn as_ref(&self) -> &BusinessAwayMessageSchedule {
    self
  }
}

/// Send away messages always
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessAwayMessageScheduleAlways {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for BusinessAwayMessageScheduleAlways {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessAwayMessageScheduleAlways"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDBusinessAwayMessageSchedule for BusinessAwayMessageScheduleAlways {}

impl BusinessAwayMessageScheduleAlways {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessAwayMessageScheduleAlwaysBuilder {
    let inner = BusinessAwayMessageScheduleAlways {
      td_name: "businessAwayMessageScheduleAlways".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDBusinessAwayMessageScheduleAlwaysBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDBusinessAwayMessageScheduleAlwaysBuilder {
  inner: BusinessAwayMessageScheduleAlways,
}

impl RTDBusinessAwayMessageScheduleAlwaysBuilder {
  pub fn build(&self) -> BusinessAwayMessageScheduleAlways {
    self.inner.clone()
  }
}

impl AsRef<BusinessAwayMessageScheduleAlways> for BusinessAwayMessageScheduleAlways {
  fn as_ref(&self) -> &BusinessAwayMessageScheduleAlways {
    self
  }
}

impl AsRef<BusinessAwayMessageScheduleAlways> for RTDBusinessAwayMessageScheduleAlwaysBuilder {
  fn as_ref(&self) -> &BusinessAwayMessageScheduleAlways {
    &self.inner
  }
}

/// Send away messages only in the specified time span
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessAwayMessageScheduleCustom {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Point in time (Unix timestamp) when the away messages will start to be sent
  start_date: i64,
  /// Point in time (Unix timestamp) when the away messages will stop to be sent
  end_date: i64,
}

impl RObject for BusinessAwayMessageScheduleCustom {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessAwayMessageScheduleCustom"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDBusinessAwayMessageSchedule for BusinessAwayMessageScheduleCustom {}

impl BusinessAwayMessageScheduleCustom {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessAwayMessageScheduleCustomBuilder {
    let inner = BusinessAwayMessageScheduleCustom {
      td_name: "businessAwayMessageScheduleCustom".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBusinessAwayMessageScheduleCustomBuilder { inner }
  }

  pub fn start_date(&self) -> i64 {
    self.start_date
  }

  pub fn end_date(&self) -> i64 {
    self.end_date
  }
}

#[doc(hidden)]
pub struct RTDBusinessAwayMessageScheduleCustomBuilder {
  inner: BusinessAwayMessageScheduleCustom,
}

impl RTDBusinessAwayMessageScheduleCustomBuilder {
  pub fn build(&self) -> BusinessAwayMessageScheduleCustom {
    self.inner.clone()
  }

  pub fn start_date(&mut self, start_date: i64) -> &mut Self {
    self.inner.start_date = start_date;
    self
  }

  pub fn end_date(&mut self, end_date: i64) -> &mut Self {
    self.inner.end_date = end_date;
    self
  }
}

impl AsRef<BusinessAwayMessageScheduleCustom> for BusinessAwayMessageScheduleCustom {
  fn as_ref(&self) -> &BusinessAwayMessageScheduleCustom {
    self
  }
}

impl AsRef<BusinessAwayMessageScheduleCustom> for RTDBusinessAwayMessageScheduleCustomBuilder {
  fn as_ref(&self) -> &BusinessAwayMessageScheduleCustom {
    &self.inner
  }
}

/// Send away messages outside of the business opening hours
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessAwayMessageScheduleOutsideOfOpeningHours {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for BusinessAwayMessageScheduleOutsideOfOpeningHours {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessAwayMessageScheduleOutsideOfOpeningHours"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDBusinessAwayMessageSchedule for BusinessAwayMessageScheduleOutsideOfOpeningHours {}

impl BusinessAwayMessageScheduleOutsideOfOpeningHours {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessAwayMessageScheduleOutsideOfOpeningHoursBuilder {
    let inner = BusinessAwayMessageScheduleOutsideOfOpeningHours {
      td_name: "businessAwayMessageScheduleOutsideOfOpeningHours".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDBusinessAwayMessageScheduleOutsideOfOpeningHoursBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDBusinessAwayMessageScheduleOutsideOfOpeningHoursBuilder {
  inner: BusinessAwayMessageScheduleOutsideOfOpeningHours,
}

impl RTDBusinessAwayMessageScheduleOutsideOfOpeningHoursBuilder {
  pub fn build(&self) -> BusinessAwayMessageScheduleOutsideOfOpeningHours {
    self.inner.clone()
  }
}

impl AsRef<BusinessAwayMessageScheduleOutsideOfOpeningHours>
  for BusinessAwayMessageScheduleOutsideOfOpeningHours
{
  fn as_ref(&self) -> &BusinessAwayMessageScheduleOutsideOfOpeningHours {
    self
  }
}

impl AsRef<BusinessAwayMessageScheduleOutsideOfOpeningHours>
  for RTDBusinessAwayMessageScheduleOutsideOfOpeningHoursBuilder
{
  fn as_ref(&self) -> &BusinessAwayMessageScheduleOutsideOfOpeningHours {
    &self.inner
  }
}
