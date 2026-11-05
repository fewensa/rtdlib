use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes manifest of the dApp that is being connected using TON Connect
pub trait TDTonConnectManifest: Debug + RObject {}

/// Describes manifest of the dApp that is being connected using TON Connect
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TonConnectManifest {
  #[doc(hidden)]
  _Default(()),
  /// The manifest has failed to download
  Failed(TonConnectManifestFailed),
  /// The manifest is available
  Info(TonConnectManifestInfo),
  /// The manifest is invalid
  Invalid(TonConnectManifestInvalid),
  /// The manifest is being downloaded
  Pending(TonConnectManifestPending),
}

impl Default for TonConnectManifest {
  fn default() -> Self {
    TonConnectManifest::_Default(())
  }
}

impl<'de> Deserialize<'de> for TonConnectManifest {
  fn deserialize<D>(deserializer: D) -> Result<TonConnectManifest, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      TonConnectManifest,
      (tonConnectManifestFailed, Failed);
      (tonConnectManifestInfo, Info);
      (tonConnectManifestInvalid, Invalid);
      (tonConnectManifestPending, Pending);

    )(deserializer)
  }
}

impl RObject for TonConnectManifest {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      TonConnectManifest::Failed(t) => t.td_name(),
      TonConnectManifest::Info(t) => t.td_name(),
      TonConnectManifest::Invalid(t) => t.td_name(),
      TonConnectManifest::Pending(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      TonConnectManifest::Failed(t) => t.extra(),
      TonConnectManifest::Info(t) => t.extra(),
      TonConnectManifest::Invalid(t) => t.extra(),
      TonConnectManifest::Pending(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonConnectManifest {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, TonConnectManifest::_Default(_))
  }

  pub fn is_failed(&self) -> bool {
    matches!(self, TonConnectManifest::Failed(_))
  }
  pub fn is_info(&self) -> bool {
    matches!(self, TonConnectManifest::Info(_))
  }
  pub fn is_invalid(&self) -> bool {
    matches!(self, TonConnectManifest::Invalid(_))
  }
  pub fn is_pending(&self) -> bool {
    matches!(self, TonConnectManifest::Pending(_))
  }

  pub fn on_failed<F: FnOnce(&TonConnectManifestFailed)>(&self, fnc: F) -> &Self {
    if let TonConnectManifest::Failed(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_info<F: FnOnce(&TonConnectManifestInfo)>(&self, fnc: F) -> &Self {
    if let TonConnectManifest::Info(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_invalid<F: FnOnce(&TonConnectManifestInvalid)>(&self, fnc: F) -> &Self {
    if let TonConnectManifest::Invalid(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_pending<F: FnOnce(&TonConnectManifestPending)>(&self, fnc: F) -> &Self {
    if let TonConnectManifest::Pending(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_failed(&self) -> Option<&TonConnectManifestFailed> {
    if let TonConnectManifest::Failed(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_info(&self) -> Option<&TonConnectManifestInfo> {
    if let TonConnectManifest::Info(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_invalid(&self) -> Option<&TonConnectManifestInvalid> {
    if let TonConnectManifest::Invalid(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_pending(&self) -> Option<&TonConnectManifestPending> {
    if let TonConnectManifest::Pending(t) = self {
      return Some(t);
    }
    None
  }

  pub fn failed<T: AsRef<TonConnectManifestFailed>>(t: T) -> Self {
    TonConnectManifest::Failed(t.as_ref().clone())
  }

  pub fn info<T: AsRef<TonConnectManifestInfo>>(t: T) -> Self {
    TonConnectManifest::Info(t.as_ref().clone())
  }

  pub fn invalid<T: AsRef<TonConnectManifestInvalid>>(t: T) -> Self {
    TonConnectManifest::Invalid(t.as_ref().clone())
  }

  pub fn pending<T: AsRef<TonConnectManifestPending>>(t: T) -> Self {
    TonConnectManifest::Pending(t.as_ref().clone())
  }
}

impl AsRef<TonConnectManifest> for TonConnectManifest {
  fn as_ref(&self) -> &TonConnectManifest {
    self
  }
}

/// The manifest has failed to download
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonConnectManifestFailed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for TonConnectManifestFailed {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonConnectManifestFailed"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonConnectManifest for TonConnectManifestFailed {}

impl TonConnectManifestFailed {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonConnectManifestFailedBuilder {
    let inner = TonConnectManifestFailed {
      td_name: "tonConnectManifestFailed".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDTonConnectManifestFailedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDTonConnectManifestFailedBuilder {
  inner: TonConnectManifestFailed,
}

impl RTDTonConnectManifestFailedBuilder {
  pub fn build(&self) -> TonConnectManifestFailed {
    self.inner.clone()
  }
}

impl AsRef<TonConnectManifestFailed> for TonConnectManifestFailed {
  fn as_ref(&self) -> &TonConnectManifestFailed {
    self
  }
}

impl AsRef<TonConnectManifestFailed> for RTDTonConnectManifestFailedBuilder {
  fn as_ref(&self) -> &TonConnectManifestFailed {
    &self.inner
  }
}

/// The manifest is available
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonConnectManifestInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// URL of the dApp
  url: String,
  /// Name of the dApp
  name: String,
  /// App icon in PNG or ICO format
  icon: Document,
}

impl RObject for TonConnectManifestInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonConnectManifestInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonConnectManifest for TonConnectManifestInfo {}

impl TonConnectManifestInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonConnectManifestInfoBuilder {
    let inner = TonConnectManifestInfo {
      td_name: "tonConnectManifestInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonConnectManifestInfoBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn icon(&self) -> &Document {
    &self.icon
  }
}

#[doc(hidden)]
pub struct RTDTonConnectManifestInfoBuilder {
  inner: TonConnectManifestInfo,
}

impl RTDTonConnectManifestInfoBuilder {
  pub fn build(&self) -> TonConnectManifestInfo {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

  pub fn icon<T: AsRef<Document>>(&mut self, icon: T) -> &mut Self {
    self.inner.icon = icon.as_ref().clone();
    self
  }
}

impl AsRef<TonConnectManifestInfo> for TonConnectManifestInfo {
  fn as_ref(&self) -> &TonConnectManifestInfo {
    self
  }
}

impl AsRef<TonConnectManifestInfo> for RTDTonConnectManifestInfoBuilder {
  fn as_ref(&self) -> &TonConnectManifestInfo {
    &self.inner
  }
}

/// The manifest is invalid
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonConnectManifestInvalid {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for TonConnectManifestInvalid {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonConnectManifestInvalid"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonConnectManifest for TonConnectManifestInvalid {}

impl TonConnectManifestInvalid {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonConnectManifestInvalidBuilder {
    let inner = TonConnectManifestInvalid {
      td_name: "tonConnectManifestInvalid".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDTonConnectManifestInvalidBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDTonConnectManifestInvalidBuilder {
  inner: TonConnectManifestInvalid,
}

impl RTDTonConnectManifestInvalidBuilder {
  pub fn build(&self) -> TonConnectManifestInvalid {
    self.inner.clone()
  }
}

impl AsRef<TonConnectManifestInvalid> for TonConnectManifestInvalid {
  fn as_ref(&self) -> &TonConnectManifestInvalid {
    self
  }
}

impl AsRef<TonConnectManifestInvalid> for RTDTonConnectManifestInvalidBuilder {
  fn as_ref(&self) -> &TonConnectManifestInvalid {
    &self.inner
  }
}

/// The manifest is being downloaded
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonConnectManifestPending {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for TonConnectManifestPending {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonConnectManifestPending"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonConnectManifest for TonConnectManifestPending {}

impl TonConnectManifestPending {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonConnectManifestPendingBuilder {
    let inner = TonConnectManifestPending {
      td_name: "tonConnectManifestPending".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDTonConnectManifestPendingBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDTonConnectManifestPendingBuilder {
  inner: TonConnectManifestPending,
}

impl RTDTonConnectManifestPendingBuilder {
  pub fn build(&self) -> TonConnectManifestPending {
    self.inner.clone()
  }
}

impl AsRef<TonConnectManifestPending> for TonConnectManifestPending {
  fn as_ref(&self) -> &TonConnectManifestPending {
    self
  }
}

impl AsRef<TonConnectManifestPending> for RTDTonConnectManifestPendingBuilder {
  fn as_ref(&self) -> &TonConnectManifestPending {
    &self.inner
  }
}
