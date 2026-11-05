use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes date and time formatting
pub trait TDDateTimeFormattingType: Debug + RObject {}

/// Describes date and time formatting
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum DateTimeFormattingType {
  #[doc(hidden)]
  _Default(()),
  /// The date and time must be shown as absolute timestamps
  Absolute(DateTimeFormattingTypeAbsolute),
  /// The time must be shown relative to the current time ([in ] X seconds, minutes, hours, days, months, years [ago])
  Relative(DateTimeFormattingTypeRelative),
}

impl Default for DateTimeFormattingType {
  fn default() -> Self {
    DateTimeFormattingType::_Default(())
  }
}

impl<'de> Deserialize<'de> for DateTimeFormattingType {
  fn deserialize<D>(deserializer: D) -> Result<DateTimeFormattingType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      DateTimeFormattingType,
      (dateTimeFormattingTypeAbsolute, Absolute);
      (dateTimeFormattingTypeRelative, Relative);

    )(deserializer)
  }
}

impl RObject for DateTimeFormattingType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      DateTimeFormattingType::Absolute(t) => t.td_name(),
      DateTimeFormattingType::Relative(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      DateTimeFormattingType::Absolute(t) => t.extra(),
      DateTimeFormattingType::Relative(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl DateTimeFormattingType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, DateTimeFormattingType::_Default(_))
  }

  pub fn is_absolute(&self) -> bool {
    matches!(self, DateTimeFormattingType::Absolute(_))
  }
  pub fn is_relative(&self) -> bool {
    matches!(self, DateTimeFormattingType::Relative(_))
  }

  pub fn on_absolute<F: FnOnce(&DateTimeFormattingTypeAbsolute)>(&self, fnc: F) -> &Self {
    if let DateTimeFormattingType::Absolute(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_relative<F: FnOnce(&DateTimeFormattingTypeRelative)>(&self, fnc: F) -> &Self {
    if let DateTimeFormattingType::Relative(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_absolute(&self) -> Option<&DateTimeFormattingTypeAbsolute> {
    if let DateTimeFormattingType::Absolute(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_relative(&self) -> Option<&DateTimeFormattingTypeRelative> {
    if let DateTimeFormattingType::Relative(t) = self {
      return Some(t);
    }
    None
  }

  pub fn absolute<T: AsRef<DateTimeFormattingTypeAbsolute>>(t: T) -> Self {
    DateTimeFormattingType::Absolute(t.as_ref().clone())
  }

  pub fn relative<T: AsRef<DateTimeFormattingTypeRelative>>(t: T) -> Self {
    DateTimeFormattingType::Relative(t.as_ref().clone())
  }
}

impl AsRef<DateTimeFormattingType> for DateTimeFormattingType {
  fn as_ref(&self) -> &DateTimeFormattingType {
    self
  }
}

/// The date and time must be shown as absolute timestamps
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DateTimeFormattingTypeAbsolute {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The precision with which hours, minutes and seconds are shown
  time_precision: DateTimePartPrecision,
  /// The precision with which the date is shown
  date_precision: DateTimePartPrecision,
  /// True, if the day of week must be shown
  show_day_of_week: bool,
}

impl RObject for DateTimeFormattingTypeAbsolute {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "dateTimeFormattingTypeAbsolute"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDDateTimeFormattingType for DateTimeFormattingTypeAbsolute {}

impl DateTimeFormattingTypeAbsolute {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDateTimeFormattingTypeAbsoluteBuilder {
    let inner = DateTimeFormattingTypeAbsolute {
      td_name: "dateTimeFormattingTypeAbsolute".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDDateTimeFormattingTypeAbsoluteBuilder { inner }
  }

  pub fn time_precision(&self) -> &DateTimePartPrecision {
    &self.time_precision
  }

  pub fn date_precision(&self) -> &DateTimePartPrecision {
    &self.date_precision
  }

  pub fn show_day_of_week(&self) -> bool {
    self.show_day_of_week
  }
}

#[doc(hidden)]
pub struct RTDDateTimeFormattingTypeAbsoluteBuilder {
  inner: DateTimeFormattingTypeAbsolute,
}

impl RTDDateTimeFormattingTypeAbsoluteBuilder {
  pub fn build(&self) -> DateTimeFormattingTypeAbsolute {
    self.inner.clone()
  }

  pub fn time_precision<T: AsRef<DateTimePartPrecision>>(
    &mut self,
    time_precision: T,
  ) -> &mut Self {
    self.inner.time_precision = time_precision.as_ref().clone();
    self
  }

  pub fn date_precision<T: AsRef<DateTimePartPrecision>>(
    &mut self,
    date_precision: T,
  ) -> &mut Self {
    self.inner.date_precision = date_precision.as_ref().clone();
    self
  }

  pub fn show_day_of_week(&mut self, show_day_of_week: bool) -> &mut Self {
    self.inner.show_day_of_week = show_day_of_week;
    self
  }
}

impl AsRef<DateTimeFormattingTypeAbsolute> for DateTimeFormattingTypeAbsolute {
  fn as_ref(&self) -> &DateTimeFormattingTypeAbsolute {
    self
  }
}

impl AsRef<DateTimeFormattingTypeAbsolute> for RTDDateTimeFormattingTypeAbsoluteBuilder {
  fn as_ref(&self) -> &DateTimeFormattingTypeAbsolute {
    &self.inner
  }
}

/// The time must be shown relative to the current time ([in ] X seconds, minutes, hours, days, months, years [ago])
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DateTimeFormattingTypeRelative {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for DateTimeFormattingTypeRelative {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "dateTimeFormattingTypeRelative"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDDateTimeFormattingType for DateTimeFormattingTypeRelative {}

impl DateTimeFormattingTypeRelative {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDateTimeFormattingTypeRelativeBuilder {
    let inner = DateTimeFormattingTypeRelative {
      td_name: "dateTimeFormattingTypeRelative".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDDateTimeFormattingTypeRelativeBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDDateTimeFormattingTypeRelativeBuilder {
  inner: DateTimeFormattingTypeRelative,
}

impl RTDDateTimeFormattingTypeRelativeBuilder {
  pub fn build(&self) -> DateTimeFormattingTypeRelative {
    self.inner.clone()
  }
}

impl AsRef<DateTimeFormattingTypeRelative> for DateTimeFormattingTypeRelative {
  fn as_ref(&self) -> &DateTimeFormattingTypeRelative {
    self
  }
}

impl AsRef<DateTimeFormattingTypeRelative> for RTDDateTimeFormattingTypeRelativeBuilder {
  fn as_ref(&self) -> &DateTimeFormattingTypeRelative {
    &self.inner
  }
}
