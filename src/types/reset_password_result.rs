
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Represents result of 2-step verification password reset
pub trait TDResetPasswordResult: Debug + RObject {}

/// Represents result of 2-step verification password reset
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ResetPasswordResult {
  #[doc(hidden)] _Default(()),
  /// Removes 2-step verification password without previous password and access to recovery email address. The password can't be reset immediately and the request needs to be repeated after the specified time
  ResetPassword(ResetPassword),
  /// The password reset request was declined
  Declined(ResetPasswordResultDeclined),
  /// The password was reset
  Ok(ResetPasswordResultOk),
  /// The password reset request is pending
  Pending(ResetPasswordResultPending),

}

impl Default for ResetPasswordResult {
  fn default() -> Self { ResetPasswordResult::_Default(()) }
}

impl<'de> Deserialize<'de> for ResetPasswordResult {
  fn deserialize<D>(deserializer: D) -> Result<ResetPasswordResult, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ResetPasswordResult,
      (resetPassword, ResetPassword);
      (resetPasswordResultDeclined, Declined);
      (resetPasswordResultOk, Ok);
      (resetPasswordResultPending, Pending);

    )(deserializer)
  }
}

impl RObject for ResetPasswordResult {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      ResetPasswordResult::ResetPassword(t) => t.td_name(),
      ResetPasswordResult::Declined(t) => t.td_name(),
      ResetPasswordResult::Ok(t) => t.td_name(),
      ResetPasswordResult::Pending(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      ResetPasswordResult::ResetPassword(t) => t.extra(),
      ResetPasswordResult::Declined(t) => t.extra(),
      ResetPasswordResult::Ok(t) => t.extra(),
      ResetPasswordResult::Pending(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl ResetPasswordResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let ResetPasswordResult::_Default(_) = self { true } else { false } }

  pub fn is_reset_password(&self) -> bool { if let ResetPasswordResult::ResetPassword(_) = self { true } else { false } }
  pub fn is_declined(&self) -> bool { if let ResetPasswordResult::Declined(_) = self { true } else { false } }
  pub fn is_ok(&self) -> bool { if let ResetPasswordResult::Ok(_) = self { true } else { false } }
  pub fn is_pending(&self) -> bool { if let ResetPasswordResult::Pending(_) = self { true } else { false } }

  pub fn on_reset_password<F: FnOnce(&ResetPassword)>(&self, fnc: F) -> &Self { if let ResetPasswordResult::ResetPassword(t) = self { fnc(t) }; self }
  pub fn on_declined<F: FnOnce(&ResetPasswordResultDeclined)>(&self, fnc: F) -> &Self { if let ResetPasswordResult::Declined(t) = self { fnc(t) }; self }
  pub fn on_ok<F: FnOnce(&ResetPasswordResultOk)>(&self, fnc: F) -> &Self { if let ResetPasswordResult::Ok(t) = self { fnc(t) }; self }
  pub fn on_pending<F: FnOnce(&ResetPasswordResultPending)>(&self, fnc: F) -> &Self { if let ResetPasswordResult::Pending(t) = self { fnc(t) }; self }

  pub fn as_reset_password(&self) -> Option<&ResetPassword> { if let ResetPasswordResult::ResetPassword(t) = self { return Some(t) } None }
  pub fn as_declined(&self) -> Option<&ResetPasswordResultDeclined> { if let ResetPasswordResult::Declined(t) = self { return Some(t) } None }
  pub fn as_ok(&self) -> Option<&ResetPasswordResultOk> { if let ResetPasswordResult::Ok(t) = self { return Some(t) } None }
  pub fn as_pending(&self) -> Option<&ResetPasswordResultPending> { if let ResetPasswordResult::Pending(t) = self { return Some(t) } None }



  pub fn reset_password<T: AsRef<ResetPassword>>(t: T) -> Self { ResetPasswordResult::ResetPassword(t.as_ref().clone()) }

  pub fn declined<T: AsRef<ResetPasswordResultDeclined>>(t: T) -> Self { ResetPasswordResult::Declined(t.as_ref().clone()) }

  pub fn ok<T: AsRef<ResetPasswordResultOk>>(t: T) -> Self { ResetPasswordResult::Ok(t.as_ref().clone()) }

  pub fn pending<T: AsRef<ResetPasswordResultPending>>(t: T) -> Self { ResetPasswordResult::Pending(t.as_ref().clone()) }

}

impl AsRef<ResetPasswordResult> for ResetPasswordResult {
  fn as_ref(&self) -> &ResetPasswordResult { self }
}







/// The password reset request was declined
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResetPasswordResultDeclined {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Point in time (Unix timestamp) when the password reset can be retried
  retry_date: i64,
  
}

impl RObject for ResetPasswordResultDeclined {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "resetPasswordResultDeclined" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDResetPasswordResult for ResetPasswordResultDeclined {}



impl ResetPasswordResultDeclined {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDResetPasswordResultDeclinedBuilder {
    let mut inner = ResetPasswordResultDeclined::default();
    inner.td_name = "resetPasswordResultDeclined".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDResetPasswordResultDeclinedBuilder { inner }
  }

