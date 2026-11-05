use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes gift types that are accepted by a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AcceptedGiftTypes {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if unlimited regular gifts are accepted
  unlimited_gifts: bool,
  /// True, if limited regular gifts are accepted
  limited_gifts: bool,
  /// True, if upgraded gifts and regular gifts that can be upgraded for free are accepted
  upgraded_gifts: bool,
  /// True, if gifts from channels are accepted subject to other restrictions
  gifts_from_channels: bool,
  /// True, if Telegram Premium subscription is accepted
  premium_subscription: bool,
}

impl RObject for AcceptedGiftTypes {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "acceptedGiftTypes"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AcceptedGiftTypes {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAcceptedGiftTypesBuilder {
    let inner = AcceptedGiftTypes {
      td_name: "acceptedGiftTypes".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAcceptedGiftTypesBuilder { inner }
  }

  pub fn unlimited_gifts(&self) -> bool {
    self.unlimited_gifts
  }

  pub fn limited_gifts(&self) -> bool {
    self.limited_gifts
  }

  pub fn upgraded_gifts(&self) -> bool {
    self.upgraded_gifts
  }

  pub fn gifts_from_channels(&self) -> bool {
    self.gifts_from_channels
  }

  pub fn premium_subscription(&self) -> bool {
    self.premium_subscription
  }
}

#[doc(hidden)]
pub struct RTDAcceptedGiftTypesBuilder {
  inner: AcceptedGiftTypes,
}

impl RTDAcceptedGiftTypesBuilder {
  pub fn build(&self) -> AcceptedGiftTypes {
    self.inner.clone()
  }

  pub fn unlimited_gifts(&mut self, unlimited_gifts: bool) -> &mut Self {
    self.inner.unlimited_gifts = unlimited_gifts;
    self
  }

  pub fn limited_gifts(&mut self, limited_gifts: bool) -> &mut Self {
    self.inner.limited_gifts = limited_gifts;
    self
  }

  pub fn upgraded_gifts(&mut self, upgraded_gifts: bool) -> &mut Self {
    self.inner.upgraded_gifts = upgraded_gifts;
    self
  }

  pub fn gifts_from_channels(&mut self, gifts_from_channels: bool) -> &mut Self {
    self.inner.gifts_from_channels = gifts_from_channels;
    self
  }

  pub fn premium_subscription(&mut self, premium_subscription: bool) -> &mut Self {
    self.inner.premium_subscription = premium_subscription;
    self
  }
}

impl AsRef<AcceptedGiftTypes> for AcceptedGiftTypes {
  fn as_ref(&self) -> &AcceptedGiftTypes {
    self
  }
}

impl AsRef<AcceptedGiftTypes> for RTDAcceptedGiftTypesBuilder {
  fn as_ref(&self) -> &AcceptedGiftTypes {
    &self.inner
  }
}
