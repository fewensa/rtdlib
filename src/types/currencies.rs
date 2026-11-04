use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of currencies
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Currencies {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A list of ISO 4217 currency codes
  currencies: Vec<String>,
}

impl RObject for Currencies {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "currencies"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl Currencies {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCurrenciesBuilder {
    let inner = Currencies {
      td_name: "currencies".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCurrenciesBuilder { inner }
  }

  pub fn currencies(&self) -> &Vec<String> {
    &self.currencies
  }
}

#[doc(hidden)]
pub struct RTDCurrenciesBuilder {
  inner: Currencies,
}

impl RTDCurrenciesBuilder {
  pub fn build(&self) -> Currencies {
    self.inner.clone()
  }

  pub fn currencies(&mut self, currencies: Vec<String>) -> &mut Self {
    self.inner.currencies = currencies;
    self
  }
}

impl AsRef<Currencies> for Currencies {
  fn as_ref(&self) -> &Currencies {
    self
  }
}

impl AsRef<Currencies> for RTDCurrenciesBuilder {
  fn as_ref(&self) -> &Currencies {
    &self.inner
  }
}
