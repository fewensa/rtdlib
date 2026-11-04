use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes state of a transaction with a TON wallet
pub trait TDTonWalletTransactionState: Debug + RObject {}

/// Describes state of a transaction with a TON wallet
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TonWalletTransactionState {
  #[doc(hidden)]
  _Default(()),
  /// The transaction failed
  Failed(TonWalletTransactionStateFailed),
  /// The transaction succeeded
  Succeeded(TonWalletTransactionStateSucceeded),
}

impl Default for TonWalletTransactionState {
  fn default() -> Self {
    TonWalletTransactionState::_Default(())
  }
}

impl<'de> Deserialize<'de> for TonWalletTransactionState {
  fn deserialize<D>(deserializer: D) -> Result<TonWalletTransactionState, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      TonWalletTransactionState,
      (tonWalletTransactionStateFailed, Failed);
      (tonWalletTransactionStateSucceeded, Succeeded);

    )(deserializer)
  }
}

impl RObject for TonWalletTransactionState {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      TonWalletTransactionState::Failed(t) => t.td_name(),
      TonWalletTransactionState::Succeeded(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      TonWalletTransactionState::Failed(t) => t.extra(),
      TonWalletTransactionState::Succeeded(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonWalletTransactionState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, TonWalletTransactionState::_Default(_))
  }

  pub fn is_failed(&self) -> bool {
    matches!(self, TonWalletTransactionState::Failed(_))
  }
  pub fn is_succeeded(&self) -> bool {
    matches!(self, TonWalletTransactionState::Succeeded(_))
  }

  pub fn on_failed<F: FnOnce(&TonWalletTransactionStateFailed)>(&self, fnc: F) -> &Self {
    if let TonWalletTransactionState::Failed(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_succeeded<F: FnOnce(&TonWalletTransactionStateSucceeded)>(&self, fnc: F) -> &Self {
    if let TonWalletTransactionState::Succeeded(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_failed(&self) -> Option<&TonWalletTransactionStateFailed> {
    if let TonWalletTransactionState::Failed(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_succeeded(&self) -> Option<&TonWalletTransactionStateSucceeded> {
    if let TonWalletTransactionState::Succeeded(t) = self {
      return Some(t);
    }
    None
  }

  pub fn failed<T: AsRef<TonWalletTransactionStateFailed>>(t: T) -> Self {
    TonWalletTransactionState::Failed(t.as_ref().clone())
  }

  pub fn succeeded<T: AsRef<TonWalletTransactionStateSucceeded>>(t: T) -> Self {
    TonWalletTransactionState::Succeeded(t.as_ref().clone())
  }
}

impl AsRef<TonWalletTransactionState> for TonWalletTransactionState {
  fn as_ref(&self) -> &TonWalletTransactionState {
    self
  }
}

/// The transaction failed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonWalletTransactionStateFailed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for TonWalletTransactionStateFailed {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonWalletTransactionStateFailed"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonWalletTransactionState for TonWalletTransactionStateFailed {}

impl TonWalletTransactionStateFailed {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonWalletTransactionStateFailedBuilder {
    let inner = TonWalletTransactionStateFailed {
      td_name: "tonWalletTransactionStateFailed".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDTonWalletTransactionStateFailedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDTonWalletTransactionStateFailedBuilder {
  inner: TonWalletTransactionStateFailed,
}

impl RTDTonWalletTransactionStateFailedBuilder {
  pub fn build(&self) -> TonWalletTransactionStateFailed {
    self.inner.clone()
  }
}

impl AsRef<TonWalletTransactionStateFailed> for TonWalletTransactionStateFailed {
  fn as_ref(&self) -> &TonWalletTransactionStateFailed {
    self
  }
}

impl AsRef<TonWalletTransactionStateFailed> for RTDTonWalletTransactionStateFailedBuilder {
  fn as_ref(&self) -> &TonWalletTransactionStateFailed {
    &self.inner
  }
}

/// The transaction succeeded
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonWalletTransactionStateSucceeded {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Hash of the transaction
  tx_hash: String,
}

impl RObject for TonWalletTransactionStateSucceeded {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonWalletTransactionStateSucceeded"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonWalletTransactionState for TonWalletTransactionStateSucceeded {}

impl TonWalletTransactionStateSucceeded {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonWalletTransactionStateSucceededBuilder {
    let inner = TonWalletTransactionStateSucceeded {
      td_name: "tonWalletTransactionStateSucceeded".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonWalletTransactionStateSucceededBuilder { inner }
  }

  pub fn tx_hash(&self) -> &String {
    &self.tx_hash
  }
}

#[doc(hidden)]
pub struct RTDTonWalletTransactionStateSucceededBuilder {
  inner: TonWalletTransactionStateSucceeded,
}

impl RTDTonWalletTransactionStateSucceededBuilder {
  pub fn build(&self) -> TonWalletTransactionStateSucceeded {
    self.inner.clone()
  }

  pub fn tx_hash<T: AsRef<str>>(&mut self, tx_hash: T) -> &mut Self {
    self.inner.tx_hash = tx_hash.as_ref().to_string();
    self
  }
}

impl AsRef<TonWalletTransactionStateSucceeded> for TonWalletTransactionStateSucceeded {
  fn as_ref(&self) -> &TonWalletTransactionStateSucceeded {
    self
  }
}

impl AsRef<TonWalletTransactionStateSucceeded> for RTDTonWalletTransactionStateSucceededBuilder {
  fn as_ref(&self) -> &TonWalletTransactionStateSucceeded {
    &self.inner
  }
}
