use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes precision with which to show a date or a time
pub trait TDDateTimePartPrecision: Debug + RObject {}

/// Describes precision with which to show a date or a time
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum DateTimePartPrecision {
  #[doc(hidden)]
  _Default(()),
  /// Show the date or time in a long way (March 17, 2022 or 22:45:00)
  Long(DateTimePartPrecisionLong),
  /// Don't show the date or time
  None(DateTimePartPrecisionNone),
  /// Show the date or time in a short way (17.03.22 or 22:45)
  Short(DateTimePartPrecisionShort),
}

impl Default for DateTimePartPrecision {
  fn default() -> Self {
    DateTimePartPrecision::_Default(())
  }
}

impl<'de> Deserialize<'de> for DateTimePartPrecision {
  fn deserialize<D>(deserializer: D) -> Result<DateTimePartPrecision, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      DateTimePartPrecision,
      (dateTimePartPrecisionLong, Long);
      (dateTimePartPrecisionNone, None);
      (dateTimePartPrecisionShort, Short);

    )(deserializer)
  }
}

impl RObject for DateTimePartPrecision {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      DateTimePartPrecision::Long(t) => t.td_name(),
      DateTimePartPrecision::None(t) => t.td_name(),
      DateTimePartPrecision::Short(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      DateTimePartPrecision::Long(t) => t.extra(),
      DateTimePartPrecision::None(t) => t.extra(),
      DateTimePartPrecision::Short(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl DateTimePartPrecision {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, DateTimePartPrecision::_Default(_))
  }

  pub fn is_long(&self) -> bool {
    matches!(self, DateTimePartPrecision::Long(_))
  }
  pub fn is_none(&self) -> bool {
    matches!(self, DateTimePartPrecision::None(_))
  }
  pub fn is_short(&self) -> bool {
    matches!(self, DateTimePartPrecision::Short(_))
  }

  pub fn on_long<F: FnOnce(&DateTimePartPrecisionLong)>(&self, fnc: F) -> &Self {
    if let DateTimePartPrecision::Long(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_none<F: FnOnce(&DateTimePartPrecisionNone)>(&self, fnc: F) -> &Self {
    if let DateTimePartPrecision::None(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_short<F: FnOnce(&DateTimePartPrecisionShort)>(&self, fnc: F) -> &Self {
    if let DateTimePartPrecision::Short(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_long(&self) -> Option<&DateTimePartPrecisionLong> {
    if let DateTimePartPrecision::Long(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_none(&self) -> Option<&DateTimePartPrecisionNone> {
    if let DateTimePartPrecision::None(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_short(&self) -> Option<&DateTimePartPrecisionShort> {
    if let DateTimePartPrecision::Short(t) = self {
      return Some(t);
    }
    None
  }

  pub fn long<T: AsRef<DateTimePartPrecisionLong>>(t: T) -> Self {
    DateTimePartPrecision::Long(t.as_ref().clone())
  }

  pub fn none<T: AsRef<DateTimePartPrecisionNone>>(t: T) -> Self {
    DateTimePartPrecision::None(t.as_ref().clone())
  }

  pub fn short<T: AsRef<DateTimePartPrecisionShort>>(t: T) -> Self {
    DateTimePartPrecision::Short(t.as_ref().clone())
  }
}

impl AsRef<DateTimePartPrecision> for DateTimePartPrecision {
  fn as_ref(&self) -> &DateTimePartPrecision {
    self
  }
}

/// Show the date or time in a long way (March 17, 2022 or 22:45:00)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DateTimePartPrecisionLong {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for DateTimePartPrecisionLong {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "dateTimePartPrecisionLong"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDDateTimePartPrecision for DateTimePartPrecisionLong {}

impl DateTimePartPrecisionLong {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDateTimePartPrecisionLongBuilder {
    let inner = DateTimePartPrecisionLong {
      td_name: "dateTimePartPrecisionLong".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDDateTimePartPrecisionLongBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDDateTimePartPrecisionLongBuilder {
  inner: DateTimePartPrecisionLong,
}

impl RTDDateTimePartPrecisionLongBuilder {
  pub fn build(&self) -> DateTimePartPrecisionLong {
    self.inner.clone()
  }
}

impl AsRef<DateTimePartPrecisionLong> for DateTimePartPrecisionLong {
  fn as_ref(&self) -> &DateTimePartPrecisionLong {
    self
  }
}

impl AsRef<DateTimePartPrecisionLong> for RTDDateTimePartPrecisionLongBuilder {
  fn as_ref(&self) -> &DateTimePartPrecisionLong {
    &self.inner
  }
}

/// Don't show the date or time
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DateTimePartPrecisionNone {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for DateTimePartPrecisionNone {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "dateTimePartPrecisionNone"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDDateTimePartPrecision for DateTimePartPrecisionNone {}

impl DateTimePartPrecisionNone {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDateTimePartPrecisionNoneBuilder {
    let inner = DateTimePartPrecisionNone {
      td_name: "dateTimePartPrecisionNone".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDDateTimePartPrecisionNoneBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDDateTimePartPrecisionNoneBuilder {
  inner: DateTimePartPrecisionNone,
}

impl RTDDateTimePartPrecisionNoneBuilder {
  pub fn build(&self) -> DateTimePartPrecisionNone {
    self.inner.clone()
  }
}

impl AsRef<DateTimePartPrecisionNone> for DateTimePartPrecisionNone {
  fn as_ref(&self) -> &DateTimePartPrecisionNone {
    self
  }
}

impl AsRef<DateTimePartPrecisionNone> for RTDDateTimePartPrecisionNoneBuilder {
  fn as_ref(&self) -> &DateTimePartPrecisionNone {
    &self.inner
  }
}

/// Show the date or time in a short way (17.03.22 or 22:45)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DateTimePartPrecisionShort {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for DateTimePartPrecisionShort {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "dateTimePartPrecisionShort"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDDateTimePartPrecision for DateTimePartPrecisionShort {}

impl DateTimePartPrecisionShort {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDateTimePartPrecisionShortBuilder {
    let inner = DateTimePartPrecisionShort {
      td_name: "dateTimePartPrecisionShort".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDDateTimePartPrecisionShortBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDDateTimePartPrecisionShortBuilder {
  inner: DateTimePartPrecisionShort,
}

impl RTDDateTimePartPrecisionShortBuilder {
  pub fn build(&self) -> DateTimePartPrecisionShort {
    self.inner.clone()
  }
}

impl AsRef<DateTimePartPrecisionShort> for DateTimePartPrecisionShort {
  fn as_ref(&self) -> &DateTimePartPrecisionShort {
    self
  }
}

impl AsRef<DateTimePartPrecisionShort> for RTDDateTimePartPrecisionShortBuilder {
  fn as_ref(&self) -> &DateTimePartPrecisionShort {
    &self.inner
  }
}
