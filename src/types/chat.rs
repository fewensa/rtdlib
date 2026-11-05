use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A chat. (Can be a private chat, basic group, supergroup, or secret chat)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Chat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat unique identifier
  id: i64,
  /// Type of the chat
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: ChatType,
  /// Chat title
  title: String,
  /// Chat photo; may be null
  photo: Option<ChatPhotoInfo>,
  /// Identifier of the accent color for message sender name, and backgrounds of chat photo, reply header, and link preview
  accent_color_id: i64,
  /// Identifier of a custom emoji to be shown on the reply header and link preview background for messages sent by the chat; 0 if none
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  background_custom_emoji_id: isize,
  /// Color scheme based on an upgraded gift to be used for the chat instead of accent_color_id and background_custom_emoji_id; may be null if none
  upgraded_gift_colors: Option<UpgradedGiftColors>,
  /// Identifier of the profile accent color for the chat's profile; 1 if none
  profile_accent_color_id: i64,
  /// Identifier of a custom emoji to be shown on the background of the chat's profile; 0 if none
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  profile_background_custom_emoji_id: isize,
  /// Actions that non-administrator chat members are allowed to take in the chat
  permissions: ChatPermissions,
  /// Last message in the chat; may be null if none or unknown
  last_message: Option<Message>,
  /// Positions of the chat in chat lists
  positions: Option<Vec<ChatPosition>>,
  /// Chat lists to which the chat belongs. A chat can have a non-zero position in a chat list even if it doesn't belong to the chat list and have no position in a chat list even if it belongs to the chat list
  chat_lists: Vec<ChatList>,
  /// Identifier of a user or chat that is selected to send messages in the chat; may be null if the user can't change message sender
  message_sender_id: Option<MessageSender>,
  /// Block list to which the chat is added; may be null if none
  block_list: Option<BlockList>,
  /// True, if chat content can't be saved locally, forwarded, or copied
  has_protected_content: bool,
  /// True, if translation of all messages in the chat must be suggested to the user
  is_translatable: bool,
  /// True, if the chat is marked as unread
  is_marked_as_unread: bool,
  /// True, if the chat is a forum supergroup that must be shown in the "View as topics" mode, or Saved Messages chat that must be shown in the "View as chats"
  view_as_topics: bool,
  /// True, if the chat has scheduled messages
  has_scheduled_messages: bool,
  /// True, if the chat has welcome messages; for chat administrators with can_change_info administrator right only
  has_welcome_messages: bool,
  /// True, if the chat messages can be deleted only for the current user while other users will continue to see the messages
  can_be_deleted_only_for_self: bool,
  /// True, if the chat messages can be deleted for all users
  can_be_deleted_for_all_users: bool,
  /// True, if the chat can be reported to Telegram moderators through reportChat or reportChatPhoto
  can_be_reported: bool,
  /// Default value of the disable_notification parameter, used when a message is sent to the chat
  default_disable_notification: bool,
  /// Number of unread messages in the chat
  unread_count: i64,
  /// Identifier of the last read incoming message
  last_read_inbox_message_id: i64,
  /// Identifier of the last read outgoing message
  last_read_outbox_message_id: i64,
  /// Number of unread messages with a mention/reply in the chat
  unread_mention_count: i64,
  /// Number of messages with unread reactions in the chat
  unread_reaction_count: i64,
  /// Number of messages with unread poll votes in the chat
  unread_poll_vote_count: i64,
  /// Notification settings for the chat
  notification_settings: ChatNotificationSettings,
  /// Types of reaction, available in the chat
  available_reactions: ChatAvailableReactions,
  /// Current message auto-delete or self-destruct timer setting for the chat, in seconds; 0 if disabled. Self-destruct timer in secret chats starts after the message or its content is viewed. Auto-delete timer in other chats starts from the send date
  message_auto_delete_time: i64,
  /// Emoji status to be shown along with chat title; may be null
  emoji_status: Option<EmojiStatus>,
  /// Background set for the chat; may be null if none
  background: Option<ChatBackground>,
  /// Theme set for the chat; may be null if none
  theme: Option<ChatTheme>,
  /// Information about actions which must be possible to do through the chat action bar; may be null if none
  action_bar: Option<ChatActionBar>,
  /// Information about bar for managing a business bot in the chat; may be null if none
  business_bot_manage_bar: Option<BusinessBotManageBar>,
  /// Information about video chat of the chat
  video_chat: VideoChat,
  /// Information about pending join requests; may be null if none
  pending_join_requests: Option<ChatJoinRequestsInfo>,
  /// Identifier of the message from which reply markup needs to be used; 0 if there is no reply markup in the chat
  reply_markup_message_id: i64,
  /// A draft of a message in the chat; may be null if none
  draft_message: Option<DraftMessage>,
  /// Application-specific data associated with the chat. (For example, the chat scroll position or local chat notification settings can be stored here.) Persistent if the message database is used
  client_data: String,
}

