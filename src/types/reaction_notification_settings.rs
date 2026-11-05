use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about notification settings for reactions and poll votes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReactionNotificationSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Source of message reactions for which notifications are shown
  message_reaction_source: ReactionNotificationSource,
  /// Source of story reactions for which notifications are shown
  story_reaction_source: ReactionNotificationSource,
  /// Source of poll votes for which notifications are shown
  poll_vote_source: ReactionNotificationSource,
  /// Identifier of the notification sound to be played; 0 if sound is disabled; pass 1 to use the app-dependent default sound
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  sound_id: isize,
  /// True, if reaction sender and emoji must be displayed in notifications
  show_preview: bool,
}

impl RObject for ReactionNotificationSettings {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reactionNotificationSettings"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ReactionNotificationSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReactionNotificationSettingsBuilder {
    let inner = ReactionNotificationSettings {
      td_name: "reactionNotificationSettings".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDReactionNotificationSettingsBuilder { inner }
  }

  pub fn message_reaction_source(&self) -> &ReactionNotificationSource {
    &self.message_reaction_source
  }

  pub fn story_reaction_source(&self) -> &ReactionNotificationSource {
    &self.story_reaction_source
  }

  pub fn poll_vote_source(&self) -> &ReactionNotificationSource {
    &self.poll_vote_source
  }

  pub fn sound_id(&self) -> isize {
    self.sound_id
  }

  pub fn show_preview(&self) -> bool {
    self.show_preview
  }
}

#[doc(hidden)]
pub struct RTDReactionNotificationSettingsBuilder {
  inner: ReactionNotificationSettings,
}

impl RTDReactionNotificationSettingsBuilder {
  pub fn build(&self) -> ReactionNotificationSettings {
    self.inner.clone()
  }

  pub fn message_reaction_source<T: AsRef<ReactionNotificationSource>>(
    &mut self,
    message_reaction_source: T,
  ) -> &mut Self {
    self.inner.message_reaction_source = message_reaction_source.as_ref().clone();
    self
  }

  pub fn story_reaction_source<T: AsRef<ReactionNotificationSource>>(
    &mut self,
    story_reaction_source: T,
  ) -> &mut Self {
    self.inner.story_reaction_source = story_reaction_source.as_ref().clone();
    self
  }

  pub fn poll_vote_source<T: AsRef<ReactionNotificationSource>>(
    &mut self,
    poll_vote_source: T,
  ) -> &mut Self {
    self.inner.poll_vote_source = poll_vote_source.as_ref().clone();
    self
  }

  pub fn sound_id(&mut self, sound_id: isize) -> &mut Self {
    self.inner.sound_id = sound_id;
    self
  }

  pub fn show_preview(&mut self, show_preview: bool) -> &mut Self {
    self.inner.show_preview = show_preview;
    self
  }
}

impl AsRef<ReactionNotificationSettings> for ReactionNotificationSettings {
  fn as_ref(&self) -> &ReactionNotificationSettings {
    self
  }
}

impl AsRef<ReactionNotificationSettings> for RTDReactionNotificationSettingsBuilder {
  fn as_ref(&self) -> &ReactionNotificationSettings {
    &self.inner
  }
}
