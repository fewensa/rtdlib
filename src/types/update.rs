
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Contains notifications about data changes
pub trait TDUpdate: Debug + RObject {}

/// Contains notifications about data changes
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Update {
  #[doc(hidden)] _Default(()),
  /// Does nothing and ensures that the Update object is used; for testing only. This is an offline method. Can be called before authorization
  TestUseUpdate(TestUseUpdate),
  /// Contains active notifications that was shown on previous application launches. This update is sent only if the message database is used. In that case it comes once before any updateNotification and updateNotificationGroup update
  ActiveNotifications(UpdateActiveNotifications),
  /// Some animated emoji message was clicked and a big animated sticker must be played if the message is visible on the screen. chatActionWatchingAnimations with the text of the message needs to be sent if the sticker is played
  AnimatedEmojiMessageClicked(UpdateAnimatedEmojiMessageClicked),
  /// The parameters of animation search through GetOption("animation_search_bot_username") bot has changed
  AnimationSearchParameters(UpdateAnimationSearchParameters),
  /// The user authorization state has changed
  AuthorizationState(UpdateAuthorizationState),
  /// Some data of a basic group has changed. This update is guaranteed to come before the basic group identifier is returned to the application
  BasicGroup(UpdateBasicGroup),
  /// Some data in basicGroupFullInfo has been changed
  BasicGroupFullInfo(UpdateBasicGroupFullInfo),
  /// New call was created or information about a call was updated
  Call(UpdateCall),
  /// A message sender activity in the chat has changed
  ChatAction(UpdateChatAction),
  /// The chat action bar was changed
  ChatActionBar(UpdateChatActionBar),
  /// The value of the default disable_notification parameter, used when a message is sent to the chat, was changed
  ChatDefaultDisableNotification(UpdateChatDefaultDisableNotification),
  /// The default message sender that is chosen to send messages in a chat has changed
  ChatDefaultMessageSenderId(UpdateChatDefaultMessageSenderId),
  /// A chat draft has changed. Be aware that the update may come in the currently opened chat but with old content of the draft. If the user has changed the content of the draft, this update mustn't be applied
  ChatDraftMessage(UpdateChatDraftMessage),
  /// The list of chat filters or a chat filter has changed
  ChatFilters(UpdateChatFilters),
  /// A chat content was allowed or restricted for saving
  ChatHasProtectedContent(UpdateChatHasProtectedContent),
  /// A chat's has_scheduled_messages field has changed
  ChatHasScheduledMessages(UpdateChatHasScheduledMessages),
  /// A chat was blocked or unblocked
  ChatIsBlocked(UpdateChatIsBlocked),
  /// A chat was marked as unread or was read
  ChatIsMarkedAsUnread(UpdateChatIsMarkedAsUnread),
  /// The last message of a chat was changed. If last_message is null, then the last message in the chat became unknown. Some new unknown messages might be added to the chat in this case
  ChatLastMessage(UpdateChatLastMessage),
  /// User rights changed in a chat; for bots only
  ChatMember(UpdateChatMember),
  /// The message Time To Live setting for a chat was changed
  ChatMessageTtlSetting(UpdateChatMessageTtlSetting),
  /// Notification settings for a chat were changed
  ChatNotificationSettings(UpdateChatNotificationSettings),
  /// The number of online group members has changed. This update with non-zero count is sent only for currently opened chats. There is no guarantee that it will be sent just after the count has changed
  ChatOnlineMemberCount(UpdateChatOnlineMemberCount),
  /// The chat pending join requests were changed
  ChatPendingJoinRequests(UpdateChatPendingJoinRequests),
  /// Chat permissions was changed
  ChatPermissions(UpdateChatPermissions),
  /// A chat photo was changed
  ChatPhoto(UpdateChatPhoto),
  /// The position of a chat in a chat list has changed. Instead of this update updateChatLastMessage or updateChatDraftMessage might be sent
  ChatPosition(UpdateChatPosition),
  /// Incoming messages were read or the number of unread messages has been changed
  ChatReadInbox(UpdateChatReadInbox),
  /// Outgoing messages were read
  ChatReadOutbox(UpdateChatReadOutbox),
  /// The default chat reply markup was changed. Can occur because new messages with reply markup were received or because an old reply markup was hidden by the user
  ChatReplyMarkup(UpdateChatReplyMarkup),
  /// The chat theme was changed
  ChatTheme(UpdateChatTheme),
  /// The list of available chat themes has changed
  ChatThemes(UpdateChatThemes),
  /// The title of a chat was changed
  ChatTitle(UpdateChatTitle),
  /// The chat unread_mention_count has changed
  ChatUnreadMentionCount(UpdateChatUnreadMentionCount),
  /// A chat video chat state has changed
  ChatVideoChat(UpdateChatVideoChat),
  /// The connection state has changed. This update must be used only to show a human-readable description of the connection state
  ConnectionState(UpdateConnectionState),
  /// Some messages were deleted
  DeleteMessages(UpdateDeleteMessages),
  /// The list of supported dice emojis has changed
  DiceEmojis(UpdateDiceEmojis),
  /// The list of favorite stickers was updated
  FavoriteStickers(UpdateFavoriteStickers),
  /// Information about a file was updated
  File(UpdateFile),
  /// The file generation process needs to be started by the application
  FileGenerationStart(UpdateFileGenerationStart),
  /// File generation is no longer needed
  FileGenerationStop(UpdateFileGenerationStop),
  /// Information about a group call was updated
  GroupCall(UpdateGroupCall),
  /// Information about a group call participant was changed. The updates are sent only after the group call is received through getGroupCall and only if the call is joined or being joined
  GroupCallParticipant(UpdateGroupCallParticipant),
  /// Describes whether there are some pending notification updates. Can be used to prevent application from killing, while there are some pending notifications
  HavePendingNotifications(UpdateHavePendingNotifications),
  /// The list of installed sticker sets was updated
  InstalledStickerSets(UpdateInstalledStickerSets),
  /// Some language pack strings have been updated
  LanguagePackStrings(UpdateLanguagePackStrings),
  /// The message content has changed
  MessageContent(UpdateMessageContent),
  /// The message content was opened. Updates voice note messages to "listened", video note messages to "viewed" and starts the TTL timer for self-destructing messages
  MessageContentOpened(UpdateMessageContentOpened),
  /// A message was edited. Changes in the message content will come in a separate updateMessageContent
  MessageEdited(UpdateMessageEdited),
  /// The information about interactions with a message has changed
  MessageInteractionInfo(UpdateMessageInteractionInfo),
  /// The message pinned state was changed
  MessageIsPinned(UpdateMessageIsPinned),
  /// A message with a live location was viewed. When the update is received, the application is supposed to update the live location
  MessageLiveLocationViewed(UpdateMessageLiveLocationViewed),
  /// A message with an unread mention was read
  MessageMentionRead(UpdateMessageMentionRead),
  /// A request to send a message has reached the Telegram server. This doesn't mean that the message will be sent successfully or even that the send message request will be processed. This update will be sent only if the option "use_quick_ack" is set to true. This update may be sent multiple times for the same message
  MessageSendAcknowledged(UpdateMessageSendAcknowledged),
  /// A message failed to send. Be aware that some messages being sent can be irrecoverably deleted, in which case updateDeleteMessages will be received instead of this update
  MessageSendFailed(UpdateMessageSendFailed),
  /// A message has been successfully sent
  MessageSendSucceeded(UpdateMessageSendSucceeded),
  /// New call signaling data arrived
  NewCallSignalingData(UpdateNewCallSignalingData),
  /// A new incoming callback query; for bots only
  NewCallbackQuery(UpdateNewCallbackQuery),
  /// A new chat has been loaded/created. This update is guaranteed to come before the chat identifier is returned to the application. The chat field changes will be reported through separate updates
  NewChat(UpdateNewChat),
  /// A user sent a join request to a chat; for bots only
  NewChatJoinRequest(UpdateNewChatJoinRequest),
  /// The user has chosen a result of an inline query; for bots only
  NewChosenInlineResult(UpdateNewChosenInlineResult),
  /// A new incoming event; for bots only
  NewCustomEvent(UpdateNewCustomEvent),
  /// A new incoming query; for bots only
  NewCustomQuery(UpdateNewCustomQuery),
  /// A new incoming callback query from a message sent via a bot; for bots only
  NewInlineCallbackQuery(UpdateNewInlineCallbackQuery),
  /// A new incoming inline query; for bots only
  NewInlineQuery(UpdateNewInlineQuery),
  /// A new message was received; can also be an outgoing message
  NewMessage(UpdateNewMessage),
  /// A new incoming pre-checkout query; for bots only. Contains full information about a checkout
  NewPreCheckoutQuery(UpdateNewPreCheckoutQuery),
  /// A new incoming shipping query; for bots only. Only for invoices with flexible price
  NewShippingQuery(UpdateNewShippingQuery),
  /// A notification was changed
  Notification(UpdateNotification),
  /// A list of active notifications in a notification group has changed
  NotificationGroup(UpdateNotificationGroup),
  /// An option changed its value
  Option(UpdateOption),
  /// A poll was updated; for bots only
  Poll(UpdatePoll),
  /// A user changed the answer to a poll; for bots only
  PollAnswer(UpdatePollAnswer),
  /// The list of recently used stickers was updated
  RecentStickers(UpdateRecentStickers),
  /// The list of saved animations was updated
  SavedAnimations(UpdateSavedAnimations),
  /// Notification settings for some type of chats were updated
  ScopeNotificationSettings(UpdateScopeNotificationSettings),
  /// Some data of a secret chat has changed. This update is guaranteed to come before the secret chat identifier is returned to the application
  SecretChat(UpdateSecretChat),
  /// The selected background has changed
  SelectedBackground(UpdateSelectedBackground),
  /// A service notification from the server was received. Upon receiving this the application must show a popup with the content of the notification
  ServiceNotification(UpdateServiceNotification),
  /// A sticker set has changed
  StickerSet(UpdateStickerSet),
  /// The list of suggested to the user actions has changed
  SuggestedActions(UpdateSuggestedActions),
  /// Some data of a supergroup or a channel has changed. This update is guaranteed to come before the supergroup identifier is returned to the application
  Supergroup(UpdateSupergroup),
  /// Some data in supergroupFullInfo has been changed
  SupergroupFullInfo(UpdateSupergroupFullInfo),
  /// New terms of service must be accepted by the user. If the terms of service are declined, then the deleteAccount method must be called with the reason "Decline ToS update"
  TermsOfService(UpdateTermsOfService),
  /// The list of trending sticker sets was updated or some of them were viewed
  TrendingStickerSets(UpdateTrendingStickerSets),
  /// Number of unread chats, i.e. with unread messages or marked as unread, has changed. This update is sent only if the message database is used
  UnreadChatCount(UpdateUnreadChatCount),
  /// Number of unread messages in a chat list has changed. This update is sent only if the message database is used
  UnreadMessageCount(UpdateUnreadMessageCount),
  /// Some data of a user has changed. This update is guaranteed to come before the user identifier is returned to the application
  User(UpdateUser),
  /// Some data in userFullInfo has been changed
  UserFullInfo(UpdateUserFullInfo),
  /// Some privacy setting rules have been changed
  UserPrivacySettingRules(UpdateUserPrivacySettingRules),
  /// The user went online or offline
  UserStatus(UpdateUserStatus),
  /// The list of users nearby has changed. The update is guaranteed to be sent only 60 seconds after a successful searchChatsNearby request
  UsersNearby(UpdateUsersNearby),

}

impl Default for Update {
  fn default() -> Self { Update::_Default(()) }
}

impl<'de> Deserialize<'de> for Update {
  fn deserialize<D>(deserializer: D) -> Result<Update, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      Update,
      (testUseUpdate, TestUseUpdate);
      (updateActiveNotifications, ActiveNotifications);
      (updateAnimatedEmojiMessageClicked, AnimatedEmojiMessageClicked);
      (updateAnimationSearchParameters, AnimationSearchParameters);
      (updateAuthorizationState, AuthorizationState);
      (updateBasicGroup, BasicGroup);
      (updateBasicGroupFullInfo, BasicGroupFullInfo);
      (updateCall, Call);
      (updateChatAction, ChatAction);
      (updateChatActionBar, ChatActionBar);
      (updateChatDefaultDisableNotification, ChatDefaultDisableNotification);
      (updateChatDefaultMessageSenderId, ChatDefaultMessageSenderId);
      (updateChatDraftMessage, ChatDraftMessage);
      (updateChatFilters, ChatFilters);
      (updateChatHasProtectedContent, ChatHasProtectedContent);
      (updateChatHasScheduledMessages, ChatHasScheduledMessages);
      (updateChatIsBlocked, ChatIsBlocked);
      (updateChatIsMarkedAsUnread, ChatIsMarkedAsUnread);
      (updateChatLastMessage, ChatLastMessage);
      (updateChatMember, ChatMember);
      (updateChatMessageTtlSetting, ChatMessageTtlSetting);
      (updateChatNotificationSettings, ChatNotificationSettings);
      (updateChatOnlineMemberCount, ChatOnlineMemberCount);
      (updateChatPendingJoinRequests, ChatPendingJoinRequests);
      (updateChatPermissions, ChatPermissions);
      (updateChatPhoto, ChatPhoto);
      (updateChatPosition, ChatPosition);
      (updateChatReadInbox, ChatReadInbox);
      (updateChatReadOutbox, ChatReadOutbox);
      (updateChatReplyMarkup, ChatReplyMarkup);
      (updateChatTheme, ChatTheme);
      (updateChatThemes, ChatThemes);
      (updateChatTitle, ChatTitle);
      (updateChatUnreadMentionCount, ChatUnreadMentionCount);
      (updateChatVideoChat, ChatVideoChat);
      (updateConnectionState, ConnectionState);
      (updateDeleteMessages, DeleteMessages);
      (updateDiceEmojis, DiceEmojis);
      (updateFavoriteStickers, FavoriteStickers);
      (updateFile, File);
      (updateFileGenerationStart, FileGenerationStart);
      (updateFileGenerationStop, FileGenerationStop);
      (updateGroupCall, GroupCall);
      (updateGroupCallParticipant, GroupCallParticipant);
      (updateHavePendingNotifications, HavePendingNotifications);
      (updateInstalledStickerSets, InstalledStickerSets);
      (updateLanguagePackStrings, LanguagePackStrings);
      (updateMessageContent, MessageContent);
      (updateMessageContentOpened, MessageContentOpened);
      (updateMessageEdited, MessageEdited);
      (updateMessageInteractionInfo, MessageInteractionInfo);
      (updateMessageIsPinned, MessageIsPinned);
      (updateMessageLiveLocationViewed, MessageLiveLocationViewed);
      (updateMessageMentionRead, MessageMentionRead);
      (updateMessageSendAcknowledged, MessageSendAcknowledged);
      (updateMessageSendFailed, MessageSendFailed);
      (updateMessageSendSucceeded, MessageSendSucceeded);
      (updateNewCallSignalingData, NewCallSignalingData);
      (updateNewCallbackQuery, NewCallbackQuery);
      (updateNewChat, NewChat);
      (updateNewChatJoinRequest, NewChatJoinRequest);
      (updateNewChosenInlineResult, NewChosenInlineResult);
      (updateNewCustomEvent, NewCustomEvent);
      (updateNewCustomQuery, NewCustomQuery);
      (updateNewInlineCallbackQuery, NewInlineCallbackQuery);
      (updateNewInlineQuery, NewInlineQuery);
      (updateNewMessage, NewMessage);
      (updateNewPreCheckoutQuery, NewPreCheckoutQuery);
      (updateNewShippingQuery, NewShippingQuery);
      (updateNotification, Notification);
      (updateNotificationGroup, NotificationGroup);
      (updateOption, Option);
      (updatePoll, Poll);
      (updatePollAnswer, PollAnswer);
      (updateRecentStickers, RecentStickers);
      (updateSavedAnimations, SavedAnimations);
      (updateScopeNotificationSettings, ScopeNotificationSettings);
      (updateSecretChat, SecretChat);
      (updateSelectedBackground, SelectedBackground);
      (updateServiceNotification, ServiceNotification);
      (updateStickerSet, StickerSet);
      (updateSuggestedActions, SuggestedActions);
      (updateSupergroup, Supergroup);
      (updateSupergroupFullInfo, SupergroupFullInfo);
      (updateTermsOfService, TermsOfService);
      (updateTrendingStickerSets, TrendingStickerSets);
      (updateUnreadChatCount, UnreadChatCount);
      (updateUnreadMessageCount, UnreadMessageCount);
      (updateUser, User);
      (updateUserFullInfo, UserFullInfo);
      (updateUserPrivacySettingRules, UserPrivacySettingRules);
      (updateUserStatus, UserStatus);
      (updateUsersNearby, UsersNearby);

    )(deserializer)
  }
}

