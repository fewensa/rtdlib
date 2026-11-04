use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about an unread reaction to a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnreadReaction {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Type of the reaction
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: ReactionType,
  /// Identifier of the sender, added the reaction
  sender_id: MessageSender,
  /// True, if the reaction was added with a big animation
  is_big: bool,
}

impl RObject for UnreadReaction {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "unreadReaction"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UnreadReaction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUnreadReactionBuilder {
    let inner = UnreadReaction {
      td_name: "unreadReaction".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUnreadReactionBuilder { inner }
  }

  pub fn type_(&self) -> &ReactionType {
    &self.type_
  }

  pub fn sender_id(&self) -> &MessageSender {
    &self.sender_id
  }

  pub fn is_big(&self) -> bool {
    self.is_big
  }
}

#[doc(hidden)]
pub struct RTDUnreadReactionBuilder {
  inner: UnreadReaction,
}

impl RTDUnreadReactionBuilder {
  pub fn build(&self) -> UnreadReaction {
    self.inner.clone()
  }

  pub fn type_<T: AsRef<ReactionType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

  pub fn sender_id<T: AsRef<MessageSender>>(&mut self, sender_id: T) -> &mut Self {
    self.inner.sender_id = sender_id.as_ref().clone();
    self
  }

  pub fn is_big(&mut self, is_big: bool) -> &mut Self {
    self.inner.is_big = is_big;
    self
  }
}

impl AsRef<UnreadReaction> for UnreadReaction {
  fn as_ref(&self) -> &UnreadReaction {
    self
  }
}

impl AsRef<UnreadReaction> for RTDUnreadReactionBuilder {
  fn as_ref(&self) -> &UnreadReaction {
    &self.inner
  }
}
