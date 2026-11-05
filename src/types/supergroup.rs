use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a supergroup or channel with zero or more members (subscribers in the case of channels)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Supergroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Supergroup or channel identifier
  id: i64,
  /// Usernames of the supergroup or channel; may be null
  usernames: Option<Usernames>,
  /// Point in time (Unix timestamp) when the current user joined, or the point in time when the supergroup or channel was created, in case the user is not a member
  date: i64,
  /// Status of the current user in the supergroup or channel
  status: ChatMemberStatus,
  /// Number of members in the supergroup or channel; 0 if unknown. Currently, it is guaranteed to be known only if the supergroup or channel was received through getChatSimilarChats, getChatsToPostStories, getCreatedPublicChats, getGroupsInCommon, getInactiveSupergroupChats, getRecommendedChats, getSuitableDiscussionChats, getUserPrivacySettingRules, getVideoChatAvailableParticipants, searchPublicChats, or in chatFolderInviteLinkInfo.missing_chat_ids, or in userFullInfo.personal_chat_id, or for chats with messages or stories from publicForwards and foundStories
  member_count: i64,
  /// Approximate boost level for the chat
  boost_level: i64,
  /// True, if automatic translation of messages is enabled in the channel
  has_automatic_translation: bool,
  /// True, if the channel has a discussion group, or the supergroup is the designated discussion group for a channel
  has_linked_chat: bool,
  /// True, if the supergroup is connected to a location, i.e. the supergroup is a location-based supergroup
  has_location: bool,
  /// True, if messages sent to the channel contains name of the sender. This field is only applicable to channels
  sign_messages: bool,
  /// True, if messages sent to the channel have information about the sender user. This field is only applicable to channels
  show_message_sender: bool,
  /// True, if users need to join the supergroup before they can send messages. May be false only for discussion supergroups and channel direct messages groups
  join_to_send_messages: bool,
  /// True, if all users directly joining the supergroup need to be approved by supergroup administrators
  join_by_request: bool,
  /// True, if the slow mode is enabled in the supergroup
  is_slow_mode_enabled: bool,
  /// True, if the supergroup is a channel, which can have an unlimited number of subscribers, but only administrators can post there and see the list of subscribers
  is_channel: bool,
  /// True, if the supergroup is a broadcast group, i.e. only administrators can send messages and there is no limit on the number of members
  is_broadcast_group: bool,
  /// True, if the supergroup is a forum with topics
  is_forum: bool,
  /// True, if the supergroup is a direct message group for a channel chat
  is_direct_messages_group: bool,
  /// True, if the supergroup is a direct messages group for a channel chat that is administered by the current user
  is_administered_direct_messages_group: bool,
  /// Information about verification status of the supergroup or channel; may be null if none
  verification_status: Option<VerificationStatus>,
  /// True, if the channel has direct messages group
  has_direct_messages_group: bool,
  /// True, if the supergroup is a forum, which topics are shown in the same way as in channel direct messages groups
  has_forum_tabs: bool,
  /// Information about the restrictions that must be applied to the corresponding supergroup or channel chat; may be null if none
  restriction_info: Option<RestrictionInfo>,
  /// Number of Telegram Stars that must be paid by non-administrator users of the supergroup chat for each sent message
  paid_message_star_count: i64,
  /// State of active stories of the supergroup or channel; may be null if there are no active stories
  active_story_state: Option<ActiveStoryState>,
}

