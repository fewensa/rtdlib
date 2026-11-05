use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes reason for refund of the payment for a suggested post
pub trait TDSuggestedPostRefundReason: Debug + RObject {}

/// Describes reason for refund of the payment for a suggested post
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum SuggestedPostRefundReason {
  #[doc(hidden)]
  _Default(()),
  /// The post was refunded, because the payment for the post was refunded
  PaymentRefunded(SuggestedPostRefundReasonPaymentRefunded),
  /// The post was refunded, because it was deleted by channel administrators in less than getOption("suggested_post_lifetime_min") seconds
  PostDeleted(SuggestedPostRefundReasonPostDeleted),
}

impl Default for SuggestedPostRefundReason {
  fn default() -> Self {
    SuggestedPostRefundReason::_Default(())
  }
}

impl<'de> Deserialize<'de> for SuggestedPostRefundReason {
  fn deserialize<D>(deserializer: D) -> Result<SuggestedPostRefundReason, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      SuggestedPostRefundReason,
      (suggestedPostRefundReasonPaymentRefunded, PaymentRefunded);
      (suggestedPostRefundReasonPostDeleted, PostDeleted);

    )(deserializer)
  }
}

impl RObject for SuggestedPostRefundReason {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      SuggestedPostRefundReason::PaymentRefunded(t) => t.td_name(),
      SuggestedPostRefundReason::PostDeleted(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      SuggestedPostRefundReason::PaymentRefunded(t) => t.extra(),
      SuggestedPostRefundReason::PostDeleted(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl SuggestedPostRefundReason {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, SuggestedPostRefundReason::_Default(_))
  }

  pub fn is_payment_refunded(&self) -> bool {
    matches!(self, SuggestedPostRefundReason::PaymentRefunded(_))
  }
  pub fn is_post_deleted(&self) -> bool {
    matches!(self, SuggestedPostRefundReason::PostDeleted(_))
  }

  pub fn on_payment_refunded<F: FnOnce(&SuggestedPostRefundReasonPaymentRefunded)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let SuggestedPostRefundReason::PaymentRefunded(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_post_deleted<F: FnOnce(&SuggestedPostRefundReasonPostDeleted)>(&self, fnc: F) -> &Self {
    if let SuggestedPostRefundReason::PostDeleted(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_payment_refunded(&self) -> Option<&SuggestedPostRefundReasonPaymentRefunded> {
    if let SuggestedPostRefundReason::PaymentRefunded(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_post_deleted(&self) -> Option<&SuggestedPostRefundReasonPostDeleted> {
    if let SuggestedPostRefundReason::PostDeleted(t) = self {
      return Some(t);
    }
    None
  }

  pub fn payment_refunded<T: AsRef<SuggestedPostRefundReasonPaymentRefunded>>(t: T) -> Self {
    SuggestedPostRefundReason::PaymentRefunded(t.as_ref().clone())
  }

  pub fn post_deleted<T: AsRef<SuggestedPostRefundReasonPostDeleted>>(t: T) -> Self {
    SuggestedPostRefundReason::PostDeleted(t.as_ref().clone())
  }
}

impl AsRef<SuggestedPostRefundReason> for SuggestedPostRefundReason {
  fn as_ref(&self) -> &SuggestedPostRefundReason {
    self
  }
}

/// The post was refunded, because the payment for the post was refunded
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedPostRefundReasonPaymentRefunded {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SuggestedPostRefundReasonPaymentRefunded {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "suggestedPostRefundReasonPaymentRefunded"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSuggestedPostRefundReason for SuggestedPostRefundReasonPaymentRefunded {}

impl SuggestedPostRefundReasonPaymentRefunded {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSuggestedPostRefundReasonPaymentRefundedBuilder {
    let inner = SuggestedPostRefundReasonPaymentRefunded {
      td_name: "suggestedPostRefundReasonPaymentRefunded".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSuggestedPostRefundReasonPaymentRefundedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSuggestedPostRefundReasonPaymentRefundedBuilder {
  inner: SuggestedPostRefundReasonPaymentRefunded,
}

impl RTDSuggestedPostRefundReasonPaymentRefundedBuilder {
  pub fn build(&self) -> SuggestedPostRefundReasonPaymentRefunded {
    self.inner.clone()
  }
}

impl AsRef<SuggestedPostRefundReasonPaymentRefunded> for SuggestedPostRefundReasonPaymentRefunded {
  fn as_ref(&self) -> &SuggestedPostRefundReasonPaymentRefunded {
    self
  }
}

impl AsRef<SuggestedPostRefundReasonPaymentRefunded>
  for RTDSuggestedPostRefundReasonPaymentRefundedBuilder
{
  fn as_ref(&self) -> &SuggestedPostRefundReasonPaymentRefunded {
    &self.inner
  }
}

/// The post was refunded, because it was deleted by channel administrators in less than getOption("suggested_post_lifetime_min") seconds
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedPostRefundReasonPostDeleted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SuggestedPostRefundReasonPostDeleted {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "suggestedPostRefundReasonPostDeleted"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSuggestedPostRefundReason for SuggestedPostRefundReasonPostDeleted {}

impl SuggestedPostRefundReasonPostDeleted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSuggestedPostRefundReasonPostDeletedBuilder {
    let inner = SuggestedPostRefundReasonPostDeleted {
      td_name: "suggestedPostRefundReasonPostDeleted".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSuggestedPostRefundReasonPostDeletedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSuggestedPostRefundReasonPostDeletedBuilder {
  inner: SuggestedPostRefundReasonPostDeleted,
}

impl RTDSuggestedPostRefundReasonPostDeletedBuilder {
  pub fn build(&self) -> SuggestedPostRefundReasonPostDeleted {
    self.inner.clone()
  }
}

impl AsRef<SuggestedPostRefundReasonPostDeleted> for SuggestedPostRefundReasonPostDeleted {
  fn as_ref(&self) -> &SuggestedPostRefundReasonPostDeleted {
    self
  }
}

impl AsRef<SuggestedPostRefundReasonPostDeleted>
  for RTDSuggestedPostRefundReasonPostDeletedBuilder
{
  fn as_ref(&self) -> &SuggestedPostRefundReasonPostDeleted {
    &self.inner
  }
}
