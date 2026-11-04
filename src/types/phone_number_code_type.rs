use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes type of the request for which a code is sent to a phone number
pub trait TDPhoneNumberCodeType: Debug + RObject {}

/// Describes type of the request for which a code is sent to a phone number
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PhoneNumberCodeType {
  #[doc(hidden)]
  _Default(()),
  /// Checks ownership of a new phone number to change the user's authentication phone number; for official Android and iOS applications only
  Change(PhoneNumberCodeTypeChange),
  /// Confirms ownership of a phone number to prevent account deletion while handling links of the type internalLinkTypePhoneNumberConfirmation
  ConfirmOwnership(PhoneNumberCodeTypeConfirmOwnership),
  /// Verifies ownership of a phone number to be added to the user's Telegram Passport
  Verify(PhoneNumberCodeTypeVerify),
}

impl Default for PhoneNumberCodeType {
  fn default() -> Self {
    PhoneNumberCodeType::_Default(())
  }
}

impl<'de> Deserialize<'de> for PhoneNumberCodeType {
  fn deserialize<D>(deserializer: D) -> Result<PhoneNumberCodeType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      PhoneNumberCodeType,
      (phoneNumberCodeTypeChange, Change);
      (phoneNumberCodeTypeConfirmOwnership, ConfirmOwnership);
      (phoneNumberCodeTypeVerify, Verify);

    )(deserializer)
  }
}