  pub fn retry_date(&self) -> i64 { self.retry_date }

}

#[doc(hidden)]
pub struct RTDResetPasswordResultDeclinedBuilder {
  inner: ResetPasswordResultDeclined
}

impl RTDResetPasswordResultDeclinedBuilder {
  pub fn build(&self) -> ResetPasswordResultDeclined { self.inner.clone() }

   
  pub fn retry_date(&mut self, retry_date: i64) -> &mut Self {
    self.inner.retry_date = retry_date;
    self
  }

}

impl AsRef<ResetPasswordResultDeclined> for ResetPasswordResultDeclined {
  fn as_ref(&self) -> &ResetPasswordResultDeclined { self }
}

impl AsRef<ResetPasswordResultDeclined> for RTDResetPasswordResultDeclinedBuilder {
  fn as_ref(&self) -> &ResetPasswordResultDeclined { &self.inner }
}







/// The password was reset
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResetPasswordResultOk {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ResetPasswordResultOk {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "resetPasswordResultOk" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDResetPasswordResult for ResetPasswordResultOk {}



impl ResetPasswordResultOk {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDResetPasswordResultOkBuilder {
    let mut inner = ResetPasswordResultOk::default();
    inner.td_name = "resetPasswordResultOk".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDResetPasswordResultOkBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDResetPasswordResultOkBuilder {
  inner: ResetPasswordResultOk
}

impl RTDResetPasswordResultOkBuilder {
  pub fn build(&self) -> ResetPasswordResultOk { self.inner.clone() }

}

impl AsRef<ResetPasswordResultOk> for ResetPasswordResultOk {
  fn as_ref(&self) -> &ResetPasswordResultOk { self }
}

impl AsRef<ResetPasswordResultOk> for RTDResetPasswordResultOkBuilder {
  fn as_ref(&self) -> &ResetPasswordResultOk { &self.inner }
}







/// The password reset request is pending
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResetPasswordResultPending {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Point in time (Unix timestamp) after which the password can be reset immediately using resetPassword
  pending_reset_date: i64,
  
}

impl RObject for ResetPasswordResultPending {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "resetPasswordResultPending" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDResetPasswordResult for ResetPasswordResultPending {}



impl ResetPasswordResultPending {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDResetPasswordResultPendingBuilder {
    let mut inner = ResetPasswordResultPending::default();
    inner.td_name = "resetPasswordResultPending".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDResetPasswordResultPendingBuilder { inner }
  }

  pub fn pending_reset_date(&self) -> i64 { self.pending_reset_date }

}

#[doc(hidden)]
pub struct RTDResetPasswordResultPendingBuilder {
  inner: ResetPasswordResultPending
}

impl RTDResetPasswordResultPendingBuilder {
  pub fn build(&self) -> ResetPasswordResultPending { self.inner.clone() }

   
  pub fn pending_reset_date(&mut self, pending_reset_date: i64) -> &mut Self {
    self.inner.pending_reset_date = pending_reset_date;
    self
  }

}

impl AsRef<ResetPasswordResultPending> for ResetPasswordResultPending {
  fn as_ref(&self) -> &ResetPasswordResultPending { self }
}

impl AsRef<ResetPasswordResultPending> for RTDResetPasswordResultPendingBuilder {
  fn as_ref(&self) -> &ResetPasswordResultPending { &self.inner }
}



