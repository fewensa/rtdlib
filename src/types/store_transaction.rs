use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes an in-store transaction
pub trait TDStoreTransaction: Debug + RObject {}

/// Describes an in-store transaction
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum StoreTransaction {
  #[doc(hidden)]
  _Default(()),
  /// A purchase through App Store
  AppStore(StoreTransactionAppStore),
  /// A purchase through Google Play
  GooglePlay(StoreTransactionGooglePlay),
}

impl Default for StoreTransaction {
  fn default() -> Self {
    StoreTransaction::_Default(())
  }
}

impl<'de> Deserialize<'de> for StoreTransaction {
  fn deserialize<D>(deserializer: D) -> Result<StoreTransaction, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      StoreTransaction,
      (storeTransactionAppStore, AppStore);
      (storeTransactionGooglePlay, GooglePlay);

    )(deserializer)
  }
}

impl RObject for StoreTransaction {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      StoreTransaction::AppStore(t) => t.td_name(),
      StoreTransaction::GooglePlay(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      StoreTransaction::AppStore(t) => t.extra(),
      StoreTransaction::GooglePlay(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StoreTransaction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, StoreTransaction::_Default(_))
  }

  pub fn is_app_store(&self) -> bool {
    matches!(self, StoreTransaction::AppStore(_))
  }
  pub fn is_google_play(&self) -> bool {
    matches!(self, StoreTransaction::GooglePlay(_))
  }

  pub fn on_app_store<F: FnOnce(&StoreTransactionAppStore)>(&self, fnc: F) -> &Self {
    if let StoreTransaction::AppStore(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_google_play<F: FnOnce(&StoreTransactionGooglePlay)>(&self, fnc: F) -> &Self {
    if let StoreTransaction::GooglePlay(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_app_store(&self) -> Option<&StoreTransactionAppStore> {
    if let StoreTransaction::AppStore(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_google_play(&self) -> Option<&StoreTransactionGooglePlay> {
    if let StoreTransaction::GooglePlay(t) = self {
      return Some(t);
    }
    None
  }

  pub fn app_store<T: AsRef<StoreTransactionAppStore>>(t: T) -> Self {
    StoreTransaction::AppStore(t.as_ref().clone())
  }

  pub fn google_play<T: AsRef<StoreTransactionGooglePlay>>(t: T) -> Self {
    StoreTransaction::GooglePlay(t.as_ref().clone())
  }
}

impl AsRef<StoreTransaction> for StoreTransaction {
  fn as_ref(&self) -> &StoreTransaction {
    self
  }
}

/// A purchase through App Store
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoreTransactionAppStore {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// App Store receipt
  receipt: String,
}

impl RObject for StoreTransactionAppStore {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storeTransactionAppStore"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoreTransaction for StoreTransactionAppStore {}

impl StoreTransactionAppStore {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoreTransactionAppStoreBuilder {
    let inner = StoreTransactionAppStore {
      td_name: "storeTransactionAppStore".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoreTransactionAppStoreBuilder { inner }
  }

  pub fn receipt(&self) -> &String {
    &self.receipt
  }
}

#[doc(hidden)]
pub struct RTDStoreTransactionAppStoreBuilder {
  inner: StoreTransactionAppStore,
}

impl RTDStoreTransactionAppStoreBuilder {
  pub fn build(&self) -> StoreTransactionAppStore {
    self.inner.clone()
  }

  pub fn receipt<T: AsRef<str>>(&mut self, receipt: T) -> &mut Self {
    self.inner.receipt = receipt.as_ref().to_string();
    self
  }
}

impl AsRef<StoreTransactionAppStore> for StoreTransactionAppStore {
  fn as_ref(&self) -> &StoreTransactionAppStore {
    self
  }
}

impl AsRef<StoreTransactionAppStore> for RTDStoreTransactionAppStoreBuilder {
  fn as_ref(&self) -> &StoreTransactionAppStore {
    &self.inner
  }
}

/// A purchase through Google Play
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoreTransactionGooglePlay {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Application package name
  package_name: String,
  /// Identifier of the purchased store product
  store_product_id: String,
  /// Google Play purchase token
  purchase_token: String,
}

impl RObject for StoreTransactionGooglePlay {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storeTransactionGooglePlay"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoreTransaction for StoreTransactionGooglePlay {}

impl StoreTransactionGooglePlay {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoreTransactionGooglePlayBuilder {
    let inner = StoreTransactionGooglePlay {
      td_name: "storeTransactionGooglePlay".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoreTransactionGooglePlayBuilder { inner }
  }

  pub fn package_name(&self) -> &String {
    &self.package_name
  }

  pub fn store_product_id(&self) -> &String {
    &self.store_product_id
  }

  pub fn purchase_token(&self) -> &String {
    &self.purchase_token
  }
}

#[doc(hidden)]
pub struct RTDStoreTransactionGooglePlayBuilder {
  inner: StoreTransactionGooglePlay,
}

impl RTDStoreTransactionGooglePlayBuilder {
  pub fn build(&self) -> StoreTransactionGooglePlay {
    self.inner.clone()
  }

  pub fn package_name<T: AsRef<str>>(&mut self, package_name: T) -> &mut Self {
    self.inner.package_name = package_name.as_ref().to_string();
    self
  }

  pub fn store_product_id<T: AsRef<str>>(&mut self, store_product_id: T) -> &mut Self {
    self.inner.store_product_id = store_product_id.as_ref().to_string();
    self
  }

  pub fn purchase_token<T: AsRef<str>>(&mut self, purchase_token: T) -> &mut Self {
    self.inner.purchase_token = purchase_token.as_ref().to_string();
    self
  }
}

impl AsRef<StoreTransactionGooglePlay> for StoreTransactionGooglePlay {
  fn as_ref(&self) -> &StoreTransactionGooglePlay {
    self
  }
}

impl AsRef<StoreTransactionGooglePlay> for RTDStoreTransactionGooglePlayBuilder {
  fn as_ref(&self) -> &StoreTransactionGooglePlay {
    &self.inner
  }
}
