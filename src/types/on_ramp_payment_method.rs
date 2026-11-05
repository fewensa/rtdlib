use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a payment method of an on-ramp provider
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OnRampPaymentMethod {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the method
  name: String,
  /// True, if the method is available to the user
  is_available: bool,
}

impl RObject for OnRampPaymentMethod {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "onRampPaymentMethod"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl OnRampPaymentMethod {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDOnRampPaymentMethodBuilder {
    let inner = OnRampPaymentMethod {
      td_name: "onRampPaymentMethod".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDOnRampPaymentMethodBuilder { inner }
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn is_available(&self) -> bool {
    self.is_available
  }
}

#[doc(hidden)]
pub struct RTDOnRampPaymentMethodBuilder {
  inner: OnRampPaymentMethod,
}

impl RTDOnRampPaymentMethodBuilder {
  pub fn build(&self) -> OnRampPaymentMethod {
    self.inner.clone()
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

  pub fn is_available(&mut self, is_available: bool) -> &mut Self {
    self.inner.is_available = is_available;
    self
  }
}

impl AsRef<OnRampPaymentMethod> for OnRampPaymentMethod {
  fn as_ref(&self) -> &OnRampPaymentMethod {
    self
  }
}

impl AsRef<OnRampPaymentMethod> for RTDOnRampPaymentMethodBuilder {
  fn as_ref(&self) -> &OnRampPaymentMethod {
    &self.inner
  }
}