impl RObject for Update {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      Update::TestUseUpdate(t) => t.td_name(),
      Update::ActiveNotifications(t) => t.td_name(),
      Update::AnimatedEmojiMessageClicked(t) => t.td_name(),
      Update::AnimationSearchParameters(t) => t.td_name(),
      Update::AuthorizationState(t) => t.td_name(),
      Update::BasicGroup(t) => t.td_name(),
      Update::BasicGroupFullInfo(t) => t.td_name(),
      Update::Call(t) => t.td_name(),
      Update::ChatAction(t) => t.td_name(),
      Update::ChatActionBar(t) => t.td_name(),
      Update::ChatDefaultDisableNotification(t) => t.td_name(),
      Update::ChatDefaultMessageSenderId(t) => t.td_name(),
      Update::ChatDraftMessage(t) => t.td_name(),
      Update::ChatFilters(t) => t.td_name(),
      Update::ChatHasProtectedContent(t) => t.td_name(),
      Update::ChatHasScheduledMessages(t) => t.td_name(),
      Update::ChatIsBlocked(t) => t.td_name(),
      Update::ChatIsMarkedAsUnread(t) => t.td_name(),
      Update::ChatLastMessage(t) => t.td_name(),
      Update::ChatMember(t) => t.td_name(),
      Update::ChatMessageTtlSetting(t) => t.td_name(),
      Update::ChatNotificationSettings(t) => t.td_name(),
      Update::ChatOnlineMemberCount(t) => t.td_name(),
      Update::ChatPendingJoinRequests(t) => t.td_name(),
      Update::ChatPermissions(t) => t.td_name(),
      Update::ChatPhoto(t) => t.td_name(),
      Update::ChatPosition(t) => t.td_name(),
      Update::ChatReadInbox(t) => t.td_name(),
      Update::ChatReadOutbox(t) => t.td_name(),
      Update::ChatReplyMarkup(t) => t.td_name(),
      Update::ChatTheme(t) => t.td_name(),
      Update::ChatThemes(t) => t.td_name(),
      Update::ChatTitle(t) => t.td_name(),
      Update::ChatUnreadMentionCount(t) => t.td_name(),
      Update::ChatVideoChat(t) => t.td_name(),
      Update::ConnectionState(t) => t.td_name(),
      Update::DeleteMessages(t) => t.td_name(),
      Update::DiceEmojis(t) => t.td_name(),
      Update::FavoriteStickers(t) => t.td_name(),
      Update::File(t) => t.td_name(),
      Update::FileGenerationStart(t) => t.td_name(),
      Update::FileGenerationStop(t) => t.td_name(),
      Update::GroupCall(t) => t.td_name(),
      Update::GroupCallParticipant(t) => t.td_name(),
      Update::HavePendingNotifications(t) => t.td_name(),
      Update::InstalledStickerSets(t) => t.td_name(),
      Update::LanguagePackStrings(t) => t.td_name(),
      Update::MessageContent(t) => t.td_name(),
      Update::MessageContentOpened(t) => t.td_name(),
      Update::MessageEdited(t) => t.td_name(),
      Update::MessageInteractionInfo(t) => t.td_name(),
      Update::MessageIsPinned(t) => t.td_name(),
      Update::MessageLiveLocationViewed(t) => t.td_name(),
      Update::MessageMentionRead(t) => t.td_name(),
      Update::MessageSendAcknowledged(t) => t.td_name(),
      Update::MessageSendFailed(t) => t.td_name(),
      Update::MessageSendSucceeded(t) => t.td_name(),
      Update::NewCallSignalingData(t) => t.td_name(),
      Update::NewCallbackQuery(t) => t.td_name(),
      Update::NewChat(t) => t.td_name(),
      Update::NewChatJoinRequest(t) => t.td_name(),
      Update::NewChosenInlineResult(t) => t.td_name(),
      Update::NewCustomEvent(t) => t.td_name(),
      Update::NewCustomQuery(t) => t.td_name(),
      Update::NewInlineCallbackQuery(t) => t.td_name(),
      Update::NewInlineQuery(t) => t.td_name(),
      Update::NewMessage(t) => t.td_name(),
      Update::NewPreCheckoutQuery(t) => t.td_name(),
      Update::NewShippingQuery(t) => t.td_name(),
      Update::Notification(t) => t.td_name(),
      Update::NotificationGroup(t) => t.td_name(),
      Update::Option(t) => t.td_name(),
      Update::Poll(t) => t.td_name(),
      Update::PollAnswer(t) => t.td_name(),
      Update::RecentStickers(t) => t.td_name(),
      Update::SavedAnimations(t) => t.td_name(),
      Update::ScopeNotificationSettings(t) => t.td_name(),
      Update::SecretChat(t) => t.td_name(),
      Update::SelectedBackground(t) => t.td_name(),
      Update::ServiceNotification(t) => t.td_name(),
      Update::StickerSet(t) => t.td_name(),
      Update::SuggestedActions(t) => t.td_name(),
      Update::Supergroup(t) => t.td_name(),
      Update::SupergroupFullInfo(t) => t.td_name(),
      Update::TermsOfService(t) => t.td_name(),
      Update::TrendingStickerSets(t) => t.td_name(),
      Update::UnreadChatCount(t) => t.td_name(),
      Update::UnreadMessageCount(t) => t.td_name(),
      Update::User(t) => t.td_name(),
      Update::UserFullInfo(t) => t.td_name(),
      Update::UserPrivacySettingRules(t) => t.td_name(),
      Update::UserStatus(t) => t.td_name(),
      Update::UsersNearby(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      Update::TestUseUpdate(t) => t.extra(),
      Update::ActiveNotifications(t) => t.extra(),
      Update::AnimatedEmojiMessageClicked(t) => t.extra(),
      Update::AnimationSearchParameters(t) => t.extra(),
      Update::AuthorizationState(t) => t.extra(),
      Update::BasicGroup(t) => t.extra(),
      Update::BasicGroupFullInfo(t) => t.extra(),
      Update::Call(t) => t.extra(),
      Update::ChatAction(t) => t.extra(),
      Update::ChatActionBar(t) => t.extra(),
      Update::ChatDefaultDisableNotification(t) => t.extra(),
      Update::ChatDefaultMessageSenderId(t) => t.extra(),
      Update::ChatDraftMessage(t) => t.extra(),
      Update::ChatFilters(t) => t.extra(),
      Update::ChatHasProtectedContent(t) => t.extra(),
      Update::ChatHasScheduledMessages(t) => t.extra(),
      Update::ChatIsBlocked(t) => t.extra(),
      Update::ChatIsMarkedAsUnread(t) => t.extra(),
      Update::ChatLastMessage(t) => t.extra(),
      Update::ChatMember(t) => t.extra(),
      Update::ChatMessageTtlSetting(t) => t.extra(),
      Update::ChatNotificationSettings(t) => t.extra(),
      Update::ChatOnlineMemberCount(t) => t.extra(),
      Update::ChatPendingJoinRequests(t) => t.extra(),
      Update::ChatPermissions(t) => t.extra(),
      Update::ChatPhoto(t) => t.extra(),
      Update::ChatPosition(t) => t.extra(),
      Update::ChatReadInbox(t) => t.extra(),
      Update::ChatReadOutbox(t) => t.extra(),
      Update::ChatReplyMarkup(t) => t.extra(),
      Update::ChatTheme(t) => t.extra(),
      Update::ChatThemes(t) => t.extra(),
      Update::ChatTitle(t) => t.extra(),
      Update::ChatUnreadMentionCount(t) => t.extra(),
      Update::ChatVideoChat(t) => t.extra(),
      Update::ConnectionState(t) => t.extra(),
      Update::DeleteMessages(t) => t.extra(),
      Update::DiceEmojis(t) => t.extra(),
      Update::FavoriteStickers(t) => t.extra(),
      Update::File(t) => t.extra(),
      Update::FileGenerationStart(t) => t.extra(),
      Update::FileGenerationStop(t) => t.extra(),
      Update::GroupCall(t) => t.extra(),
      Update::GroupCallParticipant(t) => t.extra(),
      Update::HavePendingNotifications(t) => t.extra(),
      Update::InstalledStickerSets(t) => t.extra(),
      Update::LanguagePackStrings(t) => t.extra(),
      Update::MessageContent(t) => t.extra(),
      Update::MessageContentOpened(t) => t.extra(),
      Update::MessageEdited(t) => t.extra(),
      Update::MessageInteractionInfo(t) => t.extra(),
      Update::MessageIsPinned(t) => t.extra(),
      Update::MessageLiveLocationViewed(t) => t.extra(),
      Update::MessageMentionRead(t) => t.extra(),
      Update::MessageSendAcknowledged(t) => t.extra(),
      Update::MessageSendFailed(t) => t.extra(),
      Update::MessageSendSucceeded(t) => t.extra(),
      Update::NewCallSignalingData(t) => t.extra(),
      Update::NewCallbackQuery(t) => t.extra(),
      Update::NewChat(t) => t.extra(),
      Update::NewChatJoinRequest(t) => t.extra(),
      Update::NewChosenInlineResult(t) => t.extra(),
      Update::NewCustomEvent(t) => t.extra(),
      Update::NewCustomQuery(t) => t.extra(),
      Update::NewInlineCallbackQuery(t) => t.extra(),
      Update::NewInlineQuery(t) => t.extra(),
      Update::NewMessage(t) => t.extra(),
      Update::NewPreCheckoutQuery(t) => t.extra(),
      Update::NewShippingQuery(t) => t.extra(),
      Update::Notification(t) => t.extra(),
      Update::NotificationGroup(t) => t.extra(),
      Update::Option(t) => t.extra(),
      Update::Poll(t) => t.extra(),
      Update::PollAnswer(t) => t.extra(),
      Update::RecentStickers(t) => t.extra(),
      Update::SavedAnimations(t) => t.extra(),
      Update::ScopeNotificationSettings(t) => t.extra(),
      Update::SecretChat(t) => t.extra(),
      Update::SelectedBackground(t) => t.extra(),
      Update::ServiceNotification(t) => t.extra(),
      Update::StickerSet(t) => t.extra(),
      Update::SuggestedActions(t) => t.extra(),
      Update::Supergroup(t) => t.extra(),
      Update::SupergroupFullInfo(t) => t.extra(),
      Update::TermsOfService(t) => t.extra(),
      Update::TrendingStickerSets(t) => t.extra(),
      Update::UnreadChatCount(t) => t.extra(),
      Update::UnreadMessageCount(t) => t.extra(),
      Update::User(t) => t.extra(),
      Update::UserFullInfo(t) => t.extra(),
      Update::UserPrivacySettingRules(t) => t.extra(),
      Update::UserStatus(t) => t.extra(),
      Update::UsersNearby(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl Update {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let Update::_Default(_) = self { true } else { false } }

  pub fn is_test_use_update(&self) -> bool { if let Update::TestUseUpdate(_) = self { true } else { false } }
  pub fn is_active_notifications(&self) -> bool { if let Update::ActiveNotifications(_) = self { true } else { false } }
  pub fn is_animated_emoji_message_clicked(&self) -> bool { if let Update::AnimatedEmojiMessageClicked(_) = self { true } else { false } }
  pub fn is_animation_search_parameters(&self) -> bool { if let Update::AnimationSearchParameters(_) = self { true } else { false } }
  pub fn is_authorization_state(&self) -> bool { if let Update::AuthorizationState(_) = self { true } else { false } }
  pub fn is_basic_group(&self) -> bool { if let Update::BasicGroup(_) = self { true } else { false } }
  pub fn is_basic_group_full_info(&self) -> bool { if let Update::BasicGroupFullInfo(_) = self { true } else { false } }
  pub fn is_call(&self) -> bool { if let Update::Call(_) = self { true } else { false } }
  pub fn is_chat_action(&self) -> bool { if let Update::ChatAction(_) = self { true } else { false } }
  pub fn is_chat_action_bar(&self) -> bool { if let Update::ChatActionBar(_) = self { true } else { false } }
  pub fn is_chat_default_disable_notification(&self) -> bool { if let Update::ChatDefaultDisableNotification(_) = self { true } else { false } }
  pub fn is_chat_default_message_sender_id(&self) -> bool { if let Update::ChatDefaultMessageSenderId(_) = self { true } else { false } }
  pub fn is_chat_draft_message(&self) -> bool { if let Update::ChatDraftMessage(_) = self { true } else { false } }
  pub fn is_chat_filters(&self) -> bool { if let Update::ChatFilters(_) = self { true } else { false } }
  pub fn is_chat_has_protected_content(&self) -> bool { if let Update::ChatHasProtectedContent(_) = self { true } else { false } }
  pub fn is_chat_has_scheduled_messages(&self) -> bool { if let Update::ChatHasScheduledMessages(_) = self { true } else { false } }
  pub fn is_chat_is_blocked(&self) -> bool { if let Update::ChatIsBlocked(_) = self { true } else { false } }
  pub fn is_chat_is_marked_as_unread(&self) -> bool { if let Update::ChatIsMarkedAsUnread(_) = self { true } else { false } }
  pub fn is_chat_last_message(&self) -> bool { if let Update::ChatLastMessage(_) = self { true } else { false } }
  pub fn is_chat_member(&self) -> bool { if let Update::ChatMember(_) = self { true } else { false } }
  pub fn is_chat_message_ttl_setting(&self) -> bool { if let Update::ChatMessageTtlSetting(_) = self { true } else { false } }
  pub fn is_chat_notification_settings(&self) -> bool { if let Update::ChatNotificationSettings(_) = self { true } else { false } }
  pub fn is_chat_online_member_count(&self) -> bool { if let Update::ChatOnlineMemberCount(_) = self { true } else { false } }
  pub fn is_chat_pending_join_requests(&self) -> bool { if let Update::ChatPendingJoinRequests(_) = self { true } else { false } }
  pub fn is_chat_permissions(&self) -> bool { if let Update::ChatPermissions(_) = self { true } else { false } }
  pub fn is_chat_photo(&self) -> bool { if let Update::ChatPhoto(_) = self { true } else { false } }
  pub fn is_chat_position(&self) -> bool { if let Update::ChatPosition(_) = self { true } else { false } }
  pub fn is_chat_read_inbox(&self) -> bool { if let Update::ChatReadInbox(_) = self { true } else { false } }
  pub fn is_chat_read_outbox(&self) -> bool { if let Update::ChatReadOutbox(_) = self { true } else { false } }
  pub fn is_chat_reply_markup(&self) -> bool { if let Update::ChatReplyMarkup(_) = self { true } else { false } }
  pub fn is_chat_theme(&self) -> bool { if let Update::ChatTheme(_) = self { true } else { false } }
  pub fn is_chat_themes(&self) -> bool { if let Update::ChatThemes(_) = self { true } else { false } }
  pub fn is_chat_title(&self) -> bool { if let Update::ChatTitle(_) = self { true } else { false } }
  pub fn is_chat_unread_mention_count(&self) -> bool { if let Update::ChatUnreadMentionCount(_) = self { true } else { false } }
  pub fn is_chat_video_chat(&self) -> bool { if let Update::ChatVideoChat(_) = self { true } else { false } }
  pub fn is_connection_state(&self) -> bool { if let Update::ConnectionState(_) = self { true } else { false } }
  pub fn is_delete_messages(&self) -> bool { if let Update::DeleteMessages(_) = self { true } else { false } }
  pub fn is_dice_emojis(&self) -> bool { if let Update::DiceEmojis(_) = self { true } else { false } }
  pub fn is_favorite_stickers(&self) -> bool { if let Update::FavoriteStickers(_) = self { true } else { false } }
  pub fn is_file(&self) -> bool { if let Update::File(_) = self { true } else { false } }
  pub fn is_file_generation_start(&self) -> bool { if let Update::FileGenerationStart(_) = self { true } else { false } }
  pub fn is_file_generation_stop(&self) -> bool { if let Update::FileGenerationStop(_) = self { true } else { false } }
  pub fn is_group_call(&self) -> bool { if let Update::GroupCall(_) = self { true } else { false } }
  pub fn is_group_call_participant(&self) -> bool { if let Update::GroupCallParticipant(_) = self { true } else { false } }
  pub fn is_have_pending_notifications(&self) -> bool { if let Update::HavePendingNotifications(_) = self { true } else { false } }
  pub fn is_installed_sticker_sets(&self) -> bool { if let Update::InstalledStickerSets(_) = self { true } else { false } }
  pub fn is_language_pack_strings(&self) -> bool { if let Update::LanguagePackStrings(_) = self { true } else { false } }
  pub fn is_message_content(&self) -> bool { if let Update::MessageContent(_) = self { true } else { false } }
  pub fn is_message_content_opened(&self) -> bool { if let Update::MessageContentOpened(_) = self { true } else { false } }
  pub fn is_message_edited(&self) -> bool { if let Update::MessageEdited(_) = self { true } else { false } }
  pub fn is_message_interaction_info(&self) -> bool { if let Update::MessageInteractionInfo(_) = self { true } else { false } }
  pub fn is_message_is_pinned(&self) -> bool { if let Update::MessageIsPinned(_) = self { true } else { false } }
  pub fn is_message_live_location_viewed(&self) -> bool { if let Update::MessageLiveLocationViewed(_) = self { true } else { false } }
  pub fn is_message_mention_read(&self) -> bool { if let Update::MessageMentionRead(_) = self { true } else { false } }
  pub fn is_message_send_acknowledged(&self) -> bool { if let Update::MessageSendAcknowledged(_) = self { true } else { false } }
  pub fn is_message_send_failed(&self) -> bool { if let Update::MessageSendFailed(_) = self { true } else { false } }
  pub fn is_message_send_succeeded(&self) -> bool { if let Update::MessageSendSucceeded(_) = self { true } else { false } }
  pub fn is_new_call_signaling_data(&self) -> bool { if let Update::NewCallSignalingData(_) = self { true } else { false } }
  pub fn is_new_callback_query(&self) -> bool { if let Update::NewCallbackQuery(_) = self { true } else { false } }
  pub fn is_new_chat(&self) -> bool { if let Update::NewChat(_) = self { true } else { false } }
  pub fn is_new_chat_join_request(&self) -> bool { if let Update::NewChatJoinRequest(_) = self { true } else { false } }
  pub fn is_new_chosen_inline_result(&self) -> bool { if let Update::NewChosenInlineResult(_) = self { true } else { false } }
  pub fn is_new_custom_event(&self) -> bool { if let Update::NewCustomEvent(_) = self { true } else { false } }
  pub fn is_new_custom_query(&self) -> bool { if let Update::NewCustomQuery(_) = self { true } else { false } }
  pub fn is_new_inline_callback_query(&self) -> bool { if let Update::NewInlineCallbackQuery(_) = self { true } else { false } }
  pub fn is_new_inline_query(&self) -> bool { if let Update::NewInlineQuery(_) = self { true } else { false } }
  pub fn is_new_message(&self) -> bool { if let Update::NewMessage(_) = self { true } else { false } }
  pub fn is_new_pre_checkout_query(&self) -> bool { if let Update::NewPreCheckoutQuery(_) = self { true } else { false } }
  pub fn is_new_shipping_query(&self) -> bool { if let Update::NewShippingQuery(_) = self { true } else { false } }
  pub fn is_notification(&self) -> bool { if let Update::Notification(_) = self { true } else { false } }
  pub fn is_notification_group(&self) -> bool { if let Update::NotificationGroup(_) = self { true } else { false } }
  pub fn is_option(&self) -> bool { if let Update::Option(_) = self { true } else { false } }
  pub fn is_poll(&self) -> bool { if let Update::Poll(_) = self { true } else { false } }
  pub fn is_poll_answer(&self) -> bool { if let Update::PollAnswer(_) = self { true } else { false } }
  pub fn is_recent_stickers(&self) -> bool { if let Update::RecentStickers(_) = self { true } else { false } }
  pub fn is_saved_animations(&self) -> bool { if let Update::SavedAnimations(_) = self { true } else { false } }
  pub fn is_scope_notification_settings(&self) -> bool { if let Update::ScopeNotificationSettings(_) = self { true } else { false } }
  pub fn is_secret_chat(&self) -> bool { if let Update::SecretChat(_) = self { true } else { false } }
  pub fn is_selected_background(&self) -> bool { if let Update::SelectedBackground(_) = self { true } else { false } }
  pub fn is_service_notification(&self) -> bool { if let Update::ServiceNotification(_) = self { true } else { false } }
  pub fn is_sticker_set(&self) -> bool { if let Update::StickerSet(_) = self { true } else { false } }
  pub fn is_suggested_actions(&self) -> bool { if let Update::SuggestedActions(_) = self { true } else { false } }
  pub fn is_supergroup(&self) -> bool { if let Update::Supergroup(_) = self { true } else { false } }
  pub fn is_supergroup_full_info(&self) -> bool { if let Update::SupergroupFullInfo(_) = self { true } else { false } }
  pub fn is_terms_of_service(&self) -> bool { if let Update::TermsOfService(_) = self { true } else { false } }
  pub fn is_trending_sticker_sets(&self) -> bool { if let Update::TrendingStickerSets(_) = self { true } else { false } }
  pub fn is_unread_chat_count(&self) -> bool { if let Update::UnreadChatCount(_) = self { true } else { false } }
  pub fn is_unread_message_count(&self) -> bool { if let Update::UnreadMessageCount(_) = self { true } else { false } }
  pub fn is_user(&self) -> bool { if let Update::User(_) = self { true } else { false } }
  pub fn is_user_full_info(&self) -> bool { if let Update::UserFullInfo(_) = self { true } else { false } }
  pub fn is_user_privacy_setting_rules(&self) -> bool { if let Update::UserPrivacySettingRules(_) = self { true } else { false } }
  pub fn is_user_status(&self) -> bool { if let Update::UserStatus(_) = self { true } else { false } }
  pub fn is_users_nearby(&self) -> bool { if let Update::UsersNearby(_) = self { true } else { false } }

  pub fn on_test_use_update<F: FnOnce(&TestUseUpdate)>(&self, fnc: F) -> &Self { if let Update::TestUseUpdate(t) = self { fnc(t) }; self }
  pub fn on_active_notifications<F: FnOnce(&UpdateActiveNotifications)>(&self, fnc: F) -> &Self { if let Update::ActiveNotifications(t) = self { fnc(t) }; self }
  pub fn on_animated_emoji_message_clicked<F: FnOnce(&UpdateAnimatedEmojiMessageClicked)>(&self, fnc: F) -> &Self { if let Update::AnimatedEmojiMessageClicked(t) = self { fnc(t) }; self }
  pub fn on_animation_search_parameters<F: FnOnce(&UpdateAnimationSearchParameters)>(&self, fnc: F) -> &Self { if let Update::AnimationSearchParameters(t) = self { fnc(t) }; self }
  pub fn on_authorization_state<F: FnOnce(&UpdateAuthorizationState)>(&self, fnc: F) -> &Self { if let Update::AuthorizationState(t) = self { fnc(t) }; self }
  pub fn on_basic_group<F: FnOnce(&UpdateBasicGroup)>(&self, fnc: F) -> &Self { if let Update::BasicGroup(t) = self { fnc(t) }; self }
  pub fn on_basic_group_full_info<F: FnOnce(&UpdateBasicGroupFullInfo)>(&self, fnc: F) -> &Self { if let Update::BasicGroupFullInfo(t) = self { fnc(t) }; self }
  pub fn on_call<F: FnOnce(&UpdateCall)>(&self, fnc: F) -> &Self { if let Update::Call(t) = self { fnc(t) }; self }
  pub fn on_chat_action<F: FnOnce(&UpdateChatAction)>(&self, fnc: F) -> &Self { if let Update::ChatAction(t) = self { fnc(t) }; self }
  pub fn on_chat_action_bar<F: FnOnce(&UpdateChatActionBar)>(&self, fnc: F) -> &Self { if let Update::ChatActionBar(t) = self { fnc(t) }; self }
  pub fn on_chat_default_disable_notification<F: FnOnce(&UpdateChatDefaultDisableNotification)>(&self, fnc: F) -> &Self { if let Update::ChatDefaultDisableNotification(t) = self { fnc(t) }; self }
  pub fn on_chat_default_message_sender_id<F: FnOnce(&UpdateChatDefaultMessageSenderId)>(&self, fnc: F) -> &Self { if let Update::ChatDefaultMessageSenderId(t) = self { fnc(t) }; self }
  pub fn on_chat_draft_message<F: FnOnce(&UpdateChatDraftMessage)>(&self, fnc: F) -> &Self { if let Update::ChatDraftMessage(t) = self { fnc(t) }; self }
  pub fn on_chat_filters<F: FnOnce(&UpdateChatFilters)>(&self, fnc: F) -> &Self { if let Update::ChatFilters(t) = self { fnc(t) }; self }
  pub fn on_chat_has_protected_content<F: FnOnce(&UpdateChatHasProtectedContent)>(&self, fnc: F) -> &Self { if let Update::ChatHasProtectedContent(t) = self { fnc(t) }; self }
  pub fn on_chat_has_scheduled_messages<F: FnOnce(&UpdateChatHasScheduledMessages)>(&self, fnc: F) -> &Self { if let Update::ChatHasScheduledMessages(t) = self { fnc(t) }; self }
  pub fn on_chat_is_blocked<F: FnOnce(&UpdateChatIsBlocked)>(&self, fnc: F) -> &Self { if let Update::ChatIsBlocked(t) = self { fnc(t) }; self }
  pub fn on_chat_is_marked_as_unread<F: FnOnce(&UpdateChatIsMarkedAsUnread)>(&self, fnc: F) -> &Self { if let Update::ChatIsMarkedAsUnread(t) = self { fnc(t) }; self }
  pub fn on_chat_last_message<F: FnOnce(&UpdateChatLastMessage)>(&self, fnc: F) -> &Self { if let Update::ChatLastMessage(t) = self { fnc(t) }; self }
  pub fn on_chat_member<F: FnOnce(&UpdateChatMember)>(&self, fnc: F) -> &Self { if let Update::ChatMember(t) = self { fnc(t) }; self }
  pub fn on_chat_message_ttl_setting<F: FnOnce(&UpdateChatMessageTtlSetting)>(&self, fnc: F) -> &Self { if let Update::ChatMessageTtlSetting(t) = self { fnc(t) }; self }
  pub fn on_chat_notification_settings<F: FnOnce(&UpdateChatNotificationSettings)>(&self, fnc: F) -> &Self { if let Update::ChatNotificationSettings(t) = self { fnc(t) }; self }
  pub fn on_chat_online_member_count<F: FnOnce(&UpdateChatOnlineMemberCount)>(&self, fnc: F) -> &Self { if let Update::ChatOnlineMemberCount(t) = self { fnc(t) }; self }
  pub fn on_chat_pending_join_requests<F: FnOnce(&UpdateChatPendingJoinRequests)>(&self, fnc: F) -> &Self { if let Update::ChatPendingJoinRequests(t) = self { fnc(t) }; self }
  pub fn on_chat_permissions<F: FnOnce(&UpdateChatPermissions)>(&self, fnc: F) -> &Self { if let Update::ChatPermissions(t) = self { fnc(t) }; self }
  pub fn on_chat_photo<F: FnOnce(&UpdateChatPhoto)>(&self, fnc: F) -> &Self { if let Update::ChatPhoto(t) = self { fnc(t) }; self }
  pub fn on_chat_position<F: FnOnce(&UpdateChatPosition)>(&self, fnc: F) -> &Self { if let Update::ChatPosition(t) = self { fnc(t) }; self }
  pub fn on_chat_read_inbox<F: FnOnce(&UpdateChatReadInbox)>(&self, fnc: F) -> &Self { if let Update::ChatReadInbox(t) = self { fnc(t) }; self }
  pub fn on_chat_read_outbox<F: FnOnce(&UpdateChatReadOutbox)>(&self, fnc: F) -> &Self { if let Update::ChatReadOutbox(t) = self { fnc(t) }; self }
  pub fn on_chat_reply_markup<F: FnOnce(&UpdateChatReplyMarkup)>(&self, fnc: F) -> &Self { if let Update::ChatReplyMarkup(t) = self { fnc(t) }; self }
  pub fn on_chat_theme<F: FnOnce(&UpdateChatTheme)>(&self, fnc: F) -> &Self { if let Update::ChatTheme(t) = self { fnc(t) }; self }
  pub fn on_chat_themes<F: FnOnce(&UpdateChatThemes)>(&self, fnc: F) -> &Self { if let Update::ChatThemes(t) = self { fnc(t) }; self }
  pub fn on_chat_title<F: FnOnce(&UpdateChatTitle)>(&self, fnc: F) -> &Self { if let Update::ChatTitle(t) = self { fnc(t) }; self }
  pub fn on_chat_unread_mention_count<F: FnOnce(&UpdateChatUnreadMentionCount)>(&self, fnc: F) -> &Self { if let Update::ChatUnreadMentionCount(t) = self { fnc(t) }; self }
  pub fn on_chat_video_chat<F: FnOnce(&UpdateChatVideoChat)>(&self, fnc: F) -> &Self { if let Update::ChatVideoChat(t) = self { fnc(t) }; self }
  pub fn on_connection_state<F: FnOnce(&UpdateConnectionState)>(&self, fnc: F) -> &Self { if let Update::ConnectionState(t) = self { fnc(t) }; self }
  pub fn on_delete_messages<F: FnOnce(&UpdateDeleteMessages)>(&self, fnc: F) -> &Self { if let Update::DeleteMessages(t) = self { fnc(t) }; self }
  pub fn on_dice_emojis<F: FnOnce(&UpdateDiceEmojis)>(&self, fnc: F) -> &Self { if let Update::DiceEmojis(t) = self { fnc(t) }; self }
  pub fn on_favorite_stickers<F: FnOnce(&UpdateFavoriteStickers)>(&self, fnc: F) -> &Self { if let Update::FavoriteStickers(t) = self { fnc(t) }; self }
  pub fn on_file<F: FnOnce(&UpdateFile)>(&self, fnc: F) -> &Self { if let Update::File(t) = self { fnc(t) }; self }
  pub fn on_file_generation_start<F: FnOnce(&UpdateFileGenerationStart)>(&self, fnc: F) -> &Self { if let Update::FileGenerationStart(t) = self { fnc(t) }; self }
  pub fn on_file_generation_stop<F: FnOnce(&UpdateFileGenerationStop)>(&self, fnc: F) -> &Self { if let Update::FileGenerationStop(t) = self { fnc(t) }; self }
  pub fn on_group_call<F: FnOnce(&UpdateGroupCall)>(&self, fnc: F) -> &Self { if let Update::GroupCall(t) = self { fnc(t) }; self }
  pub fn on_group_call_participant<F: FnOnce(&UpdateGroupCallParticipant)>(&self, fnc: F) -> &Self { if let Update::GroupCallParticipant(t) = self { fnc(t) }; self }
  pub fn on_have_pending_notifications<F: FnOnce(&UpdateHavePendingNotifications)>(&self, fnc: F) -> &Self { if let Update::HavePendingNotifications(t) = self { fnc(t) }; self }
  pub fn on_installed_sticker_sets<F: FnOnce(&UpdateInstalledStickerSets)>(&self, fnc: F) -> &Self { if let Update::InstalledStickerSets(t) = self { fnc(t) }; self }
  pub fn on_language_pack_strings<F: FnOnce(&UpdateLanguagePackStrings)>(&self, fnc: F) -> &Self { if let Update::LanguagePackStrings(t) = self { fnc(t) }; self }
  pub fn on_message_content<F: FnOnce(&UpdateMessageContent)>(&self, fnc: F) -> &Self { if let Update::MessageContent(t) = self { fnc(t) }; self }
  pub fn on_message_content_opened<F: FnOnce(&UpdateMessageContentOpened)>(&self, fnc: F) -> &Self { if let Update::MessageContentOpened(t) = self { fnc(t) }; self }
  pub fn on_message_edited<F: FnOnce(&UpdateMessageEdited)>(&self, fnc: F) -> &Self { if let Update::MessageEdited(t) = self { fnc(t) }; self }
  pub fn on_message_interaction_info<F: FnOnce(&UpdateMessageInteractionInfo)>(&self, fnc: F) -> &Self { if let Update::MessageInteractionInfo(t) = self { fnc(t) }; self }
  pub fn on_message_is_pinned<F: FnOnce(&UpdateMessageIsPinned)>(&self, fnc: F) -> &Self { if let Update::MessageIsPinned(t) = self { fnc(t) }; self }
  pub fn on_message_live_location_viewed<F: FnOnce(&UpdateMessageLiveLocationViewed)>(&self, fnc: F) -> &Self { if let Update::MessageLiveLocationViewed(t) = self { fnc(t) }; self }
  pub fn on_message_mention_read<F: FnOnce(&UpdateMessageMentionRead)>(&self, fnc: F) -> &Self { if let Update::MessageMentionRead(t) = self { fnc(t) }; self }
  pub fn on_message_send_acknowledged<F: FnOnce(&UpdateMessageSendAcknowledged)>(&self, fnc: F) -> &Self { if let Update::MessageSendAcknowledged(t) = self { fnc(t) }; self }
  pub fn on_message_send_failed<F: FnOnce(&UpdateMessageSendFailed)>(&self, fnc: F) -> &Self { if let Update::MessageSendFailed(t) = self { fnc(t) }; self }
  pub fn on_message_send_succeeded<F: FnOnce(&UpdateMessageSendSucceeded)>(&self, fnc: F) -> &Self { if let Update::MessageSendSucceeded(t) = self { fnc(t) }; self }
  pub fn on_new_call_signaling_data<F: FnOnce(&UpdateNewCallSignalingData)>(&self, fnc: F) -> &Self { if let Update::NewCallSignalingData(t) = self { fnc(t) }; self }
  pub fn on_new_callback_query<F: FnOnce(&UpdateNewCallbackQuery)>(&self, fnc: F) -> &Self { if let Update::NewCallbackQuery(t) = self { fnc(t) }; self }
  pub fn on_new_chat<F: FnOnce(&UpdateNewChat)>(&self, fnc: F) -> &Self { if let Update::NewChat(t) = self { fnc(t) }; self }
  pub fn on_new_chat_join_request<F: FnOnce(&UpdateNewChatJoinRequest)>(&self, fnc: F) -> &Self { if let Update::NewChatJoinRequest(t) = self { fnc(t) }; self }
  pub fn on_new_chosen_inline_result<F: FnOnce(&UpdateNewChosenInlineResult)>(&self, fnc: F) -> &Self { if let Update::NewChosenInlineResult(t) = self { fnc(t) }; self }
  pub fn on_new_custom_event<F: FnOnce(&UpdateNewCustomEvent)>(&self, fnc: F) -> &Self { if let Update::NewCustomEvent(t) = self { fnc(t) }; self }
  pub fn on_new_custom_query<F: FnOnce(&UpdateNewCustomQuery)>(&self, fnc: F) -> &Self { if let Update::NewCustomQuery(t) = self { fnc(t) }; self }
  pub fn on_new_inline_callback_query<F: FnOnce(&UpdateNewInlineCallbackQuery)>(&self, fnc: F) -> &Self { if let Update::NewInlineCallbackQuery(t) = self { fnc(t) }; self }
  pub fn on_new_inline_query<F: FnOnce(&UpdateNewInlineQuery)>(&self, fnc: F) -> &Self { if let Update::NewInlineQuery(t) = self { fnc(t) }; self }
  pub fn on_new_message<F: FnOnce(&UpdateNewMessage)>(&self, fnc: F) -> &Self { if let Update::NewMessage(t) = self { fnc(t) }; self }
  pub fn on_new_pre_checkout_query<F: FnOnce(&UpdateNewPreCheckoutQuery)>(&self, fnc: F) -> &Self { if let Update::NewPreCheckoutQuery(t) = self { fnc(t) }; self }
  pub fn on_new_shipping_query<F: FnOnce(&UpdateNewShippingQuery)>(&self, fnc: F) -> &Self { if let Update::NewShippingQuery(t) = self { fnc(t) }; self }
  pub fn on_notification<F: FnOnce(&UpdateNotification)>(&self, fnc: F) -> &Self { if let Update::Notification(t) = self { fnc(t) }; self }
  pub fn on_notification_group<F: FnOnce(&UpdateNotificationGroup)>(&self, fnc: F) -> &Self { if let Update::NotificationGroup(t) = self { fnc(t) }; self }
  pub fn on_option<F: FnOnce(&UpdateOption)>(&self, fnc: F) -> &Self { if let Update::Option(t) = self { fnc(t) }; self }
  pub fn on_poll<F: FnOnce(&UpdatePoll)>(&self, fnc: F) -> &Self { if let Update::Poll(t) = self { fnc(t) }; self }
  pub fn on_poll_answer<F: FnOnce(&UpdatePollAnswer)>(&self, fnc: F) -> &Self { if let Update::PollAnswer(t) = self { fnc(t) }; self }
  pub fn on_recent_stickers<F: FnOnce(&UpdateRecentStickers)>(&self, fnc: F) -> &Self { if let Update::RecentStickers(t) = self { fnc(t) }; self }
  pub fn on_saved_animations<F: FnOnce(&UpdateSavedAnimations)>(&self, fnc: F) -> &Self { if let Update::SavedAnimations(t) = self { fnc(t) }; self }
  pub fn on_scope_notification_settings<F: FnOnce(&UpdateScopeNotificationSettings)>(&self, fnc: F) -> &Self { if let Update::ScopeNotificationSettings(t) = self { fnc(t) }; self }
  pub fn on_secret_chat<F: FnOnce(&UpdateSecretChat)>(&self, fnc: F) -> &Self { if let Update::SecretChat(t) = self { fnc(t) }; self }
  pub fn on_selected_background<F: FnOnce(&UpdateSelectedBackground)>(&self, fnc: F) -> &Self { if let Update::SelectedBackground(t) = self { fnc(t) }; self }
  pub fn on_service_notification<F: FnOnce(&UpdateServiceNotification)>(&self, fnc: F) -> &Self { if let Update::ServiceNotification(t) = self { fnc(t) }; self }
  pub fn on_sticker_set<F: FnOnce(&UpdateStickerSet)>(&self, fnc: F) -> &Self { if let Update::StickerSet(t) = self { fnc(t) }; self }
  pub fn on_suggested_actions<F: FnOnce(&UpdateSuggestedActions)>(&self, fnc: F) -> &Self { if let Update::SuggestedActions(t) = self { fnc(t) }; self }
  pub fn on_supergroup<F: FnOnce(&UpdateSupergroup)>(&self, fnc: F) -> &Self { if let Update::Supergroup(t) = self { fnc(t) }; self }
  pub fn on_supergroup_full_info<F: FnOnce(&UpdateSupergroupFullInfo)>(&self, fnc: F) -> &Self { if let Update::SupergroupFullInfo(t) = self { fnc(t) }; self }
  pub fn on_terms_of_service<F: FnOnce(&UpdateTermsOfService)>(&self, fnc: F) -> &Self { if let Update::TermsOfService(t) = self { fnc(t) }; self }
  pub fn on_trending_sticker_sets<F: FnOnce(&UpdateTrendingStickerSets)>(&self, fnc: F) -> &Self { if let Update::TrendingStickerSets(t) = self { fnc(t) }; self }
  pub fn on_unread_chat_count<F: FnOnce(&UpdateUnreadChatCount)>(&self, fnc: F) -> &Self { if let Update::UnreadChatCount(t) = self { fnc(t) }; self }
  pub fn on_unread_message_count<F: FnOnce(&UpdateUnreadMessageCount)>(&self, fnc: F) -> &Self { if let Update::UnreadMessageCount(t) = self { fnc(t) }; self }
  pub fn on_user<F: FnOnce(&UpdateUser)>(&self, fnc: F) -> &Self { if let Update::User(t) = self { fnc(t) }; self }
  pub fn on_user_full_info<F: FnOnce(&UpdateUserFullInfo)>(&self, fnc: F) -> &Self { if let Update::UserFullInfo(t) = self { fnc(t) }; self }
  pub fn on_user_privacy_setting_rules<F: FnOnce(&UpdateUserPrivacySettingRules)>(&self, fnc: F) -> &Self { if let Update::UserPrivacySettingRules(t) = self { fnc(t) }; self }
  pub fn on_user_status<F: FnOnce(&UpdateUserStatus)>(&self, fnc: F) -> &Self { if let Update::UserStatus(t) = self { fnc(t) }; self }
  pub fn on_users_nearby<F: FnOnce(&UpdateUsersNearby)>(&self, fnc: F) -> &Self { if let Update::UsersNearby(t) = self { fnc(t) }; self }

  pub fn as_test_use_update(&self) -> Option<&TestUseUpdate> { if let Update::TestUseUpdate(t) = self { return Some(t) } None }
  pub fn as_active_notifications(&self) -> Option<&UpdateActiveNotifications> { if let Update::ActiveNotifications(t) = self { return Some(t) } None }
  pub fn as_animated_emoji_message_clicked(&self) -> Option<&UpdateAnimatedEmojiMessageClicked> { if let Update::AnimatedEmojiMessageClicked(t) = self { return Some(t) } None }
  pub fn as_animation_search_parameters(&self) -> Option<&UpdateAnimationSearchParameters> { if let Update::AnimationSearchParameters(t) = self { return Some(t) } None }
  pub fn as_authorization_state(&self) -> Option<&UpdateAuthorizationState> { if let Update::AuthorizationState(t) = self { return Some(t) } None }
  pub fn as_basic_group(&self) -> Option<&UpdateBasicGroup> { if let Update::BasicGroup(t) = self { return Some(t) } None }
  pub fn as_basic_group_full_info(&self) -> Option<&UpdateBasicGroupFullInfo> { if let Update::BasicGroupFullInfo(t) = self { return Some(t) } None }
  pub fn as_call(&self) -> Option<&UpdateCall> { if let Update::Call(t) = self { return Some(t) } None }
  pub fn as_chat_action(&self) -> Option<&UpdateChatAction> { if let Update::ChatAction(t) = self { return Some(t) } None }
  pub fn as_chat_action_bar(&self) -> Option<&UpdateChatActionBar> { if let Update::ChatActionBar(t) = self { return Some(t) } None }
  pub fn as_chat_default_disable_notification(&self) -> Option<&UpdateChatDefaultDisableNotification> { if let Update::ChatDefaultDisableNotification(t) = self { return Some(t) } None }
  pub fn as_chat_default_message_sender_id(&self) -> Option<&UpdateChatDefaultMessageSenderId> { if let Update::ChatDefaultMessageSenderId(t) = self { return Some(t) } None }
  pub fn as_chat_draft_message(&self) -> Option<&UpdateChatDraftMessage> { if let Update::ChatDraftMessage(t) = self { return Some(t) } None }
  pub fn as_chat_filters(&self) -> Option<&UpdateChatFilters> { if let Update::ChatFilters(t) = self { return Some(t) } None }
  pub fn as_chat_has_protected_content(&self) -> Option<&UpdateChatHasProtectedContent> { if let Update::ChatHasProtectedContent(t) = self { return Some(t) } None }
  pub fn as_chat_has_scheduled_messages(&self) -> Option<&UpdateChatHasScheduledMessages> { if let Update::ChatHasScheduledMessages(t) = self { return Some(t) } None }
  pub fn as_chat_is_blocked(&self) -> Option<&UpdateChatIsBlocked> { if let Update::ChatIsBlocked(t) = self { return Some(t) } None }
  pub fn as_chat_is_marked_as_unread(&self) -> Option<&UpdateChatIsMarkedAsUnread> { if let Update::ChatIsMarkedAsUnread(t) = self { return Some(t) } None }
  pub fn as_chat_last_message(&self) -> Option<&UpdateChatLastMessage> { if let Update::ChatLastMessage(t) = self { return Some(t) } None }
  pub fn as_chat_member(&self) -> Option<&UpdateChatMember> { if let Update::ChatMember(t) = self { return Some(t) } None }
  pub fn as_chat_message_ttl_setting(&self) -> Option<&UpdateChatMessageTtlSetting> { if let Update::ChatMessageTtlSetting(t) = self { return Some(t) } None }
  pub fn as_chat_notification_settings(&self) -> Option<&UpdateChatNotificationSettings> { if let Update::ChatNotificationSettings(t) = self { return Some(t) } None }
  pub fn as_chat_online_member_count(&self) -> Option<&UpdateChatOnlineMemberCount> { if let Update::ChatOnlineMemberCount(t) = self { return Some(t) } None }
  pub fn as_chat_pending_join_requests(&self) -> Option<&UpdateChatPendingJoinRequests> { if let Update::ChatPendingJoinRequests(t) = self { return Some(t) } None }
  pub fn as_chat_permissions(&self) -> Option<&UpdateChatPermissions> { if let Update::ChatPermissions(t) = self { return Some(t) } None }
  pub fn as_chat_photo(&self) -> Option<&UpdateChatPhoto> { if let Update::ChatPhoto(t) = self { return Some(t) } None }
  pub fn as_chat_position(&self) -> Option<&UpdateChatPosition> { if let Update::ChatPosition(t) = self { return Some(t) } None }
  pub fn as_chat_read_inbox(&self) -> Option<&UpdateChatReadInbox> { if let Update::ChatReadInbox(t) = self { return Some(t) } None }
  pub fn as_chat_read_outbox(&self) -> Option<&UpdateChatReadOutbox> { if let Update::ChatReadOutbox(t) = self { return Some(t) } None }
  pub fn as_chat_reply_markup(&self) -> Option<&UpdateChatReplyMarkup> { if let Update::ChatReplyMarkup(t) = self { return Some(t) } None }
  pub fn as_chat_theme(&self) -> Option<&UpdateChatTheme> { if let Update::ChatTheme(t) = self { return Some(t) } None }
  pub fn as_chat_themes(&self) -> Option<&UpdateChatThemes> { if let Update::ChatThemes(t) = self { return Some(t) } None }
  pub fn as_chat_title(&self) -> Option<&UpdateChatTitle> { if let Update::ChatTitle(t) = self { return Some(t) } None }
  pub fn as_chat_unread_mention_count(&self) -> Option<&UpdateChatUnreadMentionCount> { if let Update::ChatUnreadMentionCount(t) = self { return Some(t) } None }
  pub fn as_chat_video_chat(&self) -> Option<&UpdateChatVideoChat> { if let Update::ChatVideoChat(t) = self { return Some(t) } None }
  pub fn as_connection_state(&self) -> Option<&UpdateConnectionState> { if let Update::ConnectionState(t) = self { return Some(t) } None }
  pub fn as_delete_messages(&self) -> Option<&UpdateDeleteMessages> { if let Update::DeleteMessages(t) = self { return Some(t) } None }
  pub fn as_dice_emojis(&self) -> Option<&UpdateDiceEmojis> { if let Update::DiceEmojis(t) = self { return Some(t) } None }
  pub fn as_favorite_stickers(&self) -> Option<&UpdateFavoriteStickers> { if let Update::FavoriteStickers(t) = self { return Some(t) } None }
  pub fn as_file(&self) -> Option<&UpdateFile> { if let Update::File(t) = self { return Some(t) } None }
  pub fn as_file_generation_start(&self) -> Option<&UpdateFileGenerationStart> { if let Update::FileGenerationStart(t) = self { return Some(t) } None }
  pub fn as_file_generation_stop(&self) -> Option<&UpdateFileGenerationStop> { if let Update::FileGenerationStop(t) = self { return Some(t) } None }
  pub fn as_group_call(&self) -> Option<&UpdateGroupCall> { if let Update::GroupCall(t) = self { return Some(t) } None }
  pub fn as_group_call_participant(&self) -> Option<&UpdateGroupCallParticipant> { if let Update::GroupCallParticipant(t) = self { return Some(t) } None }
  pub fn as_have_pending_notifications(&self) -> Option<&UpdateHavePendingNotifications> { if let Update::HavePendingNotifications(t) = self { return Some(t) } None }
  pub fn as_installed_sticker_sets(&self) -> Option<&UpdateInstalledStickerSets> { if let Update::InstalledStickerSets(t) = self { return Some(t) } None }
  pub fn as_language_pack_strings(&self) -> Option<&UpdateLanguagePackStrings> { if let Update::LanguagePackStrings(t) = self { return Some(t) } None }
  pub fn as_message_content(&self) -> Option<&UpdateMessageContent> { if let Update::MessageContent(t) = self { return Some(t) } None }
  pub fn as_message_content_opened(&self) -> Option<&UpdateMessageContentOpened> { if let Update::MessageContentOpened(t) = self { return Some(t) } None }
  pub fn as_message_edited(&self) -> Option<&UpdateMessageEdited> { if let Update::MessageEdited(t) = self { return Some(t) } None }
  pub fn as_message_interaction_info(&self) -> Option<&UpdateMessageInteractionInfo> { if let Update::MessageInteractionInfo(t) = self { return Some(t) } None }
  pub fn as_message_is_pinned(&self) -> Option<&UpdateMessageIsPinned> { if let Update::MessageIsPinned(t) = self { return Some(t) } None }
  pub fn as_message_live_location_viewed(&self) -> Option<&UpdateMessageLiveLocationViewed> { if let Update::MessageLiveLocationViewed(t) = self { return Some(t) } None }
  pub fn as_message_mention_read(&self) -> Option<&UpdateMessageMentionRead> { if let Update::MessageMentionRead(t) = self { return Some(t) } None }
  pub fn as_message_send_acknowledged(&self) -> Option<&UpdateMessageSendAcknowledged> { if let Update::MessageSendAcknowledged(t) = self { return Some(t) } None }
  pub fn as_message_send_failed(&self) -> Option<&UpdateMessageSendFailed> { if let Update::MessageSendFailed(t) = self { return Some(t) } None }
  pub fn as_message_send_succeeded(&self) -> Option<&UpdateMessageSendSucceeded> { if let Update::MessageSendSucceeded(t) = self { return Some(t) } None }
  pub fn as_new_call_signaling_data(&self) -> Option<&UpdateNewCallSignalingData> { if let Update::NewCallSignalingData(t) = self { return Some(t) } None }
  pub fn as_new_callback_query(&self) -> Option<&UpdateNewCallbackQuery> { if let Update::NewCallbackQuery(t) = self { return Some(t) } None }
  pub fn as_new_chat(&self) -> Option<&UpdateNewChat> { if let Update::NewChat(t) = self { return Some(t) } None }
  pub fn as_new_chat_join_request(&self) -> Option<&UpdateNewChatJoinRequest> { if let Update::NewChatJoinRequest(t) = self { return Some(t) } None }
  pub fn as_new_chosen_inline_result(&self) -> Option<&UpdateNewChosenInlineResult> { if let Update::NewChosenInlineResult(t) = self { return Some(t) } None }
  pub fn as_new_custom_event(&self) -> Option<&UpdateNewCustomEvent> { if let Update::NewCustomEvent(t) = self { return Some(t) } None }
  pub fn as_new_custom_query(&self) -> Option<&UpdateNewCustomQuery> { if let Update::NewCustomQuery(t) = self { return Some(t) } None }
  pub fn as_new_inline_callback_query(&self) -> Option<&UpdateNewInlineCallbackQuery> { if let Update::NewInlineCallbackQuery(t) = self { return Some(t) } None }
  pub fn as_new_inline_query(&self) -> Option<&UpdateNewInlineQuery> { if let Update::NewInlineQuery(t) = self { return Some(t) } None }
  pub fn as_new_message(&self) -> Option<&UpdateNewMessage> { if let Update::NewMessage(t) = self { return Some(t) } None }
  pub fn as_new_pre_checkout_query(&self) -> Option<&UpdateNewPreCheckoutQuery> { if let Update::NewPreCheckoutQuery(t) = self { return Some(t) } None }
  pub fn as_new_shipping_query(&self) -> Option<&UpdateNewShippingQuery> { if let Update::NewShippingQuery(t) = self { return Some(t) } None }
  pub fn as_notification(&self) -> Option<&UpdateNotification> { if let Update::Notification(t) = self { return Some(t) } None }
  pub fn as_notification_group(&self) -> Option<&UpdateNotificationGroup> { if let Update::NotificationGroup(t) = self { return Some(t) } None }
  pub fn as_option(&self) -> Option<&UpdateOption> { if let Update::Option(t) = self { return Some(t) } None }
  pub fn as_poll(&self) -> Option<&UpdatePoll> { if let Update::Poll(t) = self { return Some(t) } None }
  pub fn as_poll_answer(&self) -> Option<&UpdatePollAnswer> { if let Update::PollAnswer(t) = self { return Some(t) } None }
  pub fn as_recent_stickers(&self) -> Option<&UpdateRecentStickers> { if let Update::RecentStickers(t) = self { return Some(t) } None }
  pub fn as_saved_animations(&self) -> Option<&UpdateSavedAnimations> { if let Update::SavedAnimations(t) = self { return Some(t) } None }
  pub fn as_scope_notification_settings(&self) -> Option<&UpdateScopeNotificationSettings> { if let Update::ScopeNotificationSettings(t) = self { return Some(t) } None }
  pub fn as_secret_chat(&self) -> Option<&UpdateSecretChat> { if let Update::SecretChat(t) = self { return Some(t) } None }
  pub fn as_selected_background(&self) -> Option<&UpdateSelectedBackground> { if let Update::SelectedBackground(t) = self { return Some(t) } None }
  pub fn as_service_notification(&self) -> Option<&UpdateServiceNotification> { if let Update::ServiceNotification(t) = self { return Some(t) } None }
  pub fn as_sticker_set(&self) -> Option<&UpdateStickerSet> { if let Update::StickerSet(t) = self { return Some(t) } None }
  pub fn as_suggested_actions(&self) -> Option<&UpdateSuggestedActions> { if let Update::SuggestedActions(t) = self { return Some(t) } None }
  pub fn as_supergroup(&self) -> Option<&UpdateSupergroup> { if let Update::Supergroup(t) = self { return Some(t) } None }
  pub fn as_supergroup_full_info(&self) -> Option<&UpdateSupergroupFullInfo> { if let Update::SupergroupFullInfo(t) = self { return Some(t) } None }
  pub fn as_terms_of_service(&self) -> Option<&UpdateTermsOfService> { if let Update::TermsOfService(t) = self { return Some(t) } None }
  pub fn as_trending_sticker_sets(&self) -> Option<&UpdateTrendingStickerSets> { if let Update::TrendingStickerSets(t) = self { return Some(t) } None }
  pub fn as_unread_chat_count(&self) -> Option<&UpdateUnreadChatCount> { if let Update::UnreadChatCount(t) = self { return Some(t) } None }
  pub fn as_unread_message_count(&self) -> Option<&UpdateUnreadMessageCount> { if let Update::UnreadMessageCount(t) = self { return Some(t) } None }
  pub fn as_user(&self) -> Option<&UpdateUser> { if let Update::User(t) = self { return Some(t) } None }
  pub fn as_user_full_info(&self) -> Option<&UpdateUserFullInfo> { if let Update::UserFullInfo(t) = self { return Some(t) } None }
  pub fn as_user_privacy_setting_rules(&self) -> Option<&UpdateUserPrivacySettingRules> { if let Update::UserPrivacySettingRules(t) = self { return Some(t) } None }
  pub fn as_user_status(&self) -> Option<&UpdateUserStatus> { if let Update::UserStatus(t) = self { return Some(t) } None }
  pub fn as_users_nearby(&self) -> Option<&UpdateUsersNearby> { if let Update::UsersNearby(t) = self { return Some(t) } None }



  pub fn test_use_update<T: AsRef<TestUseUpdate>>(t: T) -> Self { Update::TestUseUpdate(t.as_ref().clone()) }

  pub fn active_notifications<T: AsRef<UpdateActiveNotifications>>(t: T) -> Self { Update::ActiveNotifications(t.as_ref().clone()) }

  pub fn animated_emoji_message_clicked<T: AsRef<UpdateAnimatedEmojiMessageClicked>>(t: T) -> Self { Update::AnimatedEmojiMessageClicked(t.as_ref().clone()) }

  pub fn animation_search_parameters<T: AsRef<UpdateAnimationSearchParameters>>(t: T) -> Self { Update::AnimationSearchParameters(t.as_ref().clone()) }

  pub fn authorization_state<T: AsRef<UpdateAuthorizationState>>(t: T) -> Self { Update::AuthorizationState(t.as_ref().clone()) }

  pub fn basic_group<T: AsRef<UpdateBasicGroup>>(t: T) -> Self { Update::BasicGroup(t.as_ref().clone()) }

  pub fn basic_group_full_info<T: AsRef<UpdateBasicGroupFullInfo>>(t: T) -> Self { Update::BasicGroupFullInfo(t.as_ref().clone()) }

  pub fn call<T: AsRef<UpdateCall>>(t: T) -> Self { Update::Call(t.as_ref().clone()) }

  pub fn chat_action<T: AsRef<UpdateChatAction>>(t: T) -> Self { Update::ChatAction(t.as_ref().clone()) }

  pub fn chat_action_bar<T: AsRef<UpdateChatActionBar>>(t: T) -> Self { Update::ChatActionBar(t.as_ref().clone()) }

  pub fn chat_default_disable_notification<T: AsRef<UpdateChatDefaultDisableNotification>>(t: T) -> Self { Update::ChatDefaultDisableNotification(t.as_ref().clone()) }

  pub fn chat_default_message_sender_id<T: AsRef<UpdateChatDefaultMessageSenderId>>(t: T) -> Self { Update::ChatDefaultMessageSenderId(t.as_ref().clone()) }

  pub fn chat_draft_message<T: AsRef<UpdateChatDraftMessage>>(t: T) -> Self { Update::ChatDraftMessage(t.as_ref().clone()) }

  pub fn chat_filters<T: AsRef<UpdateChatFilters>>(t: T) -> Self { Update::ChatFilters(t.as_ref().clone()) }

  pub fn chat_has_protected_content<T: AsRef<UpdateChatHasProtectedContent>>(t: T) -> Self { Update::ChatHasProtectedContent(t.as_ref().clone()) }

  pub fn chat_has_scheduled_messages<T: AsRef<UpdateChatHasScheduledMessages>>(t: T) -> Self { Update::ChatHasScheduledMessages(t.as_ref().clone()) }

  pub fn chat_is_blocked<T: AsRef<UpdateChatIsBlocked>>(t: T) -> Self { Update::ChatIsBlocked(t.as_ref().clone()) }

  pub fn chat_is_marked_as_unread<T: AsRef<UpdateChatIsMarkedAsUnread>>(t: T) -> Self { Update::ChatIsMarkedAsUnread(t.as_ref().clone()) }

  pub fn chat_last_message<T: AsRef<UpdateChatLastMessage>>(t: T) -> Self { Update::ChatLastMessage(t.as_ref().clone()) }

  pub fn chat_member<T: AsRef<UpdateChatMember>>(t: T) -> Self { Update::ChatMember(t.as_ref().clone()) }

  pub fn chat_message_ttl_setting<T: AsRef<UpdateChatMessageTtlSetting>>(t: T) -> Self { Update::ChatMessageTtlSetting(t.as_ref().clone()) }

  pub fn chat_notification_settings<T: AsRef<UpdateChatNotificationSettings>>(t: T) -> Self { Update::ChatNotificationSettings(t.as_ref().clone()) }

  pub fn chat_online_member_count<T: AsRef<UpdateChatOnlineMemberCount>>(t: T) -> Self { Update::ChatOnlineMemberCount(t.as_ref().clone()) }

  pub fn chat_pending_join_requests<T: AsRef<UpdateChatPendingJoinRequests>>(t: T) -> Self { Update::ChatPendingJoinRequests(t.as_ref().clone()) }

  pub fn chat_permissions<T: AsRef<UpdateChatPermissions>>(t: T) -> Self { Update::ChatPermissions(t.as_ref().clone()) }

  pub fn chat_photo<T: AsRef<UpdateChatPhoto>>(t: T) -> Self { Update::ChatPhoto(t.as_ref().clone()) }

  pub fn chat_position<T: AsRef<UpdateChatPosition>>(t: T) -> Self { Update::ChatPosition(t.as_ref().clone()) }

  pub fn chat_read_inbox<T: AsRef<UpdateChatReadInbox>>(t: T) -> Self { Update::ChatReadInbox(t.as_ref().clone()) }

  pub fn chat_read_outbox<T: AsRef<UpdateChatReadOutbox>>(t: T) -> Self { Update::ChatReadOutbox(t.as_ref().clone()) }

  pub fn chat_reply_markup<T: AsRef<UpdateChatReplyMarkup>>(t: T) -> Self { Update::ChatReplyMarkup(t.as_ref().clone()) }

  pub fn chat_theme<T: AsRef<UpdateChatTheme>>(t: T) -> Self { Update::ChatTheme(t.as_ref().clone()) }

  pub fn chat_themes<T: AsRef<UpdateChatThemes>>(t: T) -> Self { Update::ChatThemes(t.as_ref().clone()) }

  pub fn chat_title<T: AsRef<UpdateChatTitle>>(t: T) -> Self { Update::ChatTitle(t.as_ref().clone()) }

  pub fn chat_unread_mention_count<T: AsRef<UpdateChatUnreadMentionCount>>(t: T) -> Self { Update::ChatUnreadMentionCount(t.as_ref().clone()) }

  pub fn chat_video_chat<T: AsRef<UpdateChatVideoChat>>(t: T) -> Self { Update::ChatVideoChat(t.as_ref().clone()) }

  pub fn connection_state<T: AsRef<UpdateConnectionState>>(t: T) -> Self { Update::ConnectionState(t.as_ref().clone()) }

  pub fn delete_messages<T: AsRef<UpdateDeleteMessages>>(t: T) -> Self { Update::DeleteMessages(t.as_ref().clone()) }

  pub fn dice_emojis<T: AsRef<UpdateDiceEmojis>>(t: T) -> Self { Update::DiceEmojis(t.as_ref().clone()) }

  pub fn favorite_stickers<T: AsRef<UpdateFavoriteStickers>>(t: T) -> Self { Update::FavoriteStickers(t.as_ref().clone()) }

  pub fn file<T: AsRef<UpdateFile>>(t: T) -> Self { Update::File(t.as_ref().clone()) }

  pub fn file_generation_start<T: AsRef<UpdateFileGenerationStart>>(t: T) -> Self { Update::FileGenerationStart(t.as_ref().clone()) }

  pub fn file_generation_stop<T: AsRef<UpdateFileGenerationStop>>(t: T) -> Self { Update::FileGenerationStop(t.as_ref().clone()) }

  pub fn group_call<T: AsRef<UpdateGroupCall>>(t: T) -> Self { Update::GroupCall(t.as_ref().clone()) }

  pub fn group_call_participant<T: AsRef<UpdateGroupCallParticipant>>(t: T) -> Self { Update::GroupCallParticipant(t.as_ref().clone()) }

  pub fn have_pending_notifications<T: AsRef<UpdateHavePendingNotifications>>(t: T) -> Self { Update::HavePendingNotifications(t.as_ref().clone()) }

  pub fn installed_sticker_sets<T: AsRef<UpdateInstalledStickerSets>>(t: T) -> Self { Update::InstalledStickerSets(t.as_ref().clone()) }

  pub fn language_pack_strings<T: AsRef<UpdateLanguagePackStrings>>(t: T) -> Self { Update::LanguagePackStrings(t.as_ref().clone()) }

  pub fn message_content<T: AsRef<UpdateMessageContent>>(t: T) -> Self { Update::MessageContent(t.as_ref().clone()) }

  pub fn message_content_opened<T: AsRef<UpdateMessageContentOpened>>(t: T) -> Self { Update::MessageContentOpened(t.as_ref().clone()) }

  pub fn message_edited<T: AsRef<UpdateMessageEdited>>(t: T) -> Self { Update::MessageEdited(t.as_ref().clone()) }

  pub fn message_interaction_info<T: AsRef<UpdateMessageInteractionInfo>>(t: T) -> Self { Update::MessageInteractionInfo(t.as_ref().clone()) }

  pub fn message_is_pinned<T: AsRef<UpdateMessageIsPinned>>(t: T) -> Self { Update::MessageIsPinned(t.as_ref().clone()) }

  pub fn message_live_location_viewed<T: AsRef<UpdateMessageLiveLocationViewed>>(t: T) -> Self { Update::MessageLiveLocationViewed(t.as_ref().clone()) }

  pub fn message_mention_read<T: AsRef<UpdateMessageMentionRead>>(t: T) -> Self { Update::MessageMentionRead(t.as_ref().clone()) }

  pub fn message_send_acknowledged<T: AsRef<UpdateMessageSendAcknowledged>>(t: T) -> Self { Update::MessageSendAcknowledged(t.as_ref().clone()) }

  pub fn message_send_failed<T: AsRef<UpdateMessageSendFailed>>(t: T) -> Self { Update::MessageSendFailed(t.as_ref().clone()) }

  pub fn message_send_succeeded<T: AsRef<UpdateMessageSendSucceeded>>(t: T) -> Self { Update::MessageSendSucceeded(t.as_ref().clone()) }

  pub fn new_call_signaling_data<T: AsRef<UpdateNewCallSignalingData>>(t: T) -> Self { Update::NewCallSignalingData(t.as_ref().clone()) }

  pub fn new_callback_query<T: AsRef<UpdateNewCallbackQuery>>(t: T) -> Self { Update::NewCallbackQuery(t.as_ref().clone()) }

  pub fn new_chat<T: AsRef<UpdateNewChat>>(t: T) -> Self { Update::NewChat(t.as_ref().clone()) }

  pub fn new_chat_join_request<T: AsRef<UpdateNewChatJoinRequest>>(t: T) -> Self { Update::NewChatJoinRequest(t.as_ref().clone()) }

  pub fn new_chosen_inline_result<T: AsRef<UpdateNewChosenInlineResult>>(t: T) -> Self { Update::NewChosenInlineResult(t.as_ref().clone()) }

  pub fn new_custom_event<T: AsRef<UpdateNewCustomEvent>>(t: T) -> Self { Update::NewCustomEvent(t.as_ref().clone()) }

  pub fn new_custom_query<T: AsRef<UpdateNewCustomQuery>>(t: T) -> Self { Update::NewCustomQuery(t.as_ref().clone()) }

  pub fn new_inline_callback_query<T: AsRef<UpdateNewInlineCallbackQuery>>(t: T) -> Self { Update::NewInlineCallbackQuery(t.as_ref().clone()) }

  pub fn new_inline_query<T: AsRef<UpdateNewInlineQuery>>(t: T) -> Self { Update::NewInlineQuery(t.as_ref().clone()) }

  pub fn new_message<T: AsRef<UpdateNewMessage>>(t: T) -> Self { Update::NewMessage(t.as_ref().clone()) }

  pub fn new_pre_checkout_query<T: AsRef<UpdateNewPreCheckoutQuery>>(t: T) -> Self { Update::NewPreCheckoutQuery(t.as_ref().clone()) }

  pub fn new_shipping_query<T: AsRef<UpdateNewShippingQuery>>(t: T) -> Self { Update::NewShippingQuery(t.as_ref().clone()) }

  pub fn notification<T: AsRef<UpdateNotification>>(t: T) -> Self { Update::Notification(t.as_ref().clone()) }

  pub fn notification_group<T: AsRef<UpdateNotificationGroup>>(t: T) -> Self { Update::NotificationGroup(t.as_ref().clone()) }

  pub fn option<T: AsRef<UpdateOption>>(t: T) -> Self { Update::Option(t.as_ref().clone()) }

  pub fn poll<T: AsRef<UpdatePoll>>(t: T) -> Self { Update::Poll(t.as_ref().clone()) }

  pub fn poll_answer<T: AsRef<UpdatePollAnswer>>(t: T) -> Self { Update::PollAnswer(t.as_ref().clone()) }

  pub fn recent_stickers<T: AsRef<UpdateRecentStickers>>(t: T) -> Self { Update::RecentStickers(t.as_ref().clone()) }

  pub fn saved_animations<T: AsRef<UpdateSavedAnimations>>(t: T) -> Self { Update::SavedAnimations(t.as_ref().clone()) }

  pub fn scope_notification_settings<T: AsRef<UpdateScopeNotificationSettings>>(t: T) -> Self { Update::ScopeNotificationSettings(t.as_ref().clone()) }

  pub fn secret_chat<T: AsRef<UpdateSecretChat>>(t: T) -> Self { Update::SecretChat(t.as_ref().clone()) }

  pub fn selected_background<T: AsRef<UpdateSelectedBackground>>(t: T) -> Self { Update::SelectedBackground(t.as_ref().clone()) }

  pub fn service_notification<T: AsRef<UpdateServiceNotification>>(t: T) -> Self { Update::ServiceNotification(t.as_ref().clone()) }

  pub fn sticker_set<T: AsRef<UpdateStickerSet>>(t: T) -> Self { Update::StickerSet(t.as_ref().clone()) }

  pub fn suggested_actions<T: AsRef<UpdateSuggestedActions>>(t: T) -> Self { Update::SuggestedActions(t.as_ref().clone()) }

  pub fn supergroup<T: AsRef<UpdateSupergroup>>(t: T) -> Self { Update::Supergroup(t.as_ref().clone()) }

  pub fn supergroup_full_info<T: AsRef<UpdateSupergroupFullInfo>>(t: T) -> Self { Update::SupergroupFullInfo(t.as_ref().clone()) }

  pub fn terms_of_service<T: AsRef<UpdateTermsOfService>>(t: T) -> Self { Update::TermsOfService(t.as_ref().clone()) }

  pub fn trending_sticker_sets<T: AsRef<UpdateTrendingStickerSets>>(t: T) -> Self { Update::TrendingStickerSets(t.as_ref().clone()) }

  pub fn unread_chat_count<T: AsRef<UpdateUnreadChatCount>>(t: T) -> Self { Update::UnreadChatCount(t.as_ref().clone()) }

  pub fn unread_message_count<T: AsRef<UpdateUnreadMessageCount>>(t: T) -> Self { Update::UnreadMessageCount(t.as_ref().clone()) }

  pub fn user<T: AsRef<UpdateUser>>(t: T) -> Self { Update::User(t.as_ref().clone()) }

  pub fn user_full_info<T: AsRef<UpdateUserFullInfo>>(t: T) -> Self { Update::UserFullInfo(t.as_ref().clone()) }

  pub fn user_privacy_setting_rules<T: AsRef<UpdateUserPrivacySettingRules>>(t: T) -> Self { Update::UserPrivacySettingRules(t.as_ref().clone()) }

  pub fn user_status<T: AsRef<UpdateUserStatus>>(t: T) -> Self { Update::UserStatus(t.as_ref().clone()) }

  pub fn users_nearby<T: AsRef<UpdateUsersNearby>>(t: T) -> Self { Update::UsersNearby(t.as_ref().clone()) }

}

impl AsRef<Update> for Update {
  fn as_ref(&self) -> &Update { self }
}







/// Contains active notifications that was shown on previous application launches. This update is sent only if the message database is used. In that case it comes once before any updateNotification and updateNotificationGroup update
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateActiveNotifications {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Lists of active notification groups
  groups: Vec<NotificationGroup>,
  
}

impl RObject for UpdateActiveNotifications {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateActiveNotifications" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateActiveNotifications {}



impl UpdateActiveNotifications {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateActiveNotificationsBuilder {
    let mut inner = UpdateActiveNotifications::default();
    inner.td_name = "updateActiveNotifications".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateActiveNotificationsBuilder { inner }
  }

  pub fn groups(&self) -> &Vec<NotificationGroup> { &self.groups }

}

#[doc(hidden)]
pub struct RTDUpdateActiveNotificationsBuilder {
  inner: UpdateActiveNotifications
}

impl RTDUpdateActiveNotificationsBuilder {
  pub fn build(&self) -> UpdateActiveNotifications { self.inner.clone() }

   
  pub fn groups(&mut self, groups: Vec<NotificationGroup>) -> &mut Self {
    self.inner.groups = groups;
    self
  }

}

impl AsRef<UpdateActiveNotifications> for UpdateActiveNotifications {
  fn as_ref(&self) -> &UpdateActiveNotifications { self }
}

impl AsRef<UpdateActiveNotifications> for RTDUpdateActiveNotificationsBuilder {
  fn as_ref(&self) -> &UpdateActiveNotifications { &self.inner }
}







/// Some animated emoji message was clicked and a big animated sticker must be played if the message is visible on the screen. chatActionWatchingAnimations with the text of the message needs to be sent if the sticker is played
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateAnimatedEmojiMessageClicked {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Message identifier
  message_id: i64,
  /// The animated sticker to be played
  sticker: Sticker,
  
}

impl RObject for UpdateAnimatedEmojiMessageClicked {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateAnimatedEmojiMessageClicked" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateAnimatedEmojiMessageClicked {}



impl UpdateAnimatedEmojiMessageClicked {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateAnimatedEmojiMessageClickedBuilder {
    let mut inner = UpdateAnimatedEmojiMessageClicked::default();
    inner.td_name = "updateAnimatedEmojiMessageClicked".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateAnimatedEmojiMessageClickedBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn sticker(&self) -> &Sticker { &self.sticker }

}

#[doc(hidden)]
pub struct RTDUpdateAnimatedEmojiMessageClickedBuilder {
  inner: UpdateAnimatedEmojiMessageClicked
}

impl RTDUpdateAnimatedEmojiMessageClickedBuilder {
  pub fn build(&self) -> UpdateAnimatedEmojiMessageClicked { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn sticker<T: AsRef<Sticker>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
    self
  }

}

impl AsRef<UpdateAnimatedEmojiMessageClicked> for UpdateAnimatedEmojiMessageClicked {
  fn as_ref(&self) -> &UpdateAnimatedEmojiMessageClicked { self }
}

impl AsRef<UpdateAnimatedEmojiMessageClicked> for RTDUpdateAnimatedEmojiMessageClickedBuilder {
  fn as_ref(&self) -> &UpdateAnimatedEmojiMessageClicked { &self.inner }
}







/// The parameters of animation search through GetOption("animation_search_bot_username") bot has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateAnimationSearchParameters {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the animation search provider
  provider: String,
  /// The new list of emojis suggested for searching
  emojis: Vec<String>,
  
}

impl RObject for UpdateAnimationSearchParameters {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateAnimationSearchParameters" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateAnimationSearchParameters {}



impl UpdateAnimationSearchParameters {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateAnimationSearchParametersBuilder {
    let mut inner = UpdateAnimationSearchParameters::default();
    inner.td_name = "updateAnimationSearchParameters".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateAnimationSearchParametersBuilder { inner }
  }

  pub fn provider(&self) -> &String { &self.provider }

  pub fn emojis(&self) -> &Vec<String> { &self.emojis }

}

#[doc(hidden)]
pub struct RTDUpdateAnimationSearchParametersBuilder {
  inner: UpdateAnimationSearchParameters
}

impl RTDUpdateAnimationSearchParametersBuilder {
  pub fn build(&self) -> UpdateAnimationSearchParameters { self.inner.clone() }

   
  pub fn provider<T: AsRef<str>>(&mut self, provider: T) -> &mut Self {
    self.inner.provider = provider.as_ref().to_string();
    self
  }

   
  pub fn emojis(&mut self, emojis: Vec<String>) -> &mut Self {
    self.inner.emojis = emojis;
    self
  }

}

impl AsRef<UpdateAnimationSearchParameters> for UpdateAnimationSearchParameters {
  fn as_ref(&self) -> &UpdateAnimationSearchParameters { self }
}

impl AsRef<UpdateAnimationSearchParameters> for RTDUpdateAnimationSearchParametersBuilder {
  fn as_ref(&self) -> &UpdateAnimationSearchParameters { &self.inner }
}







/// The user authorization state has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateAuthorizationState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New authorization state
  authorization_state: AuthorizationState,
  
}

impl RObject for UpdateAuthorizationState {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateAuthorizationState" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateAuthorizationState {}



impl UpdateAuthorizationState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateAuthorizationStateBuilder {
    let mut inner = UpdateAuthorizationState::default();
    inner.td_name = "updateAuthorizationState".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateAuthorizationStateBuilder { inner }
  }

  pub fn authorization_state(&self) -> &AuthorizationState { &self.authorization_state }

}

#[doc(hidden)]
pub struct RTDUpdateAuthorizationStateBuilder {
  inner: UpdateAuthorizationState
}

impl RTDUpdateAuthorizationStateBuilder {
  pub fn build(&self) -> UpdateAuthorizationState { self.inner.clone() }

   
  pub fn authorization_state<T: AsRef<AuthorizationState>>(&mut self, authorization_state: T) -> &mut Self {
    self.inner.authorization_state = authorization_state.as_ref().clone();
    self
  }

}

impl AsRef<UpdateAuthorizationState> for UpdateAuthorizationState {
  fn as_ref(&self) -> &UpdateAuthorizationState { self }
}

impl AsRef<UpdateAuthorizationState> for RTDUpdateAuthorizationStateBuilder {
  fn as_ref(&self) -> &UpdateAuthorizationState { &self.inner }
}







/// Some data of a basic group has changed. This update is guaranteed to come before the basic group identifier is returned to the application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateBasicGroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New data about the group
  basic_group: BasicGroup,
  
}

impl RObject for UpdateBasicGroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateBasicGroup" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateBasicGroup {}



impl UpdateBasicGroup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateBasicGroupBuilder {
    let mut inner = UpdateBasicGroup::default();
    inner.td_name = "updateBasicGroup".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateBasicGroupBuilder { inner }
  }

  pub fn basic_group(&self) -> &BasicGroup { &self.basic_group }

}

#[doc(hidden)]
pub struct RTDUpdateBasicGroupBuilder {
  inner: UpdateBasicGroup
}

impl RTDUpdateBasicGroupBuilder {
  pub fn build(&self) -> UpdateBasicGroup { self.inner.clone() }

   
  pub fn basic_group<T: AsRef<BasicGroup>>(&mut self, basic_group: T) -> &mut Self {
    self.inner.basic_group = basic_group.as_ref().clone();
    self
  }

}

impl AsRef<UpdateBasicGroup> for UpdateBasicGroup {
  fn as_ref(&self) -> &UpdateBasicGroup { self }
}

impl AsRef<UpdateBasicGroup> for RTDUpdateBasicGroupBuilder {
  fn as_ref(&self) -> &UpdateBasicGroup { &self.inner }
}







/// Some data in basicGroupFullInfo has been changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateBasicGroupFullInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of a basic group
  basic_group_id: i64,
  /// New full information about the group
  basic_group_full_info: BasicGroupFullInfo,
  
}

impl RObject for UpdateBasicGroupFullInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateBasicGroupFullInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateBasicGroupFullInfo {}



impl UpdateBasicGroupFullInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateBasicGroupFullInfoBuilder {
    let mut inner = UpdateBasicGroupFullInfo::default();
    inner.td_name = "updateBasicGroupFullInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateBasicGroupFullInfoBuilder { inner }
  }

