use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a built-in theme of an official application
pub trait TDBuiltInTheme: Debug + RObject {}

/// Describes a built-in theme of an official application
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum BuiltInTheme {
  #[doc(hidden)]
  _Default(()),
  /// Arctic light theme
  Arctic(BuiltInThemeArctic),
  /// Classic light theme
  Classic(BuiltInThemeClassic),
  /// Regular light theme
  Day(BuiltInThemeDay),
  /// Regular dark theme
  Night(BuiltInThemeNight),
  /// Tinted dark theme
  Tinted(BuiltInThemeTinted),
}

impl Default for BuiltInTheme {
  fn default() -> Self {
    BuiltInTheme::_Default(())
  }
}

impl<'de> Deserialize<'de> for BuiltInTheme {
  fn deserialize<D>(deserializer: D) -> Result<BuiltInTheme, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      BuiltInTheme,
      (builtInThemeArctic, Arctic);
      (builtInThemeClassic, Classic);
      (builtInThemeDay, Day);
      (builtInThemeNight, Night);
      (builtInThemeTinted, Tinted);

    )(deserializer)
  }
}

impl RObject for BuiltInTheme {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      BuiltInTheme::Arctic(t) => t.td_name(),
      BuiltInTheme::Classic(t) => t.td_name(),
      BuiltInTheme::Day(t) => t.td_name(),
      BuiltInTheme::Night(t) => t.td_name(),
      BuiltInTheme::Tinted(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      BuiltInTheme::Arctic(t) => t.extra(),
      BuiltInTheme::Classic(t) => t.extra(),
      BuiltInTheme::Day(t) => t.extra(),
      BuiltInTheme::Night(t) => t.extra(),
      BuiltInTheme::Tinted(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BuiltInTheme {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, BuiltInTheme::_Default(_))
  }

  pub fn is_arctic(&self) -> bool {
    matches!(self, BuiltInTheme::Arctic(_))
  }
  pub fn is_classic(&self) -> bool {
    matches!(self, BuiltInTheme::Classic(_))
  }
  pub fn is_day(&self) -> bool {
    matches!(self, BuiltInTheme::Day(_))
  }
  pub fn is_night(&self) -> bool {
    matches!(self, BuiltInTheme::Night(_))
  }
  pub fn is_tinted(&self) -> bool {
    matches!(self, BuiltInTheme::Tinted(_))
  }

  pub fn on_arctic<F: FnOnce(&BuiltInThemeArctic)>(&self, fnc: F) -> &Self {
    if let BuiltInTheme::Arctic(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_classic<F: FnOnce(&BuiltInThemeClassic)>(&self, fnc: F) -> &Self {
    if let BuiltInTheme::Classic(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_day<F: FnOnce(&BuiltInThemeDay)>(&self, fnc: F) -> &Self {
    if let BuiltInTheme::Day(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_night<F: FnOnce(&BuiltInThemeNight)>(&self, fnc: F) -> &Self {
    if let BuiltInTheme::Night(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_tinted<F: FnOnce(&BuiltInThemeTinted)>(&self, fnc: F) -> &Self {
    if let BuiltInTheme::Tinted(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_arctic(&self) -> Option<&BuiltInThemeArctic> {
    if let BuiltInTheme::Arctic(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_classic(&self) -> Option<&BuiltInThemeClassic> {
    if let BuiltInTheme::Classic(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_day(&self) -> Option<&BuiltInThemeDay> {
    if let BuiltInTheme::Day(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_night(&self) -> Option<&BuiltInThemeNight> {
    if let BuiltInTheme::Night(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_tinted(&self) -> Option<&BuiltInThemeTinted> {
    if let BuiltInTheme::Tinted(t) = self {
      return Some(t);
    }
    None
  }

  pub fn arctic<T: AsRef<BuiltInThemeArctic>>(t: T) -> Self {
    BuiltInTheme::Arctic(t.as_ref().clone())
  }

  pub fn classic<T: AsRef<BuiltInThemeClassic>>(t: T) -> Self {
    BuiltInTheme::Classic(t.as_ref().clone())
  }

  pub fn day<T: AsRef<BuiltInThemeDay>>(t: T) -> Self {
    BuiltInTheme::Day(t.as_ref().clone())
  }

  pub fn night<T: AsRef<BuiltInThemeNight>>(t: T) -> Self {
    BuiltInTheme::Night(t.as_ref().clone())
  }

  pub fn tinted<T: AsRef<BuiltInThemeTinted>>(t: T) -> Self {
    BuiltInTheme::Tinted(t.as_ref().clone())
  }
}

impl AsRef<BuiltInTheme> for BuiltInTheme {
  fn as_ref(&self) -> &BuiltInTheme {
    self
  }
}

/// Arctic light theme
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BuiltInThemeArctic {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for BuiltInThemeArctic {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "builtInThemeArctic"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDBuiltInTheme for BuiltInThemeArctic {}

impl BuiltInThemeArctic {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBuiltInThemeArcticBuilder {
    let inner = BuiltInThemeArctic {
      td_name: "builtInThemeArctic".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDBuiltInThemeArcticBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDBuiltInThemeArcticBuilder {
  inner: BuiltInThemeArctic,
}

impl RTDBuiltInThemeArcticBuilder {
  pub fn build(&self) -> BuiltInThemeArctic {
    self.inner.clone()
  }
}

impl AsRef<BuiltInThemeArctic> for BuiltInThemeArctic {
  fn as_ref(&self) -> &BuiltInThemeArctic {
    self
  }
}

impl AsRef<BuiltInThemeArctic> for RTDBuiltInThemeArcticBuilder {
  fn as_ref(&self) -> &BuiltInThemeArctic {
    &self.inner
  }
}

/// Classic light theme
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BuiltInThemeClassic {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for BuiltInThemeClassic {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "builtInThemeClassic"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDBuiltInTheme for BuiltInThemeClassic {}

impl BuiltInThemeClassic {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBuiltInThemeClassicBuilder {
    let inner = BuiltInThemeClassic {
      td_name: "builtInThemeClassic".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDBuiltInThemeClassicBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDBuiltInThemeClassicBuilder {
  inner: BuiltInThemeClassic,
}

impl RTDBuiltInThemeClassicBuilder {
  pub fn build(&self) -> BuiltInThemeClassic {
    self.inner.clone()
  }
}

impl AsRef<BuiltInThemeClassic> for BuiltInThemeClassic {
  fn as_ref(&self) -> &BuiltInThemeClassic {
    self
  }
}

impl AsRef<BuiltInThemeClassic> for RTDBuiltInThemeClassicBuilder {
  fn as_ref(&self) -> &BuiltInThemeClassic {
    &self.inner
  }
}

/// Regular light theme
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BuiltInThemeDay {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for BuiltInThemeDay {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "builtInThemeDay"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDBuiltInTheme for BuiltInThemeDay {}

impl BuiltInThemeDay {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBuiltInThemeDayBuilder {
    let inner = BuiltInThemeDay {
      td_name: "builtInThemeDay".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDBuiltInThemeDayBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDBuiltInThemeDayBuilder {
  inner: BuiltInThemeDay,
}

impl RTDBuiltInThemeDayBuilder {
  pub fn build(&self) -> BuiltInThemeDay {
    self.inner.clone()
  }
}

impl AsRef<BuiltInThemeDay> for BuiltInThemeDay {
  fn as_ref(&self) -> &BuiltInThemeDay {
    self
  }
}

impl AsRef<BuiltInThemeDay> for RTDBuiltInThemeDayBuilder {
  fn as_ref(&self) -> &BuiltInThemeDay {
    &self.inner
  }
}

/// Regular dark theme
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BuiltInThemeNight {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for BuiltInThemeNight {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "builtInThemeNight"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDBuiltInTheme for BuiltInThemeNight {}

impl BuiltInThemeNight {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBuiltInThemeNightBuilder {
    let inner = BuiltInThemeNight {
      td_name: "builtInThemeNight".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDBuiltInThemeNightBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDBuiltInThemeNightBuilder {
  inner: BuiltInThemeNight,
}

impl RTDBuiltInThemeNightBuilder {
  pub fn build(&self) -> BuiltInThemeNight {
    self.inner.clone()
  }
}

impl AsRef<BuiltInThemeNight> for BuiltInThemeNight {
  fn as_ref(&self) -> &BuiltInThemeNight {
    self
  }
}

impl AsRef<BuiltInThemeNight> for RTDBuiltInThemeNightBuilder {
  fn as_ref(&self) -> &BuiltInThemeNight {
    &self.inner
  }
}

/// Tinted dark theme
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BuiltInThemeTinted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for BuiltInThemeTinted {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "builtInThemeTinted"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDBuiltInTheme for BuiltInThemeTinted {}

impl BuiltInThemeTinted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBuiltInThemeTintedBuilder {
    let inner = BuiltInThemeTinted {
      td_name: "builtInThemeTinted".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDBuiltInThemeTintedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDBuiltInThemeTintedBuilder {
  inner: BuiltInThemeTinted,
}

impl RTDBuiltInThemeTintedBuilder {
  pub fn build(&self) -> BuiltInThemeTinted {
    self.inner.clone()
  }
}

impl AsRef<BuiltInThemeTinted> for BuiltInThemeTinted {
  fn as_ref(&self) -> &BuiltInThemeTinted {
    self
  }
}

impl AsRef<BuiltInThemeTinted> for RTDBuiltInThemeTintedBuilder {
  fn as_ref(&self) -> &BuiltInThemeTinted {
    &self.inner
  }
}