impl RObject for Chat {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chat"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl Chat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatBuilder {
    let inner = Chat {
      td_name: "chat".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatBuilder { inner }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn type_(&self) -> &ChatType {
    &self.type_
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn photo(&self) -> &Option<ChatPhotoInfo> {
    &self.photo
  }

  pub fn accent_color_id(&self) -> i64 {
    self.accent_color_id
  }

  pub fn background_custom_emoji_id(&self) -> isize {
    self.background_custom_emoji_id
  }

  pub fn upgraded_gift_colors(&self) -> &Option<UpgradedGiftColors> {
    &self.upgraded_gift_colors
  }

  pub fn profile_accent_color_id(&self) -> i64 {
    self.profile_accent_color_id
  }

  pub fn profile_background_custom_emoji_id(&self) -> isize {
    self.profile_background_custom_emoji_id
  }

  pub fn permissions(&self) -> &ChatPermissions {
    &self.permissions
  }

  pub fn last_message(&self) -> &Option<Message> {
    &self.last_message
  }

  pub fn positions(&self) -> &Option<Vec<ChatPosition>> {
    &self.positions
  }

  pub fn chat_lists(&self) -> &Vec<ChatList> {
    &self.chat_lists
  }

  pub fn message_sender_id(&self) -> &Option<MessageSender> {
    &self.message_sender_id
  }

  pub fn block_list(&self) -> &Option<BlockList> {
    &self.block_list
  }

  pub fn has_protected_content(&self) -> bool {
    self.has_protected_content
  }

  pub fn is_translatable(&self) -> bool {
    self.is_translatable
  }

  pub fn is_marked_as_unread(&self) -> bool {
    self.is_marked_as_unread
  }

  pub fn view_as_topics(&self) -> bool {
    self.view_as_topics
  }

  pub fn has_scheduled_messages(&self) -> bool {
    self.has_scheduled_messages
  }

  pub fn has_welcome_messages(&self) -> bool {
    self.has_welcome_messages
  }

  pub fn can_be_deleted_only_for_self(&self) -> bool {
    self.can_be_deleted_only_for_self
  }

  pub fn can_be_deleted_for_all_users(&self) -> bool {
    self.can_be_deleted_for_all_users
  }

  pub fn can_be_reported(&self) -> bool {
    self.can_be_reported
  }

  pub fn default_disable_notification(&self) -> bool {
    self.default_disable_notification
  }

  pub fn unread_count(&self) -> i64 {
    self.unread_count
  }

  pub fn last_read_inbox_message_id(&self) -> i64 {
    self.last_read_inbox_message_id
  }

  pub fn last_read_outbox_message_id(&self) -> i64 {
    self.last_read_outbox_message_id
  }

  pub fn unread_mention_count(&self) -> i64 {
    self.unread_mention_count
  }

  pub fn unread_reaction_count(&self) -> i64 {
    self.unread_reaction_count
  }

  pub fn unread_poll_vote_count(&self) -> i64 {
    self.unread_poll_vote_count
  }

  pub fn notification_settings(&self) -> &ChatNotificationSettings {
    &self.notification_settings
  }

  pub fn available_reactions(&self) -> &ChatAvailableReactions {
    &self.available_reactions
  }

  pub fn message_auto_delete_time(&self) -> i64 {
    self.message_auto_delete_time
  }

  pub fn emoji_status(&self) -> &Option<EmojiStatus> {
    &self.emoji_status
  }

  pub fn background(&self) -> &Option<ChatBackground> {
    &self.background
  }

  pub fn theme(&self) -> &Option<ChatTheme> {
    &self.theme
  }

  pub fn action_bar(&self) -> &Option<ChatActionBar> {
    &self.action_bar
  }

  pub fn business_bot_manage_bar(&self) -> &Option<BusinessBotManageBar> {
    &self.business_bot_manage_bar
  }

  pub fn video_chat(&self) -> &VideoChat {
    &self.video_chat
  }

  pub fn pending_join_requests(&self) -> &Option<ChatJoinRequestsInfo> {
    &self.pending_join_requests
  }

  pub fn reply_markup_message_id(&self) -> i64 {
    self.reply_markup_message_id
  }

  pub fn draft_message(&self) -> &Option<DraftMessage> {
    &self.draft_message
  }

  pub fn client_data(&self) -> &String {
    &self.client_data
  }
}

#[doc(hidden)]
pub struct RTDChatBuilder {
  inner: Chat,
}

impl RTDChatBuilder {
  pub fn build(&self) -> Chat {
    self.inner.clone()
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn type_<T: AsRef<ChatType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

  pub fn photo<T: AsRef<ChatPhotoInfo>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = Some(photo.as_ref().clone());
    self
  }

  pub fn accent_color_id(&mut self, accent_color_id: i64) -> &mut Self {
    self.inner.accent_color_id = accent_color_id;
    self
  }

  pub fn background_custom_emoji_id(&mut self, background_custom_emoji_id: isize) -> &mut Self {
    self.inner.background_custom_emoji_id = background_custom_emoji_id;
    self
  }

  pub fn upgraded_gift_colors<T: AsRef<UpgradedGiftColors>>(
    &mut self,
    upgraded_gift_colors: T,
  ) -> &mut Self {
    self.inner.upgraded_gift_colors = Some(upgraded_gift_colors.as_ref().clone());
    self
  }

  pub fn profile_accent_color_id(&mut self, profile_accent_color_id: i64) -> &mut Self {
    self.inner.profile_accent_color_id = profile_accent_color_id;
    self
  }

  pub fn profile_background_custom_emoji_id(
    &mut self,
    profile_background_custom_emoji_id: isize,
  ) -> &mut Self {
    self.inner.profile_background_custom_emoji_id = profile_background_custom_emoji_id;
    self
  }

  pub fn permissions<T: AsRef<ChatPermissions>>(&mut self, permissions: T) -> &mut Self {
    self.inner.permissions = permissions.as_ref().clone();
    self
  }

  pub fn last_message<T: AsRef<Message>>(&mut self, last_message: T) -> &mut Self {
    self.inner.last_message = Some(last_message.as_ref().clone());
    self
  }

  pub fn positions(&mut self, positions: Vec<ChatPosition>) -> &mut Self {
    self.inner.positions = Some(positions);
    self
  }

  pub fn chat_lists(&mut self, chat_lists: Vec<ChatList>) -> &mut Self {
    self.inner.chat_lists = chat_lists;
    self
  }

  pub fn message_sender_id<T: AsRef<MessageSender>>(&mut self, message_sender_id: T) -> &mut Self {
    self.inner.message_sender_id = Some(message_sender_id.as_ref().clone());
    self
  }

  pub fn block_list<T: AsRef<BlockList>>(&mut self, block_list: T) -> &mut Self {
    self.inner.block_list = Some(block_list.as_ref().clone());
    self
  }

  pub fn has_protected_content(&mut self, has_protected_content: bool) -> &mut Self {
    self.inner.has_protected_content = has_protected_content;
    self
  }

  pub fn is_translatable(&mut self, is_translatable: bool) -> &mut Self {
    self.inner.is_translatable = is_translatable;
    self
  }

  pub fn is_marked_as_unread(&mut self, is_marked_as_unread: bool) -> &mut Self {
    self.inner.is_marked_as_unread = is_marked_as_unread;
    self
  }

  pub fn view_as_topics(&mut self, view_as_topics: bool) -> &mut Self {
    self.inner.view_as_topics = view_as_topics;
    self
  }

  pub fn has_scheduled_messages(&mut self, has_scheduled_messages: bool) -> &mut Self {
    self.inner.has_scheduled_messages = has_scheduled_messages;
    self
  }

  pub fn has_welcome_messages(&mut self, has_welcome_messages: bool) -> &mut Self {
    self.inner.has_welcome_messages = has_welcome_messages;
    self
  }

  pub fn can_be_deleted_only_for_self(&mut self, can_be_deleted_only_for_self: bool) -> &mut Self {
    self.inner.can_be_deleted_only_for_self = can_be_deleted_only_for_self;
    self
  }

  pub fn can_be_deleted_for_all_users(&mut self, can_be_deleted_for_all_users: bool) -> &mut Self {
    self.inner.can_be_deleted_for_all_users = can_be_deleted_for_all_users;
    self
  }

  pub fn can_be_reported(&mut self, can_be_reported: bool) -> &mut Self {
    self.inner.can_be_reported = can_be_reported;
    self
  }

  pub fn default_disable_notification(&mut self, default_disable_notification: bool) -> &mut Self {
    self.inner.default_disable_notification = default_disable_notification;
    self
  }

  pub fn unread_count(&mut self, unread_count: i64) -> &mut Self {
    self.inner.unread_count = unread_count;
    self
  }

  pub fn last_read_inbox_message_id(&mut self, last_read_inbox_message_id: i64) -> &mut Self {
    self.inner.last_read_inbox_message_id = last_read_inbox_message_id;
    self
  }

  pub fn last_read_outbox_message_id(&mut self, last_read_outbox_message_id: i64) -> &mut Self {
    self.inner.last_read_outbox_message_id = last_read_outbox_message_id;
    self
  }

  pub fn unread_mention_count(&mut self, unread_mention_count: i64) -> &mut Self {
    self.inner.unread_mention_count = unread_mention_count;
    self
  }

  pub fn unread_reaction_count(&mut self, unread_reaction_count: i64) -> &mut Self {
    self.inner.unread_reaction_count = unread_reaction_count;
    self
  }

  pub fn unread_poll_vote_count(&mut self, unread_poll_vote_count: i64) -> &mut Self {
    self.inner.unread_poll_vote_count = unread_poll_vote_count;
    self
  }

  pub fn notification_settings<T: AsRef<ChatNotificationSettings>>(
    &mut self,
    notification_settings: T,
  ) -> &mut Self {
    self.inner.notification_settings = notification_settings.as_ref().clone();
    self
  }

  pub fn available_reactions<T: AsRef<ChatAvailableReactions>>(
    &mut self,
    available_reactions: T,
  ) -> &mut Self {
    self.inner.available_reactions = available_reactions.as_ref().clone();
    self
  }

  pub fn message_auto_delete_time(&mut self, message_auto_delete_time: i64) -> &mut Self {
    self.inner.message_auto_delete_time = message_auto_delete_time;
    self
  }

  pub fn emoji_status<T: AsRef<EmojiStatus>>(&mut self, emoji_status: T) -> &mut Self {
    self.inner.emoji_status = Some(emoji_status.as_ref().clone());
    self
  }

  pub fn background<T: AsRef<ChatBackground>>(&mut self, background: T) -> &mut Self {
    self.inner.background = Some(background.as_ref().clone());
    self
  }

  pub fn theme<T: AsRef<ChatTheme>>(&mut self, theme: T) -> &mut Self {
    self.inner.theme = Some(theme.as_ref().clone());
    self
  }

  pub fn action_bar<T: AsRef<ChatActionBar>>(&mut self, action_bar: T) -> &mut Self {
    self.inner.action_bar = Some(action_bar.as_ref().clone());
    self
  }

  pub fn business_bot_manage_bar<T: AsRef<BusinessBotManageBar>>(
    &mut self,
    business_bot_manage_bar: T,
  ) -> &mut Self {
    self.inner.business_bot_manage_bar = Some(business_bot_manage_bar.as_ref().clone());
    self
  }

  pub fn video_chat<T: AsRef<VideoChat>>(&mut self, video_chat: T) -> &mut Self {
    self.inner.video_chat = video_chat.as_ref().clone();
    self
  }

  pub fn pending_join_requests<T: AsRef<ChatJoinRequestsInfo>>(
    &mut self,
    pending_join_requests: T,
  ) -> &mut Self {
    self.inner.pending_join_requests = Some(pending_join_requests.as_ref().clone());
    self
  }

  pub fn reply_markup_message_id(&mut self, reply_markup_message_id: i64) -> &mut Self {
    self.inner.reply_markup_message_id = reply_markup_message_id;
    self
  }

  pub fn draft_message<T: AsRef<DraftMessage>>(&mut self, draft_message: T) -> &mut Self {
    self.inner.draft_message = Some(draft_message.as_ref().clone());
    self
  }

  pub fn client_data<T: AsRef<str>>(&mut self, client_data: T) -> &mut Self {
    self.inner.client_data = client_data.as_ref().to_string();
    self
  }
}

impl AsRef<Chat> for Chat {
  fn as_ref(&self) -> &Chat {
    self
  }
}

impl AsRef<Chat> for RTDChatBuilder {
  fn as_ref(&self) -> &Chat {
    &self.inner
  }
}