  pub fn basic_group_id(&self) -> i64 { self.basic_group_id }

  pub fn basic_group_full_info(&self) -> &BasicGroupFullInfo { &self.basic_group_full_info }

}

#[doc(hidden)]
pub struct RTDUpdateBasicGroupFullInfoBuilder {
  inner: UpdateBasicGroupFullInfo
}

impl RTDUpdateBasicGroupFullInfoBuilder {
  pub fn build(&self) -> UpdateBasicGroupFullInfo { self.inner.clone() }

   
  pub fn basic_group_id(&mut self, basic_group_id: i64) -> &mut Self {
    self.inner.basic_group_id = basic_group_id;
    self
  }

   
  pub fn basic_group_full_info<T: AsRef<BasicGroupFullInfo>>(&mut self, basic_group_full_info: T) -> &mut Self {
    self.inner.basic_group_full_info = basic_group_full_info.as_ref().clone();
    self
  }

}

impl AsRef<UpdateBasicGroupFullInfo> for UpdateBasicGroupFullInfo {
  fn as_ref(&self) -> &UpdateBasicGroupFullInfo { self }
}

impl AsRef<UpdateBasicGroupFullInfo> for RTDUpdateBasicGroupFullInfoBuilder {
  fn as_ref(&self) -> &UpdateBasicGroupFullInfo { &self.inner }
}







