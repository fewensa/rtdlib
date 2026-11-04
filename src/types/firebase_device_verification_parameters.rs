use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes parameters to be used for device verification
pub trait TDFirebaseDeviceVerificationParameters: Debug + RObject {}

/// Describes parameters to be used for device verification
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum FirebaseDeviceVerificationParameters {
  #[doc(hidden)]
  _Default(()),
  /// Device verification must be performed with the classic Play Integrity verification (https://developer.android.com/google/play/integrity/classic)
  PlayIntegrity(FirebaseDeviceVerificationParametersPlayIntegrity),
  /// Device verification must be performed with the SafetyNet Attestation API
  SafetyNet(FirebaseDeviceVerificationParametersSafetyNet),
}

impl Default for FirebaseDeviceVerificationParameters {
  fn default() -> Self {
    FirebaseDeviceVerificationParameters::_Default(())
  }
}

impl<'de> Deserialize<'de> for FirebaseDeviceVerificationParameters {
  fn deserialize<D>(deserializer: D) -> Result<FirebaseDeviceVerificationParameters, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      FirebaseDeviceVerificationParameters,
      (firebaseDeviceVerificationParametersPlayIntegrity, PlayIntegrity);
      (firebaseDeviceVerificationParametersSafetyNet, SafetyNet);

    )(deserializer)
  }
}

