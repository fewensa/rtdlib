use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of chat boost slots
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatBoostSlots {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of boost slots
  slots: Vec<ChatBoostSlot>,
}

impl RObject for ChatBoostSlots {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatBoostSlots"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatBoostSlots {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatBoostSlotsBuilder {
    let inner = ChatBoostSlots {
      td_name: "chatBoostSlots".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatBoostSlotsBuilder { inner }
  }

  pub fn slots(&self) -> &Vec<ChatBoostSlot> {
    &self.slots
  }
}

#[doc(hidden)]
pub struct RTDChatBoostSlotsBuilder {
  inner: ChatBoostSlots,
}

impl RTDChatBoostSlotsBuilder {
  pub fn build(&self) -> ChatBoostSlots {
    self.inner.clone()
  }

  pub fn slots(&mut self, slots: Vec<ChatBoostSlot>) -> &mut Self {
    self.inner.slots = slots;
    self
  }
}

impl AsRef<ChatBoostSlots> for ChatBoostSlots {
  fn as_ref(&self) -> &ChatBoostSlots {
    self
  }
}

impl AsRef<ChatBoostSlots> for RTDChatBoostSlotsBuilder {
  fn as_ref(&self) -> &ChatBoostSlots {
    &self.inner
  }
}
