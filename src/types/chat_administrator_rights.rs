use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes rights of the administrator
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatAdministratorRights {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report supergroup spam messages, ignore slow mode, and send messages to the chat without paying Telegram Stars. Implied by any other privilege; applicable to supergroups and channels only
  can_manage_chat: bool,
  /// True, if the administrator can change the chat title, photo, and other settings
  can_change_info: bool,
  /// True, if the administrator can create channel posts, approve suggested channel posts, or view channel statistics; applicable to channels only
  can_post_messages: bool,
  /// True, if the administrator can edit messages of other users and pin messages; applicable to channels only
  can_edit_messages: bool,
  /// True, if the administrator can delete messages of other users
  can_delete_messages: bool,
  /// True, if the administrator can invite new users to the chat
  can_invite_users: bool,
  /// True, if the administrator can restrict, ban, or unban chat members or view supergroup statistics
  can_restrict_members: bool,
  /// True, if the administrator can pin messages; applicable to basic groups and supergroups only
  can_pin_messages: bool,
  /// True, if the administrator can create, rename, close, reopen, hide, and unhide forum topics; applicable to forum supergroups only
  can_manage_topics: bool,
  /// True, if the administrator can add new administrators with a subset of their own privileges or demote administrators that were directly or indirectly promoted by them; applicable to supergroups and channels only
  can_promote_members: bool,
  /// True, if the administrator can manage video chats
  can_manage_video_chats: bool,
  /// True, if the administrator can create new chat stories, or edit and delete posted stories; applicable to supergroups and channels only
  can_post_stories: bool,
  /// True, if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access story archive; applicable to supergroups and channels only
  can_edit_stories: bool,
  /// True, if the administrator can delete stories posted by other users; applicable to supergroups and channels only
  can_delete_stories: bool,
  /// True, if the administrator can answer to channel direct messages; applicable to channels only
  can_manage_direct_messages: bool,
  /// True, if the administrator can change tags of other users; applicable to basic groups and supergroups only
  can_manage_tags: bool,
  /// True, if the administrator can manage and send welcome messages
  can_send_welcome_messages: bool,
  /// True, if the administrator isn't shown in the chat member list and sends messages anonymously; applicable to supergroups only
  is_anonymous: bool,
}

impl RObject for ChatAdministratorRights {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatAdministratorRights"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatAdministratorRights {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatAdministratorRightsBuilder {
    let inner = ChatAdministratorRights {
      td_name: "chatAdministratorRights".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatAdministratorRightsBuilder { inner }
  }

  pub fn can_manage_chat(&self) -> bool {
    self.can_manage_chat
  }

  pub fn can_change_info(&self) -> bool {
    self.can_change_info
  }

  pub fn can_post_messages(&self) -> bool {
    self.can_post_messages
  }

  pub fn can_edit_messages(&self) -> bool {
    self.can_edit_messages
  }

  pub fn can_delete_messages(&self) -> bool {
    self.can_delete_messages
  }

  pub fn can_invite_users(&self) -> bool {
    self.can_invite_users
  }

  pub fn can_restrict_members(&self) -> bool {
    self.can_restrict_members
  }

  pub fn can_pin_messages(&self) -> bool {
    self.can_pin_messages
  }

  pub fn can_manage_topics(&self) -> bool {
    self.can_manage_topics
  }

  pub fn can_promote_members(&self) -> bool {
    self.can_promote_members
  }

  pub fn can_manage_video_chats(&self) -> bool {
    self.can_manage_video_chats
  }

  pub fn can_post_stories(&self) -> bool {
    self.can_post_stories
  }

  pub fn can_edit_stories(&self) -> bool {
    self.can_edit_stories
  }

  pub fn can_delete_stories(&self) -> bool {
    self.can_delete_stories
  }

  pub fn can_manage_direct_messages(&self) -> bool {
    self.can_manage_direct_messages
  }

  pub fn can_manage_tags(&self) -> bool {
    self.can_manage_tags
  }

