use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Represents expected receiver of a TON wallet transfer
pub trait TDTonWalletTransferReceiver: Debug + RObject {}

/// Represents expected receiver of a TON wallet transfer
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TonWalletTransferReceiver {
  #[doc(hidden)]
  _Default(()),
  /// The receiver is specified by its address
  Address(TonWalletTransferReceiverAddress),
  /// The receiver is specified by its username
  User(TonWalletTransferReceiverUser),
}

impl Default for TonWalletTransferReceiver {
  fn default() -> Self {
    TonWalletTransferReceiver::_Default(())
  }
}

impl<'de> Deserialize<'de> for TonWalletTransferReceiver {
  fn deserialize<D>(deserializer: D) -> Result<TonWalletTransferReceiver, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      TonWalletTransferReceiver,
      (tonWalletTransferReceiverAddress, Address);
      (tonWalletTransferReceiverUser, User);

    )(deserializer)
  }
}

impl RObject for TonWalletTransferReceiver {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      TonWalletTransferReceiver::Address(t) => t.td_name(),
      TonWalletTransferReceiver::User(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      TonWalletTransferReceiver::Address(t) => t.extra(),
      TonWalletTransferReceiver::User(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonWalletTransferReceiver {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, TonWalletTransferReceiver::_Default(_))
  }

  pub fn is_address(&self) -> bool {
    matches!(self, TonWalletTransferReceiver::Address(_))
  }
  pub fn is_user(&self) -> bool {
    matches!(self, TonWalletTransferReceiver::User(_))
  }

  pub fn on_address<F: FnOnce(&TonWalletTransferReceiverAddress)>(&self, fnc: F) -> &Self {
    if let TonWalletTransferReceiver::Address(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_user<F: FnOnce(&TonWalletTransferReceiverUser)>(&self, fnc: F) -> &Self {
    if let TonWalletTransferReceiver::User(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_address(&self) -> Option<&TonWalletTransferReceiverAddress> {
    if let TonWalletTransferReceiver::Address(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_user(&self) -> Option<&TonWalletTransferReceiverUser> {
    if let TonWalletTransferReceiver::User(t) = self {
      return Some(t);
    }
    None
  }

  pub fn address<T: AsRef<TonWalletTransferReceiverAddress>>(t: T) -> Self {
    TonWalletTransferReceiver::Address(t.as_ref().clone())
  }

  pub fn user<T: AsRef<TonWalletTransferReceiverUser>>(t: T) -> Self {
    TonWalletTransferReceiver::User(t.as_ref().clone())
  }
}

impl AsRef<TonWalletTransferReceiver> for TonWalletTransferReceiver {
  fn as_ref(&self) -> &TonWalletTransferReceiver {
    self
  }
}

/// The receiver is specified by its address
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonWalletTransferReceiverAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Address of the receiver. Use getAddressTonWallet to get information about the user's TON wallet
  address: String,
}

impl RObject for TonWalletTransferReceiverAddress {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonWalletTransferReceiverAddress"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonWalletTransferReceiver for TonWalletTransferReceiverAddress {}

impl TonWalletTransferReceiverAddress {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonWalletTransferReceiverAddressBuilder {
    let inner = TonWalletTransferReceiverAddress {
      td_name: "tonWalletTransferReceiverAddress".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonWalletTransferReceiverAddressBuilder { inner }
  }

  pub fn address(&self) -> &String {
    &self.address
  }
}

#[doc(hidden)]
pub struct RTDTonWalletTransferReceiverAddressBuilder {
  inner: TonWalletTransferReceiverAddress,
}

impl RTDTonWalletTransferReceiverAddressBuilder {
  pub fn build(&self) -> TonWalletTransferReceiverAddress {
    self.inner.clone()
  }

  pub fn address<T: AsRef<str>>(&mut self, address: T) -> &mut Self {
    self.inner.address = address.as_ref().to_string();
    self
  }
}

impl AsRef<TonWalletTransferReceiverAddress> for TonWalletTransferReceiverAddress {
  fn as_ref(&self) -> &TonWalletTransferReceiverAddress {
    self
  }
}

impl AsRef<TonWalletTransferReceiverAddress> for RTDTonWalletTransferReceiverAddressBuilder {
  fn as_ref(&self) -> &TonWalletTransferReceiverAddress {
    &self.inner
  }
}

/// The receiver is specified by its username
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonWalletTransferReceiverUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Username of the user. Use searchPublicChat to find the user, check that it is a regular user, and then use createUserTonWallet to get information about the user's TON wallet
  username: String,
}

impl RObject for TonWalletTransferReceiverUser {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonWalletTransferReceiverUser"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonWalletTransferReceiver for TonWalletTransferReceiverUser {}

impl TonWalletTransferReceiverUser {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonWalletTransferReceiverUserBuilder {
    let inner = TonWalletTransferReceiverUser {
      td_name: "tonWalletTransferReceiverUser".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonWalletTransferReceiverUserBuilder { inner }
  }

  pub fn username(&self) -> &String {
    &self.username
  }
}

#[doc(hidden)]
pub struct RTDTonWalletTransferReceiverUserBuilder {
  inner: TonWalletTransferReceiverUser,
}

impl RTDTonWalletTransferReceiverUserBuilder {
  pub fn build(&self) -> TonWalletTransferReceiverUser {
    self.inner.clone()
  }

  pub fn username<T: AsRef<str>>(&mut self, username: T) -> &mut Self {
    self.inner.username = username.as_ref().to_string();
    self
  }
}

impl AsRef<TonWalletTransferReceiverUser> for TonWalletTransferReceiverUser {
  fn as_ref(&self) -> &TonWalletTransferReceiverUser {
    self
  }
}

impl AsRef<TonWalletTransferReceiverUser> for RTDTonWalletTransferReceiverUserBuilder {
  fn as_ref(&self) -> &TonWalletTransferReceiverUser {
    &self.inner
  }
}