/// New call was created or information about a call was updated
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New data about a call
  call: Call,
  
}

impl RObject for UpdateCall {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateCall" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateCall {}



impl UpdateCall {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateCallBuilder {
    let mut inner = UpdateCall::default();
    inner.td_name = "updateCall".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateCallBuilder { inner }
  }

  pub fn call(&self) -> &Call { &self.call }

}

#[doc(hidden)]
pub struct RTDUpdateCallBuilder {
  inner: UpdateCall
}

impl RTDUpdateCallBuilder {
  pub fn build(&self) -> UpdateCall { self.inner.clone() }

   
  pub fn call<T: AsRef<Call>>(&mut self, call: T) -> &mut Self {
    self.inner.call = call.as_ref().clone();
    self
  }

}

impl AsRef<UpdateCall> for UpdateCall {
  fn as_ref(&self) -> &UpdateCall { self }
}

impl AsRef<UpdateCall> for RTDUpdateCallBuilder {
  fn as_ref(&self) -> &UpdateCall { &self.inner }
}







/// A message sender activity in the chat has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatAction {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// If not 0, a message thread identifier in which the action was performed
  message_thread_id: i64,
  /// Identifier of a message sender performing the action
  sender_id: MessageSender,
  /// The action
  action: ChatAction,
  
}

impl RObject for UpdateChatAction {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatAction" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatAction {}



impl UpdateChatAction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatActionBuilder {
    let mut inner = UpdateChatAction::default();
    inner.td_name = "updateChatAction".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatActionBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_thread_id(&self) -> i64 { self.message_thread_id }

  pub fn sender_id(&self) -> &MessageSender { &self.sender_id }

  pub fn action(&self) -> &ChatAction { &self.action }

}

#[doc(hidden)]
pub struct RTDUpdateChatActionBuilder {
  inner: UpdateChatAction
}

impl RTDUpdateChatActionBuilder {
  pub fn build(&self) -> UpdateChatAction { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_thread_id(&mut self, message_thread_id: i64) -> &mut Self {
    self.inner.message_thread_id = message_thread_id;
    self
  }

   
  pub fn sender_id<T: AsRef<MessageSender>>(&mut self, sender_id: T) -> &mut Self {
    self.inner.sender_id = sender_id.as_ref().clone();
    self
  }

   
  pub fn action<T: AsRef<ChatAction>>(&mut self, action: T) -> &mut Self {
    self.inner.action = action.as_ref().clone();
    self
  }

}

impl AsRef<UpdateChatAction> for UpdateChatAction {
  fn as_ref(&self) -> &UpdateChatAction { self }
}

impl AsRef<UpdateChatAction> for RTDUpdateChatActionBuilder {
  fn as_ref(&self) -> &UpdateChatAction { &self.inner }
}







/// The chat action bar was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatActionBar {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The new value of the action bar; may be null
  action_bar: Option<ChatActionBar>,
  
}

impl RObject for UpdateChatActionBar {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatActionBar" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatActionBar {}



impl UpdateChatActionBar {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatActionBarBuilder {
    let mut inner = UpdateChatActionBar::default();
    inner.td_name = "updateChatActionBar".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatActionBarBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn action_bar(&self) -> &Option<ChatActionBar> { &self.action_bar }

}

#[doc(hidden)]
pub struct RTDUpdateChatActionBarBuilder {
  inner: UpdateChatActionBar
}

impl RTDUpdateChatActionBarBuilder {
  pub fn build(&self) -> UpdateChatActionBar { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn action_bar<T: AsRef<ChatActionBar>>(&mut self, action_bar: T) -> &mut Self {
    self.inner.action_bar = Some(action_bar.as_ref().clone());
    self
  }

}

impl AsRef<UpdateChatActionBar> for UpdateChatActionBar {
  fn as_ref(&self) -> &UpdateChatActionBar { self }
}

impl AsRef<UpdateChatActionBar> for RTDUpdateChatActionBarBuilder {
  fn as_ref(&self) -> &UpdateChatActionBar { &self.inner }
}







/// The value of the default disable_notification parameter, used when a message is sent to the chat, was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatDefaultDisableNotification {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The new default_disable_notification value
  default_disable_notification: bool,
  
}

impl RObject for UpdateChatDefaultDisableNotification {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatDefaultDisableNotification" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatDefaultDisableNotification {}



impl UpdateChatDefaultDisableNotification {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatDefaultDisableNotificationBuilder {
    let mut inner = UpdateChatDefaultDisableNotification::default();
    inner.td_name = "updateChatDefaultDisableNotification".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatDefaultDisableNotificationBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn default_disable_notification(&self) -> bool { self.default_disable_notification }

}

#[doc(hidden)]
pub struct RTDUpdateChatDefaultDisableNotificationBuilder {
  inner: UpdateChatDefaultDisableNotification
}

impl RTDUpdateChatDefaultDisableNotificationBuilder {
  pub fn build(&self) -> UpdateChatDefaultDisableNotification { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn default_disable_notification(&mut self, default_disable_notification: bool) -> &mut Self {
    self.inner.default_disable_notification = default_disable_notification;
    self
  }

}

impl AsRef<UpdateChatDefaultDisableNotification> for UpdateChatDefaultDisableNotification {
  fn as_ref(&self) -> &UpdateChatDefaultDisableNotification { self }
}

impl AsRef<UpdateChatDefaultDisableNotification> for RTDUpdateChatDefaultDisableNotificationBuilder {
  fn as_ref(&self) -> &UpdateChatDefaultDisableNotification { &self.inner }
}







/// The default message sender that is chosen to send messages in a chat has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatDefaultMessageSenderId {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New value of default_message_sender_id; may be null if the user can't change message sender
  default_message_sender_id: Option<MessageSender>,
  
}

impl RObject for UpdateChatDefaultMessageSenderId {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatDefaultMessageSenderId" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatDefaultMessageSenderId {}



impl UpdateChatDefaultMessageSenderId {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatDefaultMessageSenderIdBuilder {
    let mut inner = UpdateChatDefaultMessageSenderId::default();
    inner.td_name = "updateChatDefaultMessageSenderId".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatDefaultMessageSenderIdBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn default_message_sender_id(&self) -> &Option<MessageSender> { &self.default_message_sender_id }

}

#[doc(hidden)]
pub struct RTDUpdateChatDefaultMessageSenderIdBuilder {
  inner: UpdateChatDefaultMessageSenderId
}

impl RTDUpdateChatDefaultMessageSenderIdBuilder {
  pub fn build(&self) -> UpdateChatDefaultMessageSenderId { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn default_message_sender_id<T: AsRef<MessageSender>>(&mut self, default_message_sender_id: T) -> &mut Self {
    self.inner.default_message_sender_id = Some(default_message_sender_id.as_ref().clone());
    self
  }

}

impl AsRef<UpdateChatDefaultMessageSenderId> for UpdateChatDefaultMessageSenderId {
  fn as_ref(&self) -> &UpdateChatDefaultMessageSenderId { self }
}

impl AsRef<UpdateChatDefaultMessageSenderId> for RTDUpdateChatDefaultMessageSenderIdBuilder {
  fn as_ref(&self) -> &UpdateChatDefaultMessageSenderId { &self.inner }
}







/// A chat draft has changed. Be aware that the update may come in the currently opened chat but with old content of the draft. If the user has changed the content of the draft, this update mustn't be applied
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatDraftMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The new draft message; may be null
  draft_message: Option<DraftMessage>,
  /// The new chat positions in the chat lists
  positions: Vec<ChatPosition>,
  
}

impl RObject for UpdateChatDraftMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatDraftMessage" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatDraftMessage {}



impl UpdateChatDraftMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatDraftMessageBuilder {
    let mut inner = UpdateChatDraftMessage::default();
    inner.td_name = "updateChatDraftMessage".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatDraftMessageBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn draft_message(&self) -> &Option<DraftMessage> { &self.draft_message }

  pub fn positions(&self) -> &Vec<ChatPosition> { &self.positions }

}

#[doc(hidden)]
pub struct RTDUpdateChatDraftMessageBuilder {
  inner: UpdateChatDraftMessage
}

impl RTDUpdateChatDraftMessageBuilder {
  pub fn build(&self) -> UpdateChatDraftMessage { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn draft_message<T: AsRef<DraftMessage>>(&mut self, draft_message: T) -> &mut Self {
    self.inner.draft_message = Some(draft_message.as_ref().clone());
    self
  }

   
  pub fn positions(&mut self, positions: Vec<ChatPosition>) -> &mut Self {
    self.inner.positions = positions;
    self
  }

}

impl AsRef<UpdateChatDraftMessage> for UpdateChatDraftMessage {
  fn as_ref(&self) -> &UpdateChatDraftMessage { self }
}

impl AsRef<UpdateChatDraftMessage> for RTDUpdateChatDraftMessageBuilder {
  fn as_ref(&self) -> &UpdateChatDraftMessage { &self.inner }
}







/// The list of chat filters or a chat filter has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatFilters {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new list of chat filters
  chat_filters: Vec<ChatFilterInfo>,
  
}

impl RObject for UpdateChatFilters {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatFilters" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatFilters {}



impl UpdateChatFilters {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatFiltersBuilder {
    let mut inner = UpdateChatFilters::default();
    inner.td_name = "updateChatFilters".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatFiltersBuilder { inner }
  }

  pub fn chat_filters(&self) -> &Vec<ChatFilterInfo> { &self.chat_filters }

}

#[doc(hidden)]
pub struct RTDUpdateChatFiltersBuilder {
  inner: UpdateChatFilters
}

impl RTDUpdateChatFiltersBuilder {
  pub fn build(&self) -> UpdateChatFilters { self.inner.clone() }

   
  pub fn chat_filters(&mut self, chat_filters: Vec<ChatFilterInfo>) -> &mut Self {
    self.inner.chat_filters = chat_filters;
    self
  }

}

impl AsRef<UpdateChatFilters> for UpdateChatFilters {
  fn as_ref(&self) -> &UpdateChatFilters { self }
}

impl AsRef<UpdateChatFilters> for RTDUpdateChatFiltersBuilder {
  fn as_ref(&self) -> &UpdateChatFilters { &self.inner }
}







/// A chat content was allowed or restricted for saving
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatHasProtectedContent {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New value of has_protected_content
  has_protected_content: bool,
  
}

impl RObject for UpdateChatHasProtectedContent {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatHasProtectedContent" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatHasProtectedContent {}



impl UpdateChatHasProtectedContent {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatHasProtectedContentBuilder {
    let mut inner = UpdateChatHasProtectedContent::default();
    inner.td_name = "updateChatHasProtectedContent".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatHasProtectedContentBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn has_protected_content(&self) -> bool { self.has_protected_content }

}

#[doc(hidden)]
pub struct RTDUpdateChatHasProtectedContentBuilder {
  inner: UpdateChatHasProtectedContent
}

impl RTDUpdateChatHasProtectedContentBuilder {
  pub fn build(&self) -> UpdateChatHasProtectedContent { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn has_protected_content(&mut self, has_protected_content: bool) -> &mut Self {
    self.inner.has_protected_content = has_protected_content;
    self
  }

}

impl AsRef<UpdateChatHasProtectedContent> for UpdateChatHasProtectedContent {
  fn as_ref(&self) -> &UpdateChatHasProtectedContent { self }
}

impl AsRef<UpdateChatHasProtectedContent> for RTDUpdateChatHasProtectedContentBuilder {
  fn as_ref(&self) -> &UpdateChatHasProtectedContent { &self.inner }
}







/// A chat's has_scheduled_messages field has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatHasScheduledMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New value of has_scheduled_messages
  has_scheduled_messages: bool,
  
}

impl RObject for UpdateChatHasScheduledMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatHasScheduledMessages" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatHasScheduledMessages {}



impl UpdateChatHasScheduledMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatHasScheduledMessagesBuilder {
    let mut inner = UpdateChatHasScheduledMessages::default();
    inner.td_name = "updateChatHasScheduledMessages".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatHasScheduledMessagesBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn has_scheduled_messages(&self) -> bool { self.has_scheduled_messages }

}

#[doc(hidden)]
pub struct RTDUpdateChatHasScheduledMessagesBuilder {
  inner: UpdateChatHasScheduledMessages
}

impl RTDUpdateChatHasScheduledMessagesBuilder {
  pub fn build(&self) -> UpdateChatHasScheduledMessages { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn has_scheduled_messages(&mut self, has_scheduled_messages: bool) -> &mut Self {
    self.inner.has_scheduled_messages = has_scheduled_messages;
    self
  }

}

impl AsRef<UpdateChatHasScheduledMessages> for UpdateChatHasScheduledMessages {
  fn as_ref(&self) -> &UpdateChatHasScheduledMessages { self }
}

impl AsRef<UpdateChatHasScheduledMessages> for RTDUpdateChatHasScheduledMessagesBuilder {
  fn as_ref(&self) -> &UpdateChatHasScheduledMessages { &self.inner }
}







/// A chat was blocked or unblocked
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatIsBlocked {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New value of is_blocked
  is_blocked: bool,
  
}

impl RObject for UpdateChatIsBlocked {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatIsBlocked" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatIsBlocked {}



impl UpdateChatIsBlocked {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatIsBlockedBuilder {
    let mut inner = UpdateChatIsBlocked::default();
    inner.td_name = "updateChatIsBlocked".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatIsBlockedBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn is_blocked(&self) -> bool { self.is_blocked }

}

#[doc(hidden)]
pub struct RTDUpdateChatIsBlockedBuilder {
  inner: UpdateChatIsBlocked
}

impl RTDUpdateChatIsBlockedBuilder {
  pub fn build(&self) -> UpdateChatIsBlocked { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn is_blocked(&mut self, is_blocked: bool) -> &mut Self {
    self.inner.is_blocked = is_blocked;
    self
  }

}

impl AsRef<UpdateChatIsBlocked> for UpdateChatIsBlocked {
  fn as_ref(&self) -> &UpdateChatIsBlocked { self }
}

impl AsRef<UpdateChatIsBlocked> for RTDUpdateChatIsBlockedBuilder {
  fn as_ref(&self) -> &UpdateChatIsBlocked { &self.inner }
}







/// A chat was marked as unread or was read
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatIsMarkedAsUnread {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New value of is_marked_as_unread
  is_marked_as_unread: bool,
  
}

impl RObject for UpdateChatIsMarkedAsUnread {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatIsMarkedAsUnread" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatIsMarkedAsUnread {}



impl UpdateChatIsMarkedAsUnread {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatIsMarkedAsUnreadBuilder {
    let mut inner = UpdateChatIsMarkedAsUnread::default();
    inner.td_name = "updateChatIsMarkedAsUnread".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatIsMarkedAsUnreadBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn is_marked_as_unread(&self) -> bool { self.is_marked_as_unread }

}

#[doc(hidden)]
pub struct RTDUpdateChatIsMarkedAsUnreadBuilder {
  inner: UpdateChatIsMarkedAsUnread
}

impl RTDUpdateChatIsMarkedAsUnreadBuilder {
  pub fn build(&self) -> UpdateChatIsMarkedAsUnread { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn is_marked_as_unread(&mut self, is_marked_as_unread: bool) -> &mut Self {
    self.inner.is_marked_as_unread = is_marked_as_unread;
    self
  }

}

impl AsRef<UpdateChatIsMarkedAsUnread> for UpdateChatIsMarkedAsUnread {
  fn as_ref(&self) -> &UpdateChatIsMarkedAsUnread { self }
}

impl AsRef<UpdateChatIsMarkedAsUnread> for RTDUpdateChatIsMarkedAsUnreadBuilder {
  fn as_ref(&self) -> &UpdateChatIsMarkedAsUnread { &self.inner }
}







/// The last message of a chat was changed. If last_message is null, then the last message in the chat became unknown. Some new unknown messages might be added to the chat in this case
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatLastMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The new last message in the chat; may be null
  last_message: Option<Message>,
  /// The new chat positions in the chat lists
  positions: Option<Vec<ChatPosition>>,
  
}

impl RObject for UpdateChatLastMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatLastMessage" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatLastMessage {}



impl UpdateChatLastMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatLastMessageBuilder {
    let mut inner = UpdateChatLastMessage::default();
    inner.td_name = "updateChatLastMessage".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatLastMessageBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn last_message(&self) -> &Option<Message> { &self.last_message }

  pub fn positions(&self) -> &Option<Vec<ChatPosition>> { &self.positions }

}

#[doc(hidden)]
pub struct RTDUpdateChatLastMessageBuilder {
  inner: UpdateChatLastMessage
}

impl RTDUpdateChatLastMessageBuilder {
  pub fn build(&self) -> UpdateChatLastMessage { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
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

}

impl AsRef<UpdateChatLastMessage> for UpdateChatLastMessage {
  fn as_ref(&self) -> &UpdateChatLastMessage { self }
}

impl AsRef<UpdateChatLastMessage> for RTDUpdateChatLastMessageBuilder {
  fn as_ref(&self) -> &UpdateChatLastMessage { &self.inner }
}







/// User rights changed in a chat; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatMember {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Identifier of the user, changing the rights
  actor_user_id: i64,
  /// Point in time (Unix timestamp) when the user rights was changed
  date: i64,
  /// If user has joined the chat using an invite link, the invite link; may be null
  invite_link: Option<ChatInviteLink>,
  /// Previous chat member
  old_chat_member: ChatMember,
  /// New chat member
  new_chat_member: ChatMember,
  
}

impl RObject for UpdateChatMember {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatMember" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatMember {}



impl UpdateChatMember {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatMemberBuilder {
    let mut inner = UpdateChatMember::default();
    inner.td_name = "updateChatMember".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatMemberBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn actor_user_id(&self) -> i64 { self.actor_user_id }

  pub fn date(&self) -> i64 { self.date }

  pub fn invite_link(&self) -> &Option<ChatInviteLink> { &self.invite_link }

  pub fn old_chat_member(&self) -> &ChatMember { &self.old_chat_member }

