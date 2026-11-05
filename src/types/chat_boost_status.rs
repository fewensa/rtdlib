use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes current boost status of a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatBoostStatus {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// An HTTP URL, which can be used to boost the chat
  boost_url: String,
  /// Identifiers of boost slots of the current user applied to the chat
  applied_slot_ids: Vec<i64>,
  /// Current boost level of the chat
  level: i64,
  /// The number of boosts received by the chat from created Telegram Premium gift codes and giveaways; always 0 if the current user isn't an administrator in the chat
  gift_code_boost_count: i64,
  /// The number of boosts received by the chat
  boost_count: i64,
  /// The number of boosts added to reach the current level
  current_level_boost_count: i64,
  /// The number of boosts needed to reach the next level; 0 if the next level isn't available
  next_level_boost_count: i64,
  /// Approximate number of Telegram Premium subscribers joined the chat; always 0 if the current user isn't an administrator in the chat
  premium_member_count: i64,
  /// A percentage of Telegram Premium subscribers joined the chat; always 0 if the current user isn't an administrator in the chat
  premium_member_percentage: f32,
  /// The list of prepaid giveaways available for the chat; only for chat administrators
  prepaid_giveaways: Vec<PrepaidGiveaway>,
}

impl RObject for ChatBoostStatus {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatBoostStatus"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatBoostStatus {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatBoostStatusBuilder {
    let inner = ChatBoostStatus {
      td_name: "chatBoostStatus".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatBoostStatusBuilder { inner }
  }

  pub fn boost_url(&self) -> &String {
    &self.boost_url
  }

  pub fn applied_slot_ids(&self) -> &Vec<i64> {
    &self.applied_slot_ids
  }

  pub fn level(&self) -> i64 {
    self.level
  }

  pub fn gift_code_boost_count(&self) -> i64 {
    self.gift_code_boost_count
  }

  pub fn boost_count(&self) -> i64 {
    self.boost_count
  }

  pub fn current_level_boost_count(&self) -> i64 {
    self.current_level_boost_count
  }

  pub fn next_level_boost_count(&self) -> i64 {
    self.next_level_boost_count
  }

  pub fn premium_member_count(&self) -> i64 {
    self.premium_member_count
  }

  pub fn premium_member_percentage(&self) -> f32 {
    self.premium_member_percentage
  }

  pub fn prepaid_giveaways(&self) -> &Vec<PrepaidGiveaway> {
    &self.prepaid_giveaways
  }
}

#[doc(hidden)]
pub struct RTDChatBoostStatusBuilder {
  inner: ChatBoostStatus,
}

impl RTDChatBoostStatusBuilder {
  pub fn build(&self) -> ChatBoostStatus {
    self.inner.clone()
  }

  pub fn boost_url<T: AsRef<str>>(&mut self, boost_url: T) -> &mut Self {
    self.inner.boost_url = boost_url.as_ref().to_string();
    self
  }

  pub fn applied_slot_ids(&mut self, applied_slot_ids: Vec<i64>) -> &mut Self {
    self.inner.applied_slot_ids = applied_slot_ids;
    self
  }

  pub fn level(&mut self, level: i64) -> &mut Self {
    self.inner.level = level;
    self
  }

  pub fn gift_code_boost_count(&mut self, gift_code_boost_count: i64) -> &mut Self {
    self.inner.gift_code_boost_count = gift_code_boost_count;
    self
  }

  pub fn boost_count(&mut self, boost_count: i64) -> &mut Self {
    self.inner.boost_count = boost_count;
    self
  }

  pub fn current_level_boost_count(&mut self, current_level_boost_count: i64) -> &mut Self {
    self.inner.current_level_boost_count = current_level_boost_count;
    self
  }

  pub fn next_level_boost_count(&mut self, next_level_boost_count: i64) -> &mut Self {
    self.inner.next_level_boost_count = next_level_boost_count;
    self
  }

  pub fn premium_member_count(&mut self, premium_member_count: i64) -> &mut Self {
    self.inner.premium_member_count = premium_member_count;
    self
  }

  pub fn premium_member_percentage(&mut self, premium_member_percentage: f32) -> &mut Self {
    self.inner.premium_member_percentage = premium_member_percentage;
    self
  }

  pub fn prepaid_giveaways(&mut self, prepaid_giveaways: Vec<PrepaidGiveaway>) -> &mut Self {
    self.inner.prepaid_giveaways = prepaid_giveaways;
    self
  }
}

impl AsRef<ChatBoostStatus> for ChatBoostStatus {
  fn as_ref(&self) -> &ChatBoostStatus {
    self
  }
}

impl AsRef<ChatBoostStatus> for RTDChatBoostStatusBuilder {
  fn as_ref(&self) -> &ChatBoostStatus {
    &self.inner
  }
}
