use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a set of filters used to obtain a chat event log
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventLogFilters {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if message edits need to be returned
  message_edits: bool,
  /// True, if message deletions need to be returned
  message_deletions: bool,
  /// True, if pin/unpin events need to be returned
  message_pins: bool,
  /// True, if members joining events need to be returned
  member_joins: bool,
  /// True, if members leaving events need to be returned
  member_leaves: bool,
  /// True, if invited member events need to be returned
  member_invites: bool,
  /// True, if member promotion/demotion events need to be returned
  member_promotions: bool,
  /// True, if member restricted/unrestricted/banned/unbanned events need to be returned
  member_restrictions: bool,
  /// True, if member tag and custom title change events need to be returned
  member_tag_changes: bool,
  /// True, if changes in chat information need to be returned
  info_changes: bool,
  /// True, if changes in chat settings need to be returned
  setting_changes: bool,
  /// True, if changes to invite links need to be returned
  invite_link_changes: bool,
  /// True, if video chat actions need to be returned
  video_chat_changes: bool,
  /// True, if forum-related actions need to be returned
  forum_changes: bool,
  /// True, if subscription extensions need to be returned
  subscription_extensions: bool,
}

impl RObject for ChatEventLogFilters {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatEventLogFilters"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatEventLogFilters {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatEventLogFiltersBuilder {
    let inner = ChatEventLogFilters {
      td_name: "chatEventLogFilters".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatEventLogFiltersBuilder { inner }
  }

  pub fn message_edits(&self) -> bool {
    self.message_edits
  }

  pub fn message_deletions(&self) -> bool {
    self.message_deletions
  }

  pub fn message_pins(&self) -> bool {
    self.message_pins
  }

  pub fn member_joins(&self) -> bool {
    self.member_joins
  }

  pub fn member_leaves(&self) -> bool {
    self.member_leaves
  }

  pub fn member_invites(&self) -> bool {
    self.member_invites
  }

  pub fn member_promotions(&self) -> bool {
    self.member_promotions
  }

  pub fn member_restrictions(&self) -> bool {
    self.member_restrictions
  }

  pub fn member_tag_changes(&self) -> bool {
    self.member_tag_changes
  }

  pub fn info_changes(&self) -> bool {
    self.info_changes
  }

  pub fn setting_changes(&self) -> bool {
    self.setting_changes
  }

  pub fn invite_link_changes(&self) -> bool {
    self.invite_link_changes
  }

  pub fn video_chat_changes(&self) -> bool {
    self.video_chat_changes
  }

  pub fn forum_changes(&self) -> bool {
    self.forum_changes
  }

  pub fn subscription_extensions(&self) -> bool {
    self.subscription_extensions
  }
}

#[doc(hidden)]
pub struct RTDChatEventLogFiltersBuilder {
  inner: ChatEventLogFilters,
}

impl RTDChatEventLogFiltersBuilder {
  pub fn build(&self) -> ChatEventLogFilters {
    self.inner.clone()
  }

  pub fn message_edits(&mut self, message_edits: bool) -> &mut Self {
    self.inner.message_edits = message_edits;
    self
  }

  pub fn message_deletions(&mut self, message_deletions: bool) -> &mut Self {
    self.inner.message_deletions = message_deletions;
    self
  }

  pub fn message_pins(&mut self, message_pins: bool) -> &mut Self {
    self.inner.message_pins = message_pins;
    self
  }

  pub fn member_joins(&mut self, member_joins: bool) -> &mut Self {
    self.inner.member_joins = member_joins;
    self
  }

  pub fn member_leaves(&mut self, member_leaves: bool) -> &mut Self {
    self.inner.member_leaves = member_leaves;
    self
  }

  pub fn member_invites(&mut self, member_invites: bool) -> &mut Self {
    self.inner.member_invites = member_invites;
    self
  }

  pub fn member_promotions(&mut self, member_promotions: bool) -> &mut Self {
    self.inner.member_promotions = member_promotions;
    self
  }

  pub fn member_restrictions(&mut self, member_restrictions: bool) -> &mut Self {
    self.inner.member_restrictions = member_restrictions;
    self
  }

  pub fn member_tag_changes(&mut self, member_tag_changes: bool) -> &mut Self {
    self.inner.member_tag_changes = member_tag_changes;
    self
  }

  pub fn info_changes(&mut self, info_changes: bool) -> &mut Self {
    self.inner.info_changes = info_changes;
    self
  }

  pub fn setting_changes(&mut self, setting_changes: bool) -> &mut Self {
    self.inner.setting_changes = setting_changes;
    self
  }

  pub fn invite_link_changes(&mut self, invite_link_changes: bool) -> &mut Self {
    self.inner.invite_link_changes = invite_link_changes;
    self
  }

  pub fn video_chat_changes(&mut self, video_chat_changes: bool) -> &mut Self {
    self.inner.video_chat_changes = video_chat_changes;
    self
  }

  pub fn forum_changes(&mut self, forum_changes: bool) -> &mut Self {
    self.inner.forum_changes = forum_changes;
    self
  }

  pub fn subscription_extensions(&mut self, subscription_extensions: bool) -> &mut Self {
    self.inner.subscription_extensions = subscription_extensions;
    self
  }
}

impl AsRef<ChatEventLogFilters> for ChatEventLogFilters {
  fn as_ref(&self) -> &ChatEventLogFilters {
    self
  }
}

impl AsRef<ChatEventLogFilters> for RTDChatEventLogFiltersBuilder {
  fn as_ref(&self) -> &ChatEventLogFilters {
    &self.inner
  }
}