  pub fn new_chat_member(&self) -> &ChatMember { &self.new_chat_member }

}

#[doc(hidden)]
pub struct RTDUpdateChatMemberBuilder {
  inner: UpdateChatMember
}

impl RTDUpdateChatMemberBuilder {
  pub fn build(&self) -> UpdateChatMember { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn actor_user_id(&mut self, actor_user_id: i64) -> &mut Self {
    self.inner.actor_user_id = actor_user_id;
    self
  }

   
  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

   
  pub fn invite_link<T: AsRef<ChatInviteLink>>(&mut self, invite_link: T) -> &mut Self {
    self.inner.invite_link = Some(invite_link.as_ref().clone());
    self
  }

   
  pub fn old_chat_member<T: AsRef<ChatMember>>(&mut self, old_chat_member: T) -> &mut Self {
    self.inner.old_chat_member = old_chat_member.as_ref().clone();
    self
  }

   
  pub fn new_chat_member<T: AsRef<ChatMember>>(&mut self, new_chat_member: T) -> &mut Self {
    self.inner.new_chat_member = new_chat_member.as_ref().clone();
    self
  }

}

impl AsRef<UpdateChatMember> for UpdateChatMember {
  fn as_ref(&self) -> &UpdateChatMember { self }
}

impl AsRef<UpdateChatMember> for RTDUpdateChatMemberBuilder {
  fn as_ref(&self) -> &UpdateChatMember { &self.inner }
}







/// The message Time To Live setting for a chat was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatMessageTtlSetting {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New value of message_ttl_setting
  message_ttl_setting: i64,
  
}

impl RObject for UpdateChatMessageTtlSetting {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatMessageTtlSetting" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatMessageTtlSetting {}



impl UpdateChatMessageTtlSetting {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatMessageTtlSettingBuilder {
    let mut inner = UpdateChatMessageTtlSetting::default();
    inner.td_name = "updateChatMessageTtlSetting".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatMessageTtlSettingBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_ttl_setting(&self) -> i64 { self.message_ttl_setting }

}

#[doc(hidden)]
pub struct RTDUpdateChatMessageTtlSettingBuilder {
  inner: UpdateChatMessageTtlSetting
}

impl RTDUpdateChatMessageTtlSettingBuilder {
  pub fn build(&self) -> UpdateChatMessageTtlSetting { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_ttl_setting(&mut self, message_ttl_setting: i64) -> &mut Self {
    self.inner.message_ttl_setting = message_ttl_setting;
    self
  }

}

impl AsRef<UpdateChatMessageTtlSetting> for UpdateChatMessageTtlSetting {
  fn as_ref(&self) -> &UpdateChatMessageTtlSetting { self }
}

impl AsRef<UpdateChatMessageTtlSetting> for RTDUpdateChatMessageTtlSettingBuilder {
  fn as_ref(&self) -> &UpdateChatMessageTtlSetting { &self.inner }
}







/// Notification settings for a chat were changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatNotificationSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The new notification settings
  notification_settings: ChatNotificationSettings,
  
}

impl RObject for UpdateChatNotificationSettings {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatNotificationSettings" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatNotificationSettings {}



impl UpdateChatNotificationSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatNotificationSettingsBuilder {
    let mut inner = UpdateChatNotificationSettings::default();
    inner.td_name = "updateChatNotificationSettings".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatNotificationSettingsBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn notification_settings(&self) -> &ChatNotificationSettings { &self.notification_settings }

}

#[doc(hidden)]
pub struct RTDUpdateChatNotificationSettingsBuilder {
  inner: UpdateChatNotificationSettings
}

impl RTDUpdateChatNotificationSettingsBuilder {
  pub fn build(&self) -> UpdateChatNotificationSettings { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn notification_settings<T: AsRef<ChatNotificationSettings>>(&mut self, notification_settings: T) -> &mut Self {
    self.inner.notification_settings = notification_settings.as_ref().clone();
    self
  }

}

impl AsRef<UpdateChatNotificationSettings> for UpdateChatNotificationSettings {
  fn as_ref(&self) -> &UpdateChatNotificationSettings { self }
}

impl AsRef<UpdateChatNotificationSettings> for RTDUpdateChatNotificationSettingsBuilder {
  fn as_ref(&self) -> &UpdateChatNotificationSettings { &self.inner }
}







/// The number of online group members has changed. This update with non-zero count is sent only for currently opened chats. There is no guarantee that it will be sent just after the count has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatOnlineMemberCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat
  chat_id: i64,
  /// New number of online members in the chat, or 0 if unknown
  online_member_count: i64,
  
}

impl RObject for UpdateChatOnlineMemberCount {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatOnlineMemberCount" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatOnlineMemberCount {}



impl UpdateChatOnlineMemberCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatOnlineMemberCountBuilder {
    let mut inner = UpdateChatOnlineMemberCount::default();
    inner.td_name = "updateChatOnlineMemberCount".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatOnlineMemberCountBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn online_member_count(&self) -> i64 { self.online_member_count }

}

#[doc(hidden)]
pub struct RTDUpdateChatOnlineMemberCountBuilder {
  inner: UpdateChatOnlineMemberCount
}

impl RTDUpdateChatOnlineMemberCountBuilder {
  pub fn build(&self) -> UpdateChatOnlineMemberCount { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn online_member_count(&mut self, online_member_count: i64) -> &mut Self {
    self.inner.online_member_count = online_member_count;
    self
  }

}

impl AsRef<UpdateChatOnlineMemberCount> for UpdateChatOnlineMemberCount {
  fn as_ref(&self) -> &UpdateChatOnlineMemberCount { self }
}

impl AsRef<UpdateChatOnlineMemberCount> for RTDUpdateChatOnlineMemberCountBuilder {
  fn as_ref(&self) -> &UpdateChatOnlineMemberCount { &self.inner }
}







/// The chat pending join requests were changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatPendingJoinRequests {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The new data about pending join requests; may be null
  pending_join_requests: Option<ChatJoinRequestsInfo>,
  
}

impl RObject for UpdateChatPendingJoinRequests {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatPendingJoinRequests" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatPendingJoinRequests {}



impl UpdateChatPendingJoinRequests {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatPendingJoinRequestsBuilder {
    let mut inner = UpdateChatPendingJoinRequests::default();
    inner.td_name = "updateChatPendingJoinRequests".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatPendingJoinRequestsBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn pending_join_requests(&self) -> &Option<ChatJoinRequestsInfo> { &self.pending_join_requests }

}

#[doc(hidden)]
pub struct RTDUpdateChatPendingJoinRequestsBuilder {
  inner: UpdateChatPendingJoinRequests
}

impl RTDUpdateChatPendingJoinRequestsBuilder {
  pub fn build(&self) -> UpdateChatPendingJoinRequests { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn pending_join_requests<T: AsRef<ChatJoinRequestsInfo>>(&mut self, pending_join_requests: T) -> &mut Self {
    self.inner.pending_join_requests = Some(pending_join_requests.as_ref().clone());
    self
  }

}

impl AsRef<UpdateChatPendingJoinRequests> for UpdateChatPendingJoinRequests {
  fn as_ref(&self) -> &UpdateChatPendingJoinRequests { self }
}

impl AsRef<UpdateChatPendingJoinRequests> for RTDUpdateChatPendingJoinRequestsBuilder {
  fn as_ref(&self) -> &UpdateChatPendingJoinRequests { &self.inner }
}







/// Chat permissions was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatPermissions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The new chat permissions
  permissions: ChatPermissions,
  
}

impl RObject for UpdateChatPermissions {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatPermissions" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatPermissions {}



impl UpdateChatPermissions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatPermissionsBuilder {
    let mut inner = UpdateChatPermissions::default();
    inner.td_name = "updateChatPermissions".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatPermissionsBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn permissions(&self) -> &ChatPermissions { &self.permissions }

}

#[doc(hidden)]
pub struct RTDUpdateChatPermissionsBuilder {
  inner: UpdateChatPermissions
}

impl RTDUpdateChatPermissionsBuilder {
  pub fn build(&self) -> UpdateChatPermissions { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn permissions<T: AsRef<ChatPermissions>>(&mut self, permissions: T) -> &mut Self {
    self.inner.permissions = permissions.as_ref().clone();
    self
  }

}

impl AsRef<UpdateChatPermissions> for UpdateChatPermissions {
  fn as_ref(&self) -> &UpdateChatPermissions { self }
}

impl AsRef<UpdateChatPermissions> for RTDUpdateChatPermissionsBuilder {
  fn as_ref(&self) -> &UpdateChatPermissions { &self.inner }
}







/// A chat photo was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The new chat photo; may be null
  photo: Option<ChatPhotoInfo>,
  
}

impl RObject for UpdateChatPhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatPhoto" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatPhoto {}



impl UpdateChatPhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatPhotoBuilder {
    let mut inner = UpdateChatPhoto::default();
    inner.td_name = "updateChatPhoto".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatPhotoBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn photo(&self) -> &Option<ChatPhotoInfo> { &self.photo }

}

#[doc(hidden)]
pub struct RTDUpdateChatPhotoBuilder {
  inner: UpdateChatPhoto
}

impl RTDUpdateChatPhotoBuilder {
  pub fn build(&self) -> UpdateChatPhoto { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn photo<T: AsRef<ChatPhotoInfo>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = Some(photo.as_ref().clone());
    self
  }

}

impl AsRef<UpdateChatPhoto> for UpdateChatPhoto {
  fn as_ref(&self) -> &UpdateChatPhoto { self }
}

impl AsRef<UpdateChatPhoto> for RTDUpdateChatPhotoBuilder {
  fn as_ref(&self) -> &UpdateChatPhoto { &self.inner }
}







/// The position of a chat in a chat list has changed. Instead of this update updateChatLastMessage or updateChatDraftMessage might be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatPosition {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New chat position. If new order is 0, then the chat needs to be removed from the list
  position: ChatPosition,
  
}

impl RObject for UpdateChatPosition {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatPosition" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatPosition {}



impl UpdateChatPosition {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatPositionBuilder {
    let mut inner = UpdateChatPosition::default();
    inner.td_name = "updateChatPosition".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatPositionBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn position(&self) -> &ChatPosition { &self.position }

}

#[doc(hidden)]
pub struct RTDUpdateChatPositionBuilder {
  inner: UpdateChatPosition
}

impl RTDUpdateChatPositionBuilder {
  pub fn build(&self) -> UpdateChatPosition { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn position<T: AsRef<ChatPosition>>(&mut self, position: T) -> &mut Self {
    self.inner.position = position.as_ref().clone();
    self
  }

}

impl AsRef<UpdateChatPosition> for UpdateChatPosition {
  fn as_ref(&self) -> &UpdateChatPosition { self }
}

impl AsRef<UpdateChatPosition> for RTDUpdateChatPositionBuilder {
  fn as_ref(&self) -> &UpdateChatPosition { &self.inner }
}







/// Incoming messages were read or the number of unread messages has been changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatReadInbox {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Identifier of the last read incoming message
  last_read_inbox_message_id: i64,
  /// The number of unread messages left in the chat
  unread_count: i64,
  
}

impl RObject for UpdateChatReadInbox {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatReadInbox" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatReadInbox {}



impl UpdateChatReadInbox {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatReadInboxBuilder {
    let mut inner = UpdateChatReadInbox::default();
    inner.td_name = "updateChatReadInbox".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatReadInboxBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn last_read_inbox_message_id(&self) -> i64 { self.last_read_inbox_message_id }

  pub fn unread_count(&self) -> i64 { self.unread_count }

}

#[doc(hidden)]
pub struct RTDUpdateChatReadInboxBuilder {
  inner: UpdateChatReadInbox
}

impl RTDUpdateChatReadInboxBuilder {
  pub fn build(&self) -> UpdateChatReadInbox { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn last_read_inbox_message_id(&mut self, last_read_inbox_message_id: i64) -> &mut Self {
    self.inner.last_read_inbox_message_id = last_read_inbox_message_id;
    self
  }

   
  pub fn unread_count(&mut self, unread_count: i64) -> &mut Self {
    self.inner.unread_count = unread_count;
    self
  }

}

impl AsRef<UpdateChatReadInbox> for UpdateChatReadInbox {
  fn as_ref(&self) -> &UpdateChatReadInbox { self }
}

impl AsRef<UpdateChatReadInbox> for RTDUpdateChatReadInboxBuilder {
  fn as_ref(&self) -> &UpdateChatReadInbox { &self.inner }
}







/// Outgoing messages were read
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatReadOutbox {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Identifier of last read outgoing message
  last_read_outbox_message_id: i64,
  
}

impl RObject for UpdateChatReadOutbox {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatReadOutbox" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatReadOutbox {}



impl UpdateChatReadOutbox {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatReadOutboxBuilder {
    let mut inner = UpdateChatReadOutbox::default();
    inner.td_name = "updateChatReadOutbox".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatReadOutboxBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn last_read_outbox_message_id(&self) -> i64 { self.last_read_outbox_message_id }

}

#[doc(hidden)]
pub struct RTDUpdateChatReadOutboxBuilder {
  inner: UpdateChatReadOutbox
}

impl RTDUpdateChatReadOutboxBuilder {
  pub fn build(&self) -> UpdateChatReadOutbox { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn last_read_outbox_message_id(&mut self, last_read_outbox_message_id: i64) -> &mut Self {
    self.inner.last_read_outbox_message_id = last_read_outbox_message_id;
    self
  }

}

impl AsRef<UpdateChatReadOutbox> for UpdateChatReadOutbox {
  fn as_ref(&self) -> &UpdateChatReadOutbox { self }
}

impl AsRef<UpdateChatReadOutbox> for RTDUpdateChatReadOutboxBuilder {
  fn as_ref(&self) -> &UpdateChatReadOutbox { &self.inner }
}







/// The default chat reply markup was changed. Can occur because new messages with reply markup were received or because an old reply markup was hidden by the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatReplyMarkup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Identifier of the message from which reply markup needs to be used; 0 if there is no default custom reply markup in the chat
  reply_markup_message_id: i64,
  
}

impl RObject for UpdateChatReplyMarkup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatReplyMarkup" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatReplyMarkup {}



impl UpdateChatReplyMarkup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatReplyMarkupBuilder {
    let mut inner = UpdateChatReplyMarkup::default();
    inner.td_name = "updateChatReplyMarkup".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatReplyMarkupBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn reply_markup_message_id(&self) -> i64 { self.reply_markup_message_id }

}

#[doc(hidden)]
pub struct RTDUpdateChatReplyMarkupBuilder {
  inner: UpdateChatReplyMarkup
}

impl RTDUpdateChatReplyMarkupBuilder {
  pub fn build(&self) -> UpdateChatReplyMarkup { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn reply_markup_message_id(&mut self, reply_markup_message_id: i64) -> &mut Self {
    self.inner.reply_markup_message_id = reply_markup_message_id;
    self
  }

}

impl AsRef<UpdateChatReplyMarkup> for UpdateChatReplyMarkup {
  fn as_ref(&self) -> &UpdateChatReplyMarkup { self }
}

impl AsRef<UpdateChatReplyMarkup> for RTDUpdateChatReplyMarkupBuilder {
  fn as_ref(&self) -> &UpdateChatReplyMarkup { &self.inner }
}







/// The chat theme was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatTheme {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The new name of the chat theme; may be empty if theme was reset to default
  theme_name: String,
  
}

impl RObject for UpdateChatTheme {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatTheme" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatTheme {}



impl UpdateChatTheme {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatThemeBuilder {
    let mut inner = UpdateChatTheme::default();
    inner.td_name = "updateChatTheme".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatThemeBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn theme_name(&self) -> &String { &self.theme_name }

}

#[doc(hidden)]
pub struct RTDUpdateChatThemeBuilder {
  inner: UpdateChatTheme
}

impl RTDUpdateChatThemeBuilder {
  pub fn build(&self) -> UpdateChatTheme { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn theme_name<T: AsRef<str>>(&mut self, theme_name: T) -> &mut Self {
    self.inner.theme_name = theme_name.as_ref().to_string();
    self
  }

}

impl AsRef<UpdateChatTheme> for UpdateChatTheme {
  fn as_ref(&self) -> &UpdateChatTheme { self }
}

impl AsRef<UpdateChatTheme> for RTDUpdateChatThemeBuilder {
  fn as_ref(&self) -> &UpdateChatTheme { &self.inner }
}







/// The list of available chat themes has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatThemes {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new list of chat themes
  chat_themes: Vec<ChatTheme>,
  
}

impl RObject for UpdateChatThemes {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatThemes" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatThemes {}



impl UpdateChatThemes {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatThemesBuilder {
    let mut inner = UpdateChatThemes::default();
    inner.td_name = "updateChatThemes".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatThemesBuilder { inner }
  }

  pub fn chat_themes(&self) -> &Vec<ChatTheme> { &self.chat_themes }

}

#[doc(hidden)]
pub struct RTDUpdateChatThemesBuilder {
  inner: UpdateChatThemes
}

impl RTDUpdateChatThemesBuilder {
  pub fn build(&self) -> UpdateChatThemes { self.inner.clone() }

   
  pub fn chat_themes(&mut self, chat_themes: Vec<ChatTheme>) -> &mut Self {
    self.inner.chat_themes = chat_themes;
    self
  }

}

impl AsRef<UpdateChatThemes> for UpdateChatThemes {
  fn as_ref(&self) -> &UpdateChatThemes { self }
}

impl AsRef<UpdateChatThemes> for RTDUpdateChatThemesBuilder {
  fn as_ref(&self) -> &UpdateChatThemes { &self.inner }
}







/// The title of a chat was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatTitle {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The new chat title
  title: String,
  
}

impl RObject for UpdateChatTitle {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatTitle" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatTitle {}



impl UpdateChatTitle {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatTitleBuilder {
    let mut inner = UpdateChatTitle::default();
    inner.td_name = "updateChatTitle".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatTitleBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn title(&self) -> &String { &self.title }

}

#[doc(hidden)]
pub struct RTDUpdateChatTitleBuilder {
  inner: UpdateChatTitle
}

impl RTDUpdateChatTitleBuilder {
  pub fn build(&self) -> UpdateChatTitle { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

}

impl AsRef<UpdateChatTitle> for UpdateChatTitle {
  fn as_ref(&self) -> &UpdateChatTitle { self }
}

impl AsRef<UpdateChatTitle> for RTDUpdateChatTitleBuilder {
  fn as_ref(&self) -> &UpdateChatTitle { &self.inner }
}







/// The chat unread_mention_count has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatUnreadMentionCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The number of unread mention messages left in the chat
  unread_mention_count: i64,
  
}

impl RObject for UpdateChatUnreadMentionCount {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatUnreadMentionCount" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatUnreadMentionCount {}



impl UpdateChatUnreadMentionCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatUnreadMentionCountBuilder {
    let mut inner = UpdateChatUnreadMentionCount::default();
    inner.td_name = "updateChatUnreadMentionCount".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatUnreadMentionCountBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn unread_mention_count(&self) -> i64 { self.unread_mention_count }

}

#[doc(hidden)]
pub struct RTDUpdateChatUnreadMentionCountBuilder {
  inner: UpdateChatUnreadMentionCount
}

impl RTDUpdateChatUnreadMentionCountBuilder {
  pub fn build(&self) -> UpdateChatUnreadMentionCount { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn unread_mention_count(&mut self, unread_mention_count: i64) -> &mut Self {
    self.inner.unread_mention_count = unread_mention_count;
    self
  }

}

impl AsRef<UpdateChatUnreadMentionCount> for UpdateChatUnreadMentionCount {
  fn as_ref(&self) -> &UpdateChatUnreadMentionCount { self }
}

impl AsRef<UpdateChatUnreadMentionCount> for RTDUpdateChatUnreadMentionCountBuilder {
  fn as_ref(&self) -> &UpdateChatUnreadMentionCount { &self.inner }
}







/// A chat video chat state has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatVideoChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New value of video_chat
  video_chat: VideoChat,
  
}

impl RObject for UpdateChatVideoChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatVideoChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateChatVideoChat {}



impl UpdateChatVideoChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateChatVideoChatBuilder {
    let mut inner = UpdateChatVideoChat::default();
    inner.td_name = "updateChatVideoChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateChatVideoChatBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn video_chat(&self) -> &VideoChat { &self.video_chat }

}

#[doc(hidden)]
pub struct RTDUpdateChatVideoChatBuilder {
  inner: UpdateChatVideoChat
}

impl RTDUpdateChatVideoChatBuilder {
  pub fn build(&self) -> UpdateChatVideoChat { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn video_chat<T: AsRef<VideoChat>>(&mut self, video_chat: T) -> &mut Self {
    self.inner.video_chat = video_chat.as_ref().clone();
    self
  }

}

impl AsRef<UpdateChatVideoChat> for UpdateChatVideoChat {
  fn as_ref(&self) -> &UpdateChatVideoChat { self }
}

impl AsRef<UpdateChatVideoChat> for RTDUpdateChatVideoChatBuilder {
  fn as_ref(&self) -> &UpdateChatVideoChat { &self.inner }
}







/// The connection state has changed. This update must be used only to show a human-readable description of the connection state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateConnectionState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new connection state
  state: ConnectionState,
  
}

impl RObject for UpdateConnectionState {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateConnectionState" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateConnectionState {}



impl UpdateConnectionState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateConnectionStateBuilder {
    let mut inner = UpdateConnectionState::default();
    inner.td_name = "updateConnectionState".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateConnectionStateBuilder { inner }
  }

  pub fn state(&self) -> &ConnectionState { &self.state }

}

#[doc(hidden)]
pub struct RTDUpdateConnectionStateBuilder {
  inner: UpdateConnectionState
}

impl RTDUpdateConnectionStateBuilder {
  pub fn build(&self) -> UpdateConnectionState { self.inner.clone() }

   
  pub fn state<T: AsRef<ConnectionState>>(&mut self, state: T) -> &mut Self {
    self.inner.state = state.as_ref().clone();
    self
  }

}

impl AsRef<UpdateConnectionState> for UpdateConnectionState {
  fn as_ref(&self) -> &UpdateConnectionState { self }
}

impl AsRef<UpdateConnectionState> for RTDUpdateConnectionStateBuilder {
  fn as_ref(&self) -> &UpdateConnectionState { &self.inner }
}







/// Some messages were deleted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateDeleteMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Identifiers of the deleted messages
  message_ids: Vec<i64>,
  /// True, if the messages are permanently deleted by a user (as opposed to just becoming inaccessible)
  is_permanent: bool,
  /// True, if the messages are deleted only from the cache and can possibly be retrieved again in the future
  from_cache: bool,
  
}

impl RObject for UpdateDeleteMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateDeleteMessages" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateDeleteMessages {}



impl UpdateDeleteMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateDeleteMessagesBuilder {
    let mut inner = UpdateDeleteMessages::default();
    inner.td_name = "updateDeleteMessages".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateDeleteMessagesBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_ids(&self) -> &Vec<i64> { &self.message_ids }

  pub fn is_permanent(&self) -> bool { self.is_permanent }

  pub fn from_cache(&self) -> bool { self.from_cache }

}

#[doc(hidden)]
pub struct RTDUpdateDeleteMessagesBuilder {
  inner: UpdateDeleteMessages
}

impl RTDUpdateDeleteMessagesBuilder {
  pub fn build(&self) -> UpdateDeleteMessages { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self {
    self.inner.message_ids = message_ids;
    self
  }

   
  pub fn is_permanent(&mut self, is_permanent: bool) -> &mut Self {
    self.inner.is_permanent = is_permanent;
    self
  }

   
  pub fn from_cache(&mut self, from_cache: bool) -> &mut Self {
    self.inner.from_cache = from_cache;
    self
  }

}

impl AsRef<UpdateDeleteMessages> for UpdateDeleteMessages {
  fn as_ref(&self) -> &UpdateDeleteMessages { self }
}

impl AsRef<UpdateDeleteMessages> for RTDUpdateDeleteMessagesBuilder {
  fn as_ref(&self) -> &UpdateDeleteMessages { &self.inner }
}







/// The list of supported dice emojis has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateDiceEmojis {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new list of supported dice emojis
  emojis: Vec<String>,
  
}

impl RObject for UpdateDiceEmojis {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateDiceEmojis" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateDiceEmojis {}



impl UpdateDiceEmojis {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateDiceEmojisBuilder {
    let mut inner = UpdateDiceEmojis::default();
    inner.td_name = "updateDiceEmojis".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateDiceEmojisBuilder { inner }
  }

  pub fn emojis(&self) -> &Vec<String> { &self.emojis }

}

#[doc(hidden)]
pub struct RTDUpdateDiceEmojisBuilder {
  inner: UpdateDiceEmojis
}

impl RTDUpdateDiceEmojisBuilder {
  pub fn build(&self) -> UpdateDiceEmojis { self.inner.clone() }

   
  pub fn emojis(&mut self, emojis: Vec<String>) -> &mut Self {
    self.inner.emojis = emojis;
    self
  }

}

impl AsRef<UpdateDiceEmojis> for UpdateDiceEmojis {
  fn as_ref(&self) -> &UpdateDiceEmojis { self }
}

impl AsRef<UpdateDiceEmojis> for RTDUpdateDiceEmojisBuilder {
  fn as_ref(&self) -> &UpdateDiceEmojis { &self.inner }
}







/// The list of favorite stickers was updated
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateFavoriteStickers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new list of file identifiers of favorite stickers
  sticker_ids: Vec<i64>,
  
}

impl RObject for UpdateFavoriteStickers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateFavoriteStickers" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateFavoriteStickers {}



impl UpdateFavoriteStickers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateFavoriteStickersBuilder {
    let mut inner = UpdateFavoriteStickers::default();
    inner.td_name = "updateFavoriteStickers".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateFavoriteStickersBuilder { inner }
  }

  pub fn sticker_ids(&self) -> &Vec<i64> { &self.sticker_ids }

}

#[doc(hidden)]
pub struct RTDUpdateFavoriteStickersBuilder {
  inner: UpdateFavoriteStickers
}

impl RTDUpdateFavoriteStickersBuilder {
  pub fn build(&self) -> UpdateFavoriteStickers { self.inner.clone() }

   
  pub fn sticker_ids(&mut self, sticker_ids: Vec<i64>) -> &mut Self {
    self.inner.sticker_ids = sticker_ids;
    self
  }

}

impl AsRef<UpdateFavoriteStickers> for UpdateFavoriteStickers {
  fn as_ref(&self) -> &UpdateFavoriteStickers { self }
}

impl AsRef<UpdateFavoriteStickers> for RTDUpdateFavoriteStickersBuilder {
  fn as_ref(&self) -> &UpdateFavoriteStickers { &self.inner }
}







/// Information about a file was updated
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New data about the file
  file: File,
  
}

impl RObject for UpdateFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateFile" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateFile {}



impl UpdateFile {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateFileBuilder {
    let mut inner = UpdateFile::default();
    inner.td_name = "updateFile".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateFileBuilder { inner }
  }

  pub fn file(&self) -> &File { &self.file }

}

#[doc(hidden)]
pub struct RTDUpdateFileBuilder {
  inner: UpdateFile
}

impl RTDUpdateFileBuilder {
  pub fn build(&self) -> UpdateFile { self.inner.clone() }

   
  pub fn file<T: AsRef<File>>(&mut self, file: T) -> &mut Self {
    self.inner.file = file.as_ref().clone();
    self
  }

}

impl AsRef<UpdateFile> for UpdateFile {
  fn as_ref(&self) -> &UpdateFile { self }
}

impl AsRef<UpdateFile> for RTDUpdateFileBuilder {
  fn as_ref(&self) -> &UpdateFile { &self.inner }
}







/// The file generation process needs to be started by the application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateFileGenerationStart {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier for the generation process
  generation_id: isize,
  /// The path to a file from which a new file is generated; may be empty
  original_path: String,
  /// The path to a file that must be created and where the new file is generated
  destination_path: String,
  /// String specifying the conversion applied to the original file. If conversion is "#url#" than original_path contains an HTTP/HTTPS URL of a file, which must be downloaded by the application
  conversion: String,
  
}

impl RObject for UpdateFileGenerationStart {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateFileGenerationStart" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateFileGenerationStart {}



impl UpdateFileGenerationStart {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateFileGenerationStartBuilder {
    let mut inner = UpdateFileGenerationStart::default();
    inner.td_name = "updateFileGenerationStart".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateFileGenerationStartBuilder { inner }
  }

  pub fn generation_id(&self) -> isize { self.generation_id }

  pub fn original_path(&self) -> &String { &self.original_path }

  pub fn destination_path(&self) -> &String { &self.destination_path }

  pub fn conversion(&self) -> &String { &self.conversion }

}

#[doc(hidden)]
pub struct RTDUpdateFileGenerationStartBuilder {
  inner: UpdateFileGenerationStart
}

impl RTDUpdateFileGenerationStartBuilder {
  pub fn build(&self) -> UpdateFileGenerationStart { self.inner.clone() }

   
  pub fn generation_id(&mut self, generation_id: isize) -> &mut Self {
    self.inner.generation_id = generation_id;
    self
  }

   
  pub fn original_path<T: AsRef<str>>(&mut self, original_path: T) -> &mut Self {
    self.inner.original_path = original_path.as_ref().to_string();
    self
  }

   
  pub fn destination_path<T: AsRef<str>>(&mut self, destination_path: T) -> &mut Self {
    self.inner.destination_path = destination_path.as_ref().to_string();
    self
  }

   
  pub fn conversion<T: AsRef<str>>(&mut self, conversion: T) -> &mut Self {
    self.inner.conversion = conversion.as_ref().to_string();
    self
  }

}

impl AsRef<UpdateFileGenerationStart> for UpdateFileGenerationStart {
  fn as_ref(&self) -> &UpdateFileGenerationStart { self }
}

impl AsRef<UpdateFileGenerationStart> for RTDUpdateFileGenerationStartBuilder {
  fn as_ref(&self) -> &UpdateFileGenerationStart { &self.inner }
}







/// File generation is no longer needed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateFileGenerationStop {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier for the generation process
  generation_id: isize,
  
}

impl RObject for UpdateFileGenerationStop {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateFileGenerationStop" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateFileGenerationStop {}



impl UpdateFileGenerationStop {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateFileGenerationStopBuilder {
    let mut inner = UpdateFileGenerationStop::default();
    inner.td_name = "updateFileGenerationStop".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateFileGenerationStopBuilder { inner }
  }

