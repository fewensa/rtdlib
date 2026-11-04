use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes style of a button
pub trait TDButtonStyle: Debug + RObject {}

/// Describes style of a button
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ButtonStyle {
  #[doc(hidden)]
  _Default(()),
  /// The button has red color
  Danger(ButtonStyleDanger),
  /// The button has default style
  Default(ButtonStyleDefault),
  /// The button must be shown as a link. The style is allowed only for callback buttons in inlineButton
  Link(ButtonStyleLink),
  /// The button has dark blue color
  Primary(ButtonStylePrimary),
  /// The button has green color
  Success(ButtonStyleSuccess),
}

impl Default for ButtonStyle {
  fn default() -> Self {
    ButtonStyle::_Default(())
  }
}

impl<'de> Deserialize<'de> for ButtonStyle {
  fn deserialize<D>(deserializer: D) -> Result<ButtonStyle, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ButtonStyle,
      (buttonStyleDanger, Danger);
      (buttonStyleDefault, Default);
      (buttonStyleLink, Link);
      (buttonStylePrimary, Primary);
      (buttonStyleSuccess, Success);

    )(deserializer)
  }
}

impl RObject for ButtonStyle {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      ButtonStyle::Danger(t) => t.td_name(),
      ButtonStyle::Default(t) => t.td_name(),
      ButtonStyle::Link(t) => t.td_name(),
      ButtonStyle::Primary(t) => t.td_name(),
      ButtonStyle::Success(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      ButtonStyle::Danger(t) => t.extra(),
      ButtonStyle::Default(t) => t.extra(),
      ButtonStyle::Link(t) => t.extra(),
      ButtonStyle::Primary(t) => t.extra(),
      ButtonStyle::Success(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ButtonStyle {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, ButtonStyle::_Default(_))
  }

  pub fn is_danger(&self) -> bool {
    matches!(self, ButtonStyle::Danger(_))
  }
  pub fn is_default(&self) -> bool {
    matches!(self, ButtonStyle::Default(_))
  }
  pub fn is_link(&self) -> bool {
    matches!(self, ButtonStyle::Link(_))
  }
  pub fn is_primary(&self) -> bool {
    matches!(self, ButtonStyle::Primary(_))
  }
  pub fn is_success(&self) -> bool {
    matches!(self, ButtonStyle::Success(_))
  }

  pub fn on_danger<F: FnOnce(&ButtonStyleDanger)>(&self, fnc: F) -> &Self {
    if let ButtonStyle::Danger(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_default<F: FnOnce(&ButtonStyleDefault)>(&self, fnc: F) -> &Self {
    if let ButtonStyle::Default(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_link<F: FnOnce(&ButtonStyleLink)>(&self, fnc: F) -> &Self {
    if let ButtonStyle::Link(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_primary<F: FnOnce(&ButtonStylePrimary)>(&self, fnc: F) -> &Self {
    if let ButtonStyle::Primary(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_success<F: FnOnce(&ButtonStyleSuccess)>(&self, fnc: F) -> &Self {
    if let ButtonStyle::Success(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_danger(&self) -> Option<&ButtonStyleDanger> {
    if let ButtonStyle::Danger(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_default(&self) -> Option<&ButtonStyleDefault> {
    if let ButtonStyle::Default(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_link(&self) -> Option<&ButtonStyleLink> {
    if let ButtonStyle::Link(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_primary(&self) -> Option<&ButtonStylePrimary> {
    if let ButtonStyle::Primary(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_success(&self) -> Option<&ButtonStyleSuccess> {
    if let ButtonStyle::Success(t) = self {
      return Some(t);
    }
    None
  }

  pub fn danger<T: AsRef<ButtonStyleDanger>>(t: T) -> Self {
    ButtonStyle::Danger(t.as_ref().clone())
  }

  pub fn default<T: AsRef<ButtonStyleDefault>>(t: T) -> Self {
    ButtonStyle::Default(t.as_ref().clone())
  }

  pub fn link<T: AsRef<ButtonStyleLink>>(t: T) -> Self {
    ButtonStyle::Link(t.as_ref().clone())
  }

  pub fn primary<T: AsRef<ButtonStylePrimary>>(t: T) -> Self {
    ButtonStyle::Primary(t.as_ref().clone())
  }

  pub fn success<T: AsRef<ButtonStyleSuccess>>(t: T) -> Self {
    ButtonStyle::Success(t.as_ref().clone())
  }
}

impl AsRef<ButtonStyle> for ButtonStyle {
  fn as_ref(&self) -> &ButtonStyle {
    self
  }
}

/// The button has red color
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ButtonStyleDanger {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ButtonStyleDanger {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "buttonStyleDanger"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDButtonStyle for ButtonStyleDanger {}

impl ButtonStyleDanger {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDButtonStyleDangerBuilder {
    let inner = ButtonStyleDanger {
      td_name: "buttonStyleDanger".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDButtonStyleDangerBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDButtonStyleDangerBuilder {
  inner: ButtonStyleDanger,
}

impl RTDButtonStyleDangerBuilder {
  pub fn build(&self) -> ButtonStyleDanger {
    self.inner.clone()
  }
}

impl AsRef<ButtonStyleDanger> for ButtonStyleDanger {
  fn as_ref(&self) -> &ButtonStyleDanger {
    self
  }
}

impl AsRef<ButtonStyleDanger> for RTDButtonStyleDangerBuilder {
  fn as_ref(&self) -> &ButtonStyleDanger {
    &self.inner
  }
}

/// The button has default style
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ButtonStyleDefault {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ButtonStyleDefault {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "buttonStyleDefault"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDButtonStyle for ButtonStyleDefault {}

impl ButtonStyleDefault {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDButtonStyleDefaultBuilder {
    let inner = ButtonStyleDefault {
      td_name: "buttonStyleDefault".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDButtonStyleDefaultBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDButtonStyleDefaultBuilder {
  inner: ButtonStyleDefault,
}

impl RTDButtonStyleDefaultBuilder {
  pub fn build(&self) -> ButtonStyleDefault {
    self.inner.clone()
  }
}

impl AsRef<ButtonStyleDefault> for ButtonStyleDefault {
  fn as_ref(&self) -> &ButtonStyleDefault {
    self
  }
}

impl AsRef<ButtonStyleDefault> for RTDButtonStyleDefaultBuilder {
  fn as_ref(&self) -> &ButtonStyleDefault {
    &self.inner
  }
}

/// The button must be shown as a link. The style is allowed only for callback buttons in inlineButton
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ButtonStyleLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ButtonStyleLink {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "buttonStyleLink"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDButtonStyle for ButtonStyleLink {}

impl ButtonStyleLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDButtonStyleLinkBuilder {
    let inner = ButtonStyleLink {
      td_name: "buttonStyleLink".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDButtonStyleLinkBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDButtonStyleLinkBuilder {
  inner: ButtonStyleLink,
}

impl RTDButtonStyleLinkBuilder {
  pub fn build(&self) -> ButtonStyleLink {
    self.inner.clone()
  }
}

impl AsRef<ButtonStyleLink> for ButtonStyleLink {
  fn as_ref(&self) -> &ButtonStyleLink {
    self
  }
}

impl AsRef<ButtonStyleLink> for RTDButtonStyleLinkBuilder {
  fn as_ref(&self) -> &ButtonStyleLink {
    &self.inner
  }
}

/// The button has dark blue color
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ButtonStylePrimary {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ButtonStylePrimary {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "buttonStylePrimary"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDButtonStyle for ButtonStylePrimary {}

impl ButtonStylePrimary {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDButtonStylePrimaryBuilder {
    let inner = ButtonStylePrimary {
      td_name: "buttonStylePrimary".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDButtonStylePrimaryBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDButtonStylePrimaryBuilder {
  inner: ButtonStylePrimary,
}

impl RTDButtonStylePrimaryBuilder {
  pub fn build(&self) -> ButtonStylePrimary {
    self.inner.clone()
  }
}

impl AsRef<ButtonStylePrimary> for ButtonStylePrimary {
  fn as_ref(&self) -> &ButtonStylePrimary {
    self
  }
}

impl AsRef<ButtonStylePrimary> for RTDButtonStylePrimaryBuilder {
  fn as_ref(&self) -> &ButtonStylePrimary {
    &self.inner
  }
}

/// The button has green color
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ButtonStyleSuccess {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ButtonStyleSuccess {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "buttonStyleSuccess"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDButtonStyle for ButtonStyleSuccess {}

impl ButtonStyleSuccess {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDButtonStyleSuccessBuilder {
    let inner = ButtonStyleSuccess {
      td_name: "buttonStyleSuccess".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDButtonStyleSuccessBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDButtonStyleSuccessBuilder {
  inner: ButtonStyleSuccess,
}

impl RTDButtonStyleSuccessBuilder {
  pub fn build(&self) -> ButtonStyleSuccess {
    self.inner.clone()
  }
}

impl AsRef<ButtonStyleSuccess> for ButtonStyleSuccess {
  fn as_ref(&self) -> &ButtonStyleSuccess {
    self
  }
}

impl AsRef<ButtonStyleSuccess> for RTDButtonStyleSuccessBuilder {
  fn as_ref(&self) -> &ButtonStyleSuccess {
    &self.inner
  }
}
