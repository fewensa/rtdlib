
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Represents result of checking whether the current session can be used to transfer a chat ownership to another user
pub trait TDCanTransferOwnershipResult: Debug + RObject {}

/// Represents result of checking whether the current session can be used to transfer a chat ownership to another user
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CanTransferOwnershipResult {
  #[doc(hidden)] _Default(()),
  /// Checks whether the current session can be used to transfer a chat ownership to another user
  CanTransferOwnership(CanTransferOwnership),
  /// The session can be used
  Ok(CanTransferOwnershipResultOk),
  /// The 2-step verification needs to be enabled first
  PasswordNeeded(CanTransferOwnershipResultPasswordNeeded),
  /// The 2-step verification was enabled recently, user needs to wait
  PasswordTooFresh(CanTransferOwnershipResultPasswordTooFresh),
  /// The session was created recently, user needs to wait
  SessionTooFresh(CanTransferOwnershipResultSessionTooFresh),

}

impl Default for CanTransferOwnershipResult {
  fn default() -> Self { CanTransferOwnershipResult::_Default(()) }
}

impl<'de> Deserialize<'de> for CanTransferOwnershipResult {
  fn deserialize<D>(deserializer: D) -> Result<CanTransferOwnershipResult, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      CanTransferOwnershipResult,
      (canTransferOwnership, CanTransferOwnership);
      (canTransferOwnershipResultOk, Ok);
      (canTransferOwnershipResultPasswordNeeded, PasswordNeeded);
      (canTransferOwnershipResultPasswordTooFresh, PasswordTooFresh);
      (canTransferOwnershipResultSessionTooFresh, SessionTooFresh);

    )(deserializer)
  }
}

