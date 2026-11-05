use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of reactions that can be added to a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AvailableReactions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of reactions to be shown at the top
  top_reactions: Vec<AvailableReaction>,
  /// List of recently used reactions
  recent_reactions: Vec<AvailableReaction>,
  /// List of popular reactions
  popular_reactions: Vec<AvailableReaction>,
  /// True, if any custom emoji reaction can be added by Telegram Premium subscribers
  allow_custom_emoji: bool,
  /// True, if the reactions will be tags and the message can be found by them
  are_tags: bool,
  /// The reason why the current user can't add reactions to the message, despite some other users can; may be null if none
  unavailability_reason: Option<ReactionUnavailabilityReason>,
}

impl RObject for AvailableReactions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "availableReactions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AvailableReactions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAvailableReactionsBuilder {
    let inner = AvailableReactions {
      td_name: "availableReactions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAvailableReactionsBuilder { inner }
  }

  pub fn top_reactions(&self) -> &Vec<AvailableReaction> {
    &self.top_reactions
  }

  pub fn recent_reactions(&self) -> &Vec<AvailableReaction> {
    &self.recent_reactions
  }

  pub fn popular_reactions(&self) -> &Vec<AvailableReaction> {
    &self.popular_reactions
  }

  pub fn allow_custom_emoji(&self) -> bool {
    self.allow_custom_emoji
  }

  pub fn are_tags(&self) -> bool {
    self.are_tags
  }

  pub fn unavailability_reason(&self) -> &Option<ReactionUnavailabilityReason> {
    &self.unavailability_reason
  }
}

#[doc(hidden)]
pub struct RTDAvailableReactionsBuilder {
  inner: AvailableReactions,
}

impl RTDAvailableReactionsBuilder {
  pub fn build(&self) -> AvailableReactions {
    self.inner.clone()
  }

  pub fn top_reactions(&mut self, top_reactions: Vec<AvailableReaction>) -> &mut Self {
    self.inner.top_reactions = top_reactions;
    self
  }

  pub fn recent_reactions(&mut self, recent_reactions: Vec<AvailableReaction>) -> &mut Self {
    self.inner.recent_reactions = recent_reactions;
    self
  }

  pub fn popular_reactions(&mut self, popular_reactions: Vec<AvailableReaction>) -> &mut Self {
    self.inner.popular_reactions = popular_reactions;
    self
  }

  pub fn allow_custom_emoji(&mut self, allow_custom_emoji: bool) -> &mut Self {
    self.inner.allow_custom_emoji = allow_custom_emoji;
    self
  }

  pub fn are_tags(&mut self, are_tags: bool) -> &mut Self {
    self.inner.are_tags = are_tags;
    self
  }

  pub fn unavailability_reason<T: AsRef<ReactionUnavailabilityReason>>(
    &mut self,
    unavailability_reason: T,
  ) -> &mut Self {
    self.inner.unavailability_reason = Some(unavailability_reason.as_ref().clone());
    self
  }
}

impl AsRef<AvailableReactions> for AvailableReactions {
  fn as_ref(&self) -> &AvailableReactions {
    self
  }
}

impl AsRef<AvailableReactions> for RTDAvailableReactionsBuilder {
  fn as_ref(&self) -> &AvailableReactions {
    &self.inner
  }
}
