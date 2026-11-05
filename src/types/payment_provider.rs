use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains information about a payment provider
pub trait TDPaymentProvider: Debug + RObject {}

/// Contains information about a payment provider
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PaymentProvider {
  #[doc(hidden)]
  _Default(()),
  /// Some other payment provider, for which a web payment form must be shown
  Other(PaymentProviderOther),
  /// Smart Glocal payment provider
  SmartGlocal(PaymentProviderSmartGlocal),
  /// Stripe payment provider
  Stripe(PaymentProviderStripe),
}

impl Default for PaymentProvider {
  fn default() -> Self {
    PaymentProvider::_Default(())
  }
}

impl<'de> Deserialize<'de> for PaymentProvider {
  fn deserialize<D>(deserializer: D) -> Result<PaymentProvider, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      PaymentProvider,
      (paymentProviderOther, Other);
      (paymentProviderSmartGlocal, SmartGlocal);
      (paymentProviderStripe, Stripe);

    )(deserializer)
  }
}

impl RObject for PaymentProvider {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      PaymentProvider::Other(t) => t.td_name(),
      PaymentProvider::SmartGlocal(t) => t.td_name(),
      PaymentProvider::Stripe(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      PaymentProvider::Other(t) => t.extra(),
      PaymentProvider::SmartGlocal(t) => t.extra(),
      PaymentProvider::Stripe(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PaymentProvider {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, PaymentProvider::_Default(_))
  }

  pub fn is_other(&self) -> bool {
    matches!(self, PaymentProvider::Other(_))
  }
  pub fn is_smart_glocal(&self) -> bool {
    matches!(self, PaymentProvider::SmartGlocal(_))
  }
  pub fn is_stripe(&self) -> bool {
    matches!(self, PaymentProvider::Stripe(_))
  }

  pub fn on_other<F: FnOnce(&PaymentProviderOther)>(&self, fnc: F) -> &Self {
    if let PaymentProvider::Other(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_smart_glocal<F: FnOnce(&PaymentProviderSmartGlocal)>(&self, fnc: F) -> &Self {
    if let PaymentProvider::SmartGlocal(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_stripe<F: FnOnce(&PaymentProviderStripe)>(&self, fnc: F) -> &Self {
    if let PaymentProvider::Stripe(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_other(&self) -> Option<&PaymentProviderOther> {
    if let PaymentProvider::Other(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_smart_glocal(&self) -> Option<&PaymentProviderSmartGlocal> {
    if let PaymentProvider::SmartGlocal(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_stripe(&self) -> Option<&PaymentProviderStripe> {
    if let PaymentProvider::Stripe(t) = self {
      return Some(t);
    }
    None
  }

  pub fn other<T: AsRef<PaymentProviderOther>>(t: T) -> Self {
    PaymentProvider::Other(t.as_ref().clone())
  }

  pub fn smart_glocal<T: AsRef<PaymentProviderSmartGlocal>>(t: T) -> Self {
    PaymentProvider::SmartGlocal(t.as_ref().clone())
  }

  pub fn stripe<T: AsRef<PaymentProviderStripe>>(t: T) -> Self {
    PaymentProvider::Stripe(t.as_ref().clone())
  }
}

impl AsRef<PaymentProvider> for PaymentProvider {
  fn as_ref(&self) -> &PaymentProvider {
    self
  }
}

/// Some other payment provider, for which a web payment form must be shown
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentProviderOther {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Payment form URL
  url: String,
}

impl RObject for PaymentProviderOther {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "paymentProviderOther"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPaymentProvider for PaymentProviderOther {}

impl PaymentProviderOther {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPaymentProviderOtherBuilder {
    let inner = PaymentProviderOther {
      td_name: "paymentProviderOther".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPaymentProviderOtherBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }
}

#[doc(hidden)]
pub struct RTDPaymentProviderOtherBuilder {
  inner: PaymentProviderOther,
}

impl RTDPaymentProviderOtherBuilder {
  pub fn build(&self) -> PaymentProviderOther {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }
}

impl AsRef<PaymentProviderOther> for PaymentProviderOther {
  fn as_ref(&self) -> &PaymentProviderOther {
    self
  }
}

impl AsRef<PaymentProviderOther> for RTDPaymentProviderOtherBuilder {
  fn as_ref(&self) -> &PaymentProviderOther {
    &self.inner
  }
}

/// Smart Glocal payment provider
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentProviderSmartGlocal {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Public payment token
  public_token: String,
  /// URL for sending card tokenization requests
  tokenize_url: String,
}

impl RObject for PaymentProviderSmartGlocal {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "paymentProviderSmartGlocal"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPaymentProvider for PaymentProviderSmartGlocal {}

impl PaymentProviderSmartGlocal {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPaymentProviderSmartGlocalBuilder {
    let inner = PaymentProviderSmartGlocal {
      td_name: "paymentProviderSmartGlocal".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPaymentProviderSmartGlocalBuilder { inner }
  }

  pub fn public_token(&self) -> &String {
    &self.public_token
  }

  pub fn tokenize_url(&self) -> &String {
    &self.tokenize_url
  }
}

#[doc(hidden)]
pub struct RTDPaymentProviderSmartGlocalBuilder {
  inner: PaymentProviderSmartGlocal,
}

impl RTDPaymentProviderSmartGlocalBuilder {
  pub fn build(&self) -> PaymentProviderSmartGlocal {
    self.inner.clone()
  }

  pub fn public_token<T: AsRef<str>>(&mut self, public_token: T) -> &mut Self {
    self.inner.public_token = public_token.as_ref().to_string();
    self
  }

  pub fn tokenize_url<T: AsRef<str>>(&mut self, tokenize_url: T) -> &mut Self {
    self.inner.tokenize_url = tokenize_url.as_ref().to_string();
    self
  }
}

impl AsRef<PaymentProviderSmartGlocal> for PaymentProviderSmartGlocal {
  fn as_ref(&self) -> &PaymentProviderSmartGlocal {
    self
  }
}

impl AsRef<PaymentProviderSmartGlocal> for RTDPaymentProviderSmartGlocalBuilder {
  fn as_ref(&self) -> &PaymentProviderSmartGlocal {
    &self.inner
  }
}

/// Stripe payment provider
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentProviderStripe {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Stripe API publishable key
  publishable_key: String,
  /// True, if the user country must be provided
  need_country: bool,
  /// True, if the user ZIP/postal code must be provided
  need_postal_code: bool,
  /// True, if the cardholder name must be provided
  need_cardholder_name: bool,
}

impl RObject for PaymentProviderStripe {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "paymentProviderStripe"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPaymentProvider for PaymentProviderStripe {}

impl PaymentProviderStripe {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPaymentProviderStripeBuilder {
    let inner = PaymentProviderStripe {
      td_name: "paymentProviderStripe".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPaymentProviderStripeBuilder { inner }
  }

  pub fn publishable_key(&self) -> &String {
    &self.publishable_key
  }

  pub fn need_country(&self) -> bool {
    self.need_country
  }

  pub fn need_postal_code(&self) -> bool {
    self.need_postal_code
  }

  pub fn need_cardholder_name(&self) -> bool {
    self.need_cardholder_name
  }
}

#[doc(hidden)]
pub struct RTDPaymentProviderStripeBuilder {
  inner: PaymentProviderStripe,
}

impl RTDPaymentProviderStripeBuilder {
  pub fn build(&self) -> PaymentProviderStripe {
    self.inner.clone()
  }

  pub fn publishable_key<T: AsRef<str>>(&mut self, publishable_key: T) -> &mut Self {
    self.inner.publishable_key = publishable_key.as_ref().to_string();
    self
  }

  pub fn need_country(&mut self, need_country: bool) -> &mut Self {
    self.inner.need_country = need_country;
    self
  }

  pub fn need_postal_code(&mut self, need_postal_code: bool) -> &mut Self {
    self.inner.need_postal_code = need_postal_code;
    self
  }

  pub fn need_cardholder_name(&mut self, need_cardholder_name: bool) -> &mut Self {
    self.inner.need_cardholder_name = need_cardholder_name;
    self
  }
}

impl AsRef<PaymentProviderStripe> for PaymentProviderStripe {
  fn as_ref(&self) -> &PaymentProviderStripe {
    self
  }
}

impl AsRef<PaymentProviderStripe> for RTDPaymentProviderStripeBuilder {
  fn as_ref(&self) -> &PaymentProviderStripe {
    &self.inner
  }
}