impl RObject for CanTransferOwnershipResult {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      CanTransferOwnershipResult::CanTransferOwnership(t) => t.td_name(),
      CanTransferOwnershipResult::Ok(t) => t.td_name(),
      CanTransferOwnershipResult::PasswordNeeded(t) => t.td_name(),
      CanTransferOwnershipResult::PasswordTooFresh(t) => t.td_name(),
      CanTransferOwnershipResult::SessionTooFresh(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      CanTransferOwnershipResult::CanTransferOwnership(t) => t.extra(),
      CanTransferOwnershipResult::Ok(t) => t.extra(),
      CanTransferOwnershipResult::PasswordNeeded(t) => t.extra(),
      CanTransferOwnershipResult::PasswordTooFresh(t) => t.extra(),
      CanTransferOwnershipResult::SessionTooFresh(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl CanTransferOwnershipResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let CanTransferOwnershipResult::_Default(_) = self { true } else { false } }

  pub fn is_can_transfer_ownership(&self) -> bool { if let CanTransferOwnershipResult::CanTransferOwnership(_) = self { true } else { false } }
  pub fn is_ok(&self) -> bool { if let CanTransferOwnershipResult::Ok(_) = self { true } else { false } }
  pub fn is_password_needed(&self) -> bool { if let CanTransferOwnershipResult::PasswordNeeded(_) = self { true } else { false } }
  pub fn is_password_too_fresh(&self) -> bool { if let CanTransferOwnershipResult::PasswordTooFresh(_) = self { true } else { false } }
  pub fn is_session_too_fresh(&self) -> bool { if let CanTransferOwnershipResult::SessionTooFresh(_) = self { true } else { false } }

  pub fn on_can_transfer_ownership<F: FnOnce(&CanTransferOwnership)>(&self, fnc: F) -> &Self { if let CanTransferOwnershipResult::CanTransferOwnership(t) = self { fnc(t) }; self }
  pub fn on_ok<F: FnOnce(&CanTransferOwnershipResultOk)>(&self, fnc: F) -> &Self { if let CanTransferOwnershipResult::Ok(t) = self { fnc(t) }; self }
  pub fn on_password_needed<F: FnOnce(&CanTransferOwnershipResultPasswordNeeded)>(&self, fnc: F) -> &Self { if let CanTransferOwnershipResult::PasswordNeeded(t) = self { fnc(t) }; self }
  pub fn on_password_too_fresh<F: FnOnce(&CanTransferOwnershipResultPasswordTooFresh)>(&self, fnc: F) -> &Self { if let CanTransferOwnershipResult::PasswordTooFresh(t) = self { fnc(t) }; self }
  pub fn on_session_too_fresh<F: FnOnce(&CanTransferOwnershipResultSessionTooFresh)>(&self, fnc: F) -> &Self { if let CanTransferOwnershipResult::SessionTooFresh(t) = self { fnc(t) }; self }

  pub fn as_can_transfer_ownership(&self) -> Option<&CanTransferOwnership> { if let CanTransferOwnershipResult::CanTransferOwnership(t) = self { return Some(t) } None }
  pub fn as_ok(&self) -> Option<&CanTransferOwnershipResultOk> { if let CanTransferOwnershipResult::Ok(t) = self { return Some(t) } None }
  pub fn as_password_needed(&self) -> Option<&CanTransferOwnershipResultPasswordNeeded> { if let CanTransferOwnershipResult::PasswordNeeded(t) = self { return Some(t) } None }
  pub fn as_password_too_fresh(&self) -> Option<&CanTransferOwnershipResultPasswordTooFresh> { if let CanTransferOwnershipResult::PasswordTooFresh(t) = self { return Some(t) } None }
  pub fn as_session_too_fresh(&self) -> Option<&CanTransferOwnershipResultSessionTooFresh> { if let CanTransferOwnershipResult::SessionTooFresh(t) = self { return Some(t) } None }



  pub fn can_transfer_ownership<T: AsRef<CanTransferOwnership>>(t: T) -> Self { CanTransferOwnershipResult::CanTransferOwnership(t.as_ref().clone()) }

  pub fn ok<T: AsRef<CanTransferOwnershipResultOk>>(t: T) -> Self { CanTransferOwnershipResult::Ok(t.as_ref().clone()) }

  pub fn password_needed<T: AsRef<CanTransferOwnershipResultPasswordNeeded>>(t: T) -> Self { CanTransferOwnershipResult::PasswordNeeded(t.as_ref().clone()) }

  pub fn password_too_fresh<T: AsRef<CanTransferOwnershipResultPasswordTooFresh>>(t: T) -> Self { CanTransferOwnershipResult::PasswordTooFresh(t.as_ref().clone()) }

  pub fn session_too_fresh<T: AsRef<CanTransferOwnershipResultSessionTooFresh>>(t: T) -> Self { CanTransferOwnershipResult::SessionTooFresh(t.as_ref().clone()) }

}

impl AsRef<CanTransferOwnershipResult> for CanTransferOwnershipResult {
  fn as_ref(&self) -> &CanTransferOwnershipResult { self }
}







/// The session can be used
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanTransferOwnershipResultOk {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for CanTransferOwnershipResultOk {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "canTransferOwnershipResultOk" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDCanTransferOwnershipResult for CanTransferOwnershipResultOk {}



impl CanTransferOwnershipResultOk {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCanTransferOwnershipResultOkBuilder {
    let mut inner = CanTransferOwnershipResultOk::default();
    inner.td_name = "canTransferOwnershipResultOk".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCanTransferOwnershipResultOkBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDCanTransferOwnershipResultOkBuilder {
  inner: CanTransferOwnershipResultOk
}

impl RTDCanTransferOwnershipResultOkBuilder {
  pub fn build(&self) -> CanTransferOwnershipResultOk { self.inner.clone() }

}

impl AsRef<CanTransferOwnershipResultOk> for CanTransferOwnershipResultOk {
  fn as_ref(&self) -> &CanTransferOwnershipResultOk { self }
}

impl AsRef<CanTransferOwnershipResultOk> for RTDCanTransferOwnershipResultOkBuilder {
  fn as_ref(&self) -> &CanTransferOwnershipResultOk { &self.inner }
}







/// The 2-step verification needs to be enabled first
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanTransferOwnershipResultPasswordNeeded {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for CanTransferOwnershipResultPasswordNeeded {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "canTransferOwnershipResultPasswordNeeded" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDCanTransferOwnershipResult for CanTransferOwnershipResultPasswordNeeded {}



impl CanTransferOwnershipResultPasswordNeeded {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCanTransferOwnershipResultPasswordNeededBuilder {
    let mut inner = CanTransferOwnershipResultPasswordNeeded::default();
    inner.td_name = "canTransferOwnershipResultPasswordNeeded".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCanTransferOwnershipResultPasswordNeededBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDCanTransferOwnershipResultPasswordNeededBuilder {
  inner: CanTransferOwnershipResultPasswordNeeded
}

impl RTDCanTransferOwnershipResultPasswordNeededBuilder {
  pub fn build(&self) -> CanTransferOwnershipResultPasswordNeeded { self.inner.clone() }

}

impl AsRef<CanTransferOwnershipResultPasswordNeeded> for CanTransferOwnershipResultPasswordNeeded {
  fn as_ref(&self) -> &CanTransferOwnershipResultPasswordNeeded { self }
}

impl AsRef<CanTransferOwnershipResultPasswordNeeded> for RTDCanTransferOwnershipResultPasswordNeededBuilder {
  fn as_ref(&self) -> &CanTransferOwnershipResultPasswordNeeded { &self.inner }
}







/// The 2-step verification was enabled recently, user needs to wait
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanTransferOwnershipResultPasswordTooFresh {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Time left before the session can be used to transfer ownership of a chat, in seconds
  retry_after: i64,
  
}

impl RObject for CanTransferOwnershipResultPasswordTooFresh {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "canTransferOwnershipResultPasswordTooFresh" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDCanTransferOwnershipResult for CanTransferOwnershipResultPasswordTooFresh {}



impl CanTransferOwnershipResultPasswordTooFresh {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCanTransferOwnershipResultPasswordTooFreshBuilder {
    let mut inner = CanTransferOwnershipResultPasswordTooFresh::default();
    inner.td_name = "canTransferOwnershipResultPasswordTooFresh".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCanTransferOwnershipResultPasswordTooFreshBuilder { inner }
  }

  pub fn retry_after(&self) -> i64 { self.retry_after }

}

#[doc(hidden)]
pub struct RTDCanTransferOwnershipResultPasswordTooFreshBuilder {
  inner: CanTransferOwnershipResultPasswordTooFresh
}

impl RTDCanTransferOwnershipResultPasswordTooFreshBuilder {
  pub fn build(&self) -> CanTransferOwnershipResultPasswordTooFresh { self.inner.clone() }

   
  pub fn retry_after(&mut self, retry_after: i64) -> &mut Self {
    self.inner.retry_after = retry_after;
    self
  }

}

impl AsRef<CanTransferOwnershipResultPasswordTooFresh> for CanTransferOwnershipResultPasswordTooFresh {
  fn as_ref(&self) -> &CanTransferOwnershipResultPasswordTooFresh { self }
}

impl AsRef<CanTransferOwnershipResultPasswordTooFresh> for RTDCanTransferOwnershipResultPasswordTooFreshBuilder {
  fn as_ref(&self) -> &CanTransferOwnershipResultPasswordTooFresh { &self.inner }
}







/// The session was created recently, user needs to wait
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanTransferOwnershipResultSessionTooFresh {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Time left before the session can be used to transfer ownership of a chat, in seconds
  retry_after: i64,
  
}

impl RObject for CanTransferOwnershipResultSessionTooFresh {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "canTransferOwnershipResultSessionTooFresh" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDCanTransferOwnershipResult for CanTransferOwnershipResultSessionTooFresh {}



impl CanTransferOwnershipResultSessionTooFresh {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCanTransferOwnershipResultSessionTooFreshBuilder {
    let mut inner = CanTransferOwnershipResultSessionTooFresh::default();
    inner.td_name = "canTransferOwnershipResultSessionTooFresh".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCanTransferOwnershipResultSessionTooFreshBuilder { inner }
  }

  pub fn retry_after(&self) -> i64 { self.retry_after }

}

#[doc(hidden)]
pub struct RTDCanTransferOwnershipResultSessionTooFreshBuilder {
  inner: CanTransferOwnershipResultSessionTooFresh
}

impl RTDCanTransferOwnershipResultSessionTooFreshBuilder {
  pub fn build(&self) -> CanTransferOwnershipResultSessionTooFresh { self.inner.clone() }

   
  pub fn retry_after(&mut self, retry_after: i64) -> &mut Self {
    self.inner.retry_after = retry_after;
    self
  }

}

impl AsRef<CanTransferOwnershipResultSessionTooFresh> for CanTransferOwnershipResultSessionTooFresh {
  fn as_ref(&self) -> &CanTransferOwnershipResultSessionTooFresh { self }
}

impl AsRef<CanTransferOwnershipResultSessionTooFresh> for RTDCanTransferOwnershipResultSessionTooFreshBuilder {
  fn as_ref(&self) -> &CanTransferOwnershipResultSessionTooFresh { &self.inner }
}



