use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes type of payment form
pub trait TDPaymentFormType: Debug + RObject {}

/// Describes type of payment form
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PaymentFormType {
  #[doc(hidden)]
  _Default(()),
  /// The payment form is for a regular payment
  Regular(PaymentFormTypeRegular),
  /// The payment form is for a payment in Telegram Stars for subscription
  StarSubscription(PaymentFormTypeStarSubscription),
  /// The payment form is for a payment in Telegram Stars
  Stars(PaymentFormTypeStars),
}

impl Default for PaymentFormType {
  fn default() -> Self {
    PaymentFormType::_Default(())
  }
}

impl<'de> Deserialize<'de> for PaymentFormType {
  fn deserialize<D>(deserializer: D) -> Result<PaymentFormType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      PaymentFormType,
      (paymentFormTypeRegular, Regular);
      (paymentFormTypeStarSubscription, StarSubscription);
      (paymentFormTypeStars, Stars);

    )(deserializer)
  }
}

impl RObject for PaymentFormType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      PaymentFormType::Regular(t) => t.td_name(),
      PaymentFormType::StarSubscription(t) => t.td_name(),
      PaymentFormType::Stars(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      PaymentFormType::Regular(t) => t.extra(),
      PaymentFormType::StarSubscription(t) => t.extra(),
      PaymentFormType::Stars(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PaymentFormType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, PaymentFormType::_Default(_))
  }

  pub fn is_regular(&self) -> bool {
    matches!(self, PaymentFormType::Regular(_))
  }
  pub fn is_star_subscription(&self) -> bool {
    matches!(self, PaymentFormType::StarSubscription(_))
  }
  pub fn is_stars(&self) -> bool {
    matches!(self, PaymentFormType::Stars(_))
  }

  pub fn on_regular<F: FnOnce(&PaymentFormTypeRegular)>(&self, fnc: F) -> &Self {
    if let PaymentFormType::Regular(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_star_subscription<F: FnOnce(&PaymentFormTypeStarSubscription)>(&self, fnc: F) -> &Self {
    if let PaymentFormType::StarSubscription(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_stars<F: FnOnce(&PaymentFormTypeStars)>(&self, fnc: F) -> &Self {
    if let PaymentFormType::Stars(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_regular(&self) -> Option<&PaymentFormTypeRegular> {
    if let PaymentFormType::Regular(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_star_subscription(&self) -> Option<&PaymentFormTypeStarSubscription> {
    if let PaymentFormType::StarSubscription(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_stars(&self) -> Option<&PaymentFormTypeStars> {
    if let PaymentFormType::Stars(t) = self {
      return Some(t);
    }
    None
  }

  pub fn regular<T: AsRef<PaymentFormTypeRegular>>(t: T) -> Self {
    PaymentFormType::Regular(t.as_ref().clone())
  }

  pub fn star_subscription<T: AsRef<PaymentFormTypeStarSubscription>>(t: T) -> Self {
    PaymentFormType::StarSubscription(t.as_ref().clone())
  }

  pub fn stars<T: AsRef<PaymentFormTypeStars>>(t: T) -> Self {
    PaymentFormType::Stars(t.as_ref().clone())
  }
}

impl AsRef<PaymentFormType> for PaymentFormType {
  fn as_ref(&self) -> &PaymentFormType {
    self
  }
}

/// The payment form is for a regular payment
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentFormTypeRegular {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Full information about the invoice
  invoice: Invoice,
  /// User identifier of the payment provider bot
  payment_provider_user_id: i64,
  /// Information about the payment provider
  payment_provider: PaymentProvider,
  /// The list of additional payment options
  additional_payment_options: Vec<PaymentOption>,
  /// Saved server-side order information; may be null
  saved_order_info: Option<OrderInfo>,
  /// The list of saved payment credentials
  saved_credentials: Vec<SavedCredentials>,
  /// True, if the user can choose to save credentials
  can_save_credentials: bool,
  /// True, if the user will be able to save credentials, if sets up a 2-step verification password
  need_password: bool,
}

impl RObject for PaymentFormTypeRegular {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "paymentFormTypeRegular"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPaymentFormType for PaymentFormTypeRegular {}

impl PaymentFormTypeRegular {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPaymentFormTypeRegularBuilder {
    let inner = PaymentFormTypeRegular {
      td_name: "paymentFormTypeRegular".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPaymentFormTypeRegularBuilder { inner }
  }

  pub fn invoice(&self) -> &Invoice {
    &self.invoice
  }

  pub fn payment_provider_user_id(&self) -> i64 {
    self.payment_provider_user_id
  }

  pub fn payment_provider(&self) -> &PaymentProvider {
    &self.payment_provider
  }

  pub fn additional_payment_options(&self) -> &Vec<PaymentOption> {
    &self.additional_payment_options
  }

  pub fn saved_order_info(&self) -> &Option<OrderInfo> {
    &self.saved_order_info
  }

  pub fn saved_credentials(&self) -> &Vec<SavedCredentials> {
    &self.saved_credentials
  }

  pub fn can_save_credentials(&self) -> bool {
    self.can_save_credentials
  }

  pub fn need_password(&self) -> bool {
    self.need_password
  }
}

#[doc(hidden)]
pub struct RTDPaymentFormTypeRegularBuilder {
  inner: PaymentFormTypeRegular,
}

impl RTDPaymentFormTypeRegularBuilder {
  pub fn build(&self) -> PaymentFormTypeRegular {
    self.inner.clone()
  }

  pub fn invoice<T: AsRef<Invoice>>(&mut self, invoice: T) -> &mut Self {
    self.inner.invoice = invoice.as_ref().clone();
    self
  }

  pub fn payment_provider_user_id(&mut self, payment_provider_user_id: i64) -> &mut Self {
    self.inner.payment_provider_user_id = payment_provider_user_id;
    self
  }

  pub fn payment_provider<T: AsRef<PaymentProvider>>(&mut self, payment_provider: T) -> &mut Self {
    self.inner.payment_provider = payment_provider.as_ref().clone();
    self
  }

  pub fn additional_payment_options(
    &mut self,
    additional_payment_options: Vec<PaymentOption>,
  ) -> &mut Self {
    self.inner.additional_payment_options = additional_payment_options;
    self
  }

  pub fn saved_order_info<T: AsRef<OrderInfo>>(&mut self, saved_order_info: T) -> &mut Self {
    self.inner.saved_order_info = Some(saved_order_info.as_ref().clone());
    self
  }

  pub fn saved_credentials(&mut self, saved_credentials: Vec<SavedCredentials>) -> &mut Self {
    self.inner.saved_credentials = saved_credentials;
    self
  }

  pub fn can_save_credentials(&mut self, can_save_credentials: bool) -> &mut Self {
    self.inner.can_save_credentials = can_save_credentials;
    self
  }

  pub fn need_password(&mut self, need_password: bool) -> &mut Self {
    self.inner.need_password = need_password;
    self
  }
}

impl AsRef<PaymentFormTypeRegular> for PaymentFormTypeRegular {
  fn as_ref(&self) -> &PaymentFormTypeRegular {
    self
  }
}

impl AsRef<PaymentFormTypeRegular> for RTDPaymentFormTypeRegularBuilder {
  fn as_ref(&self) -> &PaymentFormTypeRegular {
    &self.inner
  }
}

/// The payment form is for a payment in Telegram Stars for subscription
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentFormTypeStarSubscription {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Information about subscription plan
  pricing: StarSubscriptionPricing,
}

impl RObject for PaymentFormTypeStarSubscription {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "paymentFormTypeStarSubscription"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPaymentFormType for PaymentFormTypeStarSubscription {}

impl PaymentFormTypeStarSubscription {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPaymentFormTypeStarSubscriptionBuilder {
    let inner = PaymentFormTypeStarSubscription {
      td_name: "paymentFormTypeStarSubscription".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPaymentFormTypeStarSubscriptionBuilder { inner }
  }

  pub fn pricing(&self) -> &StarSubscriptionPricing {
    &self.pricing
  }
}

#[doc(hidden)]
pub struct RTDPaymentFormTypeStarSubscriptionBuilder {
  inner: PaymentFormTypeStarSubscription,
}

impl RTDPaymentFormTypeStarSubscriptionBuilder {
  pub fn build(&self) -> PaymentFormTypeStarSubscription {
    self.inner.clone()
  }

  pub fn pricing<T: AsRef<StarSubscriptionPricing>>(&mut self, pricing: T) -> &mut Self {
    self.inner.pricing = pricing.as_ref().clone();
    self
  }
}

impl AsRef<PaymentFormTypeStarSubscription> for PaymentFormTypeStarSubscription {
  fn as_ref(&self) -> &PaymentFormTypeStarSubscription {
    self
  }
}

impl AsRef<PaymentFormTypeStarSubscription> for RTDPaymentFormTypeStarSubscriptionBuilder {
  fn as_ref(&self) -> &PaymentFormTypeStarSubscription {
    &self.inner
  }
}

/// The payment form is for a payment in Telegram Stars
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentFormTypeStars {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Number of Telegram Stars that will be paid
  star_count: i64,
}

impl RObject for PaymentFormTypeStars {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "paymentFormTypeStars"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPaymentFormType for PaymentFormTypeStars {}

impl PaymentFormTypeStars {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPaymentFormTypeStarsBuilder {
    let inner = PaymentFormTypeStars {
      td_name: "paymentFormTypeStars".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPaymentFormTypeStarsBuilder { inner }
  }

  pub fn star_count(&self) -> i64 {
    self.star_count
  }
}

#[doc(hidden)]
pub struct RTDPaymentFormTypeStarsBuilder {
  inner: PaymentFormTypeStars,
}

impl RTDPaymentFormTypeStarsBuilder {
  pub fn build(&self) -> PaymentFormTypeStars {
    self.inner.clone()
  }

  pub fn star_count(&mut self, star_count: i64) -> &mut Self {
    self.inner.star_count = star_count;
    self
  }
}

impl AsRef<PaymentFormTypeStars> for PaymentFormTypeStars {
  fn as_ref(&self) -> &PaymentFormTypeStars {
    self
  }
}

impl AsRef<PaymentFormTypeStars> for RTDPaymentFormTypeStarsBuilder {
  fn as_ref(&self) -> &PaymentFormTypeStars {
    &self.inner
  }
}
