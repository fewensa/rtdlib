use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a type of request to the TON Center API
pub trait TDTonCenterApiRequestType: Debug + RObject {}

/// Describes a type of request to the TON Center API
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TonCenterApiRequestType {
  #[doc(hidden)]
  _Default(()),
  /// A GET request
  Get(TonCenterApiRequestTypeGet),
  /// A POST request
  Post(TonCenterApiRequestTypePost),
}

impl Default for TonCenterApiRequestType {
  fn default() -> Self {
    TonCenterApiRequestType::_Default(())
  }
}

impl<'de> Deserialize<'de> for TonCenterApiRequestType {
  fn deserialize<D>(deserializer: D) -> Result<TonCenterApiRequestType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      TonCenterApiRequestType,
      (tonCenterApiRequestTypeGet, Get);
      (tonCenterApiRequestTypePost, Post);

    )(deserializer)
  }
}

impl RObject for TonCenterApiRequestType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      TonCenterApiRequestType::Get(t) => t.td_name(),
      TonCenterApiRequestType::Post(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      TonCenterApiRequestType::Get(t) => t.extra(),
      TonCenterApiRequestType::Post(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonCenterApiRequestType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, TonCenterApiRequestType::_Default(_))
  }

  pub fn is_get(&self) -> bool {
    matches!(self, TonCenterApiRequestType::Get(_))
  }
  pub fn is_post(&self) -> bool {
    matches!(self, TonCenterApiRequestType::Post(_))
  }

  pub fn on_get<F: FnOnce(&TonCenterApiRequestTypeGet)>(&self, fnc: F) -> &Self {
    if let TonCenterApiRequestType::Get(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_post<F: FnOnce(&TonCenterApiRequestTypePost)>(&self, fnc: F) -> &Self {
    if let TonCenterApiRequestType::Post(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_get(&self) -> Option<&TonCenterApiRequestTypeGet> {
    if let TonCenterApiRequestType::Get(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_post(&self) -> Option<&TonCenterApiRequestTypePost> {
    if let TonCenterApiRequestType::Post(t) = self {
      return Some(t);
    }
    None
  }

  pub fn get<T: AsRef<TonCenterApiRequestTypeGet>>(t: T) -> Self {
    TonCenterApiRequestType::Get(t.as_ref().clone())
  }

  pub fn post<T: AsRef<TonCenterApiRequestTypePost>>(t: T) -> Self {
    TonCenterApiRequestType::Post(t.as_ref().clone())
  }
}

impl AsRef<TonCenterApiRequestType> for TonCenterApiRequestType {
  fn as_ref(&self) -> &TonCenterApiRequestType {
    self
  }
}

/// A GET request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonCenterApiRequestTypeGet {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// URL-encoded query of the request
  query: String,
}

impl RObject for TonCenterApiRequestTypeGet {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonCenterApiRequestTypeGet"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonCenterApiRequestType for TonCenterApiRequestTypeGet {}

impl TonCenterApiRequestTypeGet {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonCenterApiRequestTypeGetBuilder {
    let inner = TonCenterApiRequestTypeGet {
      td_name: "tonCenterApiRequestTypeGet".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonCenterApiRequestTypeGetBuilder { inner }
  }

  pub fn query(&self) -> &String {
    &self.query
  }
}

#[doc(hidden)]
pub struct RTDTonCenterApiRequestTypeGetBuilder {
  inner: TonCenterApiRequestTypeGet,
}

impl RTDTonCenterApiRequestTypeGetBuilder {
  pub fn build(&self) -> TonCenterApiRequestTypeGet {
    self.inner.clone()
  }

  pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
    self.inner.query = query.as_ref().to_string();
    self
  }
}

impl AsRef<TonCenterApiRequestTypeGet> for TonCenterApiRequestTypeGet {
  fn as_ref(&self) -> &TonCenterApiRequestTypeGet {
    self
  }
}

impl AsRef<TonCenterApiRequestTypeGet> for RTDTonCenterApiRequestTypeGetBuilder {
  fn as_ref(&self) -> &TonCenterApiRequestTypeGet {
    &self.inner
  }
}

/// A POST request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonCenterApiRequestTypePost {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Payload of the request
  payload: String,
}

impl RObject for TonCenterApiRequestTypePost {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonCenterApiRequestTypePost"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonCenterApiRequestType for TonCenterApiRequestTypePost {}

impl TonCenterApiRequestTypePost {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonCenterApiRequestTypePostBuilder {
    let inner = TonCenterApiRequestTypePost {
      td_name: "tonCenterApiRequestTypePost".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonCenterApiRequestTypePostBuilder { inner }
  }

  pub fn payload(&self) -> &String {
    &self.payload
  }
}

#[doc(hidden)]
pub struct RTDTonCenterApiRequestTypePostBuilder {
  inner: TonCenterApiRequestTypePost,
}

impl RTDTonCenterApiRequestTypePostBuilder {
  pub fn build(&self) -> TonCenterApiRequestTypePost {
    self.inner.clone()
  }

  pub fn payload<T: AsRef<str>>(&mut self, payload: T) -> &mut Self {
    self.inner.payload = payload.as_ref().to_string();
    self
  }
}

impl AsRef<TonCenterApiRequestTypePost> for TonCenterApiRequestTypePost {
  fn as_ref(&self) -> &TonCenterApiRequestTypePost {
    self
  }
}

impl AsRef<TonCenterApiRequestTypePost> for RTDTonCenterApiRequestTypePostBuilder {
  fn as_ref(&self) -> &TonCenterApiRequestTypePost {
    &self.inner
  }
}
