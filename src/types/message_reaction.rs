use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a reaction to a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageReaction {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Type of the reaction
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: ReactionType,
  /// Number of times the reaction was added
  total_count: i64,
  /// True, if the reaction is chosen by the current user
  is_chosen: bool,
  /// Identifier of the message sender used by the current user to add the reaction; may be null if unknown or the reaction isn't chosen
  used_sender_id: Option<MessageSender>,
  /// Identifiers of at most 3 recent message senders, added the reaction; available in private, basic group and supergroup chats
  recent_sender_ids: Vec<MessageSender>,
}

impl RObject for MessageReaction {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageReaction"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl MessageReaction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageReactionBuilder {
    let inner = MessageReaction {
      td_name: "messageReaction".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageReactionBuilder { inner }
  }

  pub fn type_(&self) -> &ReactionType {
    &self.type_
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }

  pub fn is_chosen(&self) -> bool {
    self.is_chosen
  }

  pub fn used_sender_id(&self) -> &Option<MessageSender> {
    &self.used_sender_id
  }

  pub fn recent_sender_ids(&self) -> &Vec<MessageSender> {
    &self.recent_sender_ids
  }
}

#[doc(hidden)]
pub struct RTDMessageReactionBuilder {
  inner: MessageReaction,
}

impl RTDMessageReactionBuilder {
  pub fn build(&self) -> MessageReaction {
    self.inner.clone()
  }

  pub fn type_<T: AsRef<ReactionType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

  pub fn is_chosen(&mut self, is_chosen: bool) -> &mut Self {
    self.inner.is_chosen = is_chosen;
    self
  }

  pub fn used_sender_id<T: AsRef<MessageSender>>(&mut self, used_sender_id: T) -> &mut Self {
    self.inner.used_sender_id = Some(used_sender_id.as_ref().clone());
    self
  }

  pub fn recent_sender_ids(&mut self, recent_sender_ids: Vec<MessageSender>) -> &mut Self {
    self.inner.recent_sender_ids = recent_sender_ids;
    self
  }
}

impl AsRef<MessageReaction> for MessageReaction {
  fn as_ref(&self) -> &MessageReaction {
    self
  }
}

impl AsRef<MessageReaction> for RTDMessageReactionBuilder {
  fn as_ref(&self) -> &MessageReaction {
    &self.inner
  }
}
