use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of reactions added to a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageReactions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of added reactions
  reactions: Vec<MessageReaction>,
  /// True, if the reactions are tags and Telegram Premium users can filter messages by them
  are_tags: bool,
  /// Information about top users that added the paid reaction
  paid_reactors: Vec<PaidReactor>,
  /// True, if the list of added reactions is available using getMessageAddedReactions
  can_get_added_reactions: bool,
}

impl RObject for MessageReactions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageReactions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl MessageReactions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageReactionsBuilder {
    let inner = MessageReactions {
      td_name: "messageReactions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageReactionsBuilder { inner }
  }

  pub fn reactions(&self) -> &Vec<MessageReaction> {
    &self.reactions
  }

  pub fn are_tags(&self) -> bool {
    self.are_tags
  }

  pub fn paid_reactors(&self) -> &Vec<PaidReactor> {
    &self.paid_reactors
  }

  pub fn can_get_added_reactions(&self) -> bool {
    self.can_get_added_reactions
  }
}

#[doc(hidden)]
pub struct RTDMessageReactionsBuilder {
  inner: MessageReactions,
}

impl RTDMessageReactionsBuilder {
  pub fn build(&self) -> MessageReactions {
    self.inner.clone()
  }

  pub fn reactions(&mut self, reactions: Vec<MessageReaction>) -> &mut Self {
    self.inner.reactions = reactions;
    self
  }

  pub fn are_tags(&mut self, are_tags: bool) -> &mut Self {
    self.inner.are_tags = are_tags;
    self
  }

  pub fn paid_reactors(&mut self, paid_reactors: Vec<PaidReactor>) -> &mut Self {
    self.inner.paid_reactors = paid_reactors;
    self
  }

  pub fn can_get_added_reactions(&mut self, can_get_added_reactions: bool) -> &mut Self {
    self.inner.can_get_added_reactions = can_get_added_reactions;
    self
  }
}

impl AsRef<MessageReactions> for MessageReactions {
  fn as_ref(&self) -> &MessageReactions {
    self
  }
}

impl AsRef<MessageReactions> for RTDMessageReactionsBuilder {
  fn as_ref(&self) -> &MessageReactions {
    &self.inner
  }
}