  pub fn generation_id(&self) -> isize { self.generation_id }

}

#[doc(hidden)]
pub struct RTDUpdateFileGenerationStopBuilder {
  inner: UpdateFileGenerationStop
}

impl RTDUpdateFileGenerationStopBuilder {
  pub fn build(&self) -> UpdateFileGenerationStop { self.inner.clone() }

   
  pub fn generation_id(&mut self, generation_id: isize) -> &mut Self {
    self.inner.generation_id = generation_id;
    self
  }

}

impl AsRef<UpdateFileGenerationStop> for UpdateFileGenerationStop {
  fn as_ref(&self) -> &UpdateFileGenerationStop { self }
}

impl AsRef<UpdateFileGenerationStop> for RTDUpdateFileGenerationStopBuilder {
  fn as_ref(&self) -> &UpdateFileGenerationStop { &self.inner }
}







/// Information about a group call was updated
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateGroupCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New data about a group call
  group_call: GroupCall,
  
}

impl RObject for UpdateGroupCall {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateGroupCall" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateGroupCall {}



impl UpdateGroupCall {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateGroupCallBuilder {
    let mut inner = UpdateGroupCall::default();
    inner.td_name = "updateGroupCall".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateGroupCallBuilder { inner }
  }

  pub fn group_call(&self) -> &GroupCall { &self.group_call }

}

#[doc(hidden)]
pub struct RTDUpdateGroupCallBuilder {
  inner: UpdateGroupCall
}

impl RTDUpdateGroupCallBuilder {
  pub fn build(&self) -> UpdateGroupCall { self.inner.clone() }

   
  pub fn group_call<T: AsRef<GroupCall>>(&mut self, group_call: T) -> &mut Self {
    self.inner.group_call = group_call.as_ref().clone();
    self
  }

}

impl AsRef<UpdateGroupCall> for UpdateGroupCall {
  fn as_ref(&self) -> &UpdateGroupCall { self }
}

impl AsRef<UpdateGroupCall> for RTDUpdateGroupCallBuilder {
  fn as_ref(&self) -> &UpdateGroupCall { &self.inner }
}







/// Information about a group call participant was changed. The updates are sent only after the group call is received through getGroupCall and only if the call is joined or being joined
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateGroupCallParticipant {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of group call
  group_call_id: i64,
  /// New data about a participant
  participant: GroupCallParticipant,
  
}

impl RObject for UpdateGroupCallParticipant {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateGroupCallParticipant" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateGroupCallParticipant {}



impl UpdateGroupCallParticipant {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateGroupCallParticipantBuilder {
    let mut inner = UpdateGroupCallParticipant::default();
    inner.td_name = "updateGroupCallParticipant".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateGroupCallParticipantBuilder { inner }
  }

  pub fn group_call_id(&self) -> i64 { self.group_call_id }

  pub fn participant(&self) -> &GroupCallParticipant { &self.participant }

}

#[doc(hidden)]
pub struct RTDUpdateGroupCallParticipantBuilder {
  inner: UpdateGroupCallParticipant
}

impl RTDUpdateGroupCallParticipantBuilder {
  pub fn build(&self) -> UpdateGroupCallParticipant { self.inner.clone() }

   
  pub fn group_call_id(&mut self, group_call_id: i64) -> &mut Self {
    self.inner.group_call_id = group_call_id;
    self
  }

   
  pub fn participant<T: AsRef<GroupCallParticipant>>(&mut self, participant: T) -> &mut Self {
    self.inner.participant = participant.as_ref().clone();
    self
  }

}

impl AsRef<UpdateGroupCallParticipant> for UpdateGroupCallParticipant {
  fn as_ref(&self) -> &UpdateGroupCallParticipant { self }
}

impl AsRef<UpdateGroupCallParticipant> for RTDUpdateGroupCallParticipantBuilder {
  fn as_ref(&self) -> &UpdateGroupCallParticipant { &self.inner }
}







/// Describes whether there are some pending notification updates. Can be used to prevent application from killing, while there are some pending notifications
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateHavePendingNotifications {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if there are some delayed notification updates, which will be sent soon
  have_delayed_notifications: bool,
  /// True, if there can be some yet unreceived notifications, which are being fetched from the server
  have_unreceived_notifications: bool,
  
}

impl RObject for UpdateHavePendingNotifications {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateHavePendingNotifications" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateHavePendingNotifications {}



impl UpdateHavePendingNotifications {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateHavePendingNotificationsBuilder {
    let mut inner = UpdateHavePendingNotifications::default();
    inner.td_name = "updateHavePendingNotifications".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateHavePendingNotificationsBuilder { inner }
  }

  pub fn have_delayed_notifications(&self) -> bool { self.have_delayed_notifications }

  pub fn have_unreceived_notifications(&self) -> bool { self.have_unreceived_notifications }

}

#[doc(hidden)]
pub struct RTDUpdateHavePendingNotificationsBuilder {
  inner: UpdateHavePendingNotifications
}

impl RTDUpdateHavePendingNotificationsBuilder {
  pub fn build(&self) -> UpdateHavePendingNotifications { self.inner.clone() }

   
  pub fn have_delayed_notifications(&mut self, have_delayed_notifications: bool) -> &mut Self {
    self.inner.have_delayed_notifications = have_delayed_notifications;
    self
  }

   
  pub fn have_unreceived_notifications(&mut self, have_unreceived_notifications: bool) -> &mut Self {
    self.inner.have_unreceived_notifications = have_unreceived_notifications;
    self
  }

}

impl AsRef<UpdateHavePendingNotifications> for UpdateHavePendingNotifications {
  fn as_ref(&self) -> &UpdateHavePendingNotifications { self }
}

impl AsRef<UpdateHavePendingNotifications> for RTDUpdateHavePendingNotificationsBuilder {
  fn as_ref(&self) -> &UpdateHavePendingNotifications { &self.inner }
}







/// The list of installed sticker sets was updated
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateInstalledStickerSets {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the list of installed mask sticker sets was updated
  is_masks: bool,
  /// The new list of installed ordinary sticker sets
  sticker_set_ids: Vec<isize>,
  
}

impl RObject for UpdateInstalledStickerSets {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateInstalledStickerSets" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateInstalledStickerSets {}



impl UpdateInstalledStickerSets {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateInstalledStickerSetsBuilder {
    let mut inner = UpdateInstalledStickerSets::default();
    inner.td_name = "updateInstalledStickerSets".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateInstalledStickerSetsBuilder { inner }
  }

  pub fn is_masks(&self) -> bool { self.is_masks }

  pub fn sticker_set_ids(&self) -> &Vec<isize> { &self.sticker_set_ids }

}

#[doc(hidden)]
pub struct RTDUpdateInstalledStickerSetsBuilder {
  inner: UpdateInstalledStickerSets
}

impl RTDUpdateInstalledStickerSetsBuilder {
  pub fn build(&self) -> UpdateInstalledStickerSets { self.inner.clone() }

   
  pub fn is_masks(&mut self, is_masks: bool) -> &mut Self {
    self.inner.is_masks = is_masks;
    self
  }

   
  pub fn sticker_set_ids(&mut self, sticker_set_ids: Vec<isize>) -> &mut Self {
    self.inner.sticker_set_ids = sticker_set_ids;
    self
  }

}

impl AsRef<UpdateInstalledStickerSets> for UpdateInstalledStickerSets {
  fn as_ref(&self) -> &UpdateInstalledStickerSets { self }
}

impl AsRef<UpdateInstalledStickerSets> for RTDUpdateInstalledStickerSetsBuilder {
  fn as_ref(&self) -> &UpdateInstalledStickerSets { &self.inner }
}







/// Some language pack strings have been updated
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateLanguagePackStrings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Localization target to which the language pack belongs
  localization_target: String,
  /// Identifier of the updated language pack
  language_pack_id: String,
  /// List of changed language pack strings
  strings: Vec<LanguagePackString>,
  
}

impl RObject for UpdateLanguagePackStrings {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateLanguagePackStrings" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateLanguagePackStrings {}



impl UpdateLanguagePackStrings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateLanguagePackStringsBuilder {
    let mut inner = UpdateLanguagePackStrings::default();
    inner.td_name = "updateLanguagePackStrings".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateLanguagePackStringsBuilder { inner }
  }

  pub fn localization_target(&self) -> &String { &self.localization_target }

  pub fn language_pack_id(&self) -> &String { &self.language_pack_id }

  pub fn strings(&self) -> &Vec<LanguagePackString> { &self.strings }

}

#[doc(hidden)]
pub struct RTDUpdateLanguagePackStringsBuilder {
  inner: UpdateLanguagePackStrings
}

impl RTDUpdateLanguagePackStringsBuilder {
  pub fn build(&self) -> UpdateLanguagePackStrings { self.inner.clone() }

   
  pub fn localization_target<T: AsRef<str>>(&mut self, localization_target: T) -> &mut Self {
    self.inner.localization_target = localization_target.as_ref().to_string();
    self
  }

   
  pub fn language_pack_id<T: AsRef<str>>(&mut self, language_pack_id: T) -> &mut Self {
    self.inner.language_pack_id = language_pack_id.as_ref().to_string();
    self
  }

   
  pub fn strings(&mut self, strings: Vec<LanguagePackString>) -> &mut Self {
    self.inner.strings = strings;
    self
  }

}

impl AsRef<UpdateLanguagePackStrings> for UpdateLanguagePackStrings {
  fn as_ref(&self) -> &UpdateLanguagePackStrings { self }
}

impl AsRef<UpdateLanguagePackStrings> for RTDUpdateLanguagePackStringsBuilder {
  fn as_ref(&self) -> &UpdateLanguagePackStrings { &self.inner }
}







/// The message content has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageContent {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Message identifier
  message_id: i64,
  /// New message content
  new_content: MessageContent,
  
}

impl RObject for UpdateMessageContent {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateMessageContent" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateMessageContent {}



impl UpdateMessageContent {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateMessageContentBuilder {
    let mut inner = UpdateMessageContent::default();
    inner.td_name = "updateMessageContent".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateMessageContentBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn new_content(&self) -> &MessageContent { &self.new_content }

}

#[doc(hidden)]
pub struct RTDUpdateMessageContentBuilder {
  inner: UpdateMessageContent
}

impl RTDUpdateMessageContentBuilder {
  pub fn build(&self) -> UpdateMessageContent { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn new_content<T: AsRef<MessageContent>>(&mut self, new_content: T) -> &mut Self {
    self.inner.new_content = new_content.as_ref().clone();
    self
  }

}

impl AsRef<UpdateMessageContent> for UpdateMessageContent {
  fn as_ref(&self) -> &UpdateMessageContent { self }
}

impl AsRef<UpdateMessageContent> for RTDUpdateMessageContentBuilder {
  fn as_ref(&self) -> &UpdateMessageContent { &self.inner }
}







/// The message content was opened. Updates voice note messages to "listened", video note messages to "viewed" and starts the TTL timer for self-destructing messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageContentOpened {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Message identifier
  message_id: i64,
  
}

impl RObject for UpdateMessageContentOpened {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateMessageContentOpened" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateMessageContentOpened {}



impl UpdateMessageContentOpened {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateMessageContentOpenedBuilder {
    let mut inner = UpdateMessageContentOpened::default();
    inner.td_name = "updateMessageContentOpened".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateMessageContentOpenedBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

}

#[doc(hidden)]
pub struct RTDUpdateMessageContentOpenedBuilder {
  inner: UpdateMessageContentOpened
}

impl RTDUpdateMessageContentOpenedBuilder {
  pub fn build(&self) -> UpdateMessageContentOpened { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

}

impl AsRef<UpdateMessageContentOpened> for UpdateMessageContentOpened {
  fn as_ref(&self) -> &UpdateMessageContentOpened { self }
}

impl AsRef<UpdateMessageContentOpened> for RTDUpdateMessageContentOpenedBuilder {
  fn as_ref(&self) -> &UpdateMessageContentOpened { &self.inner }
}







/// A message was edited. Changes in the message content will come in a separate updateMessageContent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageEdited {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Message identifier
  message_id: i64,
  /// Point in time (Unix timestamp) when the message was edited
  edit_date: i64,
  /// New message reply markup; may be null
  reply_markup: Option<ReplyMarkup>,
  
}

impl RObject for UpdateMessageEdited {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateMessageEdited" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateMessageEdited {}



impl UpdateMessageEdited {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateMessageEditedBuilder {
    let mut inner = UpdateMessageEdited::default();
    inner.td_name = "updateMessageEdited".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateMessageEditedBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn edit_date(&self) -> i64 { self.edit_date }

  pub fn reply_markup(&self) -> &Option<ReplyMarkup> { &self.reply_markup }

}

#[doc(hidden)]
pub struct RTDUpdateMessageEditedBuilder {
  inner: UpdateMessageEdited
}

impl RTDUpdateMessageEditedBuilder {
  pub fn build(&self) -> UpdateMessageEdited { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn edit_date(&mut self, edit_date: i64) -> &mut Self {
    self.inner.edit_date = edit_date;
    self
  }

   
  pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
    self.inner.reply_markup = Some(reply_markup.as_ref().clone());
    self
  }

}

impl AsRef<UpdateMessageEdited> for UpdateMessageEdited {
  fn as_ref(&self) -> &UpdateMessageEdited { self }
}

impl AsRef<UpdateMessageEdited> for RTDUpdateMessageEditedBuilder {
  fn as_ref(&self) -> &UpdateMessageEdited { &self.inner }
}







/// The information about interactions with a message has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageInteractionInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Message identifier
  message_id: i64,
  /// New information about interactions with the message; may be null
  interaction_info: Option<MessageInteractionInfo>,
  
}

impl RObject for UpdateMessageInteractionInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateMessageInteractionInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateMessageInteractionInfo {}



impl UpdateMessageInteractionInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateMessageInteractionInfoBuilder {
    let mut inner = UpdateMessageInteractionInfo::default();
    inner.td_name = "updateMessageInteractionInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateMessageInteractionInfoBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn interaction_info(&self) -> &Option<MessageInteractionInfo> { &self.interaction_info }

}

#[doc(hidden)]
pub struct RTDUpdateMessageInteractionInfoBuilder {
  inner: UpdateMessageInteractionInfo
}

impl RTDUpdateMessageInteractionInfoBuilder {
  pub fn build(&self) -> UpdateMessageInteractionInfo { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn interaction_info<T: AsRef<MessageInteractionInfo>>(&mut self, interaction_info: T) -> &mut Self {
    self.inner.interaction_info = Some(interaction_info.as_ref().clone());
    self
  }

}

impl AsRef<UpdateMessageInteractionInfo> for UpdateMessageInteractionInfo {
  fn as_ref(&self) -> &UpdateMessageInteractionInfo { self }
}

impl AsRef<UpdateMessageInteractionInfo> for RTDUpdateMessageInteractionInfoBuilder {
  fn as_ref(&self) -> &UpdateMessageInteractionInfo { &self.inner }
}







/// The message pinned state was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageIsPinned {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The message identifier
  message_id: i64,
  /// True, if the message is pinned
  is_pinned: bool,
  
}

impl RObject for UpdateMessageIsPinned {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateMessageIsPinned" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateMessageIsPinned {}



impl UpdateMessageIsPinned {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateMessageIsPinnedBuilder {
    let mut inner = UpdateMessageIsPinned::default();
    inner.td_name = "updateMessageIsPinned".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateMessageIsPinnedBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn is_pinned(&self) -> bool { self.is_pinned }

}

#[doc(hidden)]
pub struct RTDUpdateMessageIsPinnedBuilder {
  inner: UpdateMessageIsPinned
}

impl RTDUpdateMessageIsPinnedBuilder {
  pub fn build(&self) -> UpdateMessageIsPinned { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }

}

impl AsRef<UpdateMessageIsPinned> for UpdateMessageIsPinned {
  fn as_ref(&self) -> &UpdateMessageIsPinned { self }
}

impl AsRef<UpdateMessageIsPinned> for RTDUpdateMessageIsPinnedBuilder {
  fn as_ref(&self) -> &UpdateMessageIsPinned { &self.inner }
}







/// A message with a live location was viewed. When the update is received, the application is supposed to update the live location
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageLiveLocationViewed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat with the live location message
  chat_id: i64,
  /// Identifier of the message with live location
  message_id: i64,
  
}

impl RObject for UpdateMessageLiveLocationViewed {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateMessageLiveLocationViewed" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateMessageLiveLocationViewed {}



impl UpdateMessageLiveLocationViewed {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateMessageLiveLocationViewedBuilder {
    let mut inner = UpdateMessageLiveLocationViewed::default();
    inner.td_name = "updateMessageLiveLocationViewed".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateMessageLiveLocationViewedBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

}

#[doc(hidden)]
pub struct RTDUpdateMessageLiveLocationViewedBuilder {
  inner: UpdateMessageLiveLocationViewed
}

impl RTDUpdateMessageLiveLocationViewedBuilder {
  pub fn build(&self) -> UpdateMessageLiveLocationViewed { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

}

impl AsRef<UpdateMessageLiveLocationViewed> for UpdateMessageLiveLocationViewed {
  fn as_ref(&self) -> &UpdateMessageLiveLocationViewed { self }
}

impl AsRef<UpdateMessageLiveLocationViewed> for RTDUpdateMessageLiveLocationViewedBuilder {
  fn as_ref(&self) -> &UpdateMessageLiveLocationViewed { &self.inner }
}







/// A message with an unread mention was read
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageMentionRead {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Message identifier
  message_id: i64,
  /// The new number of unread mention messages left in the chat
  unread_mention_count: i64,
  
}

impl RObject for UpdateMessageMentionRead {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateMessageMentionRead" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateMessageMentionRead {}



impl UpdateMessageMentionRead {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateMessageMentionReadBuilder {
    let mut inner = UpdateMessageMentionRead::default();
    inner.td_name = "updateMessageMentionRead".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateMessageMentionReadBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn unread_mention_count(&self) -> i64 { self.unread_mention_count }

}

#[doc(hidden)]
pub struct RTDUpdateMessageMentionReadBuilder {
  inner: UpdateMessageMentionRead
}

impl RTDUpdateMessageMentionReadBuilder {
  pub fn build(&self) -> UpdateMessageMentionRead { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn unread_mention_count(&mut self, unread_mention_count: i64) -> &mut Self {
    self.inner.unread_mention_count = unread_mention_count;
    self
  }

}

impl AsRef<UpdateMessageMentionRead> for UpdateMessageMentionRead {
  fn as_ref(&self) -> &UpdateMessageMentionRead { self }
}

impl AsRef<UpdateMessageMentionRead> for RTDUpdateMessageMentionReadBuilder {
  fn as_ref(&self) -> &UpdateMessageMentionRead { &self.inner }
}







/// A request to send a message has reached the Telegram server. This doesn't mean that the message will be sent successfully or even that the send message request will be processed. This update will be sent only if the option "use_quick_ack" is set to true. This update may be sent multiple times for the same message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageSendAcknowledged {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The chat identifier of the sent message
  chat_id: i64,
  /// A temporary message identifier
  message_id: i64,
  
}

impl RObject for UpdateMessageSendAcknowledged {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateMessageSendAcknowledged" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateMessageSendAcknowledged {}



impl UpdateMessageSendAcknowledged {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateMessageSendAcknowledgedBuilder {
    let mut inner = UpdateMessageSendAcknowledged::default();
    inner.td_name = "updateMessageSendAcknowledged".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateMessageSendAcknowledgedBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

}

#[doc(hidden)]
pub struct RTDUpdateMessageSendAcknowledgedBuilder {
  inner: UpdateMessageSendAcknowledged
}

impl RTDUpdateMessageSendAcknowledgedBuilder {
  pub fn build(&self) -> UpdateMessageSendAcknowledged { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

}

impl AsRef<UpdateMessageSendAcknowledged> for UpdateMessageSendAcknowledged {
  fn as_ref(&self) -> &UpdateMessageSendAcknowledged { self }
}

impl AsRef<UpdateMessageSendAcknowledged> for RTDUpdateMessageSendAcknowledgedBuilder {
  fn as_ref(&self) -> &UpdateMessageSendAcknowledged { &self.inner }
}







/// A message failed to send. Be aware that some messages being sent can be irrecoverably deleted, in which case updateDeleteMessages will be received instead of this update
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageSendFailed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The failed to send message
  message: Message,
  /// The previous temporary message identifier
  old_message_id: i64,
  /// An error code
  error_code: i64,
  /// Error message
  error_message: String,
  
}

impl RObject for UpdateMessageSendFailed {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateMessageSendFailed" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateMessageSendFailed {}



impl UpdateMessageSendFailed {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateMessageSendFailedBuilder {
    let mut inner = UpdateMessageSendFailed::default();
    inner.td_name = "updateMessageSendFailed".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateMessageSendFailedBuilder { inner }
  }

  pub fn message(&self) -> &Message { &self.message }

  pub fn old_message_id(&self) -> i64 { self.old_message_id }

  pub fn error_code(&self) -> i64 { self.error_code }

  pub fn error_message(&self) -> &String { &self.error_message }

}

#[doc(hidden)]
pub struct RTDUpdateMessageSendFailedBuilder {
  inner: UpdateMessageSendFailed
}

impl RTDUpdateMessageSendFailedBuilder {
  pub fn build(&self) -> UpdateMessageSendFailed { self.inner.clone() }

   
  pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().clone();
    self
  }

   
  pub fn old_message_id(&mut self, old_message_id: i64) -> &mut Self {
    self.inner.old_message_id = old_message_id;
    self
  }

   
  pub fn error_code(&mut self, error_code: i64) -> &mut Self {
    self.inner.error_code = error_code;
    self
  }

   
  pub fn error_message<T: AsRef<str>>(&mut self, error_message: T) -> &mut Self {
    self.inner.error_message = error_message.as_ref().to_string();
    self
  }

}

impl AsRef<UpdateMessageSendFailed> for UpdateMessageSendFailed {
  fn as_ref(&self) -> &UpdateMessageSendFailed { self }
}

impl AsRef<UpdateMessageSendFailed> for RTDUpdateMessageSendFailedBuilder {
  fn as_ref(&self) -> &UpdateMessageSendFailed { &self.inner }
}







/// A message has been successfully sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageSendSucceeded {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The sent message. Usually only the message identifier, date, and content are changed, but almost all other fields can also change
  message: Message,
  /// The previous temporary message identifier
  old_message_id: i64,
  
}

impl RObject for UpdateMessageSendSucceeded {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateMessageSendSucceeded" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateMessageSendSucceeded {}



impl UpdateMessageSendSucceeded {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateMessageSendSucceededBuilder {
    let mut inner = UpdateMessageSendSucceeded::default();
    inner.td_name = "updateMessageSendSucceeded".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateMessageSendSucceededBuilder { inner }
  }

  pub fn message(&self) -> &Message { &self.message }

  pub fn old_message_id(&self) -> i64 { self.old_message_id }

}

#[doc(hidden)]
pub struct RTDUpdateMessageSendSucceededBuilder {
  inner: UpdateMessageSendSucceeded
}

impl RTDUpdateMessageSendSucceededBuilder {
  pub fn build(&self) -> UpdateMessageSendSucceeded { self.inner.clone() }

   
  pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().clone();
    self
  }

   
  pub fn old_message_id(&mut self, old_message_id: i64) -> &mut Self {
    self.inner.old_message_id = old_message_id;
    self
  }

}

impl AsRef<UpdateMessageSendSucceeded> for UpdateMessageSendSucceeded {
  fn as_ref(&self) -> &UpdateMessageSendSucceeded { self }
}

impl AsRef<UpdateMessageSendSucceeded> for RTDUpdateMessageSendSucceededBuilder {
  fn as_ref(&self) -> &UpdateMessageSendSucceeded { &self.inner }
}







/// New call signaling data arrived
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewCallSignalingData {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The call identifier
  call_id: i64,
  /// The data
  data: String,
  
}

impl RObject for UpdateNewCallSignalingData {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateNewCallSignalingData" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateNewCallSignalingData {}



impl UpdateNewCallSignalingData {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateNewCallSignalingDataBuilder {
    let mut inner = UpdateNewCallSignalingData::default();
    inner.td_name = "updateNewCallSignalingData".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateNewCallSignalingDataBuilder { inner }
  }

  pub fn call_id(&self) -> i64 { self.call_id }

  pub fn data(&self) -> &String { &self.data }

}

#[doc(hidden)]
pub struct RTDUpdateNewCallSignalingDataBuilder {
  inner: UpdateNewCallSignalingData
}

impl RTDUpdateNewCallSignalingDataBuilder {
  pub fn build(&self) -> UpdateNewCallSignalingData { self.inner.clone() }

   
  pub fn call_id(&mut self, call_id: i64) -> &mut Self {
    self.inner.call_id = call_id;
    self
  }

   
  pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
    self.inner.data = data.as_ref().to_string();
    self
  }

}

impl AsRef<UpdateNewCallSignalingData> for UpdateNewCallSignalingData {
  fn as_ref(&self) -> &UpdateNewCallSignalingData { self }
}

impl AsRef<UpdateNewCallSignalingData> for RTDUpdateNewCallSignalingDataBuilder {
  fn as_ref(&self) -> &UpdateNewCallSignalingData { &self.inner }
}







/// A new incoming callback query; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewCallbackQuery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique query identifier
  id: isize,
  /// Identifier of the user who sent the query
  sender_user_id: i64,
  /// Identifier of the chat where the query was sent
  chat_id: i64,
  /// Identifier of the message, from which the query originated
  message_id: i64,
  /// Identifier that uniquely corresponds to the chat to which the message was sent
  chat_instance: isize,
  /// Query payload
  payload: CallbackQueryPayload,
  
}

impl RObject for UpdateNewCallbackQuery {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateNewCallbackQuery" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateNewCallbackQuery {}



impl UpdateNewCallbackQuery {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateNewCallbackQueryBuilder {
    let mut inner = UpdateNewCallbackQuery::default();
    inner.td_name = "updateNewCallbackQuery".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateNewCallbackQueryBuilder { inner }
  }

  pub fn id(&self) -> isize { self.id }

  pub fn sender_user_id(&self) -> i64 { self.sender_user_id }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn chat_instance(&self) -> isize { self.chat_instance }

  pub fn payload(&self) -> &CallbackQueryPayload { &self.payload }

}

#[doc(hidden)]
pub struct RTDUpdateNewCallbackQueryBuilder {
  inner: UpdateNewCallbackQuery
}

impl RTDUpdateNewCallbackQueryBuilder {
  pub fn build(&self) -> UpdateNewCallbackQuery { self.inner.clone() }

   
  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn sender_user_id(&mut self, sender_user_id: i64) -> &mut Self {
    self.inner.sender_user_id = sender_user_id;
    self
  }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn chat_instance(&mut self, chat_instance: isize) -> &mut Self {
    self.inner.chat_instance = chat_instance;
    self
  }

   
  pub fn payload<T: AsRef<CallbackQueryPayload>>(&mut self, payload: T) -> &mut Self {
    self.inner.payload = payload.as_ref().clone();
    self
  }

}

impl AsRef<UpdateNewCallbackQuery> for UpdateNewCallbackQuery {
  fn as_ref(&self) -> &UpdateNewCallbackQuery { self }
}

impl AsRef<UpdateNewCallbackQuery> for RTDUpdateNewCallbackQueryBuilder {
  fn as_ref(&self) -> &UpdateNewCallbackQuery { &self.inner }
}







/// A new chat has been loaded/created. This update is guaranteed to come before the chat identifier is returned to the application. The chat field changes will be reported through separate updates
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The chat
  chat: Chat,
  
}

impl RObject for UpdateNewChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateNewChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateNewChat {}



impl UpdateNewChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateNewChatBuilder {
    let mut inner = UpdateNewChat::default();
    inner.td_name = "updateNewChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateNewChatBuilder { inner }
  }