impl RObject for FirebaseDeviceVerificationParameters {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      FirebaseDeviceVerificationParameters::PlayIntegrity(t) => t.td_name(),
      FirebaseDeviceVerificationParameters::SafetyNet(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      FirebaseDeviceVerificationParameters::PlayIntegrity(t) => t.extra(),
      FirebaseDeviceVerificationParameters::SafetyNet(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl FirebaseDeviceVerificationParameters {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, FirebaseDeviceVerificationParameters::_Default(_))
  }

  pub fn is_play_integrity(&self) -> bool {
    matches!(self, FirebaseDeviceVerificationParameters::PlayIntegrity(_))
  }
  pub fn is_safety_net(&self) -> bool {
    matches!(self, FirebaseDeviceVerificationParameters::SafetyNet(_))
  }

  pub fn on_play_integrity<F: FnOnce(&FirebaseDeviceVerificationParametersPlayIntegrity)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let FirebaseDeviceVerificationParameters::PlayIntegrity(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_safety_net<F: FnOnce(&FirebaseDeviceVerificationParametersSafetyNet)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let FirebaseDeviceVerificationParameters::SafetyNet(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_play_integrity(&self) -> Option<&FirebaseDeviceVerificationParametersPlayIntegrity> {
    if let FirebaseDeviceVerificationParameters::PlayIntegrity(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_safety_net(&self) -> Option<&FirebaseDeviceVerificationParametersSafetyNet> {
    if let FirebaseDeviceVerificationParameters::SafetyNet(t) = self {
      return Some(t);
    }
    None
  }

  pub fn play_integrity<T: AsRef<FirebaseDeviceVerificationParametersPlayIntegrity>>(t: T) -> Self {
    FirebaseDeviceVerificationParameters::PlayIntegrity(t.as_ref().clone())
  }

  pub fn safety_net<T: AsRef<FirebaseDeviceVerificationParametersSafetyNet>>(t: T) -> Self {
    FirebaseDeviceVerificationParameters::SafetyNet(t.as_ref().clone())
  }
}

impl AsRef<FirebaseDeviceVerificationParameters> for FirebaseDeviceVerificationParameters {
  fn as_ref(&self) -> &FirebaseDeviceVerificationParameters {
    self
  }
}

/// Device verification must be performed with the classic Play Integrity verification (https://developer.android.com/google/play/integrity/classic)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FirebaseDeviceVerificationParametersPlayIntegrity {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Base64url-encoded nonce to pass to the Play Integrity API
  nonce: String,
  /// Cloud project number to pass to the Play Integrity API
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  cloud_project_number: isize,
}

impl RObject for FirebaseDeviceVerificationParametersPlayIntegrity {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "firebaseDeviceVerificationParametersPlayIntegrity"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFirebaseDeviceVerificationParameters for FirebaseDeviceVerificationParametersPlayIntegrity {}

impl FirebaseDeviceVerificationParametersPlayIntegrity {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFirebaseDeviceVerificationParametersPlayIntegrityBuilder {
    let inner = FirebaseDeviceVerificationParametersPlayIntegrity {
      td_name: "firebaseDeviceVerificationParametersPlayIntegrity".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDFirebaseDeviceVerificationParametersPlayIntegrityBuilder { inner }
  }

  pub fn nonce(&self) -> &String {
    &self.nonce
  }

  pub fn cloud_project_number(&self) -> isize {
    self.cloud_project_number
  }
}

#[doc(hidden)]
pub struct RTDFirebaseDeviceVerificationParametersPlayIntegrityBuilder {
  inner: FirebaseDeviceVerificationParametersPlayIntegrity,
}

impl RTDFirebaseDeviceVerificationParametersPlayIntegrityBuilder {
  pub fn build(&self) -> FirebaseDeviceVerificationParametersPlayIntegrity {
    self.inner.clone()
  }

  pub fn nonce<T: AsRef<str>>(&mut self, nonce: T) -> &mut Self {
    self.inner.nonce = nonce.as_ref().to_string();
    self
  }

  pub fn cloud_project_number(&mut self, cloud_project_number: isize) -> &mut Self {
    self.inner.cloud_project_number = cloud_project_number;
    self
  }
}

impl AsRef<FirebaseDeviceVerificationParametersPlayIntegrity>
  for FirebaseDeviceVerificationParametersPlayIntegrity
{
  fn as_ref(&self) -> &FirebaseDeviceVerificationParametersPlayIntegrity {
    self
  }
}

impl AsRef<FirebaseDeviceVerificationParametersPlayIntegrity>
  for RTDFirebaseDeviceVerificationParametersPlayIntegrityBuilder
{
  fn as_ref(&self) -> &FirebaseDeviceVerificationParametersPlayIntegrity {
    &self.inner
  }
}

/// Device verification must be performed with the SafetyNet Attestation API
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FirebaseDeviceVerificationParametersSafetyNet {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Nonce to pass to the SafetyNet Attestation API
  nonce: String,
}

impl RObject for FirebaseDeviceVerificationParametersSafetyNet {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "firebaseDeviceVerificationParametersSafetyNet"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFirebaseDeviceVerificationParameters for FirebaseDeviceVerificationParametersSafetyNet {}

impl FirebaseDeviceVerificationParametersSafetyNet {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFirebaseDeviceVerificationParametersSafetyNetBuilder {
    let inner = FirebaseDeviceVerificationParametersSafetyNet {
      td_name: "firebaseDeviceVerificationParametersSafetyNet".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDFirebaseDeviceVerificationParametersSafetyNetBuilder { inner }
  }

  pub fn nonce(&self) -> &String {
    &self.nonce
  }
}

#[doc(hidden)]
pub struct RTDFirebaseDeviceVerificationParametersSafetyNetBuilder {
  inner: FirebaseDeviceVerificationParametersSafetyNet,
}

impl RTDFirebaseDeviceVerificationParametersSafetyNetBuilder {
  pub fn build(&self) -> FirebaseDeviceVerificationParametersSafetyNet {
    self.inner.clone()
  }

  pub fn nonce<T: AsRef<str>>(&mut self, nonce: T) -> &mut Self {
    self.inner.nonce = nonce.as_ref().to_string();
    self
  }
}

impl AsRef<FirebaseDeviceVerificationParametersSafetyNet>
  for FirebaseDeviceVerificationParametersSafetyNet
{
  fn as_ref(&self) -> &FirebaseDeviceVerificationParametersSafetyNet {
    self
  }
}

impl AsRef<FirebaseDeviceVerificationParametersSafetyNet>
  for RTDFirebaseDeviceVerificationParametersSafetyNetBuilder
{
  fn as_ref(&self) -> &FirebaseDeviceVerificationParametersSafetyNet {
    &self.inner
  }
}
