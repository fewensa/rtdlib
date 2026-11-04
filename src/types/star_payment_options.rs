use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of options for buying Telegram Stars
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarPaymentOptions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The list of options
  options: Vec<StarPaymentOption>,
}

impl RObject for StarPaymentOptions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starPaymentOptions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StarPaymentOptions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarPaymentOptionsBuilder {
    let inner = StarPaymentOptions {
      td_name: "starPaymentOptions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarPaymentOptionsBuilder { inner }
  }

  pub fn options(&self) -> &Vec<StarPaymentOption> {
    &self.options
  }
}

#[doc(hidden)]
pub struct RTDStarPaymentOptionsBuilder {
  inner: StarPaymentOptions,
}

impl RTDStarPaymentOptionsBuilder {
  pub fn build(&self) -> StarPaymentOptions {
    self.inner.clone()
  }

  pub fn options(&mut self, options: Vec<StarPaymentOption>) -> &mut Self {
    self.inner.options = options;
    self
  }
}

impl AsRef<StarPaymentOptions> for StarPaymentOptions {
  fn as_ref(&self) -> &StarPaymentOptions {
    self
  }
}

impl AsRef<StarPaymentOptions> for RTDStarPaymentOptionsBuilder {
  fn as_ref(&self) -> &StarPaymentOptions {
    &self.inner
  }
}