  pub fn chat(&self) -> &Chat { &self.chat }

}

#[doc(hidden)]
pub struct RTDUpdateNewChatBuilder {
  inner: UpdateNewChat
}

impl RTDUpdateNewChatBuilder {
  pub fn build(&self) -> UpdateNewChat { self.inner.clone() }

   
  pub fn chat<T: AsRef<Chat>>(&mut self, chat: T) -> &mut Self {
    self.inner.chat = chat.as_ref().clone();
    self
  }

}

impl AsRef<UpdateNewChat> for UpdateNewChat {
  fn as_ref(&self) -> &UpdateNewChat { self }
}

impl AsRef<UpdateNewChat> for RTDUpdateNewChatBuilder {
  fn as_ref(&self) -> &UpdateNewChat { &self.inner }
}







/// A user sent a join request to a chat; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewChatJoinRequest {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Join request
  request: ChatJoinRequest,
  /// The invite link, which was used to send join request; may be null
  invite_link: Option<ChatInviteLink>,
  
}

impl RObject for UpdateNewChatJoinRequest {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateNewChatJoinRequest" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateNewChatJoinRequest {}



impl UpdateNewChatJoinRequest {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateNewChatJoinRequestBuilder {
    let mut inner = UpdateNewChatJoinRequest::default();
    inner.td_name = "updateNewChatJoinRequest".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateNewChatJoinRequestBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn request(&self) -> &ChatJoinRequest { &self.request }

  pub fn invite_link(&self) -> &Option<ChatInviteLink> { &self.invite_link }

}

#[doc(hidden)]
pub struct RTDUpdateNewChatJoinRequestBuilder {
  inner: UpdateNewChatJoinRequest
}

impl RTDUpdateNewChatJoinRequestBuilder {
  pub fn build(&self) -> UpdateNewChatJoinRequest { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn request<T: AsRef<ChatJoinRequest>>(&mut self, request: T) -> &mut Self {
    self.inner.request = request.as_ref().clone();
    self
  }

   
  pub fn invite_link<T: AsRef<ChatInviteLink>>(&mut self, invite_link: T) -> &mut Self {
    self.inner.invite_link = Some(invite_link.as_ref().clone());
    self
  }

}

impl AsRef<UpdateNewChatJoinRequest> for UpdateNewChatJoinRequest {
  fn as_ref(&self) -> &UpdateNewChatJoinRequest { self }
}

impl AsRef<UpdateNewChatJoinRequest> for RTDUpdateNewChatJoinRequestBuilder {
  fn as_ref(&self) -> &UpdateNewChatJoinRequest { &self.inner }
}







/// The user has chosen a result of an inline query; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewChosenInlineResult {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user who sent the query
  sender_user_id: i64,
  /// User location; may be null
  user_location: Option<Location>,
  /// Text of the query
  query: String,
  /// Identifier of the chosen result
  result_id: String,
  /// Identifier of the sent inline message, if known
  inline_message_id: String,
  
}

impl RObject for UpdateNewChosenInlineResult {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateNewChosenInlineResult" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateNewChosenInlineResult {}



impl UpdateNewChosenInlineResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateNewChosenInlineResultBuilder {
    let mut inner = UpdateNewChosenInlineResult::default();
    inner.td_name = "updateNewChosenInlineResult".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateNewChosenInlineResultBuilder { inner }
  }

  pub fn sender_user_id(&self) -> i64 { self.sender_user_id }

  pub fn user_location(&self) -> &Option<Location> { &self.user_location }

  pub fn query(&self) -> &String { &self.query }

  pub fn result_id(&self) -> &String { &self.result_id }

  pub fn inline_message_id(&self) -> &String { &self.inline_message_id }

}

#[doc(hidden)]
pub struct RTDUpdateNewChosenInlineResultBuilder {
  inner: UpdateNewChosenInlineResult
}

impl RTDUpdateNewChosenInlineResultBuilder {
  pub fn build(&self) -> UpdateNewChosenInlineResult { self.inner.clone() }

   
  pub fn sender_user_id(&mut self, sender_user_id: i64) -> &mut Self {
    self.inner.sender_user_id = sender_user_id;
    self
  }

   
  pub fn user_location<T: AsRef<Location>>(&mut self, user_location: T) -> &mut Self {
    self.inner.user_location = Some(user_location.as_ref().clone());
    self
  }

   
  pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
    self.inner.query = query.as_ref().to_string();
    self
  }

   
  pub fn result_id<T: AsRef<str>>(&mut self, result_id: T) -> &mut Self {
    self.inner.result_id = result_id.as_ref().to_string();
    self
  }

   
  pub fn inline_message_id<T: AsRef<str>>(&mut self, inline_message_id: T) -> &mut Self {
    self.inner.inline_message_id = inline_message_id.as_ref().to_string();
    self
  }

}

impl AsRef<UpdateNewChosenInlineResult> for UpdateNewChosenInlineResult {
  fn as_ref(&self) -> &UpdateNewChosenInlineResult { self }
}

impl AsRef<UpdateNewChosenInlineResult> for RTDUpdateNewChosenInlineResultBuilder {
  fn as_ref(&self) -> &UpdateNewChosenInlineResult { &self.inner }
}







/// A new incoming event; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewCustomEvent {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A JSON-serialized event
  event: String,
  
}

impl RObject for UpdateNewCustomEvent {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateNewCustomEvent" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateNewCustomEvent {}



impl UpdateNewCustomEvent {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateNewCustomEventBuilder {
    let mut inner = UpdateNewCustomEvent::default();
    inner.td_name = "updateNewCustomEvent".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateNewCustomEventBuilder { inner }
  }

  pub fn event(&self) -> &String { &self.event }

}

#[doc(hidden)]
pub struct RTDUpdateNewCustomEventBuilder {
  inner: UpdateNewCustomEvent
}

impl RTDUpdateNewCustomEventBuilder {
  pub fn build(&self) -> UpdateNewCustomEvent { self.inner.clone() }

   
  pub fn event<T: AsRef<str>>(&mut self, event: T) -> &mut Self {
    self.inner.event = event.as_ref().to_string();
    self
  }

}

impl AsRef<UpdateNewCustomEvent> for UpdateNewCustomEvent {
  fn as_ref(&self) -> &UpdateNewCustomEvent { self }
}

impl AsRef<UpdateNewCustomEvent> for RTDUpdateNewCustomEventBuilder {
  fn as_ref(&self) -> &UpdateNewCustomEvent { &self.inner }
}







/// A new incoming query; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewCustomQuery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The query identifier
  id: isize,
  /// JSON-serialized query data
  data: String,
  /// Query timeout
  timeout: i64,
  
}

impl RObject for UpdateNewCustomQuery {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateNewCustomQuery" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateNewCustomQuery {}



impl UpdateNewCustomQuery {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateNewCustomQueryBuilder {
    let mut inner = UpdateNewCustomQuery::default();
    inner.td_name = "updateNewCustomQuery".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateNewCustomQueryBuilder { inner }
  }

  pub fn id(&self) -> isize { self.id }

  pub fn data(&self) -> &String { &self.data }

  pub fn timeout(&self) -> i64 { self.timeout }

}

#[doc(hidden)]
pub struct RTDUpdateNewCustomQueryBuilder {
  inner: UpdateNewCustomQuery
}

impl RTDUpdateNewCustomQueryBuilder {
  pub fn build(&self) -> UpdateNewCustomQuery { self.inner.clone() }

   
  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
    self.inner.data = data.as_ref().to_string();
    self
  }

   
  pub fn timeout(&mut self, timeout: i64) -> &mut Self {
    self.inner.timeout = timeout;
    self
  }

}

impl AsRef<UpdateNewCustomQuery> for UpdateNewCustomQuery {
  fn as_ref(&self) -> &UpdateNewCustomQuery { self }
}

impl AsRef<UpdateNewCustomQuery> for RTDUpdateNewCustomQueryBuilder {
  fn as_ref(&self) -> &UpdateNewCustomQuery { &self.inner }
}







/// A new incoming callback query from a message sent via a bot; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewInlineCallbackQuery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique query identifier
  id: isize,
  /// Identifier of the user who sent the query
  sender_user_id: i64,
  /// Identifier of the inline message, from which the query originated
  inline_message_id: String,
  /// An identifier uniquely corresponding to the chat a message was sent to
  chat_instance: isize,
  /// Query payload
  payload: CallbackQueryPayload,
  
}

impl RObject for UpdateNewInlineCallbackQuery {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateNewInlineCallbackQuery" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateNewInlineCallbackQuery {}



impl UpdateNewInlineCallbackQuery {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateNewInlineCallbackQueryBuilder {
    let mut inner = UpdateNewInlineCallbackQuery::default();
    inner.td_name = "updateNewInlineCallbackQuery".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateNewInlineCallbackQueryBuilder { inner }
  }

  pub fn id(&self) -> isize { self.id }

  pub fn sender_user_id(&self) -> i64 { self.sender_user_id }

  pub fn inline_message_id(&self) -> &String { &self.inline_message_id }

  pub fn chat_instance(&self) -> isize { self.chat_instance }

  pub fn payload(&self) -> &CallbackQueryPayload { &self.payload }

}

#[doc(hidden)]
pub struct RTDUpdateNewInlineCallbackQueryBuilder {
  inner: UpdateNewInlineCallbackQuery
}

impl RTDUpdateNewInlineCallbackQueryBuilder {
  pub fn build(&self) -> UpdateNewInlineCallbackQuery { self.inner.clone() }

   
  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn sender_user_id(&mut self, sender_user_id: i64) -> &mut Self {
    self.inner.sender_user_id = sender_user_id;
    self
  }

   
  pub fn inline_message_id<T: AsRef<str>>(&mut self, inline_message_id: T) -> &mut Self {
    self.inner.inline_message_id = inline_message_id.as_ref().to_string();
    self
  }

   
  pub fn chat_instance(&mut self, chat_instance: isize) -> &mut Self {
    self.inner.chat_instance = chat_instance;
    self
  }

   
  pub fn payload<T: AsRef<CallbackQueryPayload>>(&mut self, payload: T) -> &mut Self {
    self.inner.payload = payload.as_ref().clone();
    self
  }

}

impl AsRef<UpdateNewInlineCallbackQuery> for UpdateNewInlineCallbackQuery {
  fn as_ref(&self) -> &UpdateNewInlineCallbackQuery { self }
}

impl AsRef<UpdateNewInlineCallbackQuery> for RTDUpdateNewInlineCallbackQueryBuilder {
  fn as_ref(&self) -> &UpdateNewInlineCallbackQuery { &self.inner }
}







/// A new incoming inline query; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewInlineQuery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique query identifier
  id: isize,
  /// Identifier of the user who sent the query
  sender_user_id: i64,
  /// User location; may be null
  user_location: Option<Location>,
  /// The type of the chat, from which the query originated; may be null if unknown
  chat_type: Option<ChatType>,
  /// Text of the query
  query: String,
  /// Offset of the first entry to return
  offset: String,
  
}

impl RObject for UpdateNewInlineQuery {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateNewInlineQuery" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateNewInlineQuery {}



impl UpdateNewInlineQuery {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateNewInlineQueryBuilder {
    let mut inner = UpdateNewInlineQuery::default();
    inner.td_name = "updateNewInlineQuery".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateNewInlineQueryBuilder { inner }
  }

  pub fn id(&self) -> isize { self.id }

  pub fn sender_user_id(&self) -> i64 { self.sender_user_id }

  pub fn user_location(&self) -> &Option<Location> { &self.user_location }

  pub fn chat_type(&self) -> &Option<ChatType> { &self.chat_type }

  pub fn query(&self) -> &String { &self.query }

  pub fn offset(&self) -> &String { &self.offset }

}

#[doc(hidden)]
pub struct RTDUpdateNewInlineQueryBuilder {
  inner: UpdateNewInlineQuery
}

impl RTDUpdateNewInlineQueryBuilder {
  pub fn build(&self) -> UpdateNewInlineQuery { self.inner.clone() }

   
  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn sender_user_id(&mut self, sender_user_id: i64) -> &mut Self {
    self.inner.sender_user_id = sender_user_id;
    self
  }

   
  pub fn user_location<T: AsRef<Location>>(&mut self, user_location: T) -> &mut Self {
    self.inner.user_location = Some(user_location.as_ref().clone());
    self
  }

   
  pub fn chat_type<T: AsRef<ChatType>>(&mut self, chat_type: T) -> &mut Self {
    self.inner.chat_type = Some(chat_type.as_ref().clone());
    self
  }

   
  pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
    self.inner.query = query.as_ref().to_string();
    self
  }

   
  pub fn offset<T: AsRef<str>>(&mut self, offset: T) -> &mut Self {
    self.inner.offset = offset.as_ref().to_string();
    self
  }

}

impl AsRef<UpdateNewInlineQuery> for UpdateNewInlineQuery {
  fn as_ref(&self) -> &UpdateNewInlineQuery { self }
}

impl AsRef<UpdateNewInlineQuery> for RTDUpdateNewInlineQueryBuilder {
  fn as_ref(&self) -> &UpdateNewInlineQuery { &self.inner }
}







/// A new message was received; can also be an outgoing message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new message
  message: Message,
  
}

impl RObject for UpdateNewMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateNewMessage" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateNewMessage {}



impl UpdateNewMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateNewMessageBuilder {
    let mut inner = UpdateNewMessage::default();
    inner.td_name = "updateNewMessage".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateNewMessageBuilder { inner }
  }

  pub fn message(&self) -> &Message { &self.message }

}

#[doc(hidden)]
pub struct RTDUpdateNewMessageBuilder {
  inner: UpdateNewMessage
}

impl RTDUpdateNewMessageBuilder {
  pub fn build(&self) -> UpdateNewMessage { self.inner.clone() }

   
  pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().clone();
    self
  }

}

impl AsRef<UpdateNewMessage> for UpdateNewMessage {
  fn as_ref(&self) -> &UpdateNewMessage { self }
}

impl AsRef<UpdateNewMessage> for RTDUpdateNewMessageBuilder {
  fn as_ref(&self) -> &UpdateNewMessage { &self.inner }
}







/// A new incoming pre-checkout query; for bots only. Contains full information about a checkout
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewPreCheckoutQuery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique query identifier
  id: isize,
  /// Identifier of the user who sent the query
  sender_user_id: i64,
  /// Currency for the product price
  currency: String,
  /// Total price for the product, in the smallest units of the currency
  total_amount: i64,
  /// Invoice payload
  invoice_payload: String,
  /// Identifier of a shipping option chosen by the user; may be empty if not applicable
  shipping_option_id: String,
  /// Information about the order; may be null
  order_info: Option<OrderInfo>,
  
}

impl RObject for UpdateNewPreCheckoutQuery {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateNewPreCheckoutQuery" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateNewPreCheckoutQuery {}



impl UpdateNewPreCheckoutQuery {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateNewPreCheckoutQueryBuilder {
    let mut inner = UpdateNewPreCheckoutQuery::default();
    inner.td_name = "updateNewPreCheckoutQuery".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateNewPreCheckoutQueryBuilder { inner }
  }

  pub fn id(&self) -> isize { self.id }

  pub fn sender_user_id(&self) -> i64 { self.sender_user_id }

  pub fn currency(&self) -> &String { &self.currency }

  pub fn total_amount(&self) -> i64 { self.total_amount }

  pub fn invoice_payload(&self) -> &String { &self.invoice_payload }

  pub fn shipping_option_id(&self) -> &String { &self.shipping_option_id }

  pub fn order_info(&self) -> &Option<OrderInfo> { &self.order_info }

}

#[doc(hidden)]
pub struct RTDUpdateNewPreCheckoutQueryBuilder {
  inner: UpdateNewPreCheckoutQuery
}

impl RTDUpdateNewPreCheckoutQueryBuilder {
  pub fn build(&self) -> UpdateNewPreCheckoutQuery { self.inner.clone() }

   
  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn sender_user_id(&mut self, sender_user_id: i64) -> &mut Self {
    self.inner.sender_user_id = sender_user_id;
    self
  }

   
  pub fn currency<T: AsRef<str>>(&mut self, currency: T) -> &mut Self {
    self.inner.currency = currency.as_ref().to_string();
    self
  }

   
  pub fn total_amount(&mut self, total_amount: i64) -> &mut Self {
    self.inner.total_amount = total_amount;
    self
  }

   
  pub fn invoice_payload<T: AsRef<str>>(&mut self, invoice_payload: T) -> &mut Self {
    self.inner.invoice_payload = invoice_payload.as_ref().to_string();
    self
  }

   
  pub fn shipping_option_id<T: AsRef<str>>(&mut self, shipping_option_id: T) -> &mut Self {
    self.inner.shipping_option_id = shipping_option_id.as_ref().to_string();
    self
  }

   
  pub fn order_info<T: AsRef<OrderInfo>>(&mut self, order_info: T) -> &mut Self {
    self.inner.order_info = Some(order_info.as_ref().clone());
    self
  }

}

impl AsRef<UpdateNewPreCheckoutQuery> for UpdateNewPreCheckoutQuery {
  fn as_ref(&self) -> &UpdateNewPreCheckoutQuery { self }
}

impl AsRef<UpdateNewPreCheckoutQuery> for RTDUpdateNewPreCheckoutQueryBuilder {
  fn as_ref(&self) -> &UpdateNewPreCheckoutQuery { &self.inner }
}







/// A new incoming shipping query; for bots only. Only for invoices with flexible price
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewShippingQuery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique query identifier
  id: isize,
  /// Identifier of the user who sent the query
  sender_user_id: i64,
  /// Invoice payload
  invoice_payload: String,
  /// User shipping address
  shipping_address: Address,
  
}

impl RObject for UpdateNewShippingQuery {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateNewShippingQuery" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateNewShippingQuery {}



impl UpdateNewShippingQuery {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateNewShippingQueryBuilder {
    let mut inner = UpdateNewShippingQuery::default();
    inner.td_name = "updateNewShippingQuery".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateNewShippingQueryBuilder { inner }
  }

  pub fn id(&self) -> isize { self.id }

  pub fn sender_user_id(&self) -> i64 { self.sender_user_id }

  pub fn invoice_payload(&self) -> &String { &self.invoice_payload }

  pub fn shipping_address(&self) -> &Address { &self.shipping_address }

}

#[doc(hidden)]
pub struct RTDUpdateNewShippingQueryBuilder {
  inner: UpdateNewShippingQuery
}

impl RTDUpdateNewShippingQueryBuilder {
  pub fn build(&self) -> UpdateNewShippingQuery { self.inner.clone() }

   
  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn sender_user_id(&mut self, sender_user_id: i64) -> &mut Self {
    self.inner.sender_user_id = sender_user_id;
    self
  }

   
  pub fn invoice_payload<T: AsRef<str>>(&mut self, invoice_payload: T) -> &mut Self {
    self.inner.invoice_payload = invoice_payload.as_ref().to_string();
    self
  }

   
  pub fn shipping_address<T: AsRef<Address>>(&mut self, shipping_address: T) -> &mut Self {
    self.inner.shipping_address = shipping_address.as_ref().clone();
    self
  }

}

impl AsRef<UpdateNewShippingQuery> for UpdateNewShippingQuery {
  fn as_ref(&self) -> &UpdateNewShippingQuery { self }
}

impl AsRef<UpdateNewShippingQuery> for RTDUpdateNewShippingQueryBuilder {
  fn as_ref(&self) -> &UpdateNewShippingQuery { &self.inner }
}







/// A notification was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNotification {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique notification group identifier
  notification_group_id: i64,
  /// Changed notification
  notification: Notification,
  
}

impl RObject for UpdateNotification {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateNotification" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateNotification {}



impl UpdateNotification {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateNotificationBuilder {
    let mut inner = UpdateNotification::default();
    inner.td_name = "updateNotification".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateNotificationBuilder { inner }
  }

  pub fn notification_group_id(&self) -> i64 { self.notification_group_id }

  pub fn notification(&self) -> &Notification { &self.notification }

}

#[doc(hidden)]
pub struct RTDUpdateNotificationBuilder {
  inner: UpdateNotification
}

impl RTDUpdateNotificationBuilder {
  pub fn build(&self) -> UpdateNotification { self.inner.clone() }

   
  pub fn notification_group_id(&mut self, notification_group_id: i64) -> &mut Self {
    self.inner.notification_group_id = notification_group_id;
    self
  }

   
  pub fn notification<T: AsRef<Notification>>(&mut self, notification: T) -> &mut Self {
    self.inner.notification = notification.as_ref().clone();
    self
  }

}

impl AsRef<UpdateNotification> for UpdateNotification {
  fn as_ref(&self) -> &UpdateNotification { self }
}

impl AsRef<UpdateNotification> for RTDUpdateNotificationBuilder {
  fn as_ref(&self) -> &UpdateNotification { &self.inner }
}







/// A list of active notifications in a notification group has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNotificationGroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique notification group identifier
  notification_group_id: i64,
  /// New type of the notification group
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: NotificationGroupType,
  /// Identifier of a chat to which all notifications in the group belong
  chat_id: i64,
  /// Chat identifier, which notification settings must be applied to the added notifications
  notification_settings_chat_id: i64,
  /// True, if the notifications must be shown without sound
  is_silent: bool,
  /// Total number of unread notifications in the group, can be bigger than number of active notifications
  total_count: i64,
  /// List of added group notifications, sorted by notification ID
  added_notifications: Vec<Notification>,
  /// Identifiers of removed group notifications, sorted by notification ID
  removed_notification_ids: Vec<i64>,
  
}

impl RObject for UpdateNotificationGroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateNotificationGroup" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateNotificationGroup {}



impl UpdateNotificationGroup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateNotificationGroupBuilder {
    let mut inner = UpdateNotificationGroup::default();
    inner.td_name = "updateNotificationGroup".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateNotificationGroupBuilder { inner }
  }

  pub fn notification_group_id(&self) -> i64 { self.notification_group_id }

  pub fn type_(&self) -> &NotificationGroupType { &self.type_ }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn notification_settings_chat_id(&self) -> i64 { self.notification_settings_chat_id }

  pub fn is_silent(&self) -> bool { self.is_silent }

  pub fn total_count(&self) -> i64 { self.total_count }

  pub fn added_notifications(&self) -> &Vec<Notification> { &self.added_notifications }

  pub fn removed_notification_ids(&self) -> &Vec<i64> { &self.removed_notification_ids }

}

#[doc(hidden)]
pub struct RTDUpdateNotificationGroupBuilder {
  inner: UpdateNotificationGroup
}

impl RTDUpdateNotificationGroupBuilder {
  pub fn build(&self) -> UpdateNotificationGroup { self.inner.clone() }

   
  pub fn notification_group_id(&mut self, notification_group_id: i64) -> &mut Self {
    self.inner.notification_group_id = notification_group_id;
    self
  }

   
  pub fn type_<T: AsRef<NotificationGroupType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn notification_settings_chat_id(&mut self, notification_settings_chat_id: i64) -> &mut Self {
    self.inner.notification_settings_chat_id = notification_settings_chat_id;
    self
  }

   
  pub fn is_silent(&mut self, is_silent: bool) -> &mut Self {
    self.inner.is_silent = is_silent;
    self
  }

   
  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

   
  pub fn added_notifications(&mut self, added_notifications: Vec<Notification>) -> &mut Self {
    self.inner.added_notifications = added_notifications;
    self
  }

   
  pub fn removed_notification_ids(&mut self, removed_notification_ids: Vec<i64>) -> &mut Self {
    self.inner.removed_notification_ids = removed_notification_ids;
    self
  }

}

impl AsRef<UpdateNotificationGroup> for UpdateNotificationGroup {
  fn as_ref(&self) -> &UpdateNotificationGroup { self }
}

impl AsRef<UpdateNotificationGroup> for RTDUpdateNotificationGroupBuilder {
  fn as_ref(&self) -> &UpdateNotificationGroup { &self.inner }
}







/// An option changed its value
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateOption {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The option name
  name: String,
  /// The new option value
  value: OptionValue,
  
}

impl RObject for UpdateOption {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateOption" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateOption {}



impl UpdateOption {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateOptionBuilder {
    let mut inner = UpdateOption::default();
    inner.td_name = "updateOption".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateOptionBuilder { inner }
  }

  pub fn name(&self) -> &String { &self.name }

  pub fn value(&self) -> &OptionValue { &self.value }

}

#[doc(hidden)]
pub struct RTDUpdateOptionBuilder {
  inner: UpdateOption
}

impl RTDUpdateOptionBuilder {
  pub fn build(&self) -> UpdateOption { self.inner.clone() }

   
  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

   
  pub fn value<T: AsRef<OptionValue>>(&mut self, value: T) -> &mut Self {
    self.inner.value = value.as_ref().clone();
    self
  }

}

impl AsRef<UpdateOption> for UpdateOption {
  fn as_ref(&self) -> &UpdateOption { self }
}

impl AsRef<UpdateOption> for RTDUpdateOptionBuilder {
  fn as_ref(&self) -> &UpdateOption { &self.inner }
}







/// A poll was updated; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdatePoll {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New data about the poll
  poll: Poll,
  
}

impl RObject for UpdatePoll {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updatePoll" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdatePoll {}



impl UpdatePoll {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdatePollBuilder {
    let mut inner = UpdatePoll::default();
    inner.td_name = "updatePoll".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdatePollBuilder { inner }
  }

  pub fn poll(&self) -> &Poll { &self.poll }

}

#[doc(hidden)]
pub struct RTDUpdatePollBuilder {
  inner: UpdatePoll
}

impl RTDUpdatePollBuilder {
  pub fn build(&self) -> UpdatePoll { self.inner.clone() }

   
  pub fn poll<T: AsRef<Poll>>(&mut self, poll: T) -> &mut Self {
    self.inner.poll = poll.as_ref().clone();
    self
  }

}

impl AsRef<UpdatePoll> for UpdatePoll {
  fn as_ref(&self) -> &UpdatePoll { self }
}

impl AsRef<UpdatePoll> for RTDUpdatePollBuilder {
  fn as_ref(&self) -> &UpdatePoll { &self.inner }
}







/// A user changed the answer to a poll; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdatePollAnswer {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique poll identifier
  poll_id: isize,
  /// The user, who changed the answer to the poll
  user_id: i64,
  /// 0-based identifiers of answer options, chosen by the user
  option_ids: Vec<i64>,
  
}

impl RObject for UpdatePollAnswer {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updatePollAnswer" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdatePollAnswer {}



impl UpdatePollAnswer {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdatePollAnswerBuilder {
    let mut inner = UpdatePollAnswer::default();
    inner.td_name = "updatePollAnswer".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdatePollAnswerBuilder { inner }
  }

  pub fn poll_id(&self) -> isize { self.poll_id }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn option_ids(&self) -> &Vec<i64> { &self.option_ids }

}

#[doc(hidden)]
pub struct RTDUpdatePollAnswerBuilder {
  inner: UpdatePollAnswer
}

impl RTDUpdatePollAnswerBuilder {
  pub fn build(&self) -> UpdatePollAnswer { self.inner.clone() }

   
  pub fn poll_id(&mut self, poll_id: isize) -> &mut Self {
    self.inner.poll_id = poll_id;
    self
  }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn option_ids(&mut self, option_ids: Vec<i64>) -> &mut Self {
    self.inner.option_ids = option_ids;
    self
  }

}

impl AsRef<UpdatePollAnswer> for UpdatePollAnswer {
  fn as_ref(&self) -> &UpdatePollAnswer { self }
}

impl AsRef<UpdatePollAnswer> for RTDUpdatePollAnswerBuilder {
  fn as_ref(&self) -> &UpdatePollAnswer { &self.inner }
}







/// The list of recently used stickers was updated
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateRecentStickers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the list of stickers attached to photo or video files was updated, otherwise the list of sent stickers is updated
  is_attached: bool,
  /// The new list of file identifiers of recently used stickers
  sticker_ids: Vec<i64>,
  
}

impl RObject for UpdateRecentStickers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateRecentStickers" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateRecentStickers {}



impl UpdateRecentStickers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateRecentStickersBuilder {
    let mut inner = UpdateRecentStickers::default();
    inner.td_name = "updateRecentStickers".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateRecentStickersBuilder { inner }
  }

  pub fn is_attached(&self) -> bool { self.is_attached }

  pub fn sticker_ids(&self) -> &Vec<i64> { &self.sticker_ids }

}

#[doc(hidden)]
pub struct RTDUpdateRecentStickersBuilder {
  inner: UpdateRecentStickers
}

impl RTDUpdateRecentStickersBuilder {
  pub fn build(&self) -> UpdateRecentStickers { self.inner.clone() }

   
  pub fn is_attached(&mut self, is_attached: bool) -> &mut Self {
    self.inner.is_attached = is_attached;
    self
  }

   
  pub fn sticker_ids(&mut self, sticker_ids: Vec<i64>) -> &mut Self {
    self.inner.sticker_ids = sticker_ids;
    self
  }

}

impl AsRef<UpdateRecentStickers> for UpdateRecentStickers {
  fn as_ref(&self) -> &UpdateRecentStickers { self }
}

impl AsRef<UpdateRecentStickers> for RTDUpdateRecentStickersBuilder {
  fn as_ref(&self) -> &UpdateRecentStickers { &self.inner }
}







/// The list of saved animations was updated
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSavedAnimations {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new list of file identifiers of saved animations
  animation_ids: Vec<i64>,
  
}

impl RObject for UpdateSavedAnimations {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateSavedAnimations" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateSavedAnimations {}



impl UpdateSavedAnimations {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateSavedAnimationsBuilder {
    let mut inner = UpdateSavedAnimations::default();
    inner.td_name = "updateSavedAnimations".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateSavedAnimationsBuilder { inner }
  }

