use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes exchange rate of a currency to USD
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CurrencyExchangeRate {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// ISO 4217 currency code of the payment currency
  currency: String,
  /// Exchange rate of the currency to USD
  rate: f32,
}

impl RObject for CurrencyExchangeRate {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "currencyExchangeRate"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl CurrencyExchangeRate {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCurrencyExchangeRateBuilder {
    let inner = CurrencyExchangeRate {
      td_name: "currencyExchangeRate".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCurrencyExchangeRateBuilder { inner }
  }

  pub fn currency(&self) -> &String {
    &self.currency
  }

  pub fn rate(&self) -> f32 {
    self.rate
  }
}

#[doc(hidden)]
pub struct RTDCurrencyExchangeRateBuilder {
  inner: CurrencyExchangeRate,
}

impl RTDCurrencyExchangeRateBuilder {
  pub fn build(&self) -> CurrencyExchangeRate {
    self.inner.clone()
  }

  pub fn currency<T: AsRef<str>>(&mut self, currency: T) -> &mut Self {
    self.inner.currency = currency.as_ref().to_string();
    self
  }

  pub fn rate(&mut self, rate: f32) -> &mut Self {
    self.inner.rate = rate;
    self
  }
}

impl AsRef<CurrencyExchangeRate> for CurrencyExchangeRate {
  fn as_ref(&self) -> &CurrencyExchangeRate {
    self
  }
}

impl AsRef<CurrencyExchangeRate> for RTDCurrencyExchangeRateBuilder {
  fn as_ref(&self) -> &CurrencyExchangeRate {
    &self.inner
  }
}
