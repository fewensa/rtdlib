use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a collectible item that can be purchased at https://fragment.com
pub trait TDCollectibleItemType: Debug + RObject {}

/// Describes a collectible item that can be purchased at https://fragment.com
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CollectibleItemType {
  #[doc(hidden)]
  _Default(()),
  /// A phone number
  PhoneNumber(CollectibleItemTypePhoneNumber),
  /// A username
  Username(CollectibleItemTypeUsername),
}

impl Default for CollectibleItemType {
  fn default() -> Self {
    CollectibleItemType::_Default(())
  }
}

impl<'de> Deserialize<'de> for CollectibleItemType {
  fn deserialize<D>(deserializer: D) -> Result<CollectibleItemType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      CollectibleItemType,
      (collectibleItemTypePhoneNumber, PhoneNumber);
      (collectibleItemTypeUsername, Username);

    )(deserializer)
  }
}

impl RObject for CollectibleItemType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      CollectibleItemType::PhoneNumber(t) => t.td_name(),
      CollectibleItemType::Username(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      CollectibleItemType::PhoneNumber(t) => t.extra(),
      CollectibleItemType::Username(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl CollectibleItemType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, CollectibleItemType::_Default(_))
  }

  pub fn is_phone_number(&self) -> bool {
    matches!(self, CollectibleItemType::PhoneNumber(_))
  }
  pub fn is_username(&self) -> bool {
    matches!(self, CollectibleItemType::Username(_))
  }

  pub fn on_phone_number<F: FnOnce(&CollectibleItemTypePhoneNumber)>(&self, fnc: F) -> &Self {
    if let CollectibleItemType::PhoneNumber(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_username<F: FnOnce(&CollectibleItemTypeUsername)>(&self, fnc: F) -> &Self {
    if let CollectibleItemType::Username(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_phone_number(&self) -> Option<&CollectibleItemTypePhoneNumber> {
    if let CollectibleItemType::PhoneNumber(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_username(&self) -> Option<&CollectibleItemTypeUsername> {
    if let CollectibleItemType::Username(t) = self {
      return Some(t);
    }
    None
  }

  pub fn phone_number<T: AsRef<CollectibleItemTypePhoneNumber>>(t: T) -> Self {
    CollectibleItemType::PhoneNumber(t.as_ref().clone())
  }

  pub fn username<T: AsRef<CollectibleItemTypeUsername>>(t: T) -> Self {
    CollectibleItemType::Username(t.as_ref().clone())
  }
}

impl AsRef<CollectibleItemType> for CollectibleItemType {
  fn as_ref(&self) -> &CollectibleItemType {
    self
  }
}

/// A phone number
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollectibleItemTypePhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The phone number
  phone_number: String,
}

impl RObject for CollectibleItemTypePhoneNumber {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "collectibleItemTypePhoneNumber"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCollectibleItemType for CollectibleItemTypePhoneNumber {}

impl CollectibleItemTypePhoneNumber {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCollectibleItemTypePhoneNumberBuilder {
    let inner = CollectibleItemTypePhoneNumber {
      td_name: "collectibleItemTypePhoneNumber".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCollectibleItemTypePhoneNumberBuilder { inner }
  }

  pub fn phone_number(&self) -> &String {
    &self.phone_number
  }
}

#[doc(hidden)]
pub struct RTDCollectibleItemTypePhoneNumberBuilder {
  inner: CollectibleItemTypePhoneNumber,
}

impl RTDCollectibleItemTypePhoneNumberBuilder {
  pub fn build(&self) -> CollectibleItemTypePhoneNumber {
    self.inner.clone()
  }

  pub fn phone_number<T: AsRef<str>>(&mut self, phone_number: T) -> &mut Self {
    self.inner.phone_number = phone_number.as_ref().to_string();
    self
  }
}

impl AsRef<CollectibleItemTypePhoneNumber> for CollectibleItemTypePhoneNumber {
  fn as_ref(&self) -> &CollectibleItemTypePhoneNumber {
    self
  }
}

impl AsRef<CollectibleItemTypePhoneNumber> for RTDCollectibleItemTypePhoneNumberBuilder {
  fn as_ref(&self) -> &CollectibleItemTypePhoneNumber {
    &self.inner
  }
}

/// A username
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollectibleItemTypeUsername {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The username
  username: String,
}

impl RObject for CollectibleItemTypeUsername {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "collectibleItemTypeUsername"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCollectibleItemType for CollectibleItemTypeUsername {}

impl CollectibleItemTypeUsername {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCollectibleItemTypeUsernameBuilder {
    let inner = CollectibleItemTypeUsername {
      td_name: "collectibleItemTypeUsername".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCollectibleItemTypeUsernameBuilder { inner }
  }

  pub fn username(&self) -> &String {
    &self.username
  }
}

#[doc(hidden)]
pub struct RTDCollectibleItemTypeUsernameBuilder {
  inner: CollectibleItemTypeUsername,
}

impl RTDCollectibleItemTypeUsernameBuilder {
  pub fn build(&self) -> CollectibleItemTypeUsername {
    self.inner.clone()
  }

  pub fn username<T: AsRef<str>>(&mut self, username: T) -> &mut Self {
    self.inner.username = username.as_ref().to_string();
    self
  }
}

impl AsRef<CollectibleItemTypeUsername> for CollectibleItemTypeUsername {
  fn as_ref(&self) -> &CollectibleItemTypeUsername {
    self
  }
}

impl AsRef<CollectibleItemTypeUsername> for RTDCollectibleItemTypeUsernameBuilder {
  fn as_ref(&self) -> &CollectibleItemTypeUsername {
    &self.inner
  }
}
