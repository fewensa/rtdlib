use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of options for creating of Telegram Premium giveaway or manual distribution of Telegram Premium among chat members
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumGiveawayPaymentOptions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The list of options
  options: Vec<PremiumGiveawayPaymentOption>,
}

impl RObject for PremiumGiveawayPaymentOptions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumGiveawayPaymentOptions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PremiumGiveawayPaymentOptions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumGiveawayPaymentOptionsBuilder {
    let inner = PremiumGiveawayPaymentOptions {
      td_name: "premiumGiveawayPaymentOptions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPremiumGiveawayPaymentOptionsBuilder { inner }
  }

  pub fn options(&self) -> &Vec<PremiumGiveawayPaymentOption> {
    &self.options
  }
}

#[doc(hidden)]
pub struct RTDPremiumGiveawayPaymentOptionsBuilder {
  inner: PremiumGiveawayPaymentOptions,
}

impl RTDPremiumGiveawayPaymentOptionsBuilder {
  pub fn build(&self) -> PremiumGiveawayPaymentOptions {
    self.inner.clone()
  }

  pub fn options(&mut self, options: Vec<PremiumGiveawayPaymentOption>) -> &mut Self {
    self.inner.options = options;
    self
  }
}

impl AsRef<PremiumGiveawayPaymentOptions> for PremiumGiveawayPaymentOptions {
  fn as_ref(&self) -> &PremiumGiveawayPaymentOptions {
    self
  }
}

impl AsRef<PremiumGiveawayPaymentOptions> for RTDPremiumGiveawayPaymentOptionsBuilder {
  fn as_ref(&self) -> &PremiumGiveawayPaymentOptions {
    &self.inner
  }
}
