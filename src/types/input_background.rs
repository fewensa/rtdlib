use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains information about background to set
pub trait TDInputBackground: Debug + RObject {}

/// Contains information about background to set
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputBackground {
  #[doc(hidden)]
  _Default(()),
  /// A background from a local file
  Local(InputBackgroundLocal),
  /// A background previously set in the chat; for chat backgrounds only
  Previous(InputBackgroundPrevious),
  /// A background from the server
  Remote(InputBackgroundRemote),
}

impl Default for InputBackground {
  fn default() -> Self {
    InputBackground::_Default(())
  }
}

impl<'de> Deserialize<'de> for InputBackground {
  fn deserialize<D>(deserializer: D) -> Result<InputBackground, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InputBackground,
      (inputBackgroundLocal, Local);
      (inputBackgroundPrevious, Previous);
      (inputBackgroundRemote, Remote);

    )(deserializer)
  }
}

impl RObject for InputBackground {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      InputBackground::Local(t) => t.td_name(),
      InputBackground::Previous(t) => t.td_name(),
      InputBackground::Remote(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      InputBackground::Local(t) => t.extra(),
      InputBackground::Previous(t) => t.extra(),
      InputBackground::Remote(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputBackground {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, InputBackground::_Default(_))
  }

  pub fn is_local(&self) -> bool {
    matches!(self, InputBackground::Local(_))
  }
  pub fn is_previous(&self) -> bool {
    matches!(self, InputBackground::Previous(_))
  }
  pub fn is_remote(&self) -> bool {
    matches!(self, InputBackground::Remote(_))
  }

  pub fn on_local<F: FnOnce(&InputBackgroundLocal)>(&self, fnc: F) -> &Self {
    if let InputBackground::Local(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_previous<F: FnOnce(&InputBackgroundPrevious)>(&self, fnc: F) -> &Self {
    if let InputBackground::Previous(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_remote<F: FnOnce(&InputBackgroundRemote)>(&self, fnc: F) -> &Self {
    if let InputBackground::Remote(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_local(&self) -> Option<&InputBackgroundLocal> {
    if let InputBackground::Local(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_previous(&self) -> Option<&InputBackgroundPrevious> {
    if let InputBackground::Previous(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_remote(&self) -> Option<&InputBackgroundRemote> {
    if let InputBackground::Remote(t) = self {
      return Some(t);
    }
    None
  }

  pub fn local<T: AsRef<InputBackgroundLocal>>(t: T) -> Self {
    InputBackground::Local(t.as_ref().clone())
  }

  pub fn previous<T: AsRef<InputBackgroundPrevious>>(t: T) -> Self {
    InputBackground::Previous(t.as_ref().clone())
  }

  pub fn remote<T: AsRef<InputBackgroundRemote>>(t: T) -> Self {
    InputBackground::Remote(t.as_ref().clone())
  }
}

impl AsRef<InputBackground> for InputBackground {
  fn as_ref(&self) -> &InputBackground {
    self
  }
}

/// A background from a local file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputBackgroundLocal {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Background file to use. Only inputFileLocal and inputFileGenerated are supported. The file must be in JPEG format for wallpapers and in PNG format for patterns
  background: InputFile,
}

impl RObject for InputBackgroundLocal {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputBackgroundLocal"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputBackground for InputBackgroundLocal {}

impl InputBackgroundLocal {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputBackgroundLocalBuilder {
    let inner = InputBackgroundLocal {
      td_name: "inputBackgroundLocal".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputBackgroundLocalBuilder { inner }
  }

  pub fn background(&self) -> &InputFile {
    &self.background
  }
}

#[doc(hidden)]
pub struct RTDInputBackgroundLocalBuilder {
  inner: InputBackgroundLocal,
}

impl RTDInputBackgroundLocalBuilder {
  pub fn build(&self) -> InputBackgroundLocal {
    self.inner.clone()
  }

  pub fn background<T: AsRef<InputFile>>(&mut self, background: T) -> &mut Self {
    self.inner.background = background.as_ref().clone();
    self
  }
}

impl AsRef<InputBackgroundLocal> for InputBackgroundLocal {
  fn as_ref(&self) -> &InputBackgroundLocal {
    self
  }
}

impl AsRef<InputBackgroundLocal> for RTDInputBackgroundLocalBuilder {
  fn as_ref(&self) -> &InputBackgroundLocal {
    &self.inner
  }
}

/// A background previously set in the chat; for chat backgrounds only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputBackgroundPrevious {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the message with the background
  message_id: i64,
}

impl RObject for InputBackgroundPrevious {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputBackgroundPrevious"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputBackground for InputBackgroundPrevious {}

impl InputBackgroundPrevious {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputBackgroundPreviousBuilder {
    let inner = InputBackgroundPrevious {
      td_name: "inputBackgroundPrevious".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputBackgroundPreviousBuilder { inner }
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }
}

#[doc(hidden)]
pub struct RTDInputBackgroundPreviousBuilder {
  inner: InputBackgroundPrevious,
}

impl RTDInputBackgroundPreviousBuilder {
  pub fn build(&self) -> InputBackgroundPrevious {
    self.inner.clone()
  }

  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }
}

impl AsRef<InputBackgroundPrevious> for InputBackgroundPrevious {
  fn as_ref(&self) -> &InputBackgroundPrevious {
    self
  }
}

impl AsRef<InputBackgroundPrevious> for RTDInputBackgroundPreviousBuilder {
  fn as_ref(&self) -> &InputBackgroundPrevious {
    &self.inner
  }
}

/// A background from the server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputBackgroundRemote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The background identifier
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  background_id: isize,
}

impl RObject for InputBackgroundRemote {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputBackgroundRemote"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputBackground for InputBackgroundRemote {}

impl InputBackgroundRemote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputBackgroundRemoteBuilder {
    let inner = InputBackgroundRemote {
      td_name: "inputBackgroundRemote".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputBackgroundRemoteBuilder { inner }
  }

  pub fn background_id(&self) -> isize {
    self.background_id
  }
}

#[doc(hidden)]
pub struct RTDInputBackgroundRemoteBuilder {
  inner: InputBackgroundRemote,
}

impl RTDInputBackgroundRemoteBuilder {
  pub fn build(&self) -> InputBackgroundRemote {
    self.inner.clone()
  }

  pub fn background_id(&mut self, background_id: isize) -> &mut Self {
    self.inner.background_id = background_id;
    self
  }
}

impl AsRef<InputBackgroundRemote> for InputBackgroundRemote {
  fn as_ref(&self) -> &InputBackgroundRemote {
    self
  }
}

impl AsRef<InputBackgroundRemote> for RTDInputBackgroundRemoteBuilder {
  fn as_ref(&self) -> &InputBackgroundRemote {
    &self.inner
  }
}
