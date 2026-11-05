use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of currency exchange rates
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CurrencyExchangeRates {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A list of currency exchange rates
  rates: Vec<CurrencyExchangeRate>,
}

impl RObject for CurrencyExchangeRates {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "currencyExchangeRates"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl CurrencyExchangeRates {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCurrencyExchangeRatesBuilder {
    let inner = CurrencyExchangeRates {
      td_name: "currencyExchangeRates".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCurrencyExchangeRatesBuilder { inner }
  }

  pub fn rates(&self) -> &Vec<CurrencyExchangeRate> {
    &self.rates
  }
}

#[doc(hidden)]
pub struct RTDCurrencyExchangeRatesBuilder {
  inner: CurrencyExchangeRates,
}

impl RTDCurrencyExchangeRatesBuilder {
  pub fn build(&self) -> CurrencyExchangeRates {
    self.inner.clone()
  }

  pub fn rates(&mut self, rates: Vec<CurrencyExchangeRate>) -> &mut Self {
    self.inner.rates = rates;
    self
  }
}

impl AsRef<CurrencyExchangeRates> for CurrencyExchangeRates {
  fn as_ref(&self) -> &CurrencyExchangeRates {
    self
  }
}

impl AsRef<CurrencyExchangeRates> for RTDCurrencyExchangeRatesBuilder {
  fn as_ref(&self) -> &CurrencyExchangeRates {
    &self.inner
  }
}
