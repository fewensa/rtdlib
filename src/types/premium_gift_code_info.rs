use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a Telegram Premium gift code
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumGiftCodeInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of a chat or a user who created the gift code; may be null if unknown. If null and the code is from messagePremiumGiftCode message, then creator_id from the message can be used
  creator_id: Option<MessageSender>,
  /// Point in time (Unix timestamp) when the code was created
  creation_date: i64,
  /// True, if the gift code was created for a giveaway
  is_from_giveaway: bool,
  /// Identifier of the corresponding giveaway message in the creator_id chat; may be 0 or an identifier of a deleted message
  giveaway_message_id: i64,
  /// Number of months the Telegram Premium subscription will be active after code activation; 0 if the number of months isn't integer
  month_count: i64,
  /// Number of days the Telegram Premium subscription will be active after code activation
  day_count: i64,
  /// Identifier of a user for which the code was created; 0 if none
  user_id: i64,
  /// Point in time (Unix timestamp) when the code was activated; 0 if none
  use_date: i64,
}

impl RObject for PremiumGiftCodeInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumGiftCodeInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PremiumGiftCodeInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumGiftCodeInfoBuilder {
    let inner = PremiumGiftCodeInfo {
      td_name: "premiumGiftCodeInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPremiumGiftCodeInfoBuilder { inner }
  }

  pub fn creator_id(&self) -> &Option<MessageSender> {
    &self.creator_id
  }

  pub fn creation_date(&self) -> i64 {
    self.creation_date
  }

  pub fn is_from_giveaway(&self) -> bool {
    self.is_from_giveaway
  }

  pub fn giveaway_message_id(&self) -> i64 {
    self.giveaway_message_id
  }

  pub fn month_count(&self) -> i64 {
    self.month_count
  }

  pub fn day_count(&self) -> i64 {
    self.day_count
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn use_date(&self) -> i64 {
    self.use_date
  }
}

#[doc(hidden)]
pub struct RTDPremiumGiftCodeInfoBuilder {
  inner: PremiumGiftCodeInfo,
}

impl RTDPremiumGiftCodeInfoBuilder {
  pub fn build(&self) -> PremiumGiftCodeInfo {
    self.inner.clone()
  }

  pub fn creator_id<T: AsRef<MessageSender>>(&mut self, creator_id: T) -> &mut Self {
    self.inner.creator_id = Some(creator_id.as_ref().clone());
    self
  }

  pub fn creation_date(&mut self, creation_date: i64) -> &mut Self {
    self.inner.creation_date = creation_date;
    self
  }

  pub fn is_from_giveaway(&mut self, is_from_giveaway: bool) -> &mut Self {
    self.inner.is_from_giveaway = is_from_giveaway;
    self
  }

  pub fn giveaway_message_id(&mut self, giveaway_message_id: i64) -> &mut Self {
    self.inner.giveaway_message_id = giveaway_message_id;
    self
  }

  pub fn month_count(&mut self, month_count: i64) -> &mut Self {
    self.inner.month_count = month_count;
    self
  }

  pub fn day_count(&mut self, day_count: i64) -> &mut Self {
    self.inner.day_count = day_count;
    self
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn use_date(&mut self, use_date: i64) -> &mut Self {
    self.inner.use_date = use_date;
    self
  }
}

impl AsRef<PremiumGiftCodeInfo> for PremiumGiftCodeInfo {
  fn as_ref(&self) -> &PremiumGiftCodeInfo {
    self
  }
}

impl AsRef<PremiumGiftCodeInfo> for RTDPremiumGiftCodeInfoBuilder {
  fn as_ref(&self) -> &PremiumGiftCodeInfo {
    &self.inner
  }
}
