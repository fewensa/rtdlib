use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of Telegram Star subscriptions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarSubscriptions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The amount of owned Telegram Stars
  star_amount: StarAmount,
  /// List of subscriptions for Telegram Stars
  subscriptions: Vec<StarSubscription>,
  /// The number of Telegram Stars required to buy to extend subscriptions expiring soon
  required_star_count: i64,
  /// The offset for the next request. If empty, then there are no more results
  next_offset: String,
}

impl RObject for StarSubscriptions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starSubscriptions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StarSubscriptions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarSubscriptionsBuilder {
    let inner = StarSubscriptions {
      td_name: "starSubscriptions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarSubscriptionsBuilder { inner }
  }

  pub fn star_amount(&self) -> &StarAmount {
    &self.star_amount
  }

  pub fn subscriptions(&self) -> &Vec<StarSubscription> {
    &self.subscriptions
  }

  pub fn required_star_count(&self) -> i64 {
    self.required_star_count
  }

  pub fn next_offset(&self) -> &String {
    &self.next_offset
  }
}

#[doc(hidden)]
pub struct RTDStarSubscriptionsBuilder {
  inner: StarSubscriptions,
}

impl RTDStarSubscriptionsBuilder {
  pub fn build(&self) -> StarSubscriptions {
    self.inner.clone()
  }

  pub fn star_amount<T: AsRef<StarAmount>>(&mut self, star_amount: T) -> &mut Self {
    self.inner.star_amount = star_amount.as_ref().clone();
    self
  }

  pub fn subscriptions(&mut self, subscriptions: Vec<StarSubscription>) -> &mut Self {
    self.inner.subscriptions = subscriptions;
    self
  }

  pub fn required_star_count(&mut self, required_star_count: i64) -> &mut Self {
    self.inner.required_star_count = required_star_count;
    self
  }

  pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
    self.inner.next_offset = next_offset.as_ref().to_string();
    self
  }
}

impl AsRef<StarSubscriptions> for StarSubscriptions {
  fn as_ref(&self) -> &StarSubscriptions {
    self
  }
}

impl AsRef<StarSubscriptions> for RTDStarSubscriptionsBuilder {
  fn as_ref(&self) -> &StarSubscriptions {
    &self.inner
  }
}