  pub fn can_send_welcome_messages(&self) -> bool {
    self.can_send_welcome_messages
  }

  pub fn is_anonymous(&self) -> bool {
    self.is_anonymous
  }
}

#[doc(hidden)]
pub struct RTDChatAdministratorRightsBuilder {
  inner: ChatAdministratorRights,
}

impl RTDChatAdministratorRightsBuilder {
  pub fn build(&self) -> ChatAdministratorRights {
    self.inner.clone()
  }

  pub fn can_manage_chat(&mut self, can_manage_chat: bool) -> &mut Self {
    self.inner.can_manage_chat = can_manage_chat;
    self
  }

  pub fn can_change_info(&mut self, can_change_info: bool) -> &mut Self {
    self.inner.can_change_info = can_change_info;
    self
  }

  pub fn can_post_messages(&mut self, can_post_messages: bool) -> &mut Self {
    self.inner.can_post_messages = can_post_messages;
    self
  }

  pub fn can_edit_messages(&mut self, can_edit_messages: bool) -> &mut Self {
    self.inner.can_edit_messages = can_edit_messages;
    self
  }

  pub fn can_delete_messages(&mut self, can_delete_messages: bool) -> &mut Self {
    self.inner.can_delete_messages = can_delete_messages;
    self
  }

  pub fn can_invite_users(&mut self, can_invite_users: bool) -> &mut Self {
    self.inner.can_invite_users = can_invite_users;
    self
  }

  pub fn can_restrict_members(&mut self, can_restrict_members: bool) -> &mut Self {
    self.inner.can_restrict_members = can_restrict_members;
    self
  }

  pub fn can_pin_messages(&mut self, can_pin_messages: bool) -> &mut Self {
    self.inner.can_pin_messages = can_pin_messages;
    self
  }

  pub fn can_manage_topics(&mut self, can_manage_topics: bool) -> &mut Self {
    self.inner.can_manage_topics = can_manage_topics;
    self
  }

  pub fn can_promote_members(&mut self, can_promote_members: bool) -> &mut Self {
    self.inner.can_promote_members = can_promote_members;
    self
  }

  pub fn can_manage_video_chats(&mut self, can_manage_video_chats: bool) -> &mut Self {
    self.inner.can_manage_video_chats = can_manage_video_chats;
    self
  }

  pub fn can_post_stories(&mut self, can_post_stories: bool) -> &mut Self {
    self.inner.can_post_stories = can_post_stories;
    self
  }

  pub fn can_edit_stories(&mut self, can_edit_stories: bool) -> &mut Self {
    self.inner.can_edit_stories = can_edit_stories;
    self
  }

  pub fn can_delete_stories(&mut self, can_delete_stories: bool) -> &mut Self {
    self.inner.can_delete_stories = can_delete_stories;
    self
  }

  pub fn can_manage_direct_messages(&mut self, can_manage_direct_messages: bool) -> &mut Self {
    self.inner.can_manage_direct_messages = can_manage_direct_messages;
    self
  }

  pub fn can_manage_tags(&mut self, can_manage_tags: bool) -> &mut Self {
    self.inner.can_manage_tags = can_manage_tags;
    self
  }

  pub fn can_send_welcome_messages(&mut self, can_send_welcome_messages: bool) -> &mut Self {
    self.inner.can_send_welcome_messages = can_send_welcome_messages;
    self
  }

  pub fn is_anonymous(&mut self, is_anonymous: bool) -> &mut Self {
    self.inner.is_anonymous = is_anonymous;
    self
  }
}

impl AsRef<ChatAdministratorRights> for ChatAdministratorRights {
  fn as_ref(&self) -> &ChatAdministratorRights {
    self
  }
}

impl AsRef<ChatAdministratorRights> for RTDChatAdministratorRightsBuilder {
  fn as_ref(&self) -> &ChatAdministratorRights {
    &self.inner
  }
}