impl RObject for Supergroup {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "supergroup"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl Supergroup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSupergroupBuilder {
    let inner = Supergroup {
      td_name: "supergroup".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSupergroupBuilder { inner }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn usernames(&self) -> &Option<Usernames> {
    &self.usernames
  }

  pub fn date(&self) -> i64 {
    self.date
  }

  pub fn status(&self) -> &ChatMemberStatus {
    &self.status
  }

  pub fn member_count(&self) -> i64 {
    self.member_count
  }

  pub fn boost_level(&self) -> i64 {
    self.boost_level
  }

  pub fn has_automatic_translation(&self) -> bool {
    self.has_automatic_translation
  }

  pub fn has_linked_chat(&self) -> bool {
    self.has_linked_chat
  }

  pub fn has_location(&self) -> bool {
    self.has_location
  }

  pub fn sign_messages(&self) -> bool {
    self.sign_messages
  }

  pub fn show_message_sender(&self) -> bool {
    self.show_message_sender
  }

  pub fn join_to_send_messages(&self) -> bool {
    self.join_to_send_messages
  }

  pub fn join_by_request(&self) -> bool {
    self.join_by_request
  }

  pub fn is_slow_mode_enabled(&self) -> bool {
    self.is_slow_mode_enabled
  }

  pub fn is_channel(&self) -> bool {
    self.is_channel
  }

  pub fn is_broadcast_group(&self) -> bool {
    self.is_broadcast_group
  }

  pub fn is_forum(&self) -> bool {
    self.is_forum
  }

  pub fn is_direct_messages_group(&self) -> bool {
    self.is_direct_messages_group
  }

  pub fn is_administered_direct_messages_group(&self) -> bool {
    self.is_administered_direct_messages_group
  }

  pub fn verification_status(&self) -> &Option<VerificationStatus> {
    &self.verification_status
  }

  pub fn has_direct_messages_group(&self) -> bool {
    self.has_direct_messages_group
  }

  pub fn has_forum_tabs(&self) -> bool {
    self.has_forum_tabs
  }

  pub fn restriction_info(&self) -> &Option<RestrictionInfo> {
    &self.restriction_info
  }

  pub fn paid_message_star_count(&self) -> i64 {
    self.paid_message_star_count
  }

  pub fn active_story_state(&self) -> &Option<ActiveStoryState> {
    &self.active_story_state
  }
}

#[doc(hidden)]
pub struct RTDSupergroupBuilder {
  inner: Supergroup,
}

impl RTDSupergroupBuilder {
  pub fn build(&self) -> Supergroup {
    self.inner.clone()
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn usernames<T: AsRef<Usernames>>(&mut self, usernames: T) -> &mut Self {
    self.inner.usernames = Some(usernames.as_ref().clone());
    self
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

  pub fn status<T: AsRef<ChatMemberStatus>>(&mut self, status: T) -> &mut Self {
    self.inner.status = status.as_ref().clone();
    self
  }

  pub fn member_count(&mut self, member_count: i64) -> &mut Self {
    self.inner.member_count = member_count;
    self
  }

  pub fn boost_level(&mut self, boost_level: i64) -> &mut Self {
    self.inner.boost_level = boost_level;
    self
  }

  pub fn has_automatic_translation(&mut self, has_automatic_translation: bool) -> &mut Self {
    self.inner.has_automatic_translation = has_automatic_translation;
    self
  }

  pub fn has_linked_chat(&mut self, has_linked_chat: bool) -> &mut Self {
    self.inner.has_linked_chat = has_linked_chat;
    self
  }

  pub fn has_location(&mut self, has_location: bool) -> &mut Self {
    self.inner.has_location = has_location;
    self
  }

  pub fn sign_messages(&mut self, sign_messages: bool) -> &mut Self {
    self.inner.sign_messages = sign_messages;
    self
  }

  pub fn show_message_sender(&mut self, show_message_sender: bool) -> &mut Self {
    self.inner.show_message_sender = show_message_sender;
    self
  }

  pub fn join_to_send_messages(&mut self, join_to_send_messages: bool) -> &mut Self {
    self.inner.join_to_send_messages = join_to_send_messages;
    self
  }

  pub fn join_by_request(&mut self, join_by_request: bool) -> &mut Self {
    self.inner.join_by_request = join_by_request;
    self
  }

  pub fn is_slow_mode_enabled(&mut self, is_slow_mode_enabled: bool) -> &mut Self {
    self.inner.is_slow_mode_enabled = is_slow_mode_enabled;
    self
  }

  pub fn is_channel(&mut self, is_channel: bool) -> &mut Self {
    self.inner.is_channel = is_channel;
    self
  }

  pub fn is_broadcast_group(&mut self, is_broadcast_group: bool) -> &mut Self {
    self.inner.is_broadcast_group = is_broadcast_group;
    self
  }

  pub fn is_forum(&mut self, is_forum: bool) -> &mut Self {
    self.inner.is_forum = is_forum;
    self
  }

  pub fn is_direct_messages_group(&mut self, is_direct_messages_group: bool) -> &mut Self {
    self.inner.is_direct_messages_group = is_direct_messages_group;
    self
  }

  pub fn is_administered_direct_messages_group(
    &mut self,
    is_administered_direct_messages_group: bool,
  ) -> &mut Self {
    self.inner.is_administered_direct_messages_group = is_administered_direct_messages_group;
    self
  }

  pub fn verification_status<T: AsRef<VerificationStatus>>(
    &mut self,
    verification_status: T,
  ) -> &mut Self {
    self.inner.verification_status = Some(verification_status.as_ref().clone());
    self
  }

  pub fn has_direct_messages_group(&mut self, has_direct_messages_group: bool) -> &mut Self {
    self.inner.has_direct_messages_group = has_direct_messages_group;
    self
  }

  pub fn has_forum_tabs(&mut self, has_forum_tabs: bool) -> &mut Self {
    self.inner.has_forum_tabs = has_forum_tabs;
    self
  }

  pub fn restriction_info<T: AsRef<RestrictionInfo>>(&mut self, restriction_info: T) -> &mut Self {
    self.inner.restriction_info = Some(restriction_info.as_ref().clone());
    self
  }

  pub fn paid_message_star_count(&mut self, paid_message_star_count: i64) -> &mut Self {
    self.inner.paid_message_star_count = paid_message_star_count;
    self
  }

  pub fn active_story_state<T: AsRef<ActiveStoryState>>(
    &mut self,
    active_story_state: T,
  ) -> &mut Self {
    self.inner.active_story_state = Some(active_story_state.as_ref().clone());
    self
  }
}

impl AsRef<Supergroup> for Supergroup {
  fn as_ref(&self) -> &Supergroup {
    self
  }
}

impl AsRef<Supergroup> for RTDSupergroupBuilder {
  fn as_ref(&self) -> &Supergroup {
    &self.inner
  }
}
