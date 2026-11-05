use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes type of successful payment
pub trait TDPaymentReceiptType: Debug + RObject {}

/// Describes type of successful payment
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PaymentReceiptType {
  #[doc(hidden)]
  _Default(()),
  /// The payment was done using a third-party payment provider
  Regular(PaymentReceiptTypeRegular),
  /// The payment was done using Telegram Stars
  Stars(PaymentReceiptTypeStars),
}

impl Default for PaymentReceiptType {
  fn default() -> Self {
    PaymentReceiptType::_Default(())
  }
}

impl<'de> Deserialize<'de> for PaymentReceiptType {
  fn deserialize<D>(deserializer: D) -> Result<PaymentReceiptType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      PaymentReceiptType,
      (paymentReceiptTypeRegular, Regular);
      (paymentReceiptTypeStars, Stars);

    )(deserializer)
  }
}

impl RObject for PaymentReceiptType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      PaymentReceiptType::Regular(t) => t.td_name(),
      PaymentReceiptType::Stars(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      PaymentReceiptType::Regular(t) => t.extra(),
      PaymentReceiptType::Stars(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PaymentReceiptType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, PaymentReceiptType::_Default(_))
  }

  pub fn is_regular(&self) -> bool {
    matches!(self, PaymentReceiptType::Regular(_))
  }
  pub fn is_stars(&self) -> bool {
    matches!(self, PaymentReceiptType::Stars(_))
  }

  pub fn on_regular<F: FnOnce(&PaymentReceiptTypeRegular)>(&self, fnc: F) -> &Self {
    if let PaymentReceiptType::Regular(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_stars<F: FnOnce(&PaymentReceiptTypeStars)>(&self, fnc: F) -> &Self {
    if let PaymentReceiptType::Stars(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_regular(&self) -> Option<&PaymentReceiptTypeRegular> {
    if let PaymentReceiptType::Regular(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_stars(&self) -> Option<&PaymentReceiptTypeStars> {
    if let PaymentReceiptType::Stars(t) = self {
      return Some(t);
    }
    None
  }

  pub fn regular<T: AsRef<PaymentReceiptTypeRegular>>(t: T) -> Self {
    PaymentReceiptType::Regular(t.as_ref().clone())
  }

  pub fn stars<T: AsRef<PaymentReceiptTypeStars>>(t: T) -> Self {
    PaymentReceiptType::Stars(t.as_ref().clone())
  }
}

impl AsRef<PaymentReceiptType> for PaymentReceiptType {
  fn as_ref(&self) -> &PaymentReceiptType {
    self
  }
}

/// The payment was done using a third-party payment provider
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentReceiptTypeRegular {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier of the payment provider bot
  payment_provider_user_id: i64,
  /// Information about the invoice
  invoice: Invoice,
  /// Order information; may be null
  order_info: Option<OrderInfo>,
  /// Chosen shipping option; may be null
  shipping_option: Option<ShippingOption>,
  /// Title of the saved credentials chosen by the buyer
  credentials_title: String,
  /// The amount of tip chosen by the buyer in the smallest units of the currency
  tip_amount: i64,
}

impl RObject for PaymentReceiptTypeRegular {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "paymentReceiptTypeRegular"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPaymentReceiptType for PaymentReceiptTypeRegular {}

impl PaymentReceiptTypeRegular {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPaymentReceiptTypeRegularBuilder {
    let inner = PaymentReceiptTypeRegular {
      td_name: "paymentReceiptTypeRegular".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPaymentReceiptTypeRegularBuilder { inner }
  }

  pub fn payment_provider_user_id(&self) -> i64 {
    self.payment_provider_user_id
  }

  pub fn invoice(&self) -> &Invoice {
    &self.invoice
  }

  pub fn order_info(&self) -> &Option<OrderInfo> {
    &self.order_info
  }

  pub fn shipping_option(&self) -> &Option<ShippingOption> {
    &self.shipping_option
  }

  pub fn credentials_title(&self) -> &String {
    &self.credentials_title
  }

  pub fn tip_amount(&self) -> i64 {
    self.tip_amount
  }
}

#[doc(hidden)]
pub struct RTDPaymentReceiptTypeRegularBuilder {
  inner: PaymentReceiptTypeRegular,
}

impl RTDPaymentReceiptTypeRegularBuilder {
  pub fn build(&self) -> PaymentReceiptTypeRegular {
    self.inner.clone()
  }

  pub fn payment_provider_user_id(&mut self, payment_provider_user_id: i64) -> &mut Self {
    self.inner.payment_provider_user_id = payment_provider_user_id;
    self
  }

  pub fn invoice<T: AsRef<Invoice>>(&mut self, invoice: T) -> &mut Self {
    self.inner.invoice = invoice.as_ref().clone();
    self
  }

  pub fn order_info<T: AsRef<OrderInfo>>(&mut self, order_info: T) -> &mut Self {
    self.inner.order_info = Some(order_info.as_ref().clone());
    self
  }

  pub fn shipping_option<T: AsRef<ShippingOption>>(&mut self, shipping_option: T) -> &mut Self {
    self.inner.shipping_option = Some(shipping_option.as_ref().clone());
    self
  }

  pub fn credentials_title<T: AsRef<str>>(&mut self, credentials_title: T) -> &mut Self {
    self.inner.credentials_title = credentials_title.as_ref().to_string();
    self
  }

  pub fn tip_amount(&mut self, tip_amount: i64) -> &mut Self {
    self.inner.tip_amount = tip_amount;
    self
  }
}

impl AsRef<PaymentReceiptTypeRegular> for PaymentReceiptTypeRegular {
  fn as_ref(&self) -> &PaymentReceiptTypeRegular {
    self
  }
}

impl AsRef<PaymentReceiptTypeRegular> for RTDPaymentReceiptTypeRegularBuilder {
  fn as_ref(&self) -> &PaymentReceiptTypeRegular {
    &self.inner
  }
}

/// The payment was done using Telegram Stars
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentReceiptTypeStars {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Number of Telegram Stars that were paid
  star_count: i64,
  /// Unique identifier of the transaction that can be used to dispute it
  transaction_id: String,
}

impl RObject for PaymentReceiptTypeStars {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "paymentReceiptTypeStars"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPaymentReceiptType for PaymentReceiptTypeStars {}

impl PaymentReceiptTypeStars {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPaymentReceiptTypeStarsBuilder {
    let inner = PaymentReceiptTypeStars {
      td_name: "paymentReceiptTypeStars".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPaymentReceiptTypeStarsBuilder { inner }
  }

  pub fn star_count(&self) -> i64 {
    self.star_count
  }

  pub fn transaction_id(&self) -> &String {
    &self.transaction_id
  }
}

#[doc(hidden)]
pub struct RTDPaymentReceiptTypeStarsBuilder {
  inner: PaymentReceiptTypeStars,
}

impl RTDPaymentReceiptTypeStarsBuilder {
  pub fn build(&self) -> PaymentReceiptTypeStars {
    self.inner.clone()
  }

  pub fn star_count(&mut self, star_count: i64) -> &mut Self {
    self.inner.star_count = star_count;
    self
  }

  pub fn transaction_id<T: AsRef<str>>(&mut self, transaction_id: T) -> &mut Self {
    self.inner.transaction_id = transaction_id.as_ref().to_string();
    self
  }
}

impl AsRef<PaymentReceiptTypeStars> for PaymentReceiptTypeStars {
  fn as_ref(&self) -> &PaymentReceiptTypeStars {
    self
  }
}

impl AsRef<PaymentReceiptTypeStars> for RTDPaymentReceiptTypeStarsBuilder {
  fn as_ref(&self) -> &PaymentReceiptTypeStars {
    &self.inner
  }
}
