use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a user who added paid reactions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaidReactor {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user or chat that added the reactions; may be null for anonymous reactors that aren't the current user
  sender_id: Option<MessageSender>,
  /// Number of Telegram Stars added
  star_count: i64,
  /// True, if the reactor is one of the most active reactors; may be false if the reactor is the current user
  is_top: bool,
  /// True, if the paid reaction was added by the current user
  is_me: bool,
  /// True, if the reactor is anonymous
  is_anonymous: bool,
}

impl RObject for PaidReactor {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "paidReactor"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PaidReactor {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPaidReactorBuilder {
    let inner = PaidReactor {
      td_name: "paidReactor".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPaidReactorBuilder { inner }
  }

  pub fn sender_id(&self) -> &Option<MessageSender> {
    &self.sender_id
  }

  pub fn star_count(&self) -> i64 {
    self.star_count
  }

  pub fn is_top(&self) -> bool {
    self.is_top
  }

  pub fn is_me(&self) -> bool {
    self.is_me
  }

  pub fn is_anonymous(&self) -> bool {
    self.is_anonymous
  }
}

#[doc(hidden)]
pub struct RTDPaidReactorBuilder {
  inner: PaidReactor,
}

impl RTDPaidReactorBuilder {
  pub fn build(&self) -> PaidReactor {
    self.inner.clone()
  }

  pub fn sender_id<T: AsRef<MessageSender>>(&mut self, sender_id: T) -> &mut Self {
    self.inner.sender_id = Some(sender_id.as_ref().clone());
    self
  }

  pub fn star_count(&mut self, star_count: i64) -> &mut Self {
    self.inner.star_count = star_count;
    self
  }

  pub fn is_top(&mut self, is_top: bool) -> &mut Self {
    self.inner.is_top = is_top;
    self
  }

  pub fn is_me(&mut self, is_me: bool) -> &mut Self {
    self.inner.is_me = is_me;
    self
  }

  pub fn is_anonymous(&mut self, is_anonymous: bool) -> &mut Self {
    self.inner.is_anonymous = is_anonymous;
    self
  }
}

impl AsRef<PaidReactor> for PaidReactor {
  fn as_ref(&self) -> &PaidReactor {
    self
  }
}

impl AsRef<PaidReactor> for RTDPaidReactorBuilder {
  fn as_ref(&self) -> &PaidReactor {
    &self.inner
  }
}