impl RObject for PhoneNumberCodeType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      PhoneNumberCodeType::Change(t) => t.td_name(),
      PhoneNumberCodeType::ConfirmOwnership(t) => t.td_name(),
      PhoneNumberCodeType::Verify(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      PhoneNumberCodeType::Change(t) => t.extra(),
      PhoneNumberCodeType::ConfirmOwnership(t) => t.extra(),
      PhoneNumberCodeType::Verify(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PhoneNumberCodeType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, PhoneNumberCodeType::_Default(_))
  }

  pub fn is_change(&self) -> bool {
    matches!(self, PhoneNumberCodeType::Change(_))
  }
  pub fn is_confirm_ownership(&self) -> bool {
    matches!(self, PhoneNumberCodeType::ConfirmOwnership(_))
  }
  pub fn is_verify(&self) -> bool {
    matches!(self, PhoneNumberCodeType::Verify(_))
  }

  pub fn on_change<F: FnOnce(&PhoneNumberCodeTypeChange)>(&self, fnc: F) -> &Self {
    if let PhoneNumberCodeType::Change(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_confirm_ownership<F: FnOnce(&PhoneNumberCodeTypeConfirmOwnership)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PhoneNumberCodeType::ConfirmOwnership(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_verify<F: FnOnce(&PhoneNumberCodeTypeVerify)>(&self, fnc: F) -> &Self {
    if let PhoneNumberCodeType::Verify(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_change(&self) -> Option<&PhoneNumberCodeTypeChange> {
    if let PhoneNumberCodeType::Change(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_confirm_ownership(&self) -> Option<&PhoneNumberCodeTypeConfirmOwnership> {
    if let PhoneNumberCodeType::ConfirmOwnership(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_verify(&self) -> Option<&PhoneNumberCodeTypeVerify> {
    if let PhoneNumberCodeType::Verify(t) = self {
      return Some(t);
    }
    None
  }

  pub fn change<T: AsRef<PhoneNumberCodeTypeChange>>(t: T) -> Self {
    PhoneNumberCodeType::Change(t.as_ref().clone())
  }

  pub fn confirm_ownership<T: AsRef<PhoneNumberCodeTypeConfirmOwnership>>(t: T) -> Self {
    PhoneNumberCodeType::ConfirmOwnership(t.as_ref().clone())
  }

  pub fn verify<T: AsRef<PhoneNumberCodeTypeVerify>>(t: T) -> Self {
    PhoneNumberCodeType::Verify(t.as_ref().clone())
  }
}

impl AsRef<PhoneNumberCodeType> for PhoneNumberCodeType {
  fn as_ref(&self) -> &PhoneNumberCodeType {
    self
  }
}

/// Checks ownership of a new phone number to change the user's authentication phone number; for official Android and iOS applications only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PhoneNumberCodeTypeChange {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PhoneNumberCodeTypeChange {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "phoneNumberCodeTypeChange"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPhoneNumberCodeType for PhoneNumberCodeTypeChange {}

impl PhoneNumberCodeTypeChange {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPhoneNumberCodeTypeChangeBuilder {
    let inner = PhoneNumberCodeTypeChange {
      td_name: "phoneNumberCodeTypeChange".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPhoneNumberCodeTypeChangeBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPhoneNumberCodeTypeChangeBuilder {
  inner: PhoneNumberCodeTypeChange,
}

impl RTDPhoneNumberCodeTypeChangeBuilder {
  pub fn build(&self) -> PhoneNumberCodeTypeChange {
    self.inner.clone()
  }
}

impl AsRef<PhoneNumberCodeTypeChange> for PhoneNumberCodeTypeChange {
  fn as_ref(&self) -> &PhoneNumberCodeTypeChange {
    self
  }
}

impl AsRef<PhoneNumberCodeTypeChange> for RTDPhoneNumberCodeTypeChangeBuilder {
  fn as_ref(&self) -> &PhoneNumberCodeTypeChange {
    &self.inner
  }
}

/// Confirms ownership of a phone number to prevent account deletion while handling links of the type internalLinkTypePhoneNumberConfirmation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PhoneNumberCodeTypeConfirmOwnership {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Hash value from the link
  hash: String,
}

impl RObject for PhoneNumberCodeTypeConfirmOwnership {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "phoneNumberCodeTypeConfirmOwnership"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPhoneNumberCodeType for PhoneNumberCodeTypeConfirmOwnership {}

impl PhoneNumberCodeTypeConfirmOwnership {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPhoneNumberCodeTypeConfirmOwnershipBuilder {
    let inner = PhoneNumberCodeTypeConfirmOwnership {
      td_name: "phoneNumberCodeTypeConfirmOwnership".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPhoneNumberCodeTypeConfirmOwnershipBuilder { inner }
  }

  pub fn hash(&self) -> &String {
    &self.hash
  }
}

#[doc(hidden)]
pub struct RTDPhoneNumberCodeTypeConfirmOwnershipBuilder {
  inner: PhoneNumberCodeTypeConfirmOwnership,
}

impl RTDPhoneNumberCodeTypeConfirmOwnershipBuilder {
  pub fn build(&self) -> PhoneNumberCodeTypeConfirmOwnership {
    self.inner.clone()
  }

  pub fn hash<T: AsRef<str>>(&mut self, hash: T) -> &mut Self {
    self.inner.hash = hash.as_ref().to_string();
    self
  }
}

impl AsRef<PhoneNumberCodeTypeConfirmOwnership> for PhoneNumberCodeTypeConfirmOwnership {
  fn as_ref(&self) -> &PhoneNumberCodeTypeConfirmOwnership {
    self
  }
}

impl AsRef<PhoneNumberCodeTypeConfirmOwnership> for RTDPhoneNumberCodeTypeConfirmOwnershipBuilder {
  fn as_ref(&self) -> &PhoneNumberCodeTypeConfirmOwnership {
    &self.inner
  }
}

/// Verifies ownership of a phone number to be added to the user's Telegram Passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PhoneNumberCodeTypeVerify {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PhoneNumberCodeTypeVerify {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "phoneNumberCodeTypeVerify"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPhoneNumberCodeType for PhoneNumberCodeTypeVerify {}

impl PhoneNumberCodeTypeVerify {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPhoneNumberCodeTypeVerifyBuilder {
    let inner = PhoneNumberCodeTypeVerify {
      td_name: "phoneNumberCodeTypeVerify".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPhoneNumberCodeTypeVerifyBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPhoneNumberCodeTypeVerifyBuilder {
  inner: PhoneNumberCodeTypeVerify,
}

impl RTDPhoneNumberCodeTypeVerifyBuilder {
  pub fn build(&self) -> PhoneNumberCodeTypeVerify {
    self.inner.clone()
  }
}

impl AsRef<PhoneNumberCodeTypeVerify> for PhoneNumberCodeTypeVerify {
  fn as_ref(&self) -> &PhoneNumberCodeTypeVerify {
    self
  }
}

impl AsRef<PhoneNumberCodeTypeVerify> for RTDPhoneNumberCodeTypeVerifyBuilder {
  fn as_ref(&self) -> &PhoneNumberCodeTypeVerify {
    &self.inner
  }
}
