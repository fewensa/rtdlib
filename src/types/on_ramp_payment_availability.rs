use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes availability of cryptocurrency purchase through an on-ramp provider
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OnRampPaymentAvailability {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the current user can use the provider
  is_allowed: bool,
  /// True, if the current user can purchase through the provider
  is_purchase_allowed: bool,
  /// The list of available payment methods
  methods: Vec<OnRampPaymentMethod>,
  /// A two-letter ISO 3166-1 alpha-2 country code of the current user's country
  country_code: String,
  /// Code of the state of the current user; may be empty if unknown
  state: String,
}

impl RObject for OnRampPaymentAvailability {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "onRampPaymentAvailability"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl OnRampPaymentAvailability {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDOnRampPaymentAvailabilityBuilder {
    let inner = OnRampPaymentAvailability {
      td_name: "onRampPaymentAvailability".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDOnRampPaymentAvailabilityBuilder { inner }
  }

  pub fn is_allowed(&self) -> bool {
    self.is_allowed
  }

  pub fn is_purchase_allowed(&self) -> bool {
    self.is_purchase_allowed
  }

  pub fn methods(&self) -> &Vec<OnRampPaymentMethod> {
    &self.methods
  }

  pub fn country_code(&self) -> &String {
    &self.country_code
  }

  pub fn state(&self) -> &String {
    &self.state
  }
}

#[doc(hidden)]
pub struct RTDOnRampPaymentAvailabilityBuilder {
  inner: OnRampPaymentAvailability,
}

impl RTDOnRampPaymentAvailabilityBuilder {
  pub fn build(&self) -> OnRampPaymentAvailability {
    self.inner.clone()
  }

  pub fn is_allowed(&mut self, is_allowed: bool) -> &mut Self {
    self.inner.is_allowed = is_allowed;
    self
  }

  pub fn is_purchase_allowed(&mut self, is_purchase_allowed: bool) -> &mut Self {
    self.inner.is_purchase_allowed = is_purchase_allowed;
    self
  }

  pub fn methods(&mut self, methods: Vec<OnRampPaymentMethod>) -> &mut Self {
    self.inner.methods = methods;
    self
  }

  pub fn country_code<T: AsRef<str>>(&mut self, country_code: T) -> &mut Self {
    self.inner.country_code = country_code.as_ref().to_string();
    self
  }

  pub fn state<T: AsRef<str>>(&mut self, state: T) -> &mut Self {
    self.inner.state = state.as_ref().to_string();
    self
  }
}

impl AsRef<OnRampPaymentAvailability> for OnRampPaymentAvailability {
  fn as_ref(&self) -> &OnRampPaymentAvailability {
    self
  }
}

impl AsRef<OnRampPaymentAvailability> for RTDOnRampPaymentAvailabilityBuilder {
  fn as_ref(&self) -> &OnRampPaymentAvailability {
    &self.inner
  }
}
