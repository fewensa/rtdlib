use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of options for gifting Telegram Premium to a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumGiftPaymentOptions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The list of options sorted by Telegram Premium subscription duration
  options: Vec<PremiumGiftPaymentOption>,
}

impl RObject for PremiumGiftPaymentOptions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumGiftPaymentOptions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PremiumGiftPaymentOptions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumGiftPaymentOptionsBuilder {
    let inner = PremiumGiftPaymentOptions {
      td_name: "premiumGiftPaymentOptions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPremiumGiftPaymentOptionsBuilder { inner }
  }

  pub fn options(&self) -> &Vec<PremiumGiftPaymentOption> {
    &self.options
  }
}

#[doc(hidden)]
pub struct RTDPremiumGiftPaymentOptionsBuilder {
  inner: PremiumGiftPaymentOptions,
}

impl RTDPremiumGiftPaymentOptionsBuilder {
  pub fn build(&self) -> PremiumGiftPaymentOptions {
    self.inner.clone()
  }

  pub fn options(&mut self, options: Vec<PremiumGiftPaymentOption>) -> &mut Self {
    self.inner.options = options;
    self
  }
}

impl AsRef<PremiumGiftPaymentOptions> for PremiumGiftPaymentOptions {
  fn as_ref(&self) -> &PremiumGiftPaymentOptions {
    self
  }
}

impl AsRef<PremiumGiftPaymentOptions> for RTDPremiumGiftPaymentOptionsBuilder {
  fn as_ref(&self) -> &PremiumGiftPaymentOptions {
    &self.inner
  }
}
