use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a slot for chat boost
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatBoostSlot {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the slot
  slot_id: i64,
  /// Identifier of the currently boosted chat; 0 if none
  currently_boosted_chat_id: i64,
  /// Point in time (Unix timestamp) when the chat was boosted; 0 if none
  start_date: i64,
  /// Point in time (Unix timestamp) when the boost will expire
  expiration_date: i64,
  /// Point in time (Unix timestamp) after which the boost can be used for another chat
  cooldown_until_date: i64,
}

impl RObject for ChatBoostSlot {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatBoostSlot"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatBoostSlot {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatBoostSlotBuilder {
    let inner = ChatBoostSlot {
      td_name: "chatBoostSlot".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatBoostSlotBuilder { inner }
  }

  pub fn slot_id(&self) -> i64 {
    self.slot_id
  }

  pub fn currently_boosted_chat_id(&self) -> i64 {
    self.currently_boosted_chat_id
  }

  pub fn start_date(&self) -> i64 {
    self.start_date
  }

  pub fn expiration_date(&self) -> i64 {
    self.expiration_date
  }

  pub fn cooldown_until_date(&self) -> i64 {
    self.cooldown_until_date
  }
}

#[doc(hidden)]
pub struct RTDChatBoostSlotBuilder {
  inner: ChatBoostSlot,
}

impl RTDChatBoostSlotBuilder {
  pub fn build(&self) -> ChatBoostSlot {
    self.inner.clone()
  }

  pub fn slot_id(&mut self, slot_id: i64) -> &mut Self {
    self.inner.slot_id = slot_id;
    self
  }

  pub fn currently_boosted_chat_id(&mut self, currently_boosted_chat_id: i64) -> &mut Self {
    self.inner.currently_boosted_chat_id = currently_boosted_chat_id;
    self
  }

  pub fn start_date(&mut self, start_date: i64) -> &mut Self {
    self.inner.start_date = start_date;
    self
  }

  pub fn expiration_date(&mut self, expiration_date: i64) -> &mut Self {
    self.inner.expiration_date = expiration_date;
    self
  }

  pub fn cooldown_until_date(&mut self, cooldown_until_date: i64) -> &mut Self {
    self.inner.cooldown_until_date = cooldown_until_date;
    self
  }
}

impl AsRef<ChatBoostSlot> for ChatBoostSlot {
  fn as_ref(&self) -> &ChatBoostSlot {
    self
  }
}

impl AsRef<ChatBoostSlot> for RTDChatBoostSlotBuilder {
  fn as_ref(&self) -> &ChatBoostSlot {
    &self.inner
  }
}
