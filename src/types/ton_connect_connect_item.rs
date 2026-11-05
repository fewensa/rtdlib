use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Represents a data item to share with a dApp
pub trait TDTonConnectConnectItem: Debug + RObject {}

/// Represents a data item to share with a dApp
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TonConnectConnectItem {
  #[doc(hidden)]
  _Default(()),
  /// Address of the wallet
  Address(TonConnectConnectItemAddress),
  /// Proof of the wallet ownership
  Proof(TonConnectConnectItemProof),
}

impl Default for TonConnectConnectItem {
  fn default() -> Self {
    TonConnectConnectItem::_Default(())
  }
}

impl<'de> Deserialize<'de> for TonConnectConnectItem {
  fn deserialize<D>(deserializer: D) -> Result<TonConnectConnectItem, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      TonConnectConnectItem,
      (tonConnectConnectItemAddress, Address);
      (tonConnectConnectItemProof, Proof);

    )(deserializer)
  }
}

impl RObject for TonConnectConnectItem {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      TonConnectConnectItem::Address(t) => t.td_name(),
      TonConnectConnectItem::Proof(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      TonConnectConnectItem::Address(t) => t.extra(),
      TonConnectConnectItem::Proof(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonConnectConnectItem {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, TonConnectConnectItem::_Default(_))
  }

  pub fn is_address(&self) -> bool {
    matches!(self, TonConnectConnectItem::Address(_))
  }
  pub fn is_proof(&self) -> bool {
    matches!(self, TonConnectConnectItem::Proof(_))
  }

  pub fn on_address<F: FnOnce(&TonConnectConnectItemAddress)>(&self, fnc: F) -> &Self {
    if let TonConnectConnectItem::Address(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_proof<F: FnOnce(&TonConnectConnectItemProof)>(&self, fnc: F) -> &Self {
    if let TonConnectConnectItem::Proof(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_address(&self) -> Option<&TonConnectConnectItemAddress> {
    if let TonConnectConnectItem::Address(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_proof(&self) -> Option<&TonConnectConnectItemProof> {
    if let TonConnectConnectItem::Proof(t) = self {
      return Some(t);
    }
    None
  }

  pub fn address<T: AsRef<TonConnectConnectItemAddress>>(t: T) -> Self {
    TonConnectConnectItem::Address(t.as_ref().clone())
  }

  pub fn proof<T: AsRef<TonConnectConnectItemProof>>(t: T) -> Self {
    TonConnectConnectItem::Proof(t.as_ref().clone())
  }
}

impl AsRef<TonConnectConnectItem> for TonConnectConnectItem {
  fn as_ref(&self) -> &TonConnectConnectItem {
    self
  }
}

/// Address of the wallet
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonConnectConnectItemAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Stringified TON network global_id (e.g. "-239" for mainnet); may be empty
  network: String,
}

impl RObject for TonConnectConnectItemAddress {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonConnectConnectItemAddress"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonConnectConnectItem for TonConnectConnectItemAddress {}

impl TonConnectConnectItemAddress {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonConnectConnectItemAddressBuilder {
    let inner = TonConnectConnectItemAddress {
      td_name: "tonConnectConnectItemAddress".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonConnectConnectItemAddressBuilder { inner }
  }

  pub fn network(&self) -> &String {
    &self.network
  }
}

#[doc(hidden)]
pub struct RTDTonConnectConnectItemAddressBuilder {
  inner: TonConnectConnectItemAddress,
}

impl RTDTonConnectConnectItemAddressBuilder {
  pub fn build(&self) -> TonConnectConnectItemAddress {
    self.inner.clone()
  }

  pub fn network<T: AsRef<str>>(&mut self, network: T) -> &mut Self {
    self.inner.network = network.as_ref().to_string();
    self
  }
}

impl AsRef<TonConnectConnectItemAddress> for TonConnectConnectItemAddress {
  fn as_ref(&self) -> &TonConnectConnectItemAddress {
    self
  }
}

impl AsRef<TonConnectConnectItemAddress> for RTDTonConnectConnectItemAddressBuilder {
  fn as_ref(&self) -> &TonConnectConnectItemAddress {
    &self.inner
  }
}

/// Proof of the wallet ownership
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonConnectConnectItemProof {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Payload to be used for the proof (e.g. nonce + expiration timestamp)
  payload: String,
}

impl RObject for TonConnectConnectItemProof {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonConnectConnectItemProof"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonConnectConnectItem for TonConnectConnectItemProof {}

impl TonConnectConnectItemProof {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonConnectConnectItemProofBuilder {
    let inner = TonConnectConnectItemProof {
      td_name: "tonConnectConnectItemProof".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonConnectConnectItemProofBuilder { inner }
  }

  pub fn payload(&self) -> &String {
    &self.payload
  }
}

#[doc(hidden)]
pub struct RTDTonConnectConnectItemProofBuilder {
  inner: TonConnectConnectItemProof,
}

impl RTDTonConnectConnectItemProofBuilder {
  pub fn build(&self) -> TonConnectConnectItemProof {
    self.inner.clone()
  }

  pub fn payload<T: AsRef<str>>(&mut self, payload: T) -> &mut Self {
    self.inner.payload = payload.as_ref().to_string();
    self
  }
}

impl AsRef<TonConnectConnectItemProof> for TonConnectConnectItemProof {
  fn as_ref(&self) -> &TonConnectConnectItemProof {
    self
  }
}

impl AsRef<TonConnectConnectItemProof> for RTDTonConnectConnectItemProofBuilder {
  fn as_ref(&self) -> &TonConnectConnectItemProof {
    &self.inner
  }
}
