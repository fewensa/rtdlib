use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a reaction applied to a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddedReaction {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Type of the reaction
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: ReactionType,
  /// Identifier of the chat member, applied the reaction
  sender_id: MessageSender,
  /// True, if the reaction was added by the current user
  is_outgoing: bool,
  /// Point in time (Unix timestamp) when the reaction was added
  date: i64,
}

impl RObject for AddedReaction {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "addedReaction"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AddedReaction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAddedReactionBuilder {
    let inner = AddedReaction {
      td_name: "addedReaction".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAddedReactionBuilder { inner }
  }

  pub fn type_(&self) -> &ReactionType {
    &self.type_
  }

  pub fn sender_id(&self) -> &MessageSender {
    &self.sender_id
  }

  pub fn is_outgoing(&self) -> bool {
    self.is_outgoing
  }

  pub fn date(&self) -> i64 {
    self.date
  }
}

#[doc(hidden)]
pub struct RTDAddedReactionBuilder {
  inner: AddedReaction,
}

impl RTDAddedReactionBuilder {
  pub fn build(&self) -> AddedReaction {
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

  pub fn is_outgoing(&mut self, is_outgoing: bool) -> &mut Self {
    self.inner.is_outgoing = is_outgoing;
    self
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }
}

impl AsRef<AddedReaction> for AddedReaction {
  fn as_ref(&self) -> &AddedReaction {
    self
  }
}

impl AsRef<AddedReaction> for RTDAddedReactionBuilder {
  fn as_ref(&self) -> &AddedReaction {
    &self.inner
  }
}
