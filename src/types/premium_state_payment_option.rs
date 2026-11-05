use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an option for buying or upgrading Telegram Premium for self
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumStatePaymentOption {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Information about the payment option
  payment_option: PremiumPaymentOption,
  /// True, if this is the currently used Telegram Premium subscription option
  is_current: bool,
  /// True, if the payment option can be used to upgrade the existing Telegram Premium subscription
  is_upgrade: bool,
  /// Identifier of the last in-store transaction for the currently used option
  last_transaction_id: String,
}

impl RObject for PremiumStatePaymentOption {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumStatePaymentOption"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PremiumStatePaymentOption {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumStatePaymentOptionBuilder {
    let inner = PremiumStatePaymentOption {
      td_name: "premiumStatePaymentOption".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPremiumStatePaymentOptionBuilder { inner }
  }

  pub fn payment_option(&self) -> &PremiumPaymentOption {
    &self.payment_option
  }

  pub fn is_current(&self) -> bool {
    self.is_current
  }

  pub fn is_upgrade(&self) -> bool {
    self.is_upgrade
  }

  pub fn last_transaction_id(&self) -> &String {
    &self.last_transaction_id
  }
}

#[doc(hidden)]
pub struct RTDPremiumStatePaymentOptionBuilder {
  inner: PremiumStatePaymentOption,
}

impl RTDPremiumStatePaymentOptionBuilder {
  pub fn build(&self) -> PremiumStatePaymentOption {
    self.inner.clone()
  }

  pub fn payment_option<T: AsRef<PremiumPaymentOption>>(&mut self, payment_option: T) -> &mut Self {
    self.inner.payment_option = payment_option.as_ref().clone();
    self
  }

  pub fn is_current(&mut self, is_current: bool) -> &mut Self {
    self.inner.is_current = is_current;
    self
  }

  pub fn is_upgrade(&mut self, is_upgrade: bool) -> &mut Self {
    self.inner.is_upgrade = is_upgrade;
    self
  }

  pub fn last_transaction_id<T: AsRef<str>>(&mut self, last_transaction_id: T) -> &mut Self {
    self.inner.last_transaction_id = last_transaction_id.as_ref().to_string();
    self
  }
}

impl AsRef<PremiumStatePaymentOption> for PremiumStatePaymentOption {
  fn as_ref(&self) -> &PremiumStatePaymentOption {
    self
  }
}

impl AsRef<PremiumStatePaymentOption> for RTDPremiumStatePaymentOptionBuilder {
  fn as_ref(&self) -> &PremiumStatePaymentOption {
    &self.inner
  }
}