  pub fn animation_ids(&self) -> &Vec<i64> { &self.animation_ids }

}

#[doc(hidden)]
pub struct RTDUpdateSavedAnimationsBuilder {
  inner: UpdateSavedAnimations
}

impl RTDUpdateSavedAnimationsBuilder {
  pub fn build(&self) -> UpdateSavedAnimations { self.inner.clone() }

   
  pub fn animation_ids(&mut self, animation_ids: Vec<i64>) -> &mut Self {
    self.inner.animation_ids = animation_ids;
    self
  }

}

impl AsRef<UpdateSavedAnimations> for UpdateSavedAnimations {
  fn as_ref(&self) -> &UpdateSavedAnimations { self }
}

impl AsRef<UpdateSavedAnimations> for RTDUpdateSavedAnimationsBuilder {
  fn as_ref(&self) -> &UpdateSavedAnimations { &self.inner }
}







/// Notification settings for some type of chats were updated
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateScopeNotificationSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Types of chats for which notification settings were updated
  scope: NotificationSettingsScope,
  /// The new notification settings
  notification_settings: ScopeNotificationSettings,
  
}

impl RObject for UpdateScopeNotificationSettings {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateScopeNotificationSettings" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateScopeNotificationSettings {}



impl UpdateScopeNotificationSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateScopeNotificationSettingsBuilder {
    let mut inner = UpdateScopeNotificationSettings::default();
    inner.td_name = "updateScopeNotificationSettings".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateScopeNotificationSettingsBuilder { inner }
  }

  pub fn scope(&self) -> &NotificationSettingsScope { &self.scope }

  pub fn notification_settings(&self) -> &ScopeNotificationSettings { &self.notification_settings }

}

#[doc(hidden)]
pub struct RTDUpdateScopeNotificationSettingsBuilder {
  inner: UpdateScopeNotificationSettings
}

impl RTDUpdateScopeNotificationSettingsBuilder {
  pub fn build(&self) -> UpdateScopeNotificationSettings { self.inner.clone() }

   
  pub fn scope<T: AsRef<NotificationSettingsScope>>(&mut self, scope: T) -> &mut Self {
    self.inner.scope = scope.as_ref().clone();
    self
  }

   
  pub fn notification_settings<T: AsRef<ScopeNotificationSettings>>(&mut self, notification_settings: T) -> &mut Self {
    self.inner.notification_settings = notification_settings.as_ref().clone();
    self
  }

}

impl AsRef<UpdateScopeNotificationSettings> for UpdateScopeNotificationSettings {
  fn as_ref(&self) -> &UpdateScopeNotificationSettings { self }
}

impl AsRef<UpdateScopeNotificationSettings> for RTDUpdateScopeNotificationSettingsBuilder {
  fn as_ref(&self) -> &UpdateScopeNotificationSettings { &self.inner }
}







/// Some data of a secret chat has changed. This update is guaranteed to come before the secret chat identifier is returned to the application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSecretChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New data about the secret chat
  secret_chat: SecretChat,
  
}

impl RObject for UpdateSecretChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateSecretChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateSecretChat {}



impl UpdateSecretChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateSecretChatBuilder {
    let mut inner = UpdateSecretChat::default();
    inner.td_name = "updateSecretChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateSecretChatBuilder { inner }
  }

  pub fn secret_chat(&self) -> &SecretChat { &self.secret_chat }

}

#[doc(hidden)]
pub struct RTDUpdateSecretChatBuilder {
  inner: UpdateSecretChat
}

impl RTDUpdateSecretChatBuilder {
  pub fn build(&self) -> UpdateSecretChat { self.inner.clone() }

   
  pub fn secret_chat<T: AsRef<SecretChat>>(&mut self, secret_chat: T) -> &mut Self {
    self.inner.secret_chat = secret_chat.as_ref().clone();
    self
  }

}

impl AsRef<UpdateSecretChat> for UpdateSecretChat {
  fn as_ref(&self) -> &UpdateSecretChat { self }
}

impl AsRef<UpdateSecretChat> for RTDUpdateSecretChatBuilder {
  fn as_ref(&self) -> &UpdateSecretChat { &self.inner }
}







/// The selected background has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSelectedBackground {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if background for dark theme has changed
  for_dark_theme: bool,
  /// The new selected background; may be null
  background: Option<Background>,
  
}

impl RObject for UpdateSelectedBackground {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateSelectedBackground" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateSelectedBackground {}



impl UpdateSelectedBackground {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateSelectedBackgroundBuilder {
    let mut inner = UpdateSelectedBackground::default();
    inner.td_name = "updateSelectedBackground".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateSelectedBackgroundBuilder { inner }
  }

  pub fn for_dark_theme(&self) -> bool { self.for_dark_theme }

  pub fn background(&self) -> &Option<Background> { &self.background }

}

#[doc(hidden)]
pub struct RTDUpdateSelectedBackgroundBuilder {
  inner: UpdateSelectedBackground
}

impl RTDUpdateSelectedBackgroundBuilder {
  pub fn build(&self) -> UpdateSelectedBackground { self.inner.clone() }

   
  pub fn for_dark_theme(&mut self, for_dark_theme: bool) -> &mut Self {
    self.inner.for_dark_theme = for_dark_theme;
    self
  }

   
  pub fn background<T: AsRef<Background>>(&mut self, background: T) -> &mut Self {
    self.inner.background = Some(background.as_ref().clone());
    self
  }

}

impl AsRef<UpdateSelectedBackground> for UpdateSelectedBackground {
  fn as_ref(&self) -> &UpdateSelectedBackground { self }
}

impl AsRef<UpdateSelectedBackground> for RTDUpdateSelectedBackgroundBuilder {
  fn as_ref(&self) -> &UpdateSelectedBackground { &self.inner }
}







/// A service notification from the server was received. Upon receiving this the application must show a popup with the content of the notification
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateServiceNotification {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Notification type. If type begins with "AUTH_KEY_DROP_", then two buttons "Cancel" and "Log out" must be shown under notification; if user presses the second, all local data must be destroyed using Destroy method
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: String,
  /// Notification content
  content: MessageContent,
  
}

impl RObject for UpdateServiceNotification {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateServiceNotification" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateServiceNotification {}



impl UpdateServiceNotification {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateServiceNotificationBuilder {
    let mut inner = UpdateServiceNotification::default();
    inner.td_name = "updateServiceNotification".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateServiceNotificationBuilder { inner }
  }

  pub fn type_(&self) -> &String { &self.type_ }

  pub fn content(&self) -> &MessageContent { &self.content }

}

#[doc(hidden)]
pub struct RTDUpdateServiceNotificationBuilder {
  inner: UpdateServiceNotification
}

impl RTDUpdateServiceNotificationBuilder {
  pub fn build(&self) -> UpdateServiceNotification { self.inner.clone() }

   
  pub fn type_<T: AsRef<str>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().to_string();
    self
  }

   
  pub fn content<T: AsRef<MessageContent>>(&mut self, content: T) -> &mut Self {
    self.inner.content = content.as_ref().clone();
    self
  }

}

impl AsRef<UpdateServiceNotification> for UpdateServiceNotification {
  fn as_ref(&self) -> &UpdateServiceNotification { self }
}

impl AsRef<UpdateServiceNotification> for RTDUpdateServiceNotificationBuilder {
  fn as_ref(&self) -> &UpdateServiceNotification { &self.inner }
}







/// A sticker set has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateStickerSet {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The sticker set
  sticker_set: StickerSet,
  
}

impl RObject for UpdateStickerSet {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateStickerSet" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateStickerSet {}



impl UpdateStickerSet {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateStickerSetBuilder {
    let mut inner = UpdateStickerSet::default();
    inner.td_name = "updateStickerSet".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateStickerSetBuilder { inner }
  }

  pub fn sticker_set(&self) -> &StickerSet { &self.sticker_set }

}

#[doc(hidden)]
pub struct RTDUpdateStickerSetBuilder {
  inner: UpdateStickerSet
}

impl RTDUpdateStickerSetBuilder {
  pub fn build(&self) -> UpdateStickerSet { self.inner.clone() }

   
  pub fn sticker_set<T: AsRef<StickerSet>>(&mut self, sticker_set: T) -> &mut Self {
    self.inner.sticker_set = sticker_set.as_ref().clone();
    self
  }

}

impl AsRef<UpdateStickerSet> for UpdateStickerSet {
  fn as_ref(&self) -> &UpdateStickerSet { self }
}

impl AsRef<UpdateStickerSet> for RTDUpdateStickerSetBuilder {
  fn as_ref(&self) -> &UpdateStickerSet { &self.inner }
}







/// The list of suggested to the user actions has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSuggestedActions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Added suggested actions
  added_actions: Vec<SuggestedAction>,
  /// Removed suggested actions
  removed_actions: Vec<SuggestedAction>,
  
}

impl RObject for UpdateSuggestedActions {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateSuggestedActions" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateSuggestedActions {}



impl UpdateSuggestedActions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateSuggestedActionsBuilder {
    let mut inner = UpdateSuggestedActions::default();
    inner.td_name = "updateSuggestedActions".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateSuggestedActionsBuilder { inner }
  }

  pub fn added_actions(&self) -> &Vec<SuggestedAction> { &self.added_actions }

  pub fn removed_actions(&self) -> &Vec<SuggestedAction> { &self.removed_actions }

}

#[doc(hidden)]
pub struct RTDUpdateSuggestedActionsBuilder {
  inner: UpdateSuggestedActions
}

impl RTDUpdateSuggestedActionsBuilder {
  pub fn build(&self) -> UpdateSuggestedActions { self.inner.clone() }

   
  pub fn added_actions(&mut self, added_actions: Vec<SuggestedAction>) -> &mut Self {
    self.inner.added_actions = added_actions;
    self
  }

   
  pub fn removed_actions(&mut self, removed_actions: Vec<SuggestedAction>) -> &mut Self {
    self.inner.removed_actions = removed_actions;
    self
  }

}

impl AsRef<UpdateSuggestedActions> for UpdateSuggestedActions {
  fn as_ref(&self) -> &UpdateSuggestedActions { self }
}

impl AsRef<UpdateSuggestedActions> for RTDUpdateSuggestedActionsBuilder {
  fn as_ref(&self) -> &UpdateSuggestedActions { &self.inner }
}







/// Some data of a supergroup or a channel has changed. This update is guaranteed to come before the supergroup identifier is returned to the application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSupergroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New data about the supergroup
  supergroup: Supergroup,
  
}

impl RObject for UpdateSupergroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateSupergroup" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateSupergroup {}



impl UpdateSupergroup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateSupergroupBuilder {
    let mut inner = UpdateSupergroup::default();
    inner.td_name = "updateSupergroup".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateSupergroupBuilder { inner }
  }

  pub fn supergroup(&self) -> &Supergroup { &self.supergroup }

}

#[doc(hidden)]
pub struct RTDUpdateSupergroupBuilder {
  inner: UpdateSupergroup
}

impl RTDUpdateSupergroupBuilder {
  pub fn build(&self) -> UpdateSupergroup { self.inner.clone() }

   
  pub fn supergroup<T: AsRef<Supergroup>>(&mut self, supergroup: T) -> &mut Self {
    self.inner.supergroup = supergroup.as_ref().clone();
    self
  }

}

impl AsRef<UpdateSupergroup> for UpdateSupergroup {
  fn as_ref(&self) -> &UpdateSupergroup { self }
}

impl AsRef<UpdateSupergroup> for RTDUpdateSupergroupBuilder {
  fn as_ref(&self) -> &UpdateSupergroup { &self.inner }
}







/// Some data in supergroupFullInfo has been changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSupergroupFullInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the supergroup or channel
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")] supergroup_id: i64,
  /// New full information about the supergroup
  supergroup_full_info: SupergroupFullInfo,
  
}

impl RObject for UpdateSupergroupFullInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateSupergroupFullInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateSupergroupFullInfo {}



impl UpdateSupergroupFullInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateSupergroupFullInfoBuilder {
    let mut inner = UpdateSupergroupFullInfo::default();
    inner.td_name = "updateSupergroupFullInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateSupergroupFullInfoBuilder { inner }
  }

  pub fn supergroup_id(&self) -> i64 { self.supergroup_id }

  pub fn supergroup_full_info(&self) -> &SupergroupFullInfo { &self.supergroup_full_info }

}

#[doc(hidden)]
pub struct RTDUpdateSupergroupFullInfoBuilder {
  inner: UpdateSupergroupFullInfo
}

impl RTDUpdateSupergroupFullInfoBuilder {
  pub fn build(&self) -> UpdateSupergroupFullInfo { self.inner.clone() }

   
  pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
    self.inner.supergroup_id = supergroup_id;
    self
  }

   
  pub fn supergroup_full_info<T: AsRef<SupergroupFullInfo>>(&mut self, supergroup_full_info: T) -> &mut Self {
    self.inner.supergroup_full_info = supergroup_full_info.as_ref().clone();
    self
  }

}

impl AsRef<UpdateSupergroupFullInfo> for UpdateSupergroupFullInfo {
  fn as_ref(&self) -> &UpdateSupergroupFullInfo { self }
}

impl AsRef<UpdateSupergroupFullInfo> for RTDUpdateSupergroupFullInfoBuilder {
  fn as_ref(&self) -> &UpdateSupergroupFullInfo { &self.inner }
}







/// New terms of service must be accepted by the user. If the terms of service are declined, then the deleteAccount method must be called with the reason "Decline ToS update"
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateTermsOfService {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the terms of service
  terms_of_service_id: String,
  /// The new terms of service
  terms_of_service: TermsOfService,
  
}

impl RObject for UpdateTermsOfService {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateTermsOfService" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateTermsOfService {}



impl UpdateTermsOfService {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateTermsOfServiceBuilder {
    let mut inner = UpdateTermsOfService::default();
    inner.td_name = "updateTermsOfService".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateTermsOfServiceBuilder { inner }
  }

  pub fn terms_of_service_id(&self) -> &String { &self.terms_of_service_id }

  pub fn terms_of_service(&self) -> &TermsOfService { &self.terms_of_service }

}

#[doc(hidden)]
pub struct RTDUpdateTermsOfServiceBuilder {
  inner: UpdateTermsOfService
}

impl RTDUpdateTermsOfServiceBuilder {
  pub fn build(&self) -> UpdateTermsOfService { self.inner.clone() }

   
  pub fn terms_of_service_id<T: AsRef<str>>(&mut self, terms_of_service_id: T) -> &mut Self {
    self.inner.terms_of_service_id = terms_of_service_id.as_ref().to_string();
    self
  }

   
  pub fn terms_of_service<T: AsRef<TermsOfService>>(&mut self, terms_of_service: T) -> &mut Self {
    self.inner.terms_of_service = terms_of_service.as_ref().clone();
    self
  }

}

impl AsRef<UpdateTermsOfService> for UpdateTermsOfService {
  fn as_ref(&self) -> &UpdateTermsOfService { self }
}

impl AsRef<UpdateTermsOfService> for RTDUpdateTermsOfServiceBuilder {
  fn as_ref(&self) -> &UpdateTermsOfService { &self.inner }
}







/// The list of trending sticker sets was updated or some of them were viewed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateTrendingStickerSets {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The prefix of the list of trending sticker sets with the newest trending sticker sets
  sticker_sets: StickerSets,
  
}

impl RObject for UpdateTrendingStickerSets {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateTrendingStickerSets" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateTrendingStickerSets {}



impl UpdateTrendingStickerSets {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateTrendingStickerSetsBuilder {
    let mut inner = UpdateTrendingStickerSets::default();
    inner.td_name = "updateTrendingStickerSets".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateTrendingStickerSetsBuilder { inner }
  }

  pub fn sticker_sets(&self) -> &StickerSets { &self.sticker_sets }

}

#[doc(hidden)]
pub struct RTDUpdateTrendingStickerSetsBuilder {
  inner: UpdateTrendingStickerSets
}

impl RTDUpdateTrendingStickerSetsBuilder {
  pub fn build(&self) -> UpdateTrendingStickerSets { self.inner.clone() }

   
  pub fn sticker_sets<T: AsRef<StickerSets>>(&mut self, sticker_sets: T) -> &mut Self {
    self.inner.sticker_sets = sticker_sets.as_ref().clone();
    self
  }

}

impl AsRef<UpdateTrendingStickerSets> for UpdateTrendingStickerSets {
  fn as_ref(&self) -> &UpdateTrendingStickerSets { self }
}

impl AsRef<UpdateTrendingStickerSets> for RTDUpdateTrendingStickerSetsBuilder {
  fn as_ref(&self) -> &UpdateTrendingStickerSets { &self.inner }
}







/// Number of unread chats, i.e. with unread messages or marked as unread, has changed. This update is sent only if the message database is used
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUnreadChatCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The chat list with changed number of unread messages
  chat_list: ChatList,
  /// Approximate total number of chats in the chat list
  total_count: i64,
  /// Total number of unread chats
  unread_count: i64,
  /// Total number of unread unmuted chats
  unread_unmuted_count: i64,
  /// Total number of chats marked as unread
  marked_as_unread_count: i64,
  /// Total number of unmuted chats marked as unread
  marked_as_unread_unmuted_count: i64,
  
}

impl RObject for UpdateUnreadChatCount {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateUnreadChatCount" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateUnreadChatCount {}



impl UpdateUnreadChatCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateUnreadChatCountBuilder {
    let mut inner = UpdateUnreadChatCount::default();
    inner.td_name = "updateUnreadChatCount".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateUnreadChatCountBuilder { inner }
  }

  pub fn chat_list(&self) -> &ChatList { &self.chat_list }

  pub fn total_count(&self) -> i64 { self.total_count }

  pub fn unread_count(&self) -> i64 { self.unread_count }

  pub fn unread_unmuted_count(&self) -> i64 { self.unread_unmuted_count }

  pub fn marked_as_unread_count(&self) -> i64 { self.marked_as_unread_count }

  pub fn marked_as_unread_unmuted_count(&self) -> i64 { self.marked_as_unread_unmuted_count }

}

#[doc(hidden)]
pub struct RTDUpdateUnreadChatCountBuilder {
  inner: UpdateUnreadChatCount
}

impl RTDUpdateUnreadChatCountBuilder {
  pub fn build(&self) -> UpdateUnreadChatCount { self.inner.clone() }

   
  pub fn chat_list<T: AsRef<ChatList>>(&mut self, chat_list: T) -> &mut Self {
    self.inner.chat_list = chat_list.as_ref().clone();
    self
  }

   
  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

   
  pub fn unread_count(&mut self, unread_count: i64) -> &mut Self {
    self.inner.unread_count = unread_count;
    self
  }

   
  pub fn unread_unmuted_count(&mut self, unread_unmuted_count: i64) -> &mut Self {
    self.inner.unread_unmuted_count = unread_unmuted_count;
    self
  }

   
  pub fn marked_as_unread_count(&mut self, marked_as_unread_count: i64) -> &mut Self {
    self.inner.marked_as_unread_count = marked_as_unread_count;
    self
  }

   
  pub fn marked_as_unread_unmuted_count(&mut self, marked_as_unread_unmuted_count: i64) -> &mut Self {
    self.inner.marked_as_unread_unmuted_count = marked_as_unread_unmuted_count;
    self
  }

}

impl AsRef<UpdateUnreadChatCount> for UpdateUnreadChatCount {
  fn as_ref(&self) -> &UpdateUnreadChatCount { self }
}

impl AsRef<UpdateUnreadChatCount> for RTDUpdateUnreadChatCountBuilder {
  fn as_ref(&self) -> &UpdateUnreadChatCount { &self.inner }
}







/// Number of unread messages in a chat list has changed. This update is sent only if the message database is used
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUnreadMessageCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The chat list with changed number of unread messages
  chat_list: ChatList,
  /// Total number of unread messages
  unread_count: i64,
  /// Total number of unread messages in unmuted chats
  unread_unmuted_count: i64,
  
}

impl RObject for UpdateUnreadMessageCount {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateUnreadMessageCount" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateUnreadMessageCount {}



impl UpdateUnreadMessageCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateUnreadMessageCountBuilder {
    let mut inner = UpdateUnreadMessageCount::default();
    inner.td_name = "updateUnreadMessageCount".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateUnreadMessageCountBuilder { inner }
  }

  pub fn chat_list(&self) -> &ChatList { &self.chat_list }

  pub fn unread_count(&self) -> i64 { self.unread_count }

  pub fn unread_unmuted_count(&self) -> i64 { self.unread_unmuted_count }

}

#[doc(hidden)]
pub struct RTDUpdateUnreadMessageCountBuilder {
  inner: UpdateUnreadMessageCount
}

impl RTDUpdateUnreadMessageCountBuilder {
  pub fn build(&self) -> UpdateUnreadMessageCount { self.inner.clone() }

   
  pub fn chat_list<T: AsRef<ChatList>>(&mut self, chat_list: T) -> &mut Self {
    self.inner.chat_list = chat_list.as_ref().clone();
    self
  }

   
  pub fn unread_count(&mut self, unread_count: i64) -> &mut Self {
    self.inner.unread_count = unread_count;
    self
  }

   
  pub fn unread_unmuted_count(&mut self, unread_unmuted_count: i64) -> &mut Self {
    self.inner.unread_unmuted_count = unread_unmuted_count;
    self
  }

}

impl AsRef<UpdateUnreadMessageCount> for UpdateUnreadMessageCount {
  fn as_ref(&self) -> &UpdateUnreadMessageCount { self }
}

impl AsRef<UpdateUnreadMessageCount> for RTDUpdateUnreadMessageCountBuilder {
  fn as_ref(&self) -> &UpdateUnreadMessageCount { &self.inner }
}







/// Some data of a user has changed. This update is guaranteed to come before the user identifier is returned to the application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New data about the user
  user: User,
  
}

impl RObject for UpdateUser {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateUser" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateUser {}



impl UpdateUser {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateUserBuilder {
    let mut inner = UpdateUser::default();
    inner.td_name = "updateUser".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateUserBuilder { inner }
  }

  pub fn user(&self) -> &User { &self.user }

}

#[doc(hidden)]
pub struct RTDUpdateUserBuilder {
  inner: UpdateUser
}

impl RTDUpdateUserBuilder {
  pub fn build(&self) -> UpdateUser { self.inner.clone() }

   
  pub fn user<T: AsRef<User>>(&mut self, user: T) -> &mut Self {
    self.inner.user = user.as_ref().clone();
    self
  }

}

impl AsRef<UpdateUser> for UpdateUser {
  fn as_ref(&self) -> &UpdateUser { self }
}

impl AsRef<UpdateUser> for RTDUpdateUserBuilder {
  fn as_ref(&self) -> &UpdateUser { &self.inner }
}







/// Some data in userFullInfo has been changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUserFullInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier
  user_id: i64,
  /// New full information about the user
  user_full_info: UserFullInfo,
  
}

impl RObject for UpdateUserFullInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateUserFullInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateUserFullInfo {}



impl UpdateUserFullInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateUserFullInfoBuilder {
    let mut inner = UpdateUserFullInfo::default();
    inner.td_name = "updateUserFullInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateUserFullInfoBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn user_full_info(&self) -> &UserFullInfo { &self.user_full_info }

}

#[doc(hidden)]
pub struct RTDUpdateUserFullInfoBuilder {
  inner: UpdateUserFullInfo
}

impl RTDUpdateUserFullInfoBuilder {
  pub fn build(&self) -> UpdateUserFullInfo { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn user_full_info<T: AsRef<UserFullInfo>>(&mut self, user_full_info: T) -> &mut Self {
    self.inner.user_full_info = user_full_info.as_ref().clone();
    self
  }

}

impl AsRef<UpdateUserFullInfo> for UpdateUserFullInfo {
  fn as_ref(&self) -> &UpdateUserFullInfo { self }
}

impl AsRef<UpdateUserFullInfo> for RTDUpdateUserFullInfoBuilder {
  fn as_ref(&self) -> &UpdateUserFullInfo { &self.inner }
}







/// Some privacy setting rules have been changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUserPrivacySettingRules {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The privacy setting
  setting: UserPrivacySetting,
  /// New privacy rules
  rules: UserPrivacySettingRules,
  
}

impl RObject for UpdateUserPrivacySettingRules {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateUserPrivacySettingRules" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateUserPrivacySettingRules {}



impl UpdateUserPrivacySettingRules {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateUserPrivacySettingRulesBuilder {
    let mut inner = UpdateUserPrivacySettingRules::default();
    inner.td_name = "updateUserPrivacySettingRules".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateUserPrivacySettingRulesBuilder { inner }
  }

  pub fn setting(&self) -> &UserPrivacySetting { &self.setting }

  pub fn rules(&self) -> &UserPrivacySettingRules { &self.rules }

}

#[doc(hidden)]
pub struct RTDUpdateUserPrivacySettingRulesBuilder {
  inner: UpdateUserPrivacySettingRules
}

impl RTDUpdateUserPrivacySettingRulesBuilder {
  pub fn build(&self) -> UpdateUserPrivacySettingRules { self.inner.clone() }

   
  pub fn setting<T: AsRef<UserPrivacySetting>>(&mut self, setting: T) -> &mut Self {
    self.inner.setting = setting.as_ref().clone();
    self
  }

   
  pub fn rules<T: AsRef<UserPrivacySettingRules>>(&mut self, rules: T) -> &mut Self {
    self.inner.rules = rules.as_ref().clone();
    self
  }

}

impl AsRef<UpdateUserPrivacySettingRules> for UpdateUserPrivacySettingRules {
  fn as_ref(&self) -> &UpdateUserPrivacySettingRules { self }
}

impl AsRef<UpdateUserPrivacySettingRules> for RTDUpdateUserPrivacySettingRulesBuilder {
  fn as_ref(&self) -> &UpdateUserPrivacySettingRules { &self.inner }
}







/// The user went online or offline
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUserStatus {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier
  user_id: i64,
  /// New status of the user
  status: UserStatus,
  
}

impl RObject for UpdateUserStatus {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateUserStatus" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateUserStatus {}



impl UpdateUserStatus {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateUserStatusBuilder {
    let mut inner = UpdateUserStatus::default();
    inner.td_name = "updateUserStatus".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateUserStatusBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn status(&self) -> &UserStatus { &self.status }

}

#[doc(hidden)]
pub struct RTDUpdateUserStatusBuilder {
  inner: UpdateUserStatus
}

impl RTDUpdateUserStatusBuilder {
  pub fn build(&self) -> UpdateUserStatus { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn status<T: AsRef<UserStatus>>(&mut self, status: T) -> &mut Self {
    self.inner.status = status.as_ref().clone();
    self
  }

}

impl AsRef<UpdateUserStatus> for UpdateUserStatus {
  fn as_ref(&self) -> &UpdateUserStatus { self }
}

impl AsRef<UpdateUserStatus> for RTDUpdateUserStatusBuilder {
  fn as_ref(&self) -> &UpdateUserStatus { &self.inner }
}







/// The list of users nearby has changed. The update is guaranteed to be sent only 60 seconds after a successful searchChatsNearby request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUsersNearby {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new list of users nearby
  users_nearby: Vec<ChatNearby>,
  
}

impl RObject for UpdateUsersNearby {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateUsersNearby" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for UpdateUsersNearby {}



impl UpdateUsersNearby {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpdateUsersNearbyBuilder {
    let mut inner = UpdateUsersNearby::default();
    inner.td_name = "updateUsersNearby".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpdateUsersNearbyBuilder { inner }
  }

  pub fn users_nearby(&self) -> &Vec<ChatNearby> { &self.users_nearby }

}

#[doc(hidden)]
pub struct RTDUpdateUsersNearbyBuilder {
  inner: UpdateUsersNearby
}

impl RTDUpdateUsersNearbyBuilder {
  pub fn build(&self) -> UpdateUsersNearby { self.inner.clone() }

   
  pub fn users_nearby(&mut self, users_nearby: Vec<ChatNearby>) -> &mut Self {
    self.inner.users_nearby = users_nearby;
    self
  }

}

impl AsRef<UpdateUsersNearby> for UpdateUsersNearby {
  fn as_ref(&self) -> &UpdateUsersNearby { self }
}

impl AsRef<UpdateUsersNearby> for RTDUpdateUsersNearbyBuilder {
  fn as_ref(&self) -> &UpdateUsersNearby { &self.inner }
}



