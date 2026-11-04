use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains state of Telegram Premium subscription and promotion videos for Premium features
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text description of the state of the current Premium subscription; may be empty if the current user has no Telegram Premium subscription
  state: FormattedText,
  /// The list of available options for buying Telegram Premium
  payment_options: Vec<PremiumStatePaymentOption>,
  /// The list of available promotion animations for Premium features
  animations: Vec<PremiumFeaturePromotionAnimation>,
  /// The list of available promotion animations for Business features
  business_animations: Vec<BusinessFeaturePromotionAnimation>,
}

impl RObject for PremiumState {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumState"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PremiumState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumStateBuilder {
    let inner = PremiumState {
      td_name: "premiumState".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPremiumStateBuilder { inner }
  }

  pub fn state(&self) -> &FormattedText {
    &self.state
  }

  pub fn payment_options(&self) -> &Vec<PremiumStatePaymentOption> {
    &self.payment_options
  }

  pub fn animations(&self) -> &Vec<PremiumFeaturePromotionAnimation> {
    &self.animations
  }

  pub fn business_animations(&self) -> &Vec<BusinessFeaturePromotionAnimation> {
    &self.business_animations
  }
}

#[doc(hidden)]
pub struct RTDPremiumStateBuilder {
  inner: PremiumState,
}

impl RTDPremiumStateBuilder {
  pub fn build(&self) -> PremiumState {
    self.inner.clone()
  }

  pub fn state<T: AsRef<FormattedText>>(&mut self, state: T) -> &mut Self {
    self.inner.state = state.as_ref().clone();
    self
  }

  pub fn payment_options(&mut self, payment_options: Vec<PremiumStatePaymentOption>) -> &mut Self {
    self.inner.payment_options = payment_options;
    self
  }

  pub fn animations(&mut self, animations: Vec<PremiumFeaturePromotionAnimation>) -> &mut Self {
    self.inner.animations = animations;
    self
  }

  pub fn business_animations(
    &mut self,
    business_animations: Vec<BusinessFeaturePromotionAnimation>,
  ) -> &mut Self {
    self.inner.business_animations = business_animations;
    self
  }
}

impl AsRef<PremiumState> for PremiumState {
  fn as_ref(&self) -> &PremiumState {
    self
  }
}

impl AsRef<PremiumState> for RTDPremiumStateBuilder {
  fn as_ref(&self) -> &PremiumState {
    &self.inner
  }
}
