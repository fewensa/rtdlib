use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about subscription to a channel chat, a bot, or a business account that was paid in Telegram Stars
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarSubscription {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the subscription
  id: String,
  /// Identifier of the chat that is subscribed
  chat_id: i64,
  /// Point in time (Unix timestamp) when the subscription will expire or expired
  expiration_date: i64,
  /// True, if the subscription was canceled
  is_canceled: bool,
  /// True, if the subscription expires soon and there aren't enough Telegram Stars on the user's balance to extend it
  is_expiring: bool,
  /// The subscription plan
  pricing: StarSubscriptionPricing,
  /// Type of the subscription
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: StarSubscriptionType,
}

impl RObject for StarSubscription {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starSubscription"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StarSubscription {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarSubscriptionBuilder {
    let inner = StarSubscription {
      td_name: "starSubscription".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarSubscriptionBuilder { inner }
  }

  pub fn id(&self) -> &String {
    &self.id
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn expiration_date(&self) -> i64 {
    self.expiration_date
  }

  pub fn is_canceled(&self) -> bool {
    self.is_canceled
  }

  pub fn is_expiring(&self) -> bool {
    self.is_expiring
  }

  pub fn pricing(&self) -> &StarSubscriptionPricing {
    &self.pricing
  }

  pub fn type_(&self) -> &StarSubscriptionType {
    &self.type_
  }
}

#[doc(hidden)]
pub struct RTDStarSubscriptionBuilder {
  inner: StarSubscription,
}

impl RTDStarSubscriptionBuilder {
  pub fn build(&self) -> StarSubscription {
    self.inner.clone()
  }

  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn expiration_date(&mut self, expiration_date: i64) -> &mut Self {
    self.inner.expiration_date = expiration_date;
    self
  }

  pub fn is_canceled(&mut self, is_canceled: bool) -> &mut Self {
    self.inner.is_canceled = is_canceled;
    self
  }

  pub fn is_expiring(&mut self, is_expiring: bool) -> &mut Self {
    self.inner.is_expiring = is_expiring;
    self
  }

  pub fn pricing<T: AsRef<StarSubscriptionPricing>>(&mut self, pricing: T) -> &mut Self {
    self.inner.pricing = pricing.as_ref().clone();
    self
  }

  pub fn type_<T: AsRef<StarSubscriptionType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }
}

impl AsRef<StarSubscription> for StarSubscription {
  fn as_ref(&self) -> &StarSubscription {
    self
  }
}

impl AsRef<StarSubscription> for RTDStarSubscriptionBuilder {
  fn as_ref(&self) -> &StarSubscription {
    &self.inner
  }
}
