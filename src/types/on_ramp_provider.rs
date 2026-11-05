use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an on-ramp provider
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OnRampProvider {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the provider
  id: String,
  /// Name of the provider
  name: String,
  /// List of cryptocurrencies supported by the provider
  cryptocurrencies: Vec<String>,
  /// True, if the provider supports base currencies and can be used in the method getOnRampProviderBaseCurrencies
  supports_base_currencies: bool,
  /// True, if the provider supports limits retrieval using the method getOnRampPaymentLimits
  supports_limits: bool,
  /// True, if the provider supports quote retrieval using the method getOnRampPaymentQuote
  supports_quote: bool,
}

impl RObject for OnRampProvider {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "onRampProvider"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl OnRampProvider {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDOnRampProviderBuilder {
    let inner = OnRampProvider {
      td_name: "onRampProvider".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDOnRampProviderBuilder { inner }
  }

  pub fn id(&self) -> &String {
    &self.id
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn cryptocurrencies(&self) -> &Vec<String> {
    &self.cryptocurrencies
  }

  pub fn supports_base_currencies(&self) -> bool {
    self.supports_base_currencies
  }

  pub fn supports_limits(&self) -> bool {
    self.supports_limits
  }

  pub fn supports_quote(&self) -> bool {
    self.supports_quote
  }
}

#[doc(hidden)]
pub struct RTDOnRampProviderBuilder {
  inner: OnRampProvider,
}

impl RTDOnRampProviderBuilder {
  pub fn build(&self) -> OnRampProvider {
    self.inner.clone()
  }

  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

  pub fn cryptocurrencies(&mut self, cryptocurrencies: Vec<String>) -> &mut Self {
    self.inner.cryptocurrencies = cryptocurrencies;
    self
  }

  pub fn supports_base_currencies(&mut self, supports_base_currencies: bool) -> &mut Self {
    self.inner.supports_base_currencies = supports_base_currencies;
    self
  }

  pub fn supports_limits(&mut self, supports_limits: bool) -> &mut Self {
    self.inner.supports_limits = supports_limits;
    self
  }

  pub fn supports_quote(&mut self, supports_quote: bool) -> &mut Self {
    self.inner.supports_quote = supports_quote;
    self
  }
}

impl AsRef<OnRampProvider> for OnRampProvider {
  fn as_ref(&self) -> &OnRampProvider {
    self
  }
}

impl AsRef<OnRampProvider> for RTDOnRampProviderBuilder {
  fn as_ref(&self) -> &OnRampProvider {
    &self.inner
  }
}
