use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains notifications about data changes
pub trait TDUpdate: Debug + RObject {}

/// Contains notifications about data changes
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Update {
  #[doc(hidden)]
  _Default(()),
  /// Does nothing and ensures that the Update object is used; for testing only. This is an offline method. Can be called before authorization
  TestUseUpdate(TestUseUpdate),
  /// The list of supported accent colors has changed
  AccentColors(UpdateAccentColors),
  /// The list of active emoji reactions has changed
  ActiveEmojiReactions(UpdateActiveEmojiReactions),
  /// The list of auctions in which the current user participates has changed
  ActiveGiftAuctions(UpdateActiveGiftAuctions),
  /// The list of messages with active live location that need to be updated by the application has changed. The list is persistent across application restarts only if the message database is used
  ActiveLiveLocationMessages(UpdateActiveLiveLocationMessages),
  /// Contains active notifications that were shown on previous application launches. This update is sent only if the message database is used. In that case it comes once before any updateNotification and updateNotificationGroup update
  ActiveNotifications(UpdateActiveNotifications),
  /// The parameters for age verification of the current user's account have changed
  AgeVerificationParameters(UpdateAgeVerificationParameters),
  /// Lists of suffixes allowed in secondary bot usernames, which can be added additionally to the main username if the bot's owner has a Telegram Premium subscription
  AllowedBotUsernamePremiumSuffixes(UpdateAllowedBotUsernamePremiumSuffixes),
  /// Some animated emoji message was clicked and a big animated sticker must be played if the message is visible on the screen. chatActionWatchingAnimations with the text of the message needs to be sent if the sticker is played
  AnimatedEmojiMessageClicked(UpdateAnimatedEmojiMessageClicked),
  /// The parameters of animation search through getOption("animation_search_bot_username") bot have changed
  AnimationSearchParameters(UpdateAnimationSearchParameters),
  /// A request can't be completed unless reCAPTCHA verification is performed; for official mobile applications only. The method setApplicationVerificationToken must be called once the verification is completed or failed
  ApplicationRecaptchaVerificationRequired(UpdateApplicationRecaptchaVerificationRequired),
  /// A request can't be completed unless application verification is performed; for official mobile applications only. The method setApplicationVerificationToken must be called once the verification is completed or failed
  ApplicationVerificationRequired(UpdateApplicationVerificationRequired),
  /// The list of bots added to attachment or side menu has changed
  AttachmentMenuBots(UpdateAttachmentMenuBots),
  /// The user authorization state has changed
  AuthorizationState(UpdateAuthorizationState),
  /// Autosave settings for some type of chats were updated
  AutosaveSettings(UpdateAutosaveSettings),
  /// The list of available message effects has changed
  AvailableMessageEffects(UpdateAvailableMessageEffects),
  /// Some data of a basic group has changed. This update is guaranteed to come before the basic group identifier is returned to the application
  BasicGroup(UpdateBasicGroup),
  /// Some data in basicGroupFullInfo has been changed
  BasicGroupFullInfo(UpdateBasicGroupFullInfo),
  /// A business connection has changed; for bots only
  BusinessConnection(UpdateBusinessConnection),
  /// A message in a business account was edited; for bots only
  BusinessMessageEdited(UpdateBusinessMessageEdited),
  /// Messages in a business account were deleted; for bots only
  BusinessMessagesDeleted(UpdateBusinessMessagesDeleted),
  /// New call was created or information about a call was updated
  Call(UpdateCall),
  /// Chat accent colors have changed
  ChatAccentColors(UpdateChatAccentColors),
  /// A message sender activity in the chat has changed
  ChatAction(UpdateChatAction),
  /// The chat action bar was changed
  ChatActionBar(UpdateChatActionBar),
  /// The list of active stories posted by a specific chat has changed
  ChatActiveStories(UpdateChatActiveStories),
  /// A chat was added to a chat list
  ChatAddedToList(UpdateChatAddedToList),
  /// The chat available reactions were changed
  ChatAvailableReactions(UpdateChatAvailableReactions),
  /// The chat background was changed
  ChatBackground(UpdateChatBackground),
  /// A chat was blocked or unblocked
  ChatBlockList(UpdateChatBlockList),
  /// A chat boost has changed; for bots only
  ChatBoost(UpdateChatBoost),
  /// The bar for managing business bot was changed in a chat
  ChatBusinessBotManageBar(UpdateChatBusinessBotManageBar),
  /// The value of the default disable_notification parameter, used when a message is sent to the chat, was changed
  ChatDefaultDisableNotification(UpdateChatDefaultDisableNotification),
  /// A chat draft has changed. Be aware that the update may come in the currently opened chat but with old content of the draft. If the user has changed the content of the draft, this update mustn't be applied
  ChatDraftMessage(UpdateChatDraftMessage),
  /// Chat emoji status has changed
  ChatEmojiStatus(UpdateChatEmojiStatus),
  /// The list of chat folders or a chat folder has changed
  ChatFolders(UpdateChatFolders),
  /// A chat content was allowed or restricted for saving
  ChatHasProtectedContent(UpdateChatHasProtectedContent),
  /// A chat's has_scheduled_messages field has changed
  ChatHasScheduledMessages(UpdateChatHasScheduledMessages),
  /// A chat's has_welcome_messages field has changed
  ChatHasWelcomeMessages(UpdateChatHasWelcomeMessages),
  /// A chat was marked as unread or was read
  ChatIsMarkedAsUnread(UpdateChatIsMarkedAsUnread),
  /// Translation of chat messages was enabled or disabled
  ChatIsTranslatable(UpdateChatIsTranslatable),
  /// A join request from the user was completed
  ChatJoinResult(UpdateChatJoinResult),
  /// The last message of a chat was changed
  ChatLastMessage(UpdateChatLastMessage),
  /// User rights changed in a chat; for bots only
  ChatMember(UpdateChatMember),
  /// The message auto-delete or self-destruct timer setting for a chat was changed
  ChatMessageAutoDeleteTime(UpdateChatMessageAutoDeleteTime),
  /// The message sender that is selected to send messages in a chat has changed
  ChatMessageSender(UpdateChatMessageSender),
  /// Notification settings for a chat were changed
  ChatNotificationSettings(UpdateChatNotificationSettings),
  /// The number of online group members has changed. This update with non-zero number of online group members is sent only for currently opened chats. There is no guarantee that it is sent just after the number of online users has changed
  ChatOnlineMemberCount(UpdateChatOnlineMemberCount),
  /// The chat pending join requests were changed
  ChatPendingJoinRequests(UpdateChatPendingJoinRequests),
  /// Chat permissions were changed
  ChatPermissions(UpdateChatPermissions),
  /// A chat photo was changed
  ChatPhoto(UpdateChatPhoto),
  /// The position of a chat in a chat list has changed. An updateChatLastMessage or updateChatDraftMessage update might be sent instead of the update
  ChatPosition(UpdateChatPosition),
  /// Incoming messages were read or the number of unread messages has been changed
  ChatReadInbox(UpdateChatReadInbox),
  /// Outgoing messages were read
  ChatReadOutbox(UpdateChatReadOutbox),
  /// A chat was removed from a chat list
  ChatRemovedFromList(UpdateChatRemovedFromList),
  /// The chat reply markup was changed
  ChatReplyMarkup(UpdateChatReplyMarkup),
  /// The revenue earned from sponsored messages in a chat has changed. If chat revenue screen is opened, then getChatRevenueTransactions may be called to fetch new transactions
  ChatRevenueAmount(UpdateChatRevenueAmount),
  /// The chat theme was changed
  ChatTheme(UpdateChatTheme),
  /// The title of a chat was changed
  ChatTitle(UpdateChatTitle),
  /// The chat unread_mention_count has changed
  ChatUnreadMentionCount(UpdateChatUnreadMentionCount),
  /// The chat unread_poll_vote_count has changed
  ChatUnreadPollVoteCount(UpdateChatUnreadPollVoteCount),
  /// The chat unread_reaction_count has changed
  ChatUnreadReactionCount(UpdateChatUnreadReactionCount),
  /// A chat video chat state has changed
  ChatVideoChat(UpdateChatVideoChat),
  /// A chat default appearance has changed
  ChatViewAsTopics(UpdateChatViewAsTopics),
  /// The list of welcome messages of a chat has changed
  ChatWelcomeMessages(UpdateChatWelcomeMessages),
  /// Some data of a community has changed. This update is guaranteed to come before the community identifier is returned to the application
  Community(UpdateCommunity),
  /// Some data in communityFullInfo has been changed
  CommunityFullInfo(UpdateCommunityFullInfo),
  /// The connection state has changed. This update must be used only to show a human-readable description of the connection state
  ConnectionState(UpdateConnectionState),
  /// The list of contacts that had birthdays recently or will have birthday soon has changed
  ContactCloseBirthdays(UpdateContactCloseBirthdays),
  /// The default background has changed
  DefaultBackground(UpdateDefaultBackground),
  /// The type of default paid reaction has changed
  DefaultPaidReactionType(UpdateDefaultPaidReactionType),
  /// The type of default reaction has changed
  DefaultReactionType(UpdateDefaultReactionType),
  /// Some messages were deleted
  DeleteMessages(UpdateDeleteMessages),
  /// The list of supported dice emojis has changed
  DiceEmojis(UpdateDiceEmojis),
  /// Basic information about a topic in a channel direct messages chat administered by the current user has changed. This update is guaranteed to come before the topic identifier is returned to the application
  DirectMessagesChatTopic(UpdateDirectMessagesChatTopic),
  /// The list of available emoji chat themes has changed
  EmojiChatThemes(UpdateEmojiChatThemes),
  /// The list of favorite stickers was updated
  FavoriteStickers(UpdateFavoriteStickers),
  /// Information about a file was updated
  File(UpdateFile),
  /// A file was added to the file download list. This update is sent only after file download list is loaded for the first time
  FileAddedToDownloads(UpdateFileAddedToDownloads),
  /// A file download was changed. This update is sent only after file download list is loaded for the first time
  FileDownload(UpdateFileDownload),
  /// The state of the file download list has changed
  FileDownloads(UpdateFileDownloads),
  /// The file generation process needs to be started by the application. Use setFileGenerationProgress and finishFileGeneration to generate the file
  FileGenerationStart(UpdateFileGenerationStart),
  /// File generation is no longer needed
  FileGenerationStop(UpdateFileGenerationStop),
  /// A file was removed from the file download list. This update is sent only after file download list is loaded for the first time
  FileRemovedFromDownloads(UpdateFileRemovedFromDownloads),
  /// Information about a topic in a forum chat was changed
  ForumTopic(UpdateForumTopic),
  /// Basic information about a topic in a forum chat was changed
  ForumTopicInfo(UpdateForumTopicInfo),
  /// The freeze state of the current user's account has changed
  FreezeState(UpdateFreezeState),
  /// State of a gift auction was updated
  GiftAuctionState(UpdateGiftAuctionState),
  /// The TON Gram revenue earned by the current user has changed. If Gram transaction screen of the chat is opened, then getTonTransactions may be called to fetch new transactions
  GramRevenueStatus(UpdateGramRevenueStatus),
  /// Information about a group call was updated
  GroupCall(UpdateGroupCall),
  /// The levels of live story group call messages have changed
  GroupCallMessageLevels(UpdateGroupCallMessageLevels),
  /// A group call message failed to send
  GroupCallMessageSendFailed(UpdateGroupCallMessageSendFailed),
  /// Some group call messages were deleted
  GroupCallMessagesDeleted(UpdateGroupCallMessagesDeleted),
  /// Information about a group call participant was changed. The updates are sent only after the group call is received through getGroupCall and only if the call is joined or being joined
  GroupCallParticipant(UpdateGroupCallParticipant),
  /// The list of group call participants that can send and receive encrypted call data has changed; for group calls not bound to a chat only
  GroupCallParticipants(UpdateGroupCallParticipants),
  /// The verification state of an encrypted group call has changed; for group calls not bound to a chat only
  GroupCallVerificationState(UpdateGroupCallVerificationState),
  /// Describes whether there are some pending notification updates. Can be used to prevent application from killing, while there are some pending notifications
  HavePendingNotifications(UpdateHavePendingNotifications),
  /// The list of installed sticker sets was updated
  InstalledStickerSets(UpdateInstalledStickerSets),
  /// Some language pack strings have been updated
  LanguagePackStrings(UpdateLanguagePackStrings),
  /// The list of top donors in live story group call has changed
  LiveStoryTopDonors(UpdateLiveStoryTopDonors),
  /// A bot that can be managed by the current bot was created or updated; for bots only
  ManagedBot(UpdateManagedBot),
  /// Unread votes were added or removed from a poll message
  MessageContainsUnreadPollVotes(UpdateMessageContainsUnreadPollVotes),
  /// The message content has changed
  MessageContent(UpdateMessageContent),
  /// The message content was opened. Updates voice note messages to "listened", video note messages to "viewed" and starts the self-destruct timer
  MessageContentOpened(UpdateMessageContentOpened),
  /// A message was edited. Changes in the message content will come in a separate updateMessageContent
  MessageEdited(UpdateMessageEdited),
  /// The message ephemeral content has changed
  MessageEphemeralContent(UpdateMessageEphemeralContent),
  /// A fact-check added to a message was changed
  MessageFactCheck(UpdateMessageFactCheck),
  /// The information about interactions with a message has changed
  MessageInteractionInfo(UpdateMessageInteractionInfo),
  /// The message pinned state was changed
  MessageIsPinned(UpdateMessageIsPinned),
  /// A message with a live location was viewed. When the update is received, the application is expected to update the live location
  MessageLiveLocationViewed(UpdateMessageLiveLocationViewed),
  /// A message with an unread mention was read
  MessageMentionRead(UpdateMessageMentionRead),
  /// User changed its reactions on a message with public reactions; for bots only
  MessageReaction(UpdateMessageReaction),
  /// Reactions added to a message with anonymous reactions have changed; for bots only
  MessageReactions(UpdateMessageReactions),
  /// A request to send a message has reached the Telegram server. This doesn't mean that the message will be sent successfully. This update is sent only if the option "use_quick_ack" is set to true. This update may be sent multiple times for the same message
  MessageSendAcknowledged(UpdateMessageSendAcknowledged),
  /// A message failed to send. Be aware that some messages being sent can be irrecoverably deleted, in which case updateDeleteMessages will be received instead of this update
  MessageSendFailed(UpdateMessageSendFailed),
  /// A message has been successfully sent
  MessageSendSucceeded(UpdateMessageSendSucceeded),
  /// Information about suggested post of a message was changed
  MessageSuggestedPostInfo(UpdateMessageSuggestedPostInfo),
  /// The list of unread reactions added to a message was changed
  MessageUnreadReactions(UpdateMessageUnreadReactions),
  /// A new incoming callback query from a business message; for bots only
  NewBusinessCallbackQuery(UpdateNewBusinessCallbackQuery),
  /// A new message was added to a business account; for bots only
  NewBusinessMessage(UpdateNewBusinessMessage),
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
  /// A new message was received in a group call
  NewGroupCallMessage(UpdateNewGroupCallMessage),
  /// A new paid reaction was received in a live story group call
  NewGroupCallPaidReaction(UpdateNewGroupCallPaidReaction),
  /// A new incoming guest query; for bots only
  NewGuestQuery(UpdateNewGuestQuery),
  /// A new incoming callback query from a message sent via a bot; for bots only
  NewInlineCallbackQuery(UpdateNewInlineCallbackQuery),
  /// A new incoming inline query; for bots only
  NewInlineQuery(UpdateNewInlineQuery),
  /// A new message was received; can also be an outgoing message
  NewMessage(UpdateNewMessage),
  /// An OAuth authorization request was received
  NewOauthRequest(UpdateNewOauthRequest),
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
  /// The number of TON Grams owned by the current user has changed
  OwnedGramCount(UpdateOwnedGramCount),
  /// The number of Telegram Stars owned by the current user has changed
  OwnedStarCount(UpdateOwnedStarCount),
  /// Paid media were purchased by a user; for bots only
  PaidMediaPurchased(UpdatePaidMediaPurchased),
  /// A new pending text or rich message was received in a chat with a bot. The message must be shown in the chat for at most getOption("pending_text_message_period") seconds, replace any other pending message with the same draft_id with animation, and be deleted whenever any incoming message or a pending message with another draft_id is received in the message thread
  PendingMessage(UpdatePendingMessage),
  /// A poll was updated; for bots only
  Poll(UpdatePoll),
  /// A user changed the answer to a poll; for bots only
  PollAnswer(UpdatePollAnswer),
  /// The list of supported accent colors for user profiles has changed
  ProfileAccentColors(UpdateProfileAccentColors),
  /// Basic information about a quick reply shortcut has changed. This update is guaranteed to come before the quick shortcut name is returned to the application
  QuickReplyShortcut(UpdateQuickReplyShortcut),
  /// A quick reply shortcut and all its messages were deleted
  QuickReplyShortcutDeleted(UpdateQuickReplyShortcutDeleted),
  /// The list of quick reply shortcut messages has changed
  QuickReplyShortcutMessages(UpdateQuickReplyShortcutMessages),
  /// The list of quick reply shortcuts has changed
  QuickReplyShortcuts(UpdateQuickReplyShortcuts),
  /// Notification settings for reactions were updated
  ReactionNotificationSettings(UpdateReactionNotificationSettings),
  /// The list of recently used stickers was updated
  RecentStickers(UpdateRecentStickers),
  /// The list of saved animations was updated
  SavedAnimations(UpdateSavedAnimations),
  /// Tags used in Saved Messages or a Saved Messages topic have changed
  SavedMessagesTags(UpdateSavedMessagesTags),
  /// Basic information about a Saved Messages topic has changed. This update is guaranteed to come before the topic identifier is returned to the application
  SavedMessagesTopic(UpdateSavedMessagesTopic),
  /// Number of Saved Messages topics has changed
  SavedMessagesTopicCount(UpdateSavedMessagesTopicCount),
  /// The list of saved notification sounds was updated. This update may not be sent until information about a notification sound was requested for the first time
  SavedNotificationSounds(UpdateSavedNotificationSounds),
  /// Notification settings for some type of chats were updated
  ScopeNotificationSettings(UpdateScopeNotificationSettings),
  /// Some data of a secret chat has changed. This update is guaranteed to come before the secret chat identifier is returned to the application
  SecretChat(UpdateSecretChat),
  /// A service notification from the server was received. Upon receiving this the application must show a popup with the content of the notification
  ServiceNotification(UpdateServiceNotification),
  /// The parameters of speech recognition without Telegram Premium subscription have changed
  SpeechRecognitionTrial(UpdateSpeechRecognitionTrial),
  /// Download or upload file speed for the user was limited, but it can be restored by subscription to Telegram Premium. The notification can be postponed until a file being downloaded or uploaded is visible to the user. Use getOption("premium_download_speedup") or getOption("premium_upload_speedup") to get expected speedup after subscription to Telegram Premium
  SpeedLimitNotification(UpdateSpeedLimitNotification),
  /// The stake dice state has changed
  StakeDiceState(UpdateStakeDiceState),
  /// The Telegram Star revenue earned by a user or a chat has changed. If Telegram Star transaction screen of the chat is opened, then getStarTransactions may be called to fetch new transactions
  StarRevenueStatus(UpdateStarRevenueStatus),
  /// A sticker set has changed
  StickerSet(UpdateStickerSet),
  /// A message draft generation was stopped by the user
  StopMessageDraft(UpdateStopMessageDraft),
  /// A story was changed
  Story(UpdateStory),
  /// A story became inaccessible
  StoryDeleted(UpdateStoryDeleted),
  /// Number of chats in a story list has changed
  StoryListChatCount(UpdateStoryListChatCount),
  /// A story failed to post. If the story posting is canceled, then updateStoryDeleted will be received instead of this update
  StoryPostFailed(UpdateStoryPostFailed),
  /// A story has been successfully posted
  StoryPostSucceeded(UpdateStoryPostSucceeded),
  /// Story stealth mode settings have changed
  StoryStealthMode(UpdateStoryStealthMode),
  /// The list of suggested to the user actions has changed
  SuggestedActions(UpdateSuggestedActions),
  /// Some data of a supergroup or a channel has changed. This update is guaranteed to come before the supergroup identifier is returned to the application
  Supergroup(UpdateSupergroup),
  /// Some data in supergroupFullInfo has been changed
  SupergroupFullInfo(UpdateSupergroupFullInfo),
  /// New terms of service must be accepted by the user. If the terms of service are declined, then the deleteAccount method must be called with the reason "Decline ToS update"
  TermsOfService(UpdateTermsOfService),
  /// The styles supported for text composition have changed
  TextCompositionStyles(UpdateTextCompositionStyles),
  /// The information about gasless transfers has changed
  TonWalletGaslessTransfersInfo(UpdateTonWalletGaslessTransfersInfo),
  /// The state of the TON wallet of the current user has changed
  TonWalletState(UpdateTonWalletState),
  /// A session of a TON Connect has changed
  TonWalletTonConnectSession(UpdateTonWalletTonConnectSession),
  /// TON Connect sessions must be disconnected in a regular way
  TonWalletTonConnectSessionDisconnectRequired(UpdateTonWalletTonConnectSessionDisconnectRequired),
  /// Number of messages in a topic has changed; for Saved Messages and channel direct messages chat topics only
  TopicMessageCount(UpdateTopicMessageCount),
  /// The list of trending sticker sets was updated or some of them were viewed
  TrendingStickerSets(UpdateTrendingStickerSets),
  /// Lists of bots which Mini Apps must be allowed to read text from clipboard and must be opened without a warning
  TrustedMiniAppBots(UpdateTrustedMiniAppBots),
  /// The first unconfirmed session has changed
  UnconfirmedSession(UpdateUnconfirmedSession),
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
  /// Subscription of a user to the bot was changed; for bots only
  UserSubscription(UpdateUserSubscription),
  /// An automatically scheduled message with video has been successfully sent after conversion
  VideoPublished(UpdateVideoPublished),
  /// A message was sent by an opened Web App, so the Web App needs to be closed
  WebAppMessageSent(UpdateWebAppMessageSent),
  /// Web browser settings have been updated
  WebBrowserSettings(UpdateWebBrowserSettings),
  /// A web proxy carrier must be closed. If the page has passed any message to the bridge object, then the application must deliver the string {"t":"close"} to the page, so that it can end its relay session, and destroy the web view shortly after, for example, in 200 milliseconds; otherwise, the web view must be destroyed immediately. Messages from the carrier received after this update must be ignored
  WebProxyCarrierClosed(UpdateWebProxyCarrierClosed),
  /// Data must be delivered as an ArrayBuffer to the page of a web proxy carrier through the onmessage function of the bridge object, in the order of the updates
  WebProxyCarrierData(UpdateWebProxyCarrierData),
  /// A web proxy carrier must be opened as described in webProxyCarrier. Messages passed by the page to the bridge object must be passed to processWebProxyCarrierMessage. Failures of the carrier must be reported with reportWebProxyCarrierError
  WebProxyCarrierOpened(UpdateWebProxyCarrierOpened),
}

impl Default for Update {
  fn default() -> Self {
    Update::_Default(())
  }
}

impl<'de> Deserialize<'de> for Update {
  fn deserialize<D>(deserializer: D) -> Result<Update, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      Update,
      (testUseUpdate, TestUseUpdate);
      (updateAccentColors, AccentColors);
      (updateActiveEmojiReactions, ActiveEmojiReactions);
      (updateActiveGiftAuctions, ActiveGiftAuctions);
      (updateActiveLiveLocationMessages, ActiveLiveLocationMessages);
      (updateActiveNotifications, ActiveNotifications);
      (updateAgeVerificationParameters, AgeVerificationParameters);
      (updateAllowedBotUsernamePremiumSuffixes, AllowedBotUsernamePremiumSuffixes);
      (updateAnimatedEmojiMessageClicked, AnimatedEmojiMessageClicked);
      (updateAnimationSearchParameters, AnimationSearchParameters);
      (updateApplicationRecaptchaVerificationRequired, ApplicationRecaptchaVerificationRequired);
      (updateApplicationVerificationRequired, ApplicationVerificationRequired);
      (updateAttachmentMenuBots, AttachmentMenuBots);
      (updateAuthorizationState, AuthorizationState);
      (updateAutosaveSettings, AutosaveSettings);
      (updateAvailableMessageEffects, AvailableMessageEffects);
      (updateBasicGroup, BasicGroup);
      (updateBasicGroupFullInfo, BasicGroupFullInfo);
      (updateBusinessConnection, BusinessConnection);
      (updateBusinessMessageEdited, BusinessMessageEdited);
      (updateBusinessMessagesDeleted, BusinessMessagesDeleted);
      (updateCall, Call);
      (updateChatAccentColors, ChatAccentColors);
      (updateChatAction, ChatAction);
      (updateChatActionBar, ChatActionBar);
      (updateChatActiveStories, ChatActiveStories);
      (updateChatAddedToList, ChatAddedToList);
      (updateChatAvailableReactions, ChatAvailableReactions);
      (updateChatBackground, ChatBackground);
      (updateChatBlockList, ChatBlockList);
      (updateChatBoost, ChatBoost);
      (updateChatBusinessBotManageBar, ChatBusinessBotManageBar);
      (updateChatDefaultDisableNotification, ChatDefaultDisableNotification);
      (updateChatDraftMessage, ChatDraftMessage);
      (updateChatEmojiStatus, ChatEmojiStatus);
      (updateChatFolders, ChatFolders);
      (updateChatHasProtectedContent, ChatHasProtectedContent);
      (updateChatHasScheduledMessages, ChatHasScheduledMessages);
      (updateChatHasWelcomeMessages, ChatHasWelcomeMessages);
      (updateChatIsMarkedAsUnread, ChatIsMarkedAsUnread);
      (updateChatIsTranslatable, ChatIsTranslatable);
      (updateChatJoinResult, ChatJoinResult);
      (updateChatLastMessage, ChatLastMessage);
      (updateChatMember, ChatMember);
      (updateChatMessageAutoDeleteTime, ChatMessageAutoDeleteTime);
      (updateChatMessageSender, ChatMessageSender);
      (updateChatNotificationSettings, ChatNotificationSettings);
      (updateChatOnlineMemberCount, ChatOnlineMemberCount);
      (updateChatPendingJoinRequests, ChatPendingJoinRequests);
      (updateChatPermissions, ChatPermissions);
      (updateChatPhoto, ChatPhoto);
      (updateChatPosition, ChatPosition);
      (updateChatReadInbox, ChatReadInbox);
      (updateChatReadOutbox, ChatReadOutbox);
      (updateChatRemovedFromList, ChatRemovedFromList);
      (updateChatReplyMarkup, ChatReplyMarkup);
      (updateChatRevenueAmount, ChatRevenueAmount);
      (updateChatTheme, ChatTheme);
      (updateChatTitle, ChatTitle);
      (updateChatUnreadMentionCount, ChatUnreadMentionCount);
      (updateChatUnreadPollVoteCount, ChatUnreadPollVoteCount);
      (updateChatUnreadReactionCount, ChatUnreadReactionCount);
      (updateChatVideoChat, ChatVideoChat);
      (updateChatViewAsTopics, ChatViewAsTopics);
      (updateChatWelcomeMessages, ChatWelcomeMessages);
      (updateCommunity, Community);
      (updateCommunityFullInfo, CommunityFullInfo);
      (updateConnectionState, ConnectionState);
      (updateContactCloseBirthdays, ContactCloseBirthdays);
      (updateDefaultBackground, DefaultBackground);
      (updateDefaultPaidReactionType, DefaultPaidReactionType);
      (updateDefaultReactionType, DefaultReactionType);
      (updateDeleteMessages, DeleteMessages);
      (updateDiceEmojis, DiceEmojis);
      (updateDirectMessagesChatTopic, DirectMessagesChatTopic);
      (updateEmojiChatThemes, EmojiChatThemes);
      (updateFavoriteStickers, FavoriteStickers);
      (updateFile, File);
      (updateFileAddedToDownloads, FileAddedToDownloads);
      (updateFileDownload, FileDownload);
      (updateFileDownloads, FileDownloads);
      (updateFileGenerationStart, FileGenerationStart);
      (updateFileGenerationStop, FileGenerationStop);
      (updateFileRemovedFromDownloads, FileRemovedFromDownloads);
      (updateForumTopic, ForumTopic);
      (updateForumTopicInfo, ForumTopicInfo);
      (updateFreezeState, FreezeState);
      (updateGiftAuctionState, GiftAuctionState);
      (updateGramRevenueStatus, GramRevenueStatus);
      (updateGroupCall, GroupCall);
      (updateGroupCallMessageLevels, GroupCallMessageLevels);
      (updateGroupCallMessageSendFailed, GroupCallMessageSendFailed);
      (updateGroupCallMessagesDeleted, GroupCallMessagesDeleted);
      (updateGroupCallParticipant, GroupCallParticipant);
      (updateGroupCallParticipants, GroupCallParticipants);
      (updateGroupCallVerificationState, GroupCallVerificationState);
      (updateHavePendingNotifications, HavePendingNotifications);
      (updateInstalledStickerSets, InstalledStickerSets);
      (updateLanguagePackStrings, LanguagePackStrings);
      (updateLiveStoryTopDonors, LiveStoryTopDonors);
      (updateManagedBot, ManagedBot);
      (updateMessageContainsUnreadPollVotes, MessageContainsUnreadPollVotes);
      (updateMessageContent, MessageContent);
      (updateMessageContentOpened, MessageContentOpened);
      (updateMessageEdited, MessageEdited);
      (updateMessageEphemeralContent, MessageEphemeralContent);
      (updateMessageFactCheck, MessageFactCheck);
      (updateMessageInteractionInfo, MessageInteractionInfo);
      (updateMessageIsPinned, MessageIsPinned);
      (updateMessageLiveLocationViewed, MessageLiveLocationViewed);
      (updateMessageMentionRead, MessageMentionRead);
      (updateMessageReaction, MessageReaction);
      (updateMessageReactions, MessageReactions);
      (updateMessageSendAcknowledged, MessageSendAcknowledged);
      (updateMessageSendFailed, MessageSendFailed);
      (updateMessageSendSucceeded, MessageSendSucceeded);
      (updateMessageSuggestedPostInfo, MessageSuggestedPostInfo);
      (updateMessageUnreadReactions, MessageUnreadReactions);
      (updateNewBusinessCallbackQuery, NewBusinessCallbackQuery);
      (updateNewBusinessMessage, NewBusinessMessage);
      (updateNewCallSignalingData, NewCallSignalingData);
      (updateNewCallbackQuery, NewCallbackQuery);
      (updateNewChat, NewChat);
      (updateNewChatJoinRequest, NewChatJoinRequest);
      (updateNewChosenInlineResult, NewChosenInlineResult);
      (updateNewCustomEvent, NewCustomEvent);
      (updateNewCustomQuery, NewCustomQuery);
      (updateNewGroupCallMessage, NewGroupCallMessage);
      (updateNewGroupCallPaidReaction, NewGroupCallPaidReaction);
      (updateNewGuestQuery, NewGuestQuery);
      (updateNewInlineCallbackQuery, NewInlineCallbackQuery);
      (updateNewInlineQuery, NewInlineQuery);
      (updateNewMessage, NewMessage);
      (updateNewOauthRequest, NewOauthRequest);
      (updateNewPreCheckoutQuery, NewPreCheckoutQuery);
      (updateNewShippingQuery, NewShippingQuery);
      (updateNotification, Notification);
      (updateNotificationGroup, NotificationGroup);
      (updateOption, Option);
      (updateOwnedGramCount, OwnedGramCount);
      (updateOwnedStarCount, OwnedStarCount);
      (updatePaidMediaPurchased, PaidMediaPurchased);
      (updatePendingMessage, PendingMessage);
      (updatePoll, Poll);
      (updatePollAnswer, PollAnswer);
      (updateProfileAccentColors, ProfileAccentColors);
      (updateQuickReplyShortcut, QuickReplyShortcut);
      (updateQuickReplyShortcutDeleted, QuickReplyShortcutDeleted);
      (updateQuickReplyShortcutMessages, QuickReplyShortcutMessages);
      (updateQuickReplyShortcuts, QuickReplyShortcuts);
      (updateReactionNotificationSettings, ReactionNotificationSettings);
      (updateRecentStickers, RecentStickers);
      (updateSavedAnimations, SavedAnimations);
      (updateSavedMessagesTags, SavedMessagesTags);
      (updateSavedMessagesTopic, SavedMessagesTopic);
      (updateSavedMessagesTopicCount, SavedMessagesTopicCount);
      (updateSavedNotificationSounds, SavedNotificationSounds);
      (updateScopeNotificationSettings, ScopeNotificationSettings);
      (updateSecretChat, SecretChat);
      (updateServiceNotification, ServiceNotification);
      (updateSpeechRecognitionTrial, SpeechRecognitionTrial);
      (updateSpeedLimitNotification, SpeedLimitNotification);
      (updateStakeDiceState, StakeDiceState);
      (updateStarRevenueStatus, StarRevenueStatus);
      (updateStickerSet, StickerSet);
      (updateStopMessageDraft, StopMessageDraft);
      (updateStory, Story);
      (updateStoryDeleted, StoryDeleted);
      (updateStoryListChatCount, StoryListChatCount);
      (updateStoryPostFailed, StoryPostFailed);
      (updateStoryPostSucceeded, StoryPostSucceeded);
      (updateStoryStealthMode, StoryStealthMode);
      (updateSuggestedActions, SuggestedActions);
      (updateSupergroup, Supergroup);
      (updateSupergroupFullInfo, SupergroupFullInfo);
      (updateTermsOfService, TermsOfService);
      (updateTextCompositionStyles, TextCompositionStyles);
      (updateTonWalletGaslessTransfersInfo, TonWalletGaslessTransfersInfo);
      (updateTonWalletState, TonWalletState);
      (updateTonWalletTonConnectSession, TonWalletTonConnectSession);
      (updateTonWalletTonConnectSessionDisconnectRequired, TonWalletTonConnectSessionDisconnectRequired);
      (updateTopicMessageCount, TopicMessageCount);
      (updateTrendingStickerSets, TrendingStickerSets);
      (updateTrustedMiniAppBots, TrustedMiniAppBots);
      (updateUnconfirmedSession, UnconfirmedSession);
      (updateUnreadChatCount, UnreadChatCount);
      (updateUnreadMessageCount, UnreadMessageCount);
      (updateUser, User);
      (updateUserFullInfo, UserFullInfo);
      (updateUserPrivacySettingRules, UserPrivacySettingRules);
      (updateUserStatus, UserStatus);
      (updateUserSubscription, UserSubscription);
      (updateVideoPublished, VideoPublished);
      (updateWebAppMessageSent, WebAppMessageSent);
      (updateWebBrowserSettings, WebBrowserSettings);
      (updateWebProxyCarrierClosed, WebProxyCarrierClosed);
      (updateWebProxyCarrierData, WebProxyCarrierData);
      (updateWebProxyCarrierOpened, WebProxyCarrierOpened);

    )(deserializer)
  }
}

impl RObject for Update {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      Update::TestUseUpdate(t) => t.td_name(),
      Update::AccentColors(t) => t.td_name(),
      Update::ActiveEmojiReactions(t) => t.td_name(),
      Update::ActiveGiftAuctions(t) => t.td_name(),
      Update::ActiveLiveLocationMessages(t) => t.td_name(),
      Update::ActiveNotifications(t) => t.td_name(),
      Update::AgeVerificationParameters(t) => t.td_name(),
      Update::AllowedBotUsernamePremiumSuffixes(t) => t.td_name(),
      Update::AnimatedEmojiMessageClicked(t) => t.td_name(),
      Update::AnimationSearchParameters(t) => t.td_name(),
      Update::ApplicationRecaptchaVerificationRequired(t) => t.td_name(),
      Update::ApplicationVerificationRequired(t) => t.td_name(),
      Update::AttachmentMenuBots(t) => t.td_name(),
      Update::AuthorizationState(t) => t.td_name(),
      Update::AutosaveSettings(t) => t.td_name(),
      Update::AvailableMessageEffects(t) => t.td_name(),
      Update::BasicGroup(t) => t.td_name(),
      Update::BasicGroupFullInfo(t) => t.td_name(),
      Update::BusinessConnection(t) => t.td_name(),
      Update::BusinessMessageEdited(t) => t.td_name(),
      Update::BusinessMessagesDeleted(t) => t.td_name(),
      Update::Call(t) => t.td_name(),
      Update::ChatAccentColors(t) => t.td_name(),
      Update::ChatAction(t) => t.td_name(),
      Update::ChatActionBar(t) => t.td_name(),
      Update::ChatActiveStories(t) => t.td_name(),
      Update::ChatAddedToList(t) => t.td_name(),
      Update::ChatAvailableReactions(t) => t.td_name(),
      Update::ChatBackground(t) => t.td_name(),
      Update::ChatBlockList(t) => t.td_name(),
      Update::ChatBoost(t) => t.td_name(),
      Update::ChatBusinessBotManageBar(t) => t.td_name(),
      Update::ChatDefaultDisableNotification(t) => t.td_name(),
      Update::ChatDraftMessage(t) => t.td_name(),
      Update::ChatEmojiStatus(t) => t.td_name(),
      Update::ChatFolders(t) => t.td_name(),
      Update::ChatHasProtectedContent(t) => t.td_name(),
      Update::ChatHasScheduledMessages(t) => t.td_name(),
      Update::ChatHasWelcomeMessages(t) => t.td_name(),
      Update::ChatIsMarkedAsUnread(t) => t.td_name(),
      Update::ChatIsTranslatable(t) => t.td_name(),
      Update::ChatJoinResult(t) => t.td_name(),
      Update::ChatLastMessage(t) => t.td_name(),
      Update::ChatMember(t) => t.td_name(),
      Update::ChatMessageAutoDeleteTime(t) => t.td_name(),
      Update::ChatMessageSender(t) => t.td_name(),
      Update::ChatNotificationSettings(t) => t.td_name(),
      Update::ChatOnlineMemberCount(t) => t.td_name(),
      Update::ChatPendingJoinRequests(t) => t.td_name(),
      Update::ChatPermissions(t) => t.td_name(),
      Update::ChatPhoto(t) => t.td_name(),
      Update::ChatPosition(t) => t.td_name(),
      Update::ChatReadInbox(t) => t.td_name(),
      Update::ChatReadOutbox(t) => t.td_name(),
      Update::ChatRemovedFromList(t) => t.td_name(),
      Update::ChatReplyMarkup(t) => t.td_name(),
      Update::ChatRevenueAmount(t) => t.td_name(),
      Update::ChatTheme(t) => t.td_name(),
      Update::ChatTitle(t) => t.td_name(),
      Update::ChatUnreadMentionCount(t) => t.td_name(),
      Update::ChatUnreadPollVoteCount(t) => t.td_name(),
      Update::ChatUnreadReactionCount(t) => t.td_name(),
      Update::ChatVideoChat(t) => t.td_name(),
      Update::ChatViewAsTopics(t) => t.td_name(),
      Update::ChatWelcomeMessages(t) => t.td_name(),
      Update::Community(t) => t.td_name(),
      Update::CommunityFullInfo(t) => t.td_name(),
      Update::ConnectionState(t) => t.td_name(),
      Update::ContactCloseBirthdays(t) => t.td_name(),
      Update::DefaultBackground(t) => t.td_name(),
      Update::DefaultPaidReactionType(t) => t.td_name(),
      Update::DefaultReactionType(t) => t.td_name(),
      Update::DeleteMessages(t) => t.td_name(),
      Update::DiceEmojis(t) => t.td_name(),
      Update::DirectMessagesChatTopic(t) => t.td_name(),
      Update::EmojiChatThemes(t) => t.td_name(),
      Update::FavoriteStickers(t) => t.td_name(),
      Update::File(t) => t.td_name(),
      Update::FileAddedToDownloads(t) => t.td_name(),
      Update::FileDownload(t) => t.td_name(),
      Update::FileDownloads(t) => t.td_name(),
      Update::FileGenerationStart(t) => t.td_name(),
      Update::FileGenerationStop(t) => t.td_name(),
      Update::FileRemovedFromDownloads(t) => t.td_name(),
      Update::ForumTopic(t) => t.td_name(),
      Update::ForumTopicInfo(t) => t.td_name(),
      Update::FreezeState(t) => t.td_name(),
      Update::GiftAuctionState(t) => t.td_name(),
      Update::GramRevenueStatus(t) => t.td_name(),
      Update::GroupCall(t) => t.td_name(),
      Update::GroupCallMessageLevels(t) => t.td_name(),
      Update::GroupCallMessageSendFailed(t) => t.td_name(),
      Update::GroupCallMessagesDeleted(t) => t.td_name(),
      Update::GroupCallParticipant(t) => t.td_name(),
      Update::GroupCallParticipants(t) => t.td_name(),
      Update::GroupCallVerificationState(t) => t.td_name(),
      Update::HavePendingNotifications(t) => t.td_name(),
      Update::InstalledStickerSets(t) => t.td_name(),
      Update::LanguagePackStrings(t) => t.td_name(),
      Update::LiveStoryTopDonors(t) => t.td_name(),
      Update::ManagedBot(t) => t.td_name(),
      Update::MessageContainsUnreadPollVotes(t) => t.td_name(),
      Update::MessageContent(t) => t.td_name(),
      Update::MessageContentOpened(t) => t.td_name(),
      Update::MessageEdited(t) => t.td_name(),
      Update::MessageEphemeralContent(t) => t.td_name(),
      Update::MessageFactCheck(t) => t.td_name(),
      Update::MessageInteractionInfo(t) => t.td_name(),
      Update::MessageIsPinned(t) => t.td_name(),
      Update::MessageLiveLocationViewed(t) => t.td_name(),
      Update::MessageMentionRead(t) => t.td_name(),
      Update::MessageReaction(t) => t.td_name(),
      Update::MessageReactions(t) => t.td_name(),
      Update::MessageSendAcknowledged(t) => t.td_name(),
      Update::MessageSendFailed(t) => t.td_name(),
      Update::MessageSendSucceeded(t) => t.td_name(),
      Update::MessageSuggestedPostInfo(t) => t.td_name(),
      Update::MessageUnreadReactions(t) => t.td_name(),
      Update::NewBusinessCallbackQuery(t) => t.td_name(),
      Update::NewBusinessMessage(t) => t.td_name(),
      Update::NewCallSignalingData(t) => t.td_name(),
      Update::NewCallbackQuery(t) => t.td_name(),
      Update::NewChat(t) => t.td_name(),
      Update::NewChatJoinRequest(t) => t.td_name(),
      Update::NewChosenInlineResult(t) => t.td_name(),
      Update::NewCustomEvent(t) => t.td_name(),
      Update::NewCustomQuery(t) => t.td_name(),
      Update::NewGroupCallMessage(t) => t.td_name(),
      Update::NewGroupCallPaidReaction(t) => t.td_name(),
      Update::NewGuestQuery(t) => t.td_name(),
      Update::NewInlineCallbackQuery(t) => t.td_name(),
      Update::NewInlineQuery(t) => t.td_name(),
      Update::NewMessage(t) => t.td_name(),
      Update::NewOauthRequest(t) => t.td_name(),
      Update::NewPreCheckoutQuery(t) => t.td_name(),
      Update::NewShippingQuery(t) => t.td_name(),
      Update::Notification(t) => t.td_name(),
      Update::NotificationGroup(t) => t.td_name(),
      Update::Option(t) => t.td_name(),
      Update::OwnedGramCount(t) => t.td_name(),
      Update::OwnedStarCount(t) => t.td_name(),
      Update::PaidMediaPurchased(t) => t.td_name(),
      Update::PendingMessage(t) => t.td_name(),
      Update::Poll(t) => t.td_name(),
      Update::PollAnswer(t) => t.td_name(),
      Update::ProfileAccentColors(t) => t.td_name(),
      Update::QuickReplyShortcut(t) => t.td_name(),
      Update::QuickReplyShortcutDeleted(t) => t.td_name(),
      Update::QuickReplyShortcutMessages(t) => t.td_name(),
      Update::QuickReplyShortcuts(t) => t.td_name(),
      Update::ReactionNotificationSettings(t) => t.td_name(),
      Update::RecentStickers(t) => t.td_name(),
      Update::SavedAnimations(t) => t.td_name(),
      Update::SavedMessagesTags(t) => t.td_name(),
      Update::SavedMessagesTopic(t) => t.td_name(),
      Update::SavedMessagesTopicCount(t) => t.td_name(),
      Update::SavedNotificationSounds(t) => t.td_name(),
      Update::ScopeNotificationSettings(t) => t.td_name(),
      Update::SecretChat(t) => t.td_name(),
      Update::ServiceNotification(t) => t.td_name(),
      Update::SpeechRecognitionTrial(t) => t.td_name(),
      Update::SpeedLimitNotification(t) => t.td_name(),
      Update::StakeDiceState(t) => t.td_name(),
      Update::StarRevenueStatus(t) => t.td_name(),
      Update::StickerSet(t) => t.td_name(),
      Update::StopMessageDraft(t) => t.td_name(),
      Update::Story(t) => t.td_name(),
      Update::StoryDeleted(t) => t.td_name(),
      Update::StoryListChatCount(t) => t.td_name(),
      Update::StoryPostFailed(t) => t.td_name(),
      Update::StoryPostSucceeded(t) => t.td_name(),
      Update::StoryStealthMode(t) => t.td_name(),
      Update::SuggestedActions(t) => t.td_name(),
      Update::Supergroup(t) => t.td_name(),
      Update::SupergroupFullInfo(t) => t.td_name(),
      Update::TermsOfService(t) => t.td_name(),
      Update::TextCompositionStyles(t) => t.td_name(),
      Update::TonWalletGaslessTransfersInfo(t) => t.td_name(),
      Update::TonWalletState(t) => t.td_name(),
      Update::TonWalletTonConnectSession(t) => t.td_name(),
      Update::TonWalletTonConnectSessionDisconnectRequired(t) => t.td_name(),
      Update::TopicMessageCount(t) => t.td_name(),
      Update::TrendingStickerSets(t) => t.td_name(),
      Update::TrustedMiniAppBots(t) => t.td_name(),
      Update::UnconfirmedSession(t) => t.td_name(),
      Update::UnreadChatCount(t) => t.td_name(),
      Update::UnreadMessageCount(t) => t.td_name(),
      Update::User(t) => t.td_name(),
      Update::UserFullInfo(t) => t.td_name(),
      Update::UserPrivacySettingRules(t) => t.td_name(),
      Update::UserStatus(t) => t.td_name(),
      Update::UserSubscription(t) => t.td_name(),
      Update::VideoPublished(t) => t.td_name(),
      Update::WebAppMessageSent(t) => t.td_name(),
      Update::WebBrowserSettings(t) => t.td_name(),
      Update::WebProxyCarrierClosed(t) => t.td_name(),
      Update::WebProxyCarrierData(t) => t.td_name(),
      Update::WebProxyCarrierOpened(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      Update::TestUseUpdate(t) => t.extra(),
      Update::AccentColors(t) => t.extra(),
      Update::ActiveEmojiReactions(t) => t.extra(),
      Update::ActiveGiftAuctions(t) => t.extra(),
      Update::ActiveLiveLocationMessages(t) => t.extra(),
      Update::ActiveNotifications(t) => t.extra(),
      Update::AgeVerificationParameters(t) => t.extra(),
      Update::AllowedBotUsernamePremiumSuffixes(t) => t.extra(),
      Update::AnimatedEmojiMessageClicked(t) => t.extra(),
      Update::AnimationSearchParameters(t) => t.extra(),
      Update::ApplicationRecaptchaVerificationRequired(t) => t.extra(),
      Update::ApplicationVerificationRequired(t) => t.extra(),
      Update::AttachmentMenuBots(t) => t.extra(),
      Update::AuthorizationState(t) => t.extra(),
      Update::AutosaveSettings(t) => t.extra(),
      Update::AvailableMessageEffects(t) => t.extra(),
      Update::BasicGroup(t) => t.extra(),
      Update::BasicGroupFullInfo(t) => t.extra(),
      Update::BusinessConnection(t) => t.extra(),
      Update::BusinessMessageEdited(t) => t.extra(),
      Update::BusinessMessagesDeleted(t) => t.extra(),
      Update::Call(t) => t.extra(),
      Update::ChatAccentColors(t) => t.extra(),
      Update::ChatAction(t) => t.extra(),
      Update::ChatActionBar(t) => t.extra(),
      Update::ChatActiveStories(t) => t.extra(),
      Update::ChatAddedToList(t) => t.extra(),
      Update::ChatAvailableReactions(t) => t.extra(),
      Update::ChatBackground(t) => t.extra(),
      Update::ChatBlockList(t) => t.extra(),
      Update::ChatBoost(t) => t.extra(),
      Update::ChatBusinessBotManageBar(t) => t.extra(),
      Update::ChatDefaultDisableNotification(t) => t.extra(),
      Update::ChatDraftMessage(t) => t.extra(),
      Update::ChatEmojiStatus(t) => t.extra(),
      Update::ChatFolders(t) => t.extra(),
      Update::ChatHasProtectedContent(t) => t.extra(),
      Update::ChatHasScheduledMessages(t) => t.extra(),
      Update::ChatHasWelcomeMessages(t) => t.extra(),
      Update::ChatIsMarkedAsUnread(t) => t.extra(),
      Update::ChatIsTranslatable(t) => t.extra(),
      Update::ChatJoinResult(t) => t.extra(),
      Update::ChatLastMessage(t) => t.extra(),
      Update::ChatMember(t) => t.extra(),
      Update::ChatMessageAutoDeleteTime(t) => t.extra(),
      Update::ChatMessageSender(t) => t.extra(),
      Update::ChatNotificationSettings(t) => t.extra(),
      Update::ChatOnlineMemberCount(t) => t.extra(),
      Update::ChatPendingJoinRequests(t) => t.extra(),
      Update::ChatPermissions(t) => t.extra(),
      Update::ChatPhoto(t) => t.extra(),
      Update::ChatPosition(t) => t.extra(),
      Update::ChatReadInbox(t) => t.extra(),
      Update::ChatReadOutbox(t) => t.extra(),
      Update::ChatRemovedFromList(t) => t.extra(),
      Update::ChatReplyMarkup(t) => t.extra(),
      Update::ChatRevenueAmount(t) => t.extra(),
      Update::ChatTheme(t) => t.extra(),
      Update::ChatTitle(t) => t.extra(),
      Update::ChatUnreadMentionCount(t) => t.extra(),
      Update::ChatUnreadPollVoteCount(t) => t.extra(),
      Update::ChatUnreadReactionCount(t) => t.extra(),
      Update::ChatVideoChat(t) => t.extra(),
      Update::ChatViewAsTopics(t) => t.extra(),
      Update::ChatWelcomeMessages(t) => t.extra(),
      Update::Community(t) => t.extra(),
      Update::CommunityFullInfo(t) => t.extra(),
      Update::ConnectionState(t) => t.extra(),
      Update::ContactCloseBirthdays(t) => t.extra(),
      Update::DefaultBackground(t) => t.extra(),
      Update::DefaultPaidReactionType(t) => t.extra(),
      Update::DefaultReactionType(t) => t.extra(),
      Update::DeleteMessages(t) => t.extra(),
      Update::DiceEmojis(t) => t.extra(),
      Update::DirectMessagesChatTopic(t) => t.extra(),
      Update::EmojiChatThemes(t) => t.extra(),
      Update::FavoriteStickers(t) => t.extra(),
      Update::File(t) => t.extra(),
      Update::FileAddedToDownloads(t) => t.extra(),
      Update::FileDownload(t) => t.extra(),
      Update::FileDownloads(t) => t.extra(),
      Update::FileGenerationStart(t) => t.extra(),
      Update::FileGenerationStop(t) => t.extra(),
      Update::FileRemovedFromDownloads(t) => t.extra(),
      Update::ForumTopic(t) => t.extra(),
      Update::ForumTopicInfo(t) => t.extra(),
      Update::FreezeState(t) => t.extra(),
      Update::GiftAuctionState(t) => t.extra(),
      Update::GramRevenueStatus(t) => t.extra(),
      Update::GroupCall(t) => t.extra(),
      Update::GroupCallMessageLevels(t) => t.extra(),
      Update::GroupCallMessageSendFailed(t) => t.extra(),
      Update::GroupCallMessagesDeleted(t) => t.extra(),
      Update::GroupCallParticipant(t) => t.extra(),
      Update::GroupCallParticipants(t) => t.extra(),
      Update::GroupCallVerificationState(t) => t.extra(),
      Update::HavePendingNotifications(t) => t.extra(),
      Update::InstalledStickerSets(t) => t.extra(),
      Update::LanguagePackStrings(t) => t.extra(),
      Update::LiveStoryTopDonors(t) => t.extra(),
      Update::ManagedBot(t) => t.extra(),
      Update::MessageContainsUnreadPollVotes(t) => t.extra(),
      Update::MessageContent(t) => t.extra(),
      Update::MessageContentOpened(t) => t.extra(),
      Update::MessageEdited(t) => t.extra(),
      Update::MessageEphemeralContent(t) => t.extra(),
      Update::MessageFactCheck(t) => t.extra(),
      Update::MessageInteractionInfo(t) => t.extra(),
      Update::MessageIsPinned(t) => t.extra(),
      Update::MessageLiveLocationViewed(t) => t.extra(),
      Update::MessageMentionRead(t) => t.extra(),
      Update::MessageReaction(t) => t.extra(),
      Update::MessageReactions(t) => t.extra(),
      Update::MessageSendAcknowledged(t) => t.extra(),
      Update::MessageSendFailed(t) => t.extra(),
      Update::MessageSendSucceeded(t) => t.extra(),
      Update::MessageSuggestedPostInfo(t) => t.extra(),
      Update::MessageUnreadReactions(t) => t.extra(),
      Update::NewBusinessCallbackQuery(t) => t.extra(),
      Update::NewBusinessMessage(t) => t.extra(),
      Update::NewCallSignalingData(t) => t.extra(),
      Update::NewCallbackQuery(t) => t.extra(),
      Update::NewChat(t) => t.extra(),
      Update::NewChatJoinRequest(t) => t.extra(),
      Update::NewChosenInlineResult(t) => t.extra(),
      Update::NewCustomEvent(t) => t.extra(),
      Update::NewCustomQuery(t) => t.extra(),
      Update::NewGroupCallMessage(t) => t.extra(),
      Update::NewGroupCallPaidReaction(t) => t.extra(),
      Update::NewGuestQuery(t) => t.extra(),
      Update::NewInlineCallbackQuery(t) => t.extra(),
      Update::NewInlineQuery(t) => t.extra(),
      Update::NewMessage(t) => t.extra(),
      Update::NewOauthRequest(t) => t.extra(),
      Update::NewPreCheckoutQuery(t) => t.extra(),
      Update::NewShippingQuery(t) => t.extra(),
      Update::Notification(t) => t.extra(),
      Update::NotificationGroup(t) => t.extra(),
      Update::Option(t) => t.extra(),
      Update::OwnedGramCount(t) => t.extra(),
      Update::OwnedStarCount(t) => t.extra(),
      Update::PaidMediaPurchased(t) => t.extra(),
      Update::PendingMessage(t) => t.extra(),
      Update::Poll(t) => t.extra(),
      Update::PollAnswer(t) => t.extra(),
      Update::ProfileAccentColors(t) => t.extra(),
      Update::QuickReplyShortcut(t) => t.extra(),
      Update::QuickReplyShortcutDeleted(t) => t.extra(),
      Update::QuickReplyShortcutMessages(t) => t.extra(),
      Update::QuickReplyShortcuts(t) => t.extra(),
      Update::ReactionNotificationSettings(t) => t.extra(),
      Update::RecentStickers(t) => t.extra(),
      Update::SavedAnimations(t) => t.extra(),
      Update::SavedMessagesTags(t) => t.extra(),
      Update::SavedMessagesTopic(t) => t.extra(),
      Update::SavedMessagesTopicCount(t) => t.extra(),
      Update::SavedNotificationSounds(t) => t.extra(),
      Update::ScopeNotificationSettings(t) => t.extra(),
      Update::SecretChat(t) => t.extra(),
      Update::ServiceNotification(t) => t.extra(),
      Update::SpeechRecognitionTrial(t) => t.extra(),
      Update::SpeedLimitNotification(t) => t.extra(),
      Update::StakeDiceState(t) => t.extra(),
      Update::StarRevenueStatus(t) => t.extra(),
      Update::StickerSet(t) => t.extra(),
      Update::StopMessageDraft(t) => t.extra(),
      Update::Story(t) => t.extra(),
      Update::StoryDeleted(t) => t.extra(),
      Update::StoryListChatCount(t) => t.extra(),
      Update::StoryPostFailed(t) => t.extra(),
      Update::StoryPostSucceeded(t) => t.extra(),
      Update::StoryStealthMode(t) => t.extra(),
      Update::SuggestedActions(t) => t.extra(),
      Update::Supergroup(t) => t.extra(),
      Update::SupergroupFullInfo(t) => t.extra(),
      Update::TermsOfService(t) => t.extra(),
      Update::TextCompositionStyles(t) => t.extra(),
      Update::TonWalletGaslessTransfersInfo(t) => t.extra(),
      Update::TonWalletState(t) => t.extra(),
      Update::TonWalletTonConnectSession(t) => t.extra(),
      Update::TonWalletTonConnectSessionDisconnectRequired(t) => t.extra(),
      Update::TopicMessageCount(t) => t.extra(),
      Update::TrendingStickerSets(t) => t.extra(),
      Update::TrustedMiniAppBots(t) => t.extra(),
      Update::UnconfirmedSession(t) => t.extra(),
      Update::UnreadChatCount(t) => t.extra(),
      Update::UnreadMessageCount(t) => t.extra(),
      Update::User(t) => t.extra(),
      Update::UserFullInfo(t) => t.extra(),
      Update::UserPrivacySettingRules(t) => t.extra(),
      Update::UserStatus(t) => t.extra(),
      Update::UserSubscription(t) => t.extra(),
      Update::VideoPublished(t) => t.extra(),
      Update::WebAppMessageSent(t) => t.extra(),
      Update::WebBrowserSettings(t) => t.extra(),
      Update::WebProxyCarrierClosed(t) => t.extra(),
      Update::WebProxyCarrierData(t) => t.extra(),
      Update::WebProxyCarrierOpened(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl Update {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, Update::_Default(_))
  }

  pub fn is_test_use_update(&self) -> bool {
    matches!(self, Update::TestUseUpdate(_))
  }
  pub fn is_accent_colors(&self) -> bool {
    matches!(self, Update::AccentColors(_))
  }
  pub fn is_active_emoji_reactions(&self) -> bool {
    matches!(self, Update::ActiveEmojiReactions(_))
  }
  pub fn is_active_gift_auctions(&self) -> bool {
    matches!(self, Update::ActiveGiftAuctions(_))
  }
  pub fn is_active_live_location_messages(&self) -> bool {
    matches!(self, Update::ActiveLiveLocationMessages(_))
  }
  pub fn is_active_notifications(&self) -> bool {
    matches!(self, Update::ActiveNotifications(_))
  }
  pub fn is_age_verification_parameters(&self) -> bool {
    matches!(self, Update::AgeVerificationParameters(_))
  }
  pub fn is_allowed_bot_username_premium_suffixes(&self) -> bool {
    matches!(self, Update::AllowedBotUsernamePremiumSuffixes(_))
  }
  pub fn is_animated_emoji_message_clicked(&self) -> bool {
    matches!(self, Update::AnimatedEmojiMessageClicked(_))
  }
  pub fn is_animation_search_parameters(&self) -> bool {
    matches!(self, Update::AnimationSearchParameters(_))
  }
  pub fn is_application_recaptcha_verification_required(&self) -> bool {
    matches!(self, Update::ApplicationRecaptchaVerificationRequired(_))
  }
  pub fn is_application_verification_required(&self) -> bool {
    matches!(self, Update::ApplicationVerificationRequired(_))
  }
  pub fn is_attachment_menu_bots(&self) -> bool {
    matches!(self, Update::AttachmentMenuBots(_))
  }
  pub fn is_authorization_state(&self) -> bool {
    matches!(self, Update::AuthorizationState(_))
  }
  pub fn is_autosave_settings(&self) -> bool {
    matches!(self, Update::AutosaveSettings(_))
  }
  pub fn is_available_message_effects(&self) -> bool {
    matches!(self, Update::AvailableMessageEffects(_))
  }
  pub fn is_basic_group(&self) -> bool {
    matches!(self, Update::BasicGroup(_))
  }
  pub fn is_basic_group_full_info(&self) -> bool {
    matches!(self, Update::BasicGroupFullInfo(_))
  }
  pub fn is_business_connection(&self) -> bool {
    matches!(self, Update::BusinessConnection(_))
  }
  pub fn is_business_message_edited(&self) -> bool {
    matches!(self, Update::BusinessMessageEdited(_))
  }
  pub fn is_business_messages_deleted(&self) -> bool {
    matches!(self, Update::BusinessMessagesDeleted(_))
  }
  pub fn is_call(&self) -> bool {
    matches!(self, Update::Call(_))
  }
  pub fn is_chat_accent_colors(&self) -> bool {
    matches!(self, Update::ChatAccentColors(_))
  }
  pub fn is_chat_action(&self) -> bool {
    matches!(self, Update::ChatAction(_))
  }
  pub fn is_chat_action_bar(&self) -> bool {
    matches!(self, Update::ChatActionBar(_))
  }
  pub fn is_chat_active_stories(&self) -> bool {
    matches!(self, Update::ChatActiveStories(_))
  }
  pub fn is_chat_added_to_list(&self) -> bool {
    matches!(self, Update::ChatAddedToList(_))
  }
  pub fn is_chat_available_reactions(&self) -> bool {
    matches!(self, Update::ChatAvailableReactions(_))
  }
  pub fn is_chat_background(&self) -> bool {
    matches!(self, Update::ChatBackground(_))
  }
  pub fn is_chat_block_list(&self) -> bool {
    matches!(self, Update::ChatBlockList(_))
  }
  pub fn is_chat_boost(&self) -> bool {
    matches!(self, Update::ChatBoost(_))
  }
  pub fn is_chat_business_bot_manage_bar(&self) -> bool {
    matches!(self, Update::ChatBusinessBotManageBar(_))
  }
  pub fn is_chat_default_disable_notification(&self) -> bool {
    matches!(self, Update::ChatDefaultDisableNotification(_))
  }
  pub fn is_chat_draft_message(&self) -> bool {
    matches!(self, Update::ChatDraftMessage(_))
  }
  pub fn is_chat_emoji_status(&self) -> bool {
    matches!(self, Update::ChatEmojiStatus(_))
  }
  pub fn is_chat_folders(&self) -> bool {
    matches!(self, Update::ChatFolders(_))
  }
  pub fn is_chat_has_protected_content(&self) -> bool {
    matches!(self, Update::ChatHasProtectedContent(_))
  }
  pub fn is_chat_has_scheduled_messages(&self) -> bool {
    matches!(self, Update::ChatHasScheduledMessages(_))
  }
  pub fn is_chat_has_welcome_messages(&self) -> bool {
    matches!(self, Update::ChatHasWelcomeMessages(_))
  }
  pub fn is_chat_is_marked_as_unread(&self) -> bool {
    matches!(self, Update::ChatIsMarkedAsUnread(_))
  }
  pub fn is_chat_is_translatable(&self) -> bool {
    matches!(self, Update::ChatIsTranslatable(_))
  }
  pub fn is_chat_join_result(&self) -> bool {
    matches!(self, Update::ChatJoinResult(_))
  }
  pub fn is_chat_last_message(&self) -> bool {
    matches!(self, Update::ChatLastMessage(_))
  }
  pub fn is_chat_member(&self) -> bool {
    matches!(self, Update::ChatMember(_))
  }
  pub fn is_chat_message_auto_delete_time(&self) -> bool {
    matches!(self, Update::ChatMessageAutoDeleteTime(_))
  }
  pub fn is_chat_message_sender(&self) -> bool {
    matches!(self, Update::ChatMessageSender(_))
  }
  pub fn is_chat_notification_settings(&self) -> bool {
    matches!(self, Update::ChatNotificationSettings(_))
  }
  pub fn is_chat_online_member_count(&self) -> bool {
    matches!(self, Update::ChatOnlineMemberCount(_))
  }
  pub fn is_chat_pending_join_requests(&self) -> bool {
    matches!(self, Update::ChatPendingJoinRequests(_))
  }
  pub fn is_chat_permissions(&self) -> bool {
    matches!(self, Update::ChatPermissions(_))
  }
  pub fn is_chat_photo(&self) -> bool {
    matches!(self, Update::ChatPhoto(_))
  }
  pub fn is_chat_position(&self) -> bool {
    matches!(self, Update::ChatPosition(_))
  }
  pub fn is_chat_read_inbox(&self) -> bool {
    matches!(self, Update::ChatReadInbox(_))
  }
  pub fn is_chat_read_outbox(&self) -> bool {
    matches!(self, Update::ChatReadOutbox(_))
  }
  pub fn is_chat_removed_from_list(&self) -> bool {
    matches!(self, Update::ChatRemovedFromList(_))
  }
  pub fn is_chat_reply_markup(&self) -> bool {
    matches!(self, Update::ChatReplyMarkup(_))
  }
  pub fn is_chat_revenue_amount(&self) -> bool {
    matches!(self, Update::ChatRevenueAmount(_))
  }
  pub fn is_chat_theme(&self) -> bool {
    matches!(self, Update::ChatTheme(_))
  }
  pub fn is_chat_title(&self) -> bool {
    matches!(self, Update::ChatTitle(_))
  }
  pub fn is_chat_unread_mention_count(&self) -> bool {
    matches!(self, Update::ChatUnreadMentionCount(_))
  }
  pub fn is_chat_unread_poll_vote_count(&self) -> bool {
    matches!(self, Update::ChatUnreadPollVoteCount(_))
  }
  pub fn is_chat_unread_reaction_count(&self) -> bool {
    matches!(self, Update::ChatUnreadReactionCount(_))
  }
  pub fn is_chat_video_chat(&self) -> bool {
    matches!(self, Update::ChatVideoChat(_))
  }
  pub fn is_chat_view_as_topics(&self) -> bool {
    matches!(self, Update::ChatViewAsTopics(_))
  }
  pub fn is_chat_welcome_messages(&self) -> bool {
    matches!(self, Update::ChatWelcomeMessages(_))
  }
  pub fn is_community(&self) -> bool {
    matches!(self, Update::Community(_))
  }
  pub fn is_community_full_info(&self) -> bool {
    matches!(self, Update::CommunityFullInfo(_))
  }
  pub fn is_connection_state(&self) -> bool {
    matches!(self, Update::ConnectionState(_))
  }
  pub fn is_contact_close_birthdays(&self) -> bool {
    matches!(self, Update::ContactCloseBirthdays(_))
  }
  pub fn is_default_background(&self) -> bool {
    matches!(self, Update::DefaultBackground(_))
  }
  pub fn is_default_paid_reaction_type(&self) -> bool {
    matches!(self, Update::DefaultPaidReactionType(_))
  }
  pub fn is_default_reaction_type(&self) -> bool {
    matches!(self, Update::DefaultReactionType(_))
  }
  pub fn is_delete_messages(&self) -> bool {
    matches!(self, Update::DeleteMessages(_))
  }
  pub fn is_dice_emojis(&self) -> bool {
    matches!(self, Update::DiceEmojis(_))
  }
  pub fn is_direct_messages_chat_topic(&self) -> bool {
    matches!(self, Update::DirectMessagesChatTopic(_))
  }
  pub fn is_emoji_chat_themes(&self) -> bool {
    matches!(self, Update::EmojiChatThemes(_))
  }
  pub fn is_favorite_stickers(&self) -> bool {
    matches!(self, Update::FavoriteStickers(_))
  }
  pub fn is_file(&self) -> bool {
    matches!(self, Update::File(_))
  }
  pub fn is_file_added_to_downloads(&self) -> bool {
    matches!(self, Update::FileAddedToDownloads(_))
  }
  pub fn is_file_download(&self) -> bool {
    matches!(self, Update::FileDownload(_))
  }
  pub fn is_file_downloads(&self) -> bool {
    matches!(self, Update::FileDownloads(_))
  }
  pub fn is_file_generation_start(&self) -> bool {
    matches!(self, Update::FileGenerationStart(_))
  }
  pub fn is_file_generation_stop(&self) -> bool {
    matches!(self, Update::FileGenerationStop(_))
  }
  pub fn is_file_removed_from_downloads(&self) -> bool {
    matches!(self, Update::FileRemovedFromDownloads(_))
  }
  pub fn is_forum_topic(&self) -> bool {
    matches!(self, Update::ForumTopic(_))
  }
  pub fn is_forum_topic_info(&self) -> bool {
    matches!(self, Update::ForumTopicInfo(_))
  }
  pub fn is_freeze_state(&self) -> bool {
    matches!(self, Update::FreezeState(_))
  }
  pub fn is_gift_auction_state(&self) -> bool {
    matches!(self, Update::GiftAuctionState(_))
  }
  pub fn is_gram_revenue_status(&self) -> bool {
    matches!(self, Update::GramRevenueStatus(_))
  }
  pub fn is_group_call(&self) -> bool {
    matches!(self, Update::GroupCall(_))
  }
  pub fn is_group_call_message_levels(&self) -> bool {
    matches!(self, Update::GroupCallMessageLevels(_))
  }
  pub fn is_group_call_message_send_failed(&self) -> bool {
    matches!(self, Update::GroupCallMessageSendFailed(_))
  }
  pub fn is_group_call_messages_deleted(&self) -> bool {
    matches!(self, Update::GroupCallMessagesDeleted(_))
  }
  pub fn is_group_call_participant(&self) -> bool {
    matches!(self, Update::GroupCallParticipant(_))
  }
  pub fn is_group_call_participants(&self) -> bool {
    matches!(self, Update::GroupCallParticipants(_))
  }
  pub fn is_group_call_verification_state(&self) -> bool {
    matches!(self, Update::GroupCallVerificationState(_))
  }
  pub fn is_have_pending_notifications(&self) -> bool {
    matches!(self, Update::HavePendingNotifications(_))
  }
  pub fn is_installed_sticker_sets(&self) -> bool {
    matches!(self, Update::InstalledStickerSets(_))
  }
  pub fn is_language_pack_strings(&self) -> bool {
    matches!(self, Update::LanguagePackStrings(_))
  }
  pub fn is_live_story_top_donors(&self) -> bool {
    matches!(self, Update::LiveStoryTopDonors(_))
  }
  pub fn is_managed_bot(&self) -> bool {
    matches!(self, Update::ManagedBot(_))
  }
  pub fn is_message_contains_unread_poll_votes(&self) -> bool {
    matches!(self, Update::MessageContainsUnreadPollVotes(_))
  }
  pub fn is_message_content(&self) -> bool {
    matches!(self, Update::MessageContent(_))
  }
  pub fn is_message_content_opened(&self) -> bool {
    matches!(self, Update::MessageContentOpened(_))
  }
  pub fn is_message_edited(&self) -> bool {
    matches!(self, Update::MessageEdited(_))
  }
  pub fn is_message_ephemeral_content(&self) -> bool {
    matches!(self, Update::MessageEphemeralContent(_))
  }
  pub fn is_message_fact_check(&self) -> bool {
    matches!(self, Update::MessageFactCheck(_))
  }
  pub fn is_message_interaction_info(&self) -> bool {
    matches!(self, Update::MessageInteractionInfo(_))
  }
  pub fn is_message_is_pinned(&self) -> bool {
    matches!(self, Update::MessageIsPinned(_))
  }
  pub fn is_message_live_location_viewed(&self) -> bool {
    matches!(self, Update::MessageLiveLocationViewed(_))
  }
  pub fn is_message_mention_read(&self) -> bool {
    matches!(self, Update::MessageMentionRead(_))
  }
  pub fn is_message_reaction(&self) -> bool {
    matches!(self, Update::MessageReaction(_))
  }
  pub fn is_message_reactions(&self) -> bool {
    matches!(self, Update::MessageReactions(_))
  }
  pub fn is_message_send_acknowledged(&self) -> bool {
    matches!(self, Update::MessageSendAcknowledged(_))
  }
  pub fn is_message_send_failed(&self) -> bool {
    matches!(self, Update::MessageSendFailed(_))
  }
  pub fn is_message_send_succeeded(&self) -> bool {
    matches!(self, Update::MessageSendSucceeded(_))
  }
  pub fn is_message_suggested_post_info(&self) -> bool {
    matches!(self, Update::MessageSuggestedPostInfo(_))
  }
  pub fn is_message_unread_reactions(&self) -> bool {
    matches!(self, Update::MessageUnreadReactions(_))
  }
  pub fn is_new_business_callback_query(&self) -> bool {
    matches!(self, Update::NewBusinessCallbackQuery(_))
  }
  pub fn is_new_business_message(&self) -> bool {
    matches!(self, Update::NewBusinessMessage(_))
  }
  pub fn is_new_call_signaling_data(&self) -> bool {
    matches!(self, Update::NewCallSignalingData(_))
  }
  pub fn is_new_callback_query(&self) -> bool {
    matches!(self, Update::NewCallbackQuery(_))
  }
  pub fn is_new_chat(&self) -> bool {
    matches!(self, Update::NewChat(_))
  }
  pub fn is_new_chat_join_request(&self) -> bool {
    matches!(self, Update::NewChatJoinRequest(_))
  }
  pub fn is_new_chosen_inline_result(&self) -> bool {
    matches!(self, Update::NewChosenInlineResult(_))
  }
  pub fn is_new_custom_event(&self) -> bool {
    matches!(self, Update::NewCustomEvent(_))
  }
  pub fn is_new_custom_query(&self) -> bool {
    matches!(self, Update::NewCustomQuery(_))
  }
  pub fn is_new_group_call_message(&self) -> bool {
    matches!(self, Update::NewGroupCallMessage(_))
  }
  pub fn is_new_group_call_paid_reaction(&self) -> bool {
    matches!(self, Update::NewGroupCallPaidReaction(_))
  }
  pub fn is_new_guest_query(&self) -> bool {
    matches!(self, Update::NewGuestQuery(_))
  }
  pub fn is_new_inline_callback_query(&self) -> bool {
    matches!(self, Update::NewInlineCallbackQuery(_))
  }
  pub fn is_new_inline_query(&self) -> bool {
    matches!(self, Update::NewInlineQuery(_))
  }
  pub fn is_new_message(&self) -> bool {
    matches!(self, Update::NewMessage(_))
  }
  pub fn is_new_oauth_request(&self) -> bool {
    matches!(self, Update::NewOauthRequest(_))
  }
  pub fn is_new_pre_checkout_query(&self) -> bool {
    matches!(self, Update::NewPreCheckoutQuery(_))
  }
  pub fn is_new_shipping_query(&self) -> bool {
    matches!(self, Update::NewShippingQuery(_))
  }
  pub fn is_notification(&self) -> bool {
    matches!(self, Update::Notification(_))
  }
  pub fn is_notification_group(&self) -> bool {
    matches!(self, Update::NotificationGroup(_))
  }
  pub fn is_option(&self) -> bool {
    matches!(self, Update::Option(_))
  }
  pub fn is_owned_gram_count(&self) -> bool {
    matches!(self, Update::OwnedGramCount(_))
  }
  pub fn is_owned_star_count(&self) -> bool {
    matches!(self, Update::OwnedStarCount(_))
  }
  pub fn is_paid_media_purchased(&self) -> bool {
    matches!(self, Update::PaidMediaPurchased(_))
  }
  pub fn is_pending_message(&self) -> bool {
    matches!(self, Update::PendingMessage(_))
  }
  pub fn is_poll(&self) -> bool {
    matches!(self, Update::Poll(_))
  }
  pub fn is_poll_answer(&self) -> bool {
    matches!(self, Update::PollAnswer(_))
  }
  pub fn is_profile_accent_colors(&self) -> bool {
    matches!(self, Update::ProfileAccentColors(_))
  }
  pub fn is_quick_reply_shortcut(&self) -> bool {
    matches!(self, Update::QuickReplyShortcut(_))
  }
  pub fn is_quick_reply_shortcut_deleted(&self) -> bool {
    matches!(self, Update::QuickReplyShortcutDeleted(_))
  }
  pub fn is_quick_reply_shortcut_messages(&self) -> bool {
    matches!(self, Update::QuickReplyShortcutMessages(_))
  }
  pub fn is_quick_reply_shortcuts(&self) -> bool {
    matches!(self, Update::QuickReplyShortcuts(_))
  }
  pub fn is_reaction_notification_settings(&self) -> bool {
    matches!(self, Update::ReactionNotificationSettings(_))
  }
  pub fn is_recent_stickers(&self) -> bool {
    matches!(self, Update::RecentStickers(_))
  }
  pub fn is_saved_animations(&self) -> bool {
    matches!(self, Update::SavedAnimations(_))
  }
  pub fn is_saved_messages_tags(&self) -> bool {
    matches!(self, Update::SavedMessagesTags(_))
  }
  pub fn is_saved_messages_topic(&self) -> bool {
    matches!(self, Update::SavedMessagesTopic(_))
  }
  pub fn is_saved_messages_topic_count(&self) -> bool {
    matches!(self, Update::SavedMessagesTopicCount(_))
  }
  pub fn is_saved_notification_sounds(&self) -> bool {
    matches!(self, Update::SavedNotificationSounds(_))
  }
  pub fn is_scope_notification_settings(&self) -> bool {
    matches!(self, Update::ScopeNotificationSettings(_))
  }
  pub fn is_secret_chat(&self) -> bool {
    matches!(self, Update::SecretChat(_))
  }
  pub fn is_service_notification(&self) -> bool {
    matches!(self, Update::ServiceNotification(_))
  }
  pub fn is_speech_recognition_trial(&self) -> bool {
    matches!(self, Update::SpeechRecognitionTrial(_))
  }
  pub fn is_speed_limit_notification(&self) -> bool {
    matches!(self, Update::SpeedLimitNotification(_))
  }
  pub fn is_stake_dice_state(&self) -> bool {
    matches!(self, Update::StakeDiceState(_))
  }
  pub fn is_star_revenue_status(&self) -> bool {
    matches!(self, Update::StarRevenueStatus(_))
  }
  pub fn is_sticker_set(&self) -> bool {
    matches!(self, Update::StickerSet(_))
  }
  pub fn is_stop_message_draft(&self) -> bool {
    matches!(self, Update::StopMessageDraft(_))
  }
  pub fn is_story(&self) -> bool {
    matches!(self, Update::Story(_))
  }
  pub fn is_story_deleted(&self) -> bool {
    matches!(self, Update::StoryDeleted(_))
  }
  pub fn is_story_list_chat_count(&self) -> bool {
    matches!(self, Update::StoryListChatCount(_))
  }
  pub fn is_story_post_failed(&self) -> bool {
    matches!(self, Update::StoryPostFailed(_))
  }
  pub fn is_story_post_succeeded(&self) -> bool {
    matches!(self, Update::StoryPostSucceeded(_))
  }
  pub fn is_story_stealth_mode(&self) -> bool {
    matches!(self, Update::StoryStealthMode(_))
  }
  pub fn is_suggested_actions(&self) -> bool {
    matches!(self, Update::SuggestedActions(_))
  }
  pub fn is_supergroup(&self) -> bool {
    matches!(self, Update::Supergroup(_))
  }
  pub fn is_supergroup_full_info(&self) -> bool {
    matches!(self, Update::SupergroupFullInfo(_))
  }
  pub fn is_terms_of_service(&self) -> bool {
    matches!(self, Update::TermsOfService(_))
  }
  pub fn is_text_composition_styles(&self) -> bool {
    matches!(self, Update::TextCompositionStyles(_))
  }
  pub fn is_ton_wallet_gasless_transfers_info(&self) -> bool {
    matches!(self, Update::TonWalletGaslessTransfersInfo(_))
  }
  pub fn is_ton_wallet_state(&self) -> bool {
    matches!(self, Update::TonWalletState(_))
  }
  pub fn is_ton_wallet_ton_connect_session(&self) -> bool {
    matches!(self, Update::TonWalletTonConnectSession(_))
  }
  pub fn is_ton_wallet_ton_connect_session_disconnect_required(&self) -> bool {
    matches!(
      self,
      Update::TonWalletTonConnectSessionDisconnectRequired(_)
    )
  }
  pub fn is_topic_message_count(&self) -> bool {
    matches!(self, Update::TopicMessageCount(_))
  }
  pub fn is_trending_sticker_sets(&self) -> bool {
    matches!(self, Update::TrendingStickerSets(_))
  }
  pub fn is_trusted_mini_app_bots(&self) -> bool {
    matches!(self, Update::TrustedMiniAppBots(_))
  }
  pub fn is_unconfirmed_session(&self) -> bool {
    matches!(self, Update::UnconfirmedSession(_))
  }
  pub fn is_unread_chat_count(&self) -> bool {
    matches!(self, Update::UnreadChatCount(_))
  }
  pub fn is_unread_message_count(&self) -> bool {
    matches!(self, Update::UnreadMessageCount(_))
  }
  pub fn is_user(&self) -> bool {
    matches!(self, Update::User(_))
  }
  pub fn is_user_full_info(&self) -> bool {
    matches!(self, Update::UserFullInfo(_))
  }
  pub fn is_user_privacy_setting_rules(&self) -> bool {
    matches!(self, Update::UserPrivacySettingRules(_))
  }
  pub fn is_user_status(&self) -> bool {
    matches!(self, Update::UserStatus(_))
  }
  pub fn is_user_subscription(&self) -> bool {
    matches!(self, Update::UserSubscription(_))
  }
  pub fn is_video_published(&self) -> bool {
    matches!(self, Update::VideoPublished(_))
  }
  pub fn is_web_app_message_sent(&self) -> bool {
    matches!(self, Update::WebAppMessageSent(_))
  }
  pub fn is_web_browser_settings(&self) -> bool {
    matches!(self, Update::WebBrowserSettings(_))
  }
  pub fn is_web_proxy_carrier_closed(&self) -> bool {
    matches!(self, Update::WebProxyCarrierClosed(_))
  }
  pub fn is_web_proxy_carrier_data(&self) -> bool {
    matches!(self, Update::WebProxyCarrierData(_))
  }
  pub fn is_web_proxy_carrier_opened(&self) -> bool {
    matches!(self, Update::WebProxyCarrierOpened(_))
  }

  pub fn on_test_use_update<F: FnOnce(&TestUseUpdate)>(&self, fnc: F) -> &Self {
    if let Update::TestUseUpdate(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_accent_colors<F: FnOnce(&UpdateAccentColors)>(&self, fnc: F) -> &Self {
    if let Update::AccentColors(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_active_emoji_reactions<F: FnOnce(&UpdateActiveEmojiReactions)>(&self, fnc: F) -> &Self {
    if let Update::ActiveEmojiReactions(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_active_gift_auctions<F: FnOnce(&UpdateActiveGiftAuctions)>(&self, fnc: F) -> &Self {
    if let Update::ActiveGiftAuctions(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_active_live_location_messages<F: FnOnce(&UpdateActiveLiveLocationMessages)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::ActiveLiveLocationMessages(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_active_notifications<F: FnOnce(&UpdateActiveNotifications)>(&self, fnc: F) -> &Self {
    if let Update::ActiveNotifications(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_age_verification_parameters<F: FnOnce(&UpdateAgeVerificationParameters)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::AgeVerificationParameters(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_allowed_bot_username_premium_suffixes<
    F: FnOnce(&UpdateAllowedBotUsernamePremiumSuffixes),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::AllowedBotUsernamePremiumSuffixes(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_animated_emoji_message_clicked<F: FnOnce(&UpdateAnimatedEmojiMessageClicked)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::AnimatedEmojiMessageClicked(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_animation_search_parameters<F: FnOnce(&UpdateAnimationSearchParameters)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::AnimationSearchParameters(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_application_recaptcha_verification_required<
    F: FnOnce(&UpdateApplicationRecaptchaVerificationRequired),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::ApplicationRecaptchaVerificationRequired(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_application_verification_required<F: FnOnce(&UpdateApplicationVerificationRequired)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::ApplicationVerificationRequired(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_attachment_menu_bots<F: FnOnce(&UpdateAttachmentMenuBots)>(&self, fnc: F) -> &Self {
    if let Update::AttachmentMenuBots(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_authorization_state<F: FnOnce(&UpdateAuthorizationState)>(&self, fnc: F) -> &Self {
    if let Update::AuthorizationState(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_autosave_settings<F: FnOnce(&UpdateAutosaveSettings)>(&self, fnc: F) -> &Self {
    if let Update::AutosaveSettings(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_available_message_effects<F: FnOnce(&UpdateAvailableMessageEffects)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::AvailableMessageEffects(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_basic_group<F: FnOnce(&UpdateBasicGroup)>(&self, fnc: F) -> &Self {
    if let Update::BasicGroup(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_basic_group_full_info<F: FnOnce(&UpdateBasicGroupFullInfo)>(&self, fnc: F) -> &Self {
    if let Update::BasicGroupFullInfo(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_business_connection<F: FnOnce(&UpdateBusinessConnection)>(&self, fnc: F) -> &Self {
    if let Update::BusinessConnection(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_business_message_edited<F: FnOnce(&UpdateBusinessMessageEdited)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::BusinessMessageEdited(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_business_messages_deleted<F: FnOnce(&UpdateBusinessMessagesDeleted)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::BusinessMessagesDeleted(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_call<F: FnOnce(&UpdateCall)>(&self, fnc: F) -> &Self {
    if let Update::Call(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_accent_colors<F: FnOnce(&UpdateChatAccentColors)>(&self, fnc: F) -> &Self {
    if let Update::ChatAccentColors(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_action<F: FnOnce(&UpdateChatAction)>(&self, fnc: F) -> &Self {
    if let Update::ChatAction(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_action_bar<F: FnOnce(&UpdateChatActionBar)>(&self, fnc: F) -> &Self {
    if let Update::ChatActionBar(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_active_stories<F: FnOnce(&UpdateChatActiveStories)>(&self, fnc: F) -> &Self {
    if let Update::ChatActiveStories(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_added_to_list<F: FnOnce(&UpdateChatAddedToList)>(&self, fnc: F) -> &Self {
    if let Update::ChatAddedToList(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_available_reactions<F: FnOnce(&UpdateChatAvailableReactions)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::ChatAvailableReactions(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_background<F: FnOnce(&UpdateChatBackground)>(&self, fnc: F) -> &Self {
    if let Update::ChatBackground(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_block_list<F: FnOnce(&UpdateChatBlockList)>(&self, fnc: F) -> &Self {
    if let Update::ChatBlockList(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_boost<F: FnOnce(&UpdateChatBoost)>(&self, fnc: F) -> &Self {
    if let Update::ChatBoost(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_business_bot_manage_bar<F: FnOnce(&UpdateChatBusinessBotManageBar)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::ChatBusinessBotManageBar(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_default_disable_notification<F: FnOnce(&UpdateChatDefaultDisableNotification)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::ChatDefaultDisableNotification(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_draft_message<F: FnOnce(&UpdateChatDraftMessage)>(&self, fnc: F) -> &Self {
    if let Update::ChatDraftMessage(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_emoji_status<F: FnOnce(&UpdateChatEmojiStatus)>(&self, fnc: F) -> &Self {
    if let Update::ChatEmojiStatus(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_folders<F: FnOnce(&UpdateChatFolders)>(&self, fnc: F) -> &Self {
    if let Update::ChatFolders(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_has_protected_content<F: FnOnce(&UpdateChatHasProtectedContent)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::ChatHasProtectedContent(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_has_scheduled_messages<F: FnOnce(&UpdateChatHasScheduledMessages)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::ChatHasScheduledMessages(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_has_welcome_messages<F: FnOnce(&UpdateChatHasWelcomeMessages)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::ChatHasWelcomeMessages(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_is_marked_as_unread<F: FnOnce(&UpdateChatIsMarkedAsUnread)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::ChatIsMarkedAsUnread(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_is_translatable<F: FnOnce(&UpdateChatIsTranslatable)>(&self, fnc: F) -> &Self {
    if let Update::ChatIsTranslatable(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_join_result<F: FnOnce(&UpdateChatJoinResult)>(&self, fnc: F) -> &Self {
    if let Update::ChatJoinResult(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_last_message<F: FnOnce(&UpdateChatLastMessage)>(&self, fnc: F) -> &Self {
    if let Update::ChatLastMessage(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_member<F: FnOnce(&UpdateChatMember)>(&self, fnc: F) -> &Self {
    if let Update::ChatMember(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_message_auto_delete_time<F: FnOnce(&UpdateChatMessageAutoDeleteTime)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::ChatMessageAutoDeleteTime(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_message_sender<F: FnOnce(&UpdateChatMessageSender)>(&self, fnc: F) -> &Self {
    if let Update::ChatMessageSender(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_notification_settings<F: FnOnce(&UpdateChatNotificationSettings)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::ChatNotificationSettings(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_online_member_count<F: FnOnce(&UpdateChatOnlineMemberCount)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::ChatOnlineMemberCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_pending_join_requests<F: FnOnce(&UpdateChatPendingJoinRequests)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::ChatPendingJoinRequests(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_permissions<F: FnOnce(&UpdateChatPermissions)>(&self, fnc: F) -> &Self {
    if let Update::ChatPermissions(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_photo<F: FnOnce(&UpdateChatPhoto)>(&self, fnc: F) -> &Self {
    if let Update::ChatPhoto(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_position<F: FnOnce(&UpdateChatPosition)>(&self, fnc: F) -> &Self {
    if let Update::ChatPosition(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_read_inbox<F: FnOnce(&UpdateChatReadInbox)>(&self, fnc: F) -> &Self {
    if let Update::ChatReadInbox(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_read_outbox<F: FnOnce(&UpdateChatReadOutbox)>(&self, fnc: F) -> &Self {
    if let Update::ChatReadOutbox(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_removed_from_list<F: FnOnce(&UpdateChatRemovedFromList)>(&self, fnc: F) -> &Self {
    if let Update::ChatRemovedFromList(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_reply_markup<F: FnOnce(&UpdateChatReplyMarkup)>(&self, fnc: F) -> &Self {
    if let Update::ChatReplyMarkup(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_revenue_amount<F: FnOnce(&UpdateChatRevenueAmount)>(&self, fnc: F) -> &Self {
    if let Update::ChatRevenueAmount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_theme<F: FnOnce(&UpdateChatTheme)>(&self, fnc: F) -> &Self {
    if let Update::ChatTheme(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_title<F: FnOnce(&UpdateChatTitle)>(&self, fnc: F) -> &Self {
    if let Update::ChatTitle(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_unread_mention_count<F: FnOnce(&UpdateChatUnreadMentionCount)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::ChatUnreadMentionCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_unread_poll_vote_count<F: FnOnce(&UpdateChatUnreadPollVoteCount)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::ChatUnreadPollVoteCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_unread_reaction_count<F: FnOnce(&UpdateChatUnreadReactionCount)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::ChatUnreadReactionCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_video_chat<F: FnOnce(&UpdateChatVideoChat)>(&self, fnc: F) -> &Self {
    if let Update::ChatVideoChat(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_view_as_topics<F: FnOnce(&UpdateChatViewAsTopics)>(&self, fnc: F) -> &Self {
    if let Update::ChatViewAsTopics(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_welcome_messages<F: FnOnce(&UpdateChatWelcomeMessages)>(&self, fnc: F) -> &Self {
    if let Update::ChatWelcomeMessages(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_community<F: FnOnce(&UpdateCommunity)>(&self, fnc: F) -> &Self {
    if let Update::Community(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_community_full_info<F: FnOnce(&UpdateCommunityFullInfo)>(&self, fnc: F) -> &Self {
    if let Update::CommunityFullInfo(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_connection_state<F: FnOnce(&UpdateConnectionState)>(&self, fnc: F) -> &Self {
    if let Update::ConnectionState(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_contact_close_birthdays<F: FnOnce(&UpdateContactCloseBirthdays)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::ContactCloseBirthdays(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_default_background<F: FnOnce(&UpdateDefaultBackground)>(&self, fnc: F) -> &Self {
    if let Update::DefaultBackground(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_default_paid_reaction_type<F: FnOnce(&UpdateDefaultPaidReactionType)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::DefaultPaidReactionType(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_default_reaction_type<F: FnOnce(&UpdateDefaultReactionType)>(&self, fnc: F) -> &Self {
    if let Update::DefaultReactionType(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_delete_messages<F: FnOnce(&UpdateDeleteMessages)>(&self, fnc: F) -> &Self {
    if let Update::DeleteMessages(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_dice_emojis<F: FnOnce(&UpdateDiceEmojis)>(&self, fnc: F) -> &Self {
    if let Update::DiceEmojis(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_direct_messages_chat_topic<F: FnOnce(&UpdateDirectMessagesChatTopic)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::DirectMessagesChatTopic(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_emoji_chat_themes<F: FnOnce(&UpdateEmojiChatThemes)>(&self, fnc: F) -> &Self {
    if let Update::EmojiChatThemes(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_favorite_stickers<F: FnOnce(&UpdateFavoriteStickers)>(&self, fnc: F) -> &Self {
    if let Update::FavoriteStickers(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_file<F: FnOnce(&UpdateFile)>(&self, fnc: F) -> &Self {
    if let Update::File(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_file_added_to_downloads<F: FnOnce(&UpdateFileAddedToDownloads)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::FileAddedToDownloads(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_file_download<F: FnOnce(&UpdateFileDownload)>(&self, fnc: F) -> &Self {
    if let Update::FileDownload(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_file_downloads<F: FnOnce(&UpdateFileDownloads)>(&self, fnc: F) -> &Self {
    if let Update::FileDownloads(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_file_generation_start<F: FnOnce(&UpdateFileGenerationStart)>(&self, fnc: F) -> &Self {
    if let Update::FileGenerationStart(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_file_generation_stop<F: FnOnce(&UpdateFileGenerationStop)>(&self, fnc: F) -> &Self {
    if let Update::FileGenerationStop(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_file_removed_from_downloads<F: FnOnce(&UpdateFileRemovedFromDownloads)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::FileRemovedFromDownloads(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_forum_topic<F: FnOnce(&UpdateForumTopic)>(&self, fnc: F) -> &Self {
    if let Update::ForumTopic(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_forum_topic_info<F: FnOnce(&UpdateForumTopicInfo)>(&self, fnc: F) -> &Self {
    if let Update::ForumTopicInfo(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_freeze_state<F: FnOnce(&UpdateFreezeState)>(&self, fnc: F) -> &Self {
    if let Update::FreezeState(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_gift_auction_state<F: FnOnce(&UpdateGiftAuctionState)>(&self, fnc: F) -> &Self {
    if let Update::GiftAuctionState(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_gram_revenue_status<F: FnOnce(&UpdateGramRevenueStatus)>(&self, fnc: F) -> &Self {
    if let Update::GramRevenueStatus(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_group_call<F: FnOnce(&UpdateGroupCall)>(&self, fnc: F) -> &Self {
    if let Update::GroupCall(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_group_call_message_levels<F: FnOnce(&UpdateGroupCallMessageLevels)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::GroupCallMessageLevels(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_group_call_message_send_failed<F: FnOnce(&UpdateGroupCallMessageSendFailed)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::GroupCallMessageSendFailed(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_group_call_messages_deleted<F: FnOnce(&UpdateGroupCallMessagesDeleted)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::GroupCallMessagesDeleted(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_group_call_participant<F: FnOnce(&UpdateGroupCallParticipant)>(&self, fnc: F) -> &Self {
    if let Update::GroupCallParticipant(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_group_call_participants<F: FnOnce(&UpdateGroupCallParticipants)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::GroupCallParticipants(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_group_call_verification_state<F: FnOnce(&UpdateGroupCallVerificationState)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::GroupCallVerificationState(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_have_pending_notifications<F: FnOnce(&UpdateHavePendingNotifications)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::HavePendingNotifications(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_installed_sticker_sets<F: FnOnce(&UpdateInstalledStickerSets)>(&self, fnc: F) -> &Self {
    if let Update::InstalledStickerSets(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_language_pack_strings<F: FnOnce(&UpdateLanguagePackStrings)>(&self, fnc: F) -> &Self {
    if let Update::LanguagePackStrings(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_live_story_top_donors<F: FnOnce(&UpdateLiveStoryTopDonors)>(&self, fnc: F) -> &Self {
    if let Update::LiveStoryTopDonors(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_managed_bot<F: FnOnce(&UpdateManagedBot)>(&self, fnc: F) -> &Self {
    if let Update::ManagedBot(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message_contains_unread_poll_votes<F: FnOnce(&UpdateMessageContainsUnreadPollVotes)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::MessageContainsUnreadPollVotes(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message_content<F: FnOnce(&UpdateMessageContent)>(&self, fnc: F) -> &Self {
    if let Update::MessageContent(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message_content_opened<F: FnOnce(&UpdateMessageContentOpened)>(&self, fnc: F) -> &Self {
    if let Update::MessageContentOpened(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message_edited<F: FnOnce(&UpdateMessageEdited)>(&self, fnc: F) -> &Self {
    if let Update::MessageEdited(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message_ephemeral_content<F: FnOnce(&UpdateMessageEphemeralContent)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::MessageEphemeralContent(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message_fact_check<F: FnOnce(&UpdateMessageFactCheck)>(&self, fnc: F) -> &Self {
    if let Update::MessageFactCheck(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message_interaction_info<F: FnOnce(&UpdateMessageInteractionInfo)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::MessageInteractionInfo(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message_is_pinned<F: FnOnce(&UpdateMessageIsPinned)>(&self, fnc: F) -> &Self {
    if let Update::MessageIsPinned(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message_live_location_viewed<F: FnOnce(&UpdateMessageLiveLocationViewed)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::MessageLiveLocationViewed(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message_mention_read<F: FnOnce(&UpdateMessageMentionRead)>(&self, fnc: F) -> &Self {
    if let Update::MessageMentionRead(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message_reaction<F: FnOnce(&UpdateMessageReaction)>(&self, fnc: F) -> &Self {
    if let Update::MessageReaction(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message_reactions<F: FnOnce(&UpdateMessageReactions)>(&self, fnc: F) -> &Self {
    if let Update::MessageReactions(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message_send_acknowledged<F: FnOnce(&UpdateMessageSendAcknowledged)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::MessageSendAcknowledged(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message_send_failed<F: FnOnce(&UpdateMessageSendFailed)>(&self, fnc: F) -> &Self {
    if let Update::MessageSendFailed(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message_send_succeeded<F: FnOnce(&UpdateMessageSendSucceeded)>(&self, fnc: F) -> &Self {
    if let Update::MessageSendSucceeded(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message_suggested_post_info<F: FnOnce(&UpdateMessageSuggestedPostInfo)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::MessageSuggestedPostInfo(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message_unread_reactions<F: FnOnce(&UpdateMessageUnreadReactions)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::MessageUnreadReactions(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_new_business_callback_query<F: FnOnce(&UpdateNewBusinessCallbackQuery)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::NewBusinessCallbackQuery(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_new_business_message<F: FnOnce(&UpdateNewBusinessMessage)>(&self, fnc: F) -> &Self {
    if let Update::NewBusinessMessage(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_new_call_signaling_data<F: FnOnce(&UpdateNewCallSignalingData)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::NewCallSignalingData(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_new_callback_query<F: FnOnce(&UpdateNewCallbackQuery)>(&self, fnc: F) -> &Self {
    if let Update::NewCallbackQuery(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_new_chat<F: FnOnce(&UpdateNewChat)>(&self, fnc: F) -> &Self {
    if let Update::NewChat(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_new_chat_join_request<F: FnOnce(&UpdateNewChatJoinRequest)>(&self, fnc: F) -> &Self {
    if let Update::NewChatJoinRequest(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_new_chosen_inline_result<F: FnOnce(&UpdateNewChosenInlineResult)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::NewChosenInlineResult(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_new_custom_event<F: FnOnce(&UpdateNewCustomEvent)>(&self, fnc: F) -> &Self {
    if let Update::NewCustomEvent(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_new_custom_query<F: FnOnce(&UpdateNewCustomQuery)>(&self, fnc: F) -> &Self {
    if let Update::NewCustomQuery(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_new_group_call_message<F: FnOnce(&UpdateNewGroupCallMessage)>(&self, fnc: F) -> &Self {
    if let Update::NewGroupCallMessage(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_new_group_call_paid_reaction<F: FnOnce(&UpdateNewGroupCallPaidReaction)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::NewGroupCallPaidReaction(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_new_guest_query<F: FnOnce(&UpdateNewGuestQuery)>(&self, fnc: F) -> &Self {
    if let Update::NewGuestQuery(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_new_inline_callback_query<F: FnOnce(&UpdateNewInlineCallbackQuery)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::NewInlineCallbackQuery(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_new_inline_query<F: FnOnce(&UpdateNewInlineQuery)>(&self, fnc: F) -> &Self {
    if let Update::NewInlineQuery(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_new_message<F: FnOnce(&UpdateNewMessage)>(&self, fnc: F) -> &Self {
    if let Update::NewMessage(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_new_oauth_request<F: FnOnce(&UpdateNewOauthRequest)>(&self, fnc: F) -> &Self {
    if let Update::NewOauthRequest(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_new_pre_checkout_query<F: FnOnce(&UpdateNewPreCheckoutQuery)>(&self, fnc: F) -> &Self {
    if let Update::NewPreCheckoutQuery(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_new_shipping_query<F: FnOnce(&UpdateNewShippingQuery)>(&self, fnc: F) -> &Self {
    if let Update::NewShippingQuery(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_notification<F: FnOnce(&UpdateNotification)>(&self, fnc: F) -> &Self {
    if let Update::Notification(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_notification_group<F: FnOnce(&UpdateNotificationGroup)>(&self, fnc: F) -> &Self {
    if let Update::NotificationGroup(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_option<F: FnOnce(&UpdateOption)>(&self, fnc: F) -> &Self {
    if let Update::Option(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_owned_gram_count<F: FnOnce(&UpdateOwnedGramCount)>(&self, fnc: F) -> &Self {
    if let Update::OwnedGramCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_owned_star_count<F: FnOnce(&UpdateOwnedStarCount)>(&self, fnc: F) -> &Self {
    if let Update::OwnedStarCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_paid_media_purchased<F: FnOnce(&UpdatePaidMediaPurchased)>(&self, fnc: F) -> &Self {
    if let Update::PaidMediaPurchased(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_pending_message<F: FnOnce(&UpdatePendingMessage)>(&self, fnc: F) -> &Self {
    if let Update::PendingMessage(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_poll<F: FnOnce(&UpdatePoll)>(&self, fnc: F) -> &Self {
    if let Update::Poll(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_poll_answer<F: FnOnce(&UpdatePollAnswer)>(&self, fnc: F) -> &Self {
    if let Update::PollAnswer(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_profile_accent_colors<F: FnOnce(&UpdateProfileAccentColors)>(&self, fnc: F) -> &Self {
    if let Update::ProfileAccentColors(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_quick_reply_shortcut<F: FnOnce(&UpdateQuickReplyShortcut)>(&self, fnc: F) -> &Self {
    if let Update::QuickReplyShortcut(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_quick_reply_shortcut_deleted<F: FnOnce(&UpdateQuickReplyShortcutDeleted)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::QuickReplyShortcutDeleted(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_quick_reply_shortcut_messages<F: FnOnce(&UpdateQuickReplyShortcutMessages)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::QuickReplyShortcutMessages(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_quick_reply_shortcuts<F: FnOnce(&UpdateQuickReplyShortcuts)>(&self, fnc: F) -> &Self {
    if let Update::QuickReplyShortcuts(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_reaction_notification_settings<F: FnOnce(&UpdateReactionNotificationSettings)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::ReactionNotificationSettings(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_recent_stickers<F: FnOnce(&UpdateRecentStickers)>(&self, fnc: F) -> &Self {
    if let Update::RecentStickers(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_saved_animations<F: FnOnce(&UpdateSavedAnimations)>(&self, fnc: F) -> &Self {
    if let Update::SavedAnimations(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_saved_messages_tags<F: FnOnce(&UpdateSavedMessagesTags)>(&self, fnc: F) -> &Self {
    if let Update::SavedMessagesTags(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_saved_messages_topic<F: FnOnce(&UpdateSavedMessagesTopic)>(&self, fnc: F) -> &Self {
    if let Update::SavedMessagesTopic(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_saved_messages_topic_count<F: FnOnce(&UpdateSavedMessagesTopicCount)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::SavedMessagesTopicCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_saved_notification_sounds<F: FnOnce(&UpdateSavedNotificationSounds)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::SavedNotificationSounds(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_scope_notification_settings<F: FnOnce(&UpdateScopeNotificationSettings)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::ScopeNotificationSettings(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_secret_chat<F: FnOnce(&UpdateSecretChat)>(&self, fnc: F) -> &Self {
    if let Update::SecretChat(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_service_notification<F: FnOnce(&UpdateServiceNotification)>(&self, fnc: F) -> &Self {
    if let Update::ServiceNotification(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_speech_recognition_trial<F: FnOnce(&UpdateSpeechRecognitionTrial)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::SpeechRecognitionTrial(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_speed_limit_notification<F: FnOnce(&UpdateSpeedLimitNotification)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::SpeedLimitNotification(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_stake_dice_state<F: FnOnce(&UpdateStakeDiceState)>(&self, fnc: F) -> &Self {
    if let Update::StakeDiceState(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_star_revenue_status<F: FnOnce(&UpdateStarRevenueStatus)>(&self, fnc: F) -> &Self {
    if let Update::StarRevenueStatus(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_sticker_set<F: FnOnce(&UpdateStickerSet)>(&self, fnc: F) -> &Self {
    if let Update::StickerSet(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_stop_message_draft<F: FnOnce(&UpdateStopMessageDraft)>(&self, fnc: F) -> &Self {
    if let Update::StopMessageDraft(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_story<F: FnOnce(&UpdateStory)>(&self, fnc: F) -> &Self {
    if let Update::Story(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_story_deleted<F: FnOnce(&UpdateStoryDeleted)>(&self, fnc: F) -> &Self {
    if let Update::StoryDeleted(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_story_list_chat_count<F: FnOnce(&UpdateStoryListChatCount)>(&self, fnc: F) -> &Self {
    if let Update::StoryListChatCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_story_post_failed<F: FnOnce(&UpdateStoryPostFailed)>(&self, fnc: F) -> &Self {
    if let Update::StoryPostFailed(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_story_post_succeeded<F: FnOnce(&UpdateStoryPostSucceeded)>(&self, fnc: F) -> &Self {
    if let Update::StoryPostSucceeded(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_story_stealth_mode<F: FnOnce(&UpdateStoryStealthMode)>(&self, fnc: F) -> &Self {
    if let Update::StoryStealthMode(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_suggested_actions<F: FnOnce(&UpdateSuggestedActions)>(&self, fnc: F) -> &Self {
    if let Update::SuggestedActions(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_supergroup<F: FnOnce(&UpdateSupergroup)>(&self, fnc: F) -> &Self {
    if let Update::Supergroup(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_supergroup_full_info<F: FnOnce(&UpdateSupergroupFullInfo)>(&self, fnc: F) -> &Self {
    if let Update::SupergroupFullInfo(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_terms_of_service<F: FnOnce(&UpdateTermsOfService)>(&self, fnc: F) -> &Self {
    if let Update::TermsOfService(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_text_composition_styles<F: FnOnce(&UpdateTextCompositionStyles)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::TextCompositionStyles(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ton_wallet_gasless_transfers_info<F: FnOnce(&UpdateTonWalletGaslessTransfersInfo)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::TonWalletGaslessTransfersInfo(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ton_wallet_state<F: FnOnce(&UpdateTonWalletState)>(&self, fnc: F) -> &Self {
    if let Update::TonWalletState(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ton_wallet_ton_connect_session<F: FnOnce(&UpdateTonWalletTonConnectSession)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::TonWalletTonConnectSession(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ton_wallet_ton_connect_session_disconnect_required<
    F: FnOnce(&UpdateTonWalletTonConnectSessionDisconnectRequired),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::TonWalletTonConnectSessionDisconnectRequired(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_topic_message_count<F: FnOnce(&UpdateTopicMessageCount)>(&self, fnc: F) -> &Self {
    if let Update::TopicMessageCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_trending_sticker_sets<F: FnOnce(&UpdateTrendingStickerSets)>(&self, fnc: F) -> &Self {
    if let Update::TrendingStickerSets(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_trusted_mini_app_bots<F: FnOnce(&UpdateTrustedMiniAppBots)>(&self, fnc: F) -> &Self {
    if let Update::TrustedMiniAppBots(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_unconfirmed_session<F: FnOnce(&UpdateUnconfirmedSession)>(&self, fnc: F) -> &Self {
    if let Update::UnconfirmedSession(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_unread_chat_count<F: FnOnce(&UpdateUnreadChatCount)>(&self, fnc: F) -> &Self {
    if let Update::UnreadChatCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_unread_message_count<F: FnOnce(&UpdateUnreadMessageCount)>(&self, fnc: F) -> &Self {
    if let Update::UnreadMessageCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_user<F: FnOnce(&UpdateUser)>(&self, fnc: F) -> &Self {
    if let Update::User(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_user_full_info<F: FnOnce(&UpdateUserFullInfo)>(&self, fnc: F) -> &Self {
    if let Update::UserFullInfo(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_user_privacy_setting_rules<F: FnOnce(&UpdateUserPrivacySettingRules)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::UserPrivacySettingRules(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_user_status<F: FnOnce(&UpdateUserStatus)>(&self, fnc: F) -> &Self {
    if let Update::UserStatus(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_user_subscription<F: FnOnce(&UpdateUserSubscription)>(&self, fnc: F) -> &Self {
    if let Update::UserSubscription(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_video_published<F: FnOnce(&UpdateVideoPublished)>(&self, fnc: F) -> &Self {
    if let Update::VideoPublished(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_web_app_message_sent<F: FnOnce(&UpdateWebAppMessageSent)>(&self, fnc: F) -> &Self {
    if let Update::WebAppMessageSent(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_web_browser_settings<F: FnOnce(&UpdateWebBrowserSettings)>(&self, fnc: F) -> &Self {
    if let Update::WebBrowserSettings(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_web_proxy_carrier_closed<F: FnOnce(&UpdateWebProxyCarrierClosed)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::WebProxyCarrierClosed(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_web_proxy_carrier_data<F: FnOnce(&UpdateWebProxyCarrierData)>(&self, fnc: F) -> &Self {
    if let Update::WebProxyCarrierData(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_web_proxy_carrier_opened<F: FnOnce(&UpdateWebProxyCarrierOpened)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let Update::WebProxyCarrierOpened(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_test_use_update(&self) -> Option<&TestUseUpdate> {
    if let Update::TestUseUpdate(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_accent_colors(&self) -> Option<&UpdateAccentColors> {
    if let Update::AccentColors(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_active_emoji_reactions(&self) -> Option<&UpdateActiveEmojiReactions> {
    if let Update::ActiveEmojiReactions(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_active_gift_auctions(&self) -> Option<&UpdateActiveGiftAuctions> {
    if let Update::ActiveGiftAuctions(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_active_live_location_messages(&self) -> Option<&UpdateActiveLiveLocationMessages> {
    if let Update::ActiveLiveLocationMessages(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_active_notifications(&self) -> Option<&UpdateActiveNotifications> {
    if let Update::ActiveNotifications(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_age_verification_parameters(&self) -> Option<&UpdateAgeVerificationParameters> {
    if let Update::AgeVerificationParameters(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_allowed_bot_username_premium_suffixes(
    &self,
  ) -> Option<&UpdateAllowedBotUsernamePremiumSuffixes> {
    if let Update::AllowedBotUsernamePremiumSuffixes(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_animated_emoji_message_clicked(&self) -> Option<&UpdateAnimatedEmojiMessageClicked> {
    if let Update::AnimatedEmojiMessageClicked(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_animation_search_parameters(&self) -> Option<&UpdateAnimationSearchParameters> {
    if let Update::AnimationSearchParameters(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_application_recaptcha_verification_required(
    &self,
  ) -> Option<&UpdateApplicationRecaptchaVerificationRequired> {
    if let Update::ApplicationRecaptchaVerificationRequired(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_application_verification_required(
    &self,
  ) -> Option<&UpdateApplicationVerificationRequired> {
    if let Update::ApplicationVerificationRequired(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_attachment_menu_bots(&self) -> Option<&UpdateAttachmentMenuBots> {
    if let Update::AttachmentMenuBots(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_authorization_state(&self) -> Option<&UpdateAuthorizationState> {
    if let Update::AuthorizationState(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_autosave_settings(&self) -> Option<&UpdateAutosaveSettings> {
    if let Update::AutosaveSettings(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_available_message_effects(&self) -> Option<&UpdateAvailableMessageEffects> {
    if let Update::AvailableMessageEffects(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_basic_group(&self) -> Option<&UpdateBasicGroup> {
    if let Update::BasicGroup(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_basic_group_full_info(&self) -> Option<&UpdateBasicGroupFullInfo> {
    if let Update::BasicGroupFullInfo(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_business_connection(&self) -> Option<&UpdateBusinessConnection> {
    if let Update::BusinessConnection(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_business_message_edited(&self) -> Option<&UpdateBusinessMessageEdited> {
    if let Update::BusinessMessageEdited(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_business_messages_deleted(&self) -> Option<&UpdateBusinessMessagesDeleted> {
    if let Update::BusinessMessagesDeleted(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_call(&self) -> Option<&UpdateCall> {
    if let Update::Call(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_accent_colors(&self) -> Option<&UpdateChatAccentColors> {
    if let Update::ChatAccentColors(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_action(&self) -> Option<&UpdateChatAction> {
    if let Update::ChatAction(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_action_bar(&self) -> Option<&UpdateChatActionBar> {
    if let Update::ChatActionBar(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_active_stories(&self) -> Option<&UpdateChatActiveStories> {
    if let Update::ChatActiveStories(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_added_to_list(&self) -> Option<&UpdateChatAddedToList> {
    if let Update::ChatAddedToList(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_available_reactions(&self) -> Option<&UpdateChatAvailableReactions> {
    if let Update::ChatAvailableReactions(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_background(&self) -> Option<&UpdateChatBackground> {
    if let Update::ChatBackground(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_block_list(&self) -> Option<&UpdateChatBlockList> {
    if let Update::ChatBlockList(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_boost(&self) -> Option<&UpdateChatBoost> {
    if let Update::ChatBoost(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_business_bot_manage_bar(&self) -> Option<&UpdateChatBusinessBotManageBar> {
    if let Update::ChatBusinessBotManageBar(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_default_disable_notification(
    &self,
  ) -> Option<&UpdateChatDefaultDisableNotification> {
    if let Update::ChatDefaultDisableNotification(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_draft_message(&self) -> Option<&UpdateChatDraftMessage> {
    if let Update::ChatDraftMessage(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_emoji_status(&self) -> Option<&UpdateChatEmojiStatus> {
    if let Update::ChatEmojiStatus(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_folders(&self) -> Option<&UpdateChatFolders> {
    if let Update::ChatFolders(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_has_protected_content(&self) -> Option<&UpdateChatHasProtectedContent> {
    if let Update::ChatHasProtectedContent(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_has_scheduled_messages(&self) -> Option<&UpdateChatHasScheduledMessages> {
    if let Update::ChatHasScheduledMessages(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_has_welcome_messages(&self) -> Option<&UpdateChatHasWelcomeMessages> {
    if let Update::ChatHasWelcomeMessages(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_is_marked_as_unread(&self) -> Option<&UpdateChatIsMarkedAsUnread> {
    if let Update::ChatIsMarkedAsUnread(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_is_translatable(&self) -> Option<&UpdateChatIsTranslatable> {
    if let Update::ChatIsTranslatable(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_join_result(&self) -> Option<&UpdateChatJoinResult> {
    if let Update::ChatJoinResult(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_last_message(&self) -> Option<&UpdateChatLastMessage> {
    if let Update::ChatLastMessage(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_member(&self) -> Option<&UpdateChatMember> {
    if let Update::ChatMember(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_message_auto_delete_time(&self) -> Option<&UpdateChatMessageAutoDeleteTime> {
    if let Update::ChatMessageAutoDeleteTime(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_message_sender(&self) -> Option<&UpdateChatMessageSender> {
    if let Update::ChatMessageSender(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_notification_settings(&self) -> Option<&UpdateChatNotificationSettings> {
    if let Update::ChatNotificationSettings(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_online_member_count(&self) -> Option<&UpdateChatOnlineMemberCount> {
    if let Update::ChatOnlineMemberCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_pending_join_requests(&self) -> Option<&UpdateChatPendingJoinRequests> {
    if let Update::ChatPendingJoinRequests(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_permissions(&self) -> Option<&UpdateChatPermissions> {
    if let Update::ChatPermissions(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_photo(&self) -> Option<&UpdateChatPhoto> {
    if let Update::ChatPhoto(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_position(&self) -> Option<&UpdateChatPosition> {
    if let Update::ChatPosition(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_read_inbox(&self) -> Option<&UpdateChatReadInbox> {
    if let Update::ChatReadInbox(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_read_outbox(&self) -> Option<&UpdateChatReadOutbox> {
    if let Update::ChatReadOutbox(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_removed_from_list(&self) -> Option<&UpdateChatRemovedFromList> {
    if let Update::ChatRemovedFromList(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_reply_markup(&self) -> Option<&UpdateChatReplyMarkup> {
    if let Update::ChatReplyMarkup(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_revenue_amount(&self) -> Option<&UpdateChatRevenueAmount> {
    if let Update::ChatRevenueAmount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_theme(&self) -> Option<&UpdateChatTheme> {
    if let Update::ChatTheme(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_title(&self) -> Option<&UpdateChatTitle> {
    if let Update::ChatTitle(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_unread_mention_count(&self) -> Option<&UpdateChatUnreadMentionCount> {
    if let Update::ChatUnreadMentionCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_unread_poll_vote_count(&self) -> Option<&UpdateChatUnreadPollVoteCount> {
    if let Update::ChatUnreadPollVoteCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_unread_reaction_count(&self) -> Option<&UpdateChatUnreadReactionCount> {
    if let Update::ChatUnreadReactionCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_video_chat(&self) -> Option<&UpdateChatVideoChat> {
    if let Update::ChatVideoChat(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_view_as_topics(&self) -> Option<&UpdateChatViewAsTopics> {
    if let Update::ChatViewAsTopics(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_welcome_messages(&self) -> Option<&UpdateChatWelcomeMessages> {
    if let Update::ChatWelcomeMessages(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_community(&self) -> Option<&UpdateCommunity> {
    if let Update::Community(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_community_full_info(&self) -> Option<&UpdateCommunityFullInfo> {
    if let Update::CommunityFullInfo(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_connection_state(&self) -> Option<&UpdateConnectionState> {
    if let Update::ConnectionState(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_contact_close_birthdays(&self) -> Option<&UpdateContactCloseBirthdays> {
    if let Update::ContactCloseBirthdays(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_default_background(&self) -> Option<&UpdateDefaultBackground> {
    if let Update::DefaultBackground(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_default_paid_reaction_type(&self) -> Option<&UpdateDefaultPaidReactionType> {
    if let Update::DefaultPaidReactionType(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_default_reaction_type(&self) -> Option<&UpdateDefaultReactionType> {
    if let Update::DefaultReactionType(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_delete_messages(&self) -> Option<&UpdateDeleteMessages> {
    if let Update::DeleteMessages(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_dice_emojis(&self) -> Option<&UpdateDiceEmojis> {
    if let Update::DiceEmojis(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_direct_messages_chat_topic(&self) -> Option<&UpdateDirectMessagesChatTopic> {
    if let Update::DirectMessagesChatTopic(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_emoji_chat_themes(&self) -> Option<&UpdateEmojiChatThemes> {
    if let Update::EmojiChatThemes(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_favorite_stickers(&self) -> Option<&UpdateFavoriteStickers> {
    if let Update::FavoriteStickers(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_file(&self) -> Option<&UpdateFile> {
    if let Update::File(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_file_added_to_downloads(&self) -> Option<&UpdateFileAddedToDownloads> {
    if let Update::FileAddedToDownloads(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_file_download(&self) -> Option<&UpdateFileDownload> {
    if let Update::FileDownload(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_file_downloads(&self) -> Option<&UpdateFileDownloads> {
    if let Update::FileDownloads(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_file_generation_start(&self) -> Option<&UpdateFileGenerationStart> {
    if let Update::FileGenerationStart(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_file_generation_stop(&self) -> Option<&UpdateFileGenerationStop> {
    if let Update::FileGenerationStop(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_file_removed_from_downloads(&self) -> Option<&UpdateFileRemovedFromDownloads> {
    if let Update::FileRemovedFromDownloads(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_forum_topic(&self) -> Option<&UpdateForumTopic> {
    if let Update::ForumTopic(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_forum_topic_info(&self) -> Option<&UpdateForumTopicInfo> {
    if let Update::ForumTopicInfo(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_freeze_state(&self) -> Option<&UpdateFreezeState> {
    if let Update::FreezeState(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_gift_auction_state(&self) -> Option<&UpdateGiftAuctionState> {
    if let Update::GiftAuctionState(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_gram_revenue_status(&self) -> Option<&UpdateGramRevenueStatus> {
    if let Update::GramRevenueStatus(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_group_call(&self) -> Option<&UpdateGroupCall> {
    if let Update::GroupCall(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_group_call_message_levels(&self) -> Option<&UpdateGroupCallMessageLevels> {
    if let Update::GroupCallMessageLevels(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_group_call_message_send_failed(&self) -> Option<&UpdateGroupCallMessageSendFailed> {
    if let Update::GroupCallMessageSendFailed(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_group_call_messages_deleted(&self) -> Option<&UpdateGroupCallMessagesDeleted> {
    if let Update::GroupCallMessagesDeleted(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_group_call_participant(&self) -> Option<&UpdateGroupCallParticipant> {
    if let Update::GroupCallParticipant(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_group_call_participants(&self) -> Option<&UpdateGroupCallParticipants> {
    if let Update::GroupCallParticipants(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_group_call_verification_state(&self) -> Option<&UpdateGroupCallVerificationState> {
    if let Update::GroupCallVerificationState(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_have_pending_notifications(&self) -> Option<&UpdateHavePendingNotifications> {
    if let Update::HavePendingNotifications(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_installed_sticker_sets(&self) -> Option<&UpdateInstalledStickerSets> {
    if let Update::InstalledStickerSets(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_language_pack_strings(&self) -> Option<&UpdateLanguagePackStrings> {
    if let Update::LanguagePackStrings(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_live_story_top_donors(&self) -> Option<&UpdateLiveStoryTopDonors> {
    if let Update::LiveStoryTopDonors(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_managed_bot(&self) -> Option<&UpdateManagedBot> {
    if let Update::ManagedBot(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message_contains_unread_poll_votes(
    &self,
  ) -> Option<&UpdateMessageContainsUnreadPollVotes> {
    if let Update::MessageContainsUnreadPollVotes(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message_content(&self) -> Option<&UpdateMessageContent> {
    if let Update::MessageContent(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message_content_opened(&self) -> Option<&UpdateMessageContentOpened> {
    if let Update::MessageContentOpened(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message_edited(&self) -> Option<&UpdateMessageEdited> {
    if let Update::MessageEdited(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message_ephemeral_content(&self) -> Option<&UpdateMessageEphemeralContent> {
    if let Update::MessageEphemeralContent(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message_fact_check(&self) -> Option<&UpdateMessageFactCheck> {
    if let Update::MessageFactCheck(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message_interaction_info(&self) -> Option<&UpdateMessageInteractionInfo> {
    if let Update::MessageInteractionInfo(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message_is_pinned(&self) -> Option<&UpdateMessageIsPinned> {
    if let Update::MessageIsPinned(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message_live_location_viewed(&self) -> Option<&UpdateMessageLiveLocationViewed> {
    if let Update::MessageLiveLocationViewed(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message_mention_read(&self) -> Option<&UpdateMessageMentionRead> {
    if let Update::MessageMentionRead(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message_reaction(&self) -> Option<&UpdateMessageReaction> {
    if let Update::MessageReaction(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message_reactions(&self) -> Option<&UpdateMessageReactions> {
    if let Update::MessageReactions(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message_send_acknowledged(&self) -> Option<&UpdateMessageSendAcknowledged> {
    if let Update::MessageSendAcknowledged(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message_send_failed(&self) -> Option<&UpdateMessageSendFailed> {
    if let Update::MessageSendFailed(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message_send_succeeded(&self) -> Option<&UpdateMessageSendSucceeded> {
    if let Update::MessageSendSucceeded(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message_suggested_post_info(&self) -> Option<&UpdateMessageSuggestedPostInfo> {
    if let Update::MessageSuggestedPostInfo(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message_unread_reactions(&self) -> Option<&UpdateMessageUnreadReactions> {
    if let Update::MessageUnreadReactions(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_new_business_callback_query(&self) -> Option<&UpdateNewBusinessCallbackQuery> {
    if let Update::NewBusinessCallbackQuery(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_new_business_message(&self) -> Option<&UpdateNewBusinessMessage> {
    if let Update::NewBusinessMessage(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_new_call_signaling_data(&self) -> Option<&UpdateNewCallSignalingData> {
    if let Update::NewCallSignalingData(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_new_callback_query(&self) -> Option<&UpdateNewCallbackQuery> {
    if let Update::NewCallbackQuery(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_new_chat(&self) -> Option<&UpdateNewChat> {
    if let Update::NewChat(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_new_chat_join_request(&self) -> Option<&UpdateNewChatJoinRequest> {
    if let Update::NewChatJoinRequest(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_new_chosen_inline_result(&self) -> Option<&UpdateNewChosenInlineResult> {
    if let Update::NewChosenInlineResult(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_new_custom_event(&self) -> Option<&UpdateNewCustomEvent> {
    if let Update::NewCustomEvent(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_new_custom_query(&self) -> Option<&UpdateNewCustomQuery> {
    if let Update::NewCustomQuery(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_new_group_call_message(&self) -> Option<&UpdateNewGroupCallMessage> {
    if let Update::NewGroupCallMessage(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_new_group_call_paid_reaction(&self) -> Option<&UpdateNewGroupCallPaidReaction> {
    if let Update::NewGroupCallPaidReaction(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_new_guest_query(&self) -> Option<&UpdateNewGuestQuery> {
    if let Update::NewGuestQuery(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_new_inline_callback_query(&self) -> Option<&UpdateNewInlineCallbackQuery> {
    if let Update::NewInlineCallbackQuery(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_new_inline_query(&self) -> Option<&UpdateNewInlineQuery> {
    if let Update::NewInlineQuery(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_new_message(&self) -> Option<&UpdateNewMessage> {
    if let Update::NewMessage(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_new_oauth_request(&self) -> Option<&UpdateNewOauthRequest> {
    if let Update::NewOauthRequest(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_new_pre_checkout_query(&self) -> Option<&UpdateNewPreCheckoutQuery> {
    if let Update::NewPreCheckoutQuery(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_new_shipping_query(&self) -> Option<&UpdateNewShippingQuery> {
    if let Update::NewShippingQuery(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_notification(&self) -> Option<&UpdateNotification> {
    if let Update::Notification(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_notification_group(&self) -> Option<&UpdateNotificationGroup> {
    if let Update::NotificationGroup(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_option(&self) -> Option<&UpdateOption> {
    if let Update::Option(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_owned_gram_count(&self) -> Option<&UpdateOwnedGramCount> {
    if let Update::OwnedGramCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_owned_star_count(&self) -> Option<&UpdateOwnedStarCount> {
    if let Update::OwnedStarCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_paid_media_purchased(&self) -> Option<&UpdatePaidMediaPurchased> {
    if let Update::PaidMediaPurchased(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_pending_message(&self) -> Option<&UpdatePendingMessage> {
    if let Update::PendingMessage(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_poll(&self) -> Option<&UpdatePoll> {
    if let Update::Poll(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_poll_answer(&self) -> Option<&UpdatePollAnswer> {
    if let Update::PollAnswer(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_profile_accent_colors(&self) -> Option<&UpdateProfileAccentColors> {
    if let Update::ProfileAccentColors(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_quick_reply_shortcut(&self) -> Option<&UpdateQuickReplyShortcut> {
    if let Update::QuickReplyShortcut(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_quick_reply_shortcut_deleted(&self) -> Option<&UpdateQuickReplyShortcutDeleted> {
    if let Update::QuickReplyShortcutDeleted(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_quick_reply_shortcut_messages(&self) -> Option<&UpdateQuickReplyShortcutMessages> {
    if let Update::QuickReplyShortcutMessages(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_quick_reply_shortcuts(&self) -> Option<&UpdateQuickReplyShortcuts> {
    if let Update::QuickReplyShortcuts(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_reaction_notification_settings(&self) -> Option<&UpdateReactionNotificationSettings> {
    if let Update::ReactionNotificationSettings(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_recent_stickers(&self) -> Option<&UpdateRecentStickers> {
    if let Update::RecentStickers(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_saved_animations(&self) -> Option<&UpdateSavedAnimations> {
    if let Update::SavedAnimations(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_saved_messages_tags(&self) -> Option<&UpdateSavedMessagesTags> {
    if let Update::SavedMessagesTags(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_saved_messages_topic(&self) -> Option<&UpdateSavedMessagesTopic> {
    if let Update::SavedMessagesTopic(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_saved_messages_topic_count(&self) -> Option<&UpdateSavedMessagesTopicCount> {
    if let Update::SavedMessagesTopicCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_saved_notification_sounds(&self) -> Option<&UpdateSavedNotificationSounds> {
    if let Update::SavedNotificationSounds(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_scope_notification_settings(&self) -> Option<&UpdateScopeNotificationSettings> {
    if let Update::ScopeNotificationSettings(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_secret_chat(&self) -> Option<&UpdateSecretChat> {
    if let Update::SecretChat(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_service_notification(&self) -> Option<&UpdateServiceNotification> {
    if let Update::ServiceNotification(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_speech_recognition_trial(&self) -> Option<&UpdateSpeechRecognitionTrial> {
    if let Update::SpeechRecognitionTrial(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_speed_limit_notification(&self) -> Option<&UpdateSpeedLimitNotification> {
    if let Update::SpeedLimitNotification(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_stake_dice_state(&self) -> Option<&UpdateStakeDiceState> {
    if let Update::StakeDiceState(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_star_revenue_status(&self) -> Option<&UpdateStarRevenueStatus> {
    if let Update::StarRevenueStatus(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_sticker_set(&self) -> Option<&UpdateStickerSet> {
    if let Update::StickerSet(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_stop_message_draft(&self) -> Option<&UpdateStopMessageDraft> {
    if let Update::StopMessageDraft(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_story(&self) -> Option<&UpdateStory> {
    if let Update::Story(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_story_deleted(&self) -> Option<&UpdateStoryDeleted> {
    if let Update::StoryDeleted(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_story_list_chat_count(&self) -> Option<&UpdateStoryListChatCount> {
    if let Update::StoryListChatCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_story_post_failed(&self) -> Option<&UpdateStoryPostFailed> {
    if let Update::StoryPostFailed(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_story_post_succeeded(&self) -> Option<&UpdateStoryPostSucceeded> {
    if let Update::StoryPostSucceeded(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_story_stealth_mode(&self) -> Option<&UpdateStoryStealthMode> {
    if let Update::StoryStealthMode(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_suggested_actions(&self) -> Option<&UpdateSuggestedActions> {
    if let Update::SuggestedActions(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_supergroup(&self) -> Option<&UpdateSupergroup> {
    if let Update::Supergroup(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_supergroup_full_info(&self) -> Option<&UpdateSupergroupFullInfo> {
    if let Update::SupergroupFullInfo(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_terms_of_service(&self) -> Option<&UpdateTermsOfService> {
    if let Update::TermsOfService(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_text_composition_styles(&self) -> Option<&UpdateTextCompositionStyles> {
    if let Update::TextCompositionStyles(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ton_wallet_gasless_transfers_info(
    &self,
  ) -> Option<&UpdateTonWalletGaslessTransfersInfo> {
    if let Update::TonWalletGaslessTransfersInfo(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ton_wallet_state(&self) -> Option<&UpdateTonWalletState> {
    if let Update::TonWalletState(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ton_wallet_ton_connect_session(&self) -> Option<&UpdateTonWalletTonConnectSession> {
    if let Update::TonWalletTonConnectSession(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ton_wallet_ton_connect_session_disconnect_required(
    &self,
  ) -> Option<&UpdateTonWalletTonConnectSessionDisconnectRequired> {
    if let Update::TonWalletTonConnectSessionDisconnectRequired(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_topic_message_count(&self) -> Option<&UpdateTopicMessageCount> {
    if let Update::TopicMessageCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_trending_sticker_sets(&self) -> Option<&UpdateTrendingStickerSets> {
    if let Update::TrendingStickerSets(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_trusted_mini_app_bots(&self) -> Option<&UpdateTrustedMiniAppBots> {
    if let Update::TrustedMiniAppBots(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_unconfirmed_session(&self) -> Option<&UpdateUnconfirmedSession> {
    if let Update::UnconfirmedSession(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_unread_chat_count(&self) -> Option<&UpdateUnreadChatCount> {
    if let Update::UnreadChatCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_unread_message_count(&self) -> Option<&UpdateUnreadMessageCount> {
    if let Update::UnreadMessageCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_user(&self) -> Option<&UpdateUser> {
    if let Update::User(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_user_full_info(&self) -> Option<&UpdateUserFullInfo> {
    if let Update::UserFullInfo(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_user_privacy_setting_rules(&self) -> Option<&UpdateUserPrivacySettingRules> {
    if let Update::UserPrivacySettingRules(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_user_status(&self) -> Option<&UpdateUserStatus> {
    if let Update::UserStatus(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_user_subscription(&self) -> Option<&UpdateUserSubscription> {
    if let Update::UserSubscription(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_video_published(&self) -> Option<&UpdateVideoPublished> {
    if let Update::VideoPublished(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_web_app_message_sent(&self) -> Option<&UpdateWebAppMessageSent> {
    if let Update::WebAppMessageSent(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_web_browser_settings(&self) -> Option<&UpdateWebBrowserSettings> {
    if let Update::WebBrowserSettings(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_web_proxy_carrier_closed(&self) -> Option<&UpdateWebProxyCarrierClosed> {
    if let Update::WebProxyCarrierClosed(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_web_proxy_carrier_data(&self) -> Option<&UpdateWebProxyCarrierData> {
    if let Update::WebProxyCarrierData(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_web_proxy_carrier_opened(&self) -> Option<&UpdateWebProxyCarrierOpened> {
    if let Update::WebProxyCarrierOpened(t) = self {
      return Some(t);
    }
    None
  }

  pub fn test_use_update<T: AsRef<TestUseUpdate>>(t: T) -> Self {
    Update::TestUseUpdate(t.as_ref().clone())
  }

  pub fn accent_colors<T: AsRef<UpdateAccentColors>>(t: T) -> Self {
    Update::AccentColors(t.as_ref().clone())
  }

  pub fn active_emoji_reactions<T: AsRef<UpdateActiveEmojiReactions>>(t: T) -> Self {
    Update::ActiveEmojiReactions(t.as_ref().clone())
  }

  pub fn active_gift_auctions<T: AsRef<UpdateActiveGiftAuctions>>(t: T) -> Self {
    Update::ActiveGiftAuctions(t.as_ref().clone())
  }

  pub fn active_live_location_messages<T: AsRef<UpdateActiveLiveLocationMessages>>(t: T) -> Self {
    Update::ActiveLiveLocationMessages(t.as_ref().clone())
  }

  pub fn active_notifications<T: AsRef<UpdateActiveNotifications>>(t: T) -> Self {
    Update::ActiveNotifications(t.as_ref().clone())
  }

  pub fn age_verification_parameters<T: AsRef<UpdateAgeVerificationParameters>>(t: T) -> Self {
    Update::AgeVerificationParameters(t.as_ref().clone())
  }

  pub fn allowed_bot_username_premium_suffixes<
    T: AsRef<UpdateAllowedBotUsernamePremiumSuffixes>,
  >(
    t: T,
  ) -> Self {
    Update::AllowedBotUsernamePremiumSuffixes(t.as_ref().clone())
  }

  pub fn animated_emoji_message_clicked<T: AsRef<UpdateAnimatedEmojiMessageClicked>>(t: T) -> Self {
    Update::AnimatedEmojiMessageClicked(t.as_ref().clone())
  }

  pub fn animation_search_parameters<T: AsRef<UpdateAnimationSearchParameters>>(t: T) -> Self {
    Update::AnimationSearchParameters(t.as_ref().clone())
  }

  pub fn application_recaptcha_verification_required<
    T: AsRef<UpdateApplicationRecaptchaVerificationRequired>,
  >(
    t: T,
  ) -> Self {
    Update::ApplicationRecaptchaVerificationRequired(t.as_ref().clone())
  }

  pub fn application_verification_required<T: AsRef<UpdateApplicationVerificationRequired>>(
    t: T,
  ) -> Self {
    Update::ApplicationVerificationRequired(t.as_ref().clone())
  }

  pub fn attachment_menu_bots<T: AsRef<UpdateAttachmentMenuBots>>(t: T) -> Self {
    Update::AttachmentMenuBots(t.as_ref().clone())
  }

  pub fn authorization_state<T: AsRef<UpdateAuthorizationState>>(t: T) -> Self {
    Update::AuthorizationState(t.as_ref().clone())
  }

  pub fn autosave_settings<T: AsRef<UpdateAutosaveSettings>>(t: T) -> Self {
    Update::AutosaveSettings(t.as_ref().clone())
  }

  pub fn available_message_effects<T: AsRef<UpdateAvailableMessageEffects>>(t: T) -> Self {
    Update::AvailableMessageEffects(t.as_ref().clone())
  }

  pub fn basic_group<T: AsRef<UpdateBasicGroup>>(t: T) -> Self {
    Update::BasicGroup(t.as_ref().clone())
  }

  pub fn basic_group_full_info<T: AsRef<UpdateBasicGroupFullInfo>>(t: T) -> Self {
    Update::BasicGroupFullInfo(t.as_ref().clone())
  }

  pub fn business_connection<T: AsRef<UpdateBusinessConnection>>(t: T) -> Self {
    Update::BusinessConnection(t.as_ref().clone())
  }

  pub fn business_message_edited<T: AsRef<UpdateBusinessMessageEdited>>(t: T) -> Self {
    Update::BusinessMessageEdited(t.as_ref().clone())
  }

  pub fn business_messages_deleted<T: AsRef<UpdateBusinessMessagesDeleted>>(t: T) -> Self {
    Update::BusinessMessagesDeleted(t.as_ref().clone())
  }

  pub fn call<T: AsRef<UpdateCall>>(t: T) -> Self {
    Update::Call(t.as_ref().clone())
  }

  pub fn chat_accent_colors<T: AsRef<UpdateChatAccentColors>>(t: T) -> Self {
    Update::ChatAccentColors(t.as_ref().clone())
  }

  pub fn chat_action<T: AsRef<UpdateChatAction>>(t: T) -> Self {
    Update::ChatAction(t.as_ref().clone())
  }

  pub fn chat_action_bar<T: AsRef<UpdateChatActionBar>>(t: T) -> Self {
    Update::ChatActionBar(t.as_ref().clone())
  }

  pub fn chat_active_stories<T: AsRef<UpdateChatActiveStories>>(t: T) -> Self {
    Update::ChatActiveStories(t.as_ref().clone())
  }

  pub fn chat_added_to_list<T: AsRef<UpdateChatAddedToList>>(t: T) -> Self {
    Update::ChatAddedToList(t.as_ref().clone())
  }

  pub fn chat_available_reactions<T: AsRef<UpdateChatAvailableReactions>>(t: T) -> Self {
    Update::ChatAvailableReactions(t.as_ref().clone())
  }

  pub fn chat_background<T: AsRef<UpdateChatBackground>>(t: T) -> Self {
    Update::ChatBackground(t.as_ref().clone())
  }

  pub fn chat_block_list<T: AsRef<UpdateChatBlockList>>(t: T) -> Self {
    Update::ChatBlockList(t.as_ref().clone())
  }

  pub fn chat_boost<T: AsRef<UpdateChatBoost>>(t: T) -> Self {
    Update::ChatBoost(t.as_ref().clone())
  }

  pub fn chat_business_bot_manage_bar<T: AsRef<UpdateChatBusinessBotManageBar>>(t: T) -> Self {
    Update::ChatBusinessBotManageBar(t.as_ref().clone())
  }

  pub fn chat_default_disable_notification<T: AsRef<UpdateChatDefaultDisableNotification>>(
    t: T,
  ) -> Self {
    Update::ChatDefaultDisableNotification(t.as_ref().clone())
  }

  pub fn chat_draft_message<T: AsRef<UpdateChatDraftMessage>>(t: T) -> Self {
    Update::ChatDraftMessage(t.as_ref().clone())
  }

  pub fn chat_emoji_status<T: AsRef<UpdateChatEmojiStatus>>(t: T) -> Self {
    Update::ChatEmojiStatus(t.as_ref().clone())
  }

  pub fn chat_folders<T: AsRef<UpdateChatFolders>>(t: T) -> Self {
    Update::ChatFolders(t.as_ref().clone())
  }

  pub fn chat_has_protected_content<T: AsRef<UpdateChatHasProtectedContent>>(t: T) -> Self {
    Update::ChatHasProtectedContent(t.as_ref().clone())
  }

  pub fn chat_has_scheduled_messages<T: AsRef<UpdateChatHasScheduledMessages>>(t: T) -> Self {
    Update::ChatHasScheduledMessages(t.as_ref().clone())
  }

  pub fn chat_has_welcome_messages<T: AsRef<UpdateChatHasWelcomeMessages>>(t: T) -> Self {
    Update::ChatHasWelcomeMessages(t.as_ref().clone())
  }

  pub fn chat_is_marked_as_unread<T: AsRef<UpdateChatIsMarkedAsUnread>>(t: T) -> Self {
    Update::ChatIsMarkedAsUnread(t.as_ref().clone())
  }

  pub fn chat_is_translatable<T: AsRef<UpdateChatIsTranslatable>>(t: T) -> Self {
    Update::ChatIsTranslatable(t.as_ref().clone())
  }

  pub fn chat_join_result<T: AsRef<UpdateChatJoinResult>>(t: T) -> Self {
    Update::ChatJoinResult(t.as_ref().clone())
  }

  pub fn chat_last_message<T: AsRef<UpdateChatLastMessage>>(t: T) -> Self {
    Update::ChatLastMessage(t.as_ref().clone())
  }

  pub fn chat_member<T: AsRef<UpdateChatMember>>(t: T) -> Self {
    Update::ChatMember(t.as_ref().clone())
  }

  pub fn chat_message_auto_delete_time<T: AsRef<UpdateChatMessageAutoDeleteTime>>(t: T) -> Self {
    Update::ChatMessageAutoDeleteTime(t.as_ref().clone())
  }

  pub fn chat_message_sender<T: AsRef<UpdateChatMessageSender>>(t: T) -> Self {
    Update::ChatMessageSender(t.as_ref().clone())
  }

  pub fn chat_notification_settings<T: AsRef<UpdateChatNotificationSettings>>(t: T) -> Self {
    Update::ChatNotificationSettings(t.as_ref().clone())
  }

  pub fn chat_online_member_count<T: AsRef<UpdateChatOnlineMemberCount>>(t: T) -> Self {
    Update::ChatOnlineMemberCount(t.as_ref().clone())
  }

  pub fn chat_pending_join_requests<T: AsRef<UpdateChatPendingJoinRequests>>(t: T) -> Self {
    Update::ChatPendingJoinRequests(t.as_ref().clone())
  }

  pub fn chat_permissions<T: AsRef<UpdateChatPermissions>>(t: T) -> Self {
    Update::ChatPermissions(t.as_ref().clone())
  }

  pub fn chat_photo<T: AsRef<UpdateChatPhoto>>(t: T) -> Self {
    Update::ChatPhoto(t.as_ref().clone())
  }

  pub fn chat_position<T: AsRef<UpdateChatPosition>>(t: T) -> Self {
    Update::ChatPosition(t.as_ref().clone())
  }

  pub fn chat_read_inbox<T: AsRef<UpdateChatReadInbox>>(t: T) -> Self {
    Update::ChatReadInbox(t.as_ref().clone())
  }

  pub fn chat_read_outbox<T: AsRef<UpdateChatReadOutbox>>(t: T) -> Self {
    Update::ChatReadOutbox(t.as_ref().clone())
  }

  pub fn chat_removed_from_list<T: AsRef<UpdateChatRemovedFromList>>(t: T) -> Self {
    Update::ChatRemovedFromList(t.as_ref().clone())
  }

  pub fn chat_reply_markup<T: AsRef<UpdateChatReplyMarkup>>(t: T) -> Self {
    Update::ChatReplyMarkup(t.as_ref().clone())
  }

  pub fn chat_revenue_amount<T: AsRef<UpdateChatRevenueAmount>>(t: T) -> Self {
    Update::ChatRevenueAmount(t.as_ref().clone())
  }

  pub fn chat_theme<T: AsRef<UpdateChatTheme>>(t: T) -> Self {
    Update::ChatTheme(t.as_ref().clone())
  }

  pub fn chat_title<T: AsRef<UpdateChatTitle>>(t: T) -> Self {
    Update::ChatTitle(t.as_ref().clone())
  }

  pub fn chat_unread_mention_count<T: AsRef<UpdateChatUnreadMentionCount>>(t: T) -> Self {
    Update::ChatUnreadMentionCount(t.as_ref().clone())
  }

  pub fn chat_unread_poll_vote_count<T: AsRef<UpdateChatUnreadPollVoteCount>>(t: T) -> Self {
    Update::ChatUnreadPollVoteCount(t.as_ref().clone())
  }

  pub fn chat_unread_reaction_count<T: AsRef<UpdateChatUnreadReactionCount>>(t: T) -> Self {
    Update::ChatUnreadReactionCount(t.as_ref().clone())
  }

  pub fn chat_video_chat<T: AsRef<UpdateChatVideoChat>>(t: T) -> Self {
    Update::ChatVideoChat(t.as_ref().clone())
  }

  pub fn chat_view_as_topics<T: AsRef<UpdateChatViewAsTopics>>(t: T) -> Self {
    Update::ChatViewAsTopics(t.as_ref().clone())
  }

  pub fn chat_welcome_messages<T: AsRef<UpdateChatWelcomeMessages>>(t: T) -> Self {
    Update::ChatWelcomeMessages(t.as_ref().clone())
  }

  pub fn community<T: AsRef<UpdateCommunity>>(t: T) -> Self {
    Update::Community(t.as_ref().clone())
  }

  pub fn community_full_info<T: AsRef<UpdateCommunityFullInfo>>(t: T) -> Self {
    Update::CommunityFullInfo(t.as_ref().clone())
  }

  pub fn connection_state<T: AsRef<UpdateConnectionState>>(t: T) -> Self {
    Update::ConnectionState(t.as_ref().clone())
  }

  pub fn contact_close_birthdays<T: AsRef<UpdateContactCloseBirthdays>>(t: T) -> Self {
    Update::ContactCloseBirthdays(t.as_ref().clone())
  }

  pub fn default_background<T: AsRef<UpdateDefaultBackground>>(t: T) -> Self {
    Update::DefaultBackground(t.as_ref().clone())
  }

  pub fn default_paid_reaction_type<T: AsRef<UpdateDefaultPaidReactionType>>(t: T) -> Self {
    Update::DefaultPaidReactionType(t.as_ref().clone())
  }

  pub fn default_reaction_type<T: AsRef<UpdateDefaultReactionType>>(t: T) -> Self {
    Update::DefaultReactionType(t.as_ref().clone())
  }

  pub fn delete_messages<T: AsRef<UpdateDeleteMessages>>(t: T) -> Self {
    Update::DeleteMessages(t.as_ref().clone())
  }

  pub fn dice_emojis<T: AsRef<UpdateDiceEmojis>>(t: T) -> Self {
    Update::DiceEmojis(t.as_ref().clone())
  }

  pub fn direct_messages_chat_topic<T: AsRef<UpdateDirectMessagesChatTopic>>(t: T) -> Self {
    Update::DirectMessagesChatTopic(t.as_ref().clone())
  }

  pub fn emoji_chat_themes<T: AsRef<UpdateEmojiChatThemes>>(t: T) -> Self {
    Update::EmojiChatThemes(t.as_ref().clone())
  }

  pub fn favorite_stickers<T: AsRef<UpdateFavoriteStickers>>(t: T) -> Self {
    Update::FavoriteStickers(t.as_ref().clone())
  }

  pub fn file<T: AsRef<UpdateFile>>(t: T) -> Self {
    Update::File(t.as_ref().clone())
  }

  pub fn file_added_to_downloads<T: AsRef<UpdateFileAddedToDownloads>>(t: T) -> Self {
    Update::FileAddedToDownloads(t.as_ref().clone())
  }

  pub fn file_download<T: AsRef<UpdateFileDownload>>(t: T) -> Self {
    Update::FileDownload(t.as_ref().clone())
  }

  pub fn file_downloads<T: AsRef<UpdateFileDownloads>>(t: T) -> Self {
    Update::FileDownloads(t.as_ref().clone())
  }

  pub fn file_generation_start<T: AsRef<UpdateFileGenerationStart>>(t: T) -> Self {
    Update::FileGenerationStart(t.as_ref().clone())
  }

  pub fn file_generation_stop<T: AsRef<UpdateFileGenerationStop>>(t: T) -> Self {
    Update::FileGenerationStop(t.as_ref().clone())
  }

  pub fn file_removed_from_downloads<T: AsRef<UpdateFileRemovedFromDownloads>>(t: T) -> Self {
    Update::FileRemovedFromDownloads(t.as_ref().clone())
  }

  pub fn forum_topic<T: AsRef<UpdateForumTopic>>(t: T) -> Self {
    Update::ForumTopic(t.as_ref().clone())
  }

  pub fn forum_topic_info<T: AsRef<UpdateForumTopicInfo>>(t: T) -> Self {
    Update::ForumTopicInfo(t.as_ref().clone())
  }

  pub fn freeze_state<T: AsRef<UpdateFreezeState>>(t: T) -> Self {
    Update::FreezeState(t.as_ref().clone())
  }

  pub fn gift_auction_state<T: AsRef<UpdateGiftAuctionState>>(t: T) -> Self {
    Update::GiftAuctionState(t.as_ref().clone())
  }

  pub fn gram_revenue_status<T: AsRef<UpdateGramRevenueStatus>>(t: T) -> Self {
    Update::GramRevenueStatus(t.as_ref().clone())
  }

  pub fn group_call<T: AsRef<UpdateGroupCall>>(t: T) -> Self {
    Update::GroupCall(t.as_ref().clone())
  }

  pub fn group_call_message_levels<T: AsRef<UpdateGroupCallMessageLevels>>(t: T) -> Self {
    Update::GroupCallMessageLevels(t.as_ref().clone())
  }

  pub fn group_call_message_send_failed<T: AsRef<UpdateGroupCallMessageSendFailed>>(t: T) -> Self {
    Update::GroupCallMessageSendFailed(t.as_ref().clone())
  }

  pub fn group_call_messages_deleted<T: AsRef<UpdateGroupCallMessagesDeleted>>(t: T) -> Self {
    Update::GroupCallMessagesDeleted(t.as_ref().clone())
  }

  pub fn group_call_participant<T: AsRef<UpdateGroupCallParticipant>>(t: T) -> Self {
    Update::GroupCallParticipant(t.as_ref().clone())
  }

  pub fn group_call_participants<T: AsRef<UpdateGroupCallParticipants>>(t: T) -> Self {
    Update::GroupCallParticipants(t.as_ref().clone())
  }

  pub fn group_call_verification_state<T: AsRef<UpdateGroupCallVerificationState>>(t: T) -> Self {
    Update::GroupCallVerificationState(t.as_ref().clone())
  }

  pub fn have_pending_notifications<T: AsRef<UpdateHavePendingNotifications>>(t: T) -> Self {
    Update::HavePendingNotifications(t.as_ref().clone())
  }

  pub fn installed_sticker_sets<T: AsRef<UpdateInstalledStickerSets>>(t: T) -> Self {
    Update::InstalledStickerSets(t.as_ref().clone())
  }

  pub fn language_pack_strings<T: AsRef<UpdateLanguagePackStrings>>(t: T) -> Self {
    Update::LanguagePackStrings(t.as_ref().clone())
  }

  pub fn live_story_top_donors<T: AsRef<UpdateLiveStoryTopDonors>>(t: T) -> Self {
    Update::LiveStoryTopDonors(t.as_ref().clone())
  }

  pub fn managed_bot<T: AsRef<UpdateManagedBot>>(t: T) -> Self {
    Update::ManagedBot(t.as_ref().clone())
  }

  pub fn message_contains_unread_poll_votes<T: AsRef<UpdateMessageContainsUnreadPollVotes>>(
    t: T,
  ) -> Self {
    Update::MessageContainsUnreadPollVotes(t.as_ref().clone())
  }

  pub fn message_content<T: AsRef<UpdateMessageContent>>(t: T) -> Self {
    Update::MessageContent(t.as_ref().clone())
  }

  pub fn message_content_opened<T: AsRef<UpdateMessageContentOpened>>(t: T) -> Self {
    Update::MessageContentOpened(t.as_ref().clone())
  }

  pub fn message_edited<T: AsRef<UpdateMessageEdited>>(t: T) -> Self {
    Update::MessageEdited(t.as_ref().clone())
  }

  pub fn message_ephemeral_content<T: AsRef<UpdateMessageEphemeralContent>>(t: T) -> Self {
    Update::MessageEphemeralContent(t.as_ref().clone())
  }

  pub fn message_fact_check<T: AsRef<UpdateMessageFactCheck>>(t: T) -> Self {
    Update::MessageFactCheck(t.as_ref().clone())
  }

  pub fn message_interaction_info<T: AsRef<UpdateMessageInteractionInfo>>(t: T) -> Self {
    Update::MessageInteractionInfo(t.as_ref().clone())
  }

  pub fn message_is_pinned<T: AsRef<UpdateMessageIsPinned>>(t: T) -> Self {
    Update::MessageIsPinned(t.as_ref().clone())
  }

  pub fn message_live_location_viewed<T: AsRef<UpdateMessageLiveLocationViewed>>(t: T) -> Self {
    Update::MessageLiveLocationViewed(t.as_ref().clone())
  }

  pub fn message_mention_read<T: AsRef<UpdateMessageMentionRead>>(t: T) -> Self {
    Update::MessageMentionRead(t.as_ref().clone())
  }

  pub fn message_reaction<T: AsRef<UpdateMessageReaction>>(t: T) -> Self {
    Update::MessageReaction(t.as_ref().clone())
  }

  pub fn message_reactions<T: AsRef<UpdateMessageReactions>>(t: T) -> Self {
    Update::MessageReactions(t.as_ref().clone())
  }

  pub fn message_send_acknowledged<T: AsRef<UpdateMessageSendAcknowledged>>(t: T) -> Self {
    Update::MessageSendAcknowledged(t.as_ref().clone())
  }

  pub fn message_send_failed<T: AsRef<UpdateMessageSendFailed>>(t: T) -> Self {
    Update::MessageSendFailed(t.as_ref().clone())
  }

  pub fn message_send_succeeded<T: AsRef<UpdateMessageSendSucceeded>>(t: T) -> Self {
    Update::MessageSendSucceeded(t.as_ref().clone())
  }

  pub fn message_suggested_post_info<T: AsRef<UpdateMessageSuggestedPostInfo>>(t: T) -> Self {
    Update::MessageSuggestedPostInfo(t.as_ref().clone())
  }

  pub fn message_unread_reactions<T: AsRef<UpdateMessageUnreadReactions>>(t: T) -> Self {
    Update::MessageUnreadReactions(t.as_ref().clone())
  }

  pub fn new_business_callback_query<T: AsRef<UpdateNewBusinessCallbackQuery>>(t: T) -> Self {
    Update::NewBusinessCallbackQuery(t.as_ref().clone())
  }

  pub fn new_business_message<T: AsRef<UpdateNewBusinessMessage>>(t: T) -> Self {
    Update::NewBusinessMessage(t.as_ref().clone())
  }

  pub fn new_call_signaling_data<T: AsRef<UpdateNewCallSignalingData>>(t: T) -> Self {
    Update::NewCallSignalingData(t.as_ref().clone())
  }

  pub fn new_callback_query<T: AsRef<UpdateNewCallbackQuery>>(t: T) -> Self {
    Update::NewCallbackQuery(t.as_ref().clone())
  }

  pub fn new_chat<T: AsRef<UpdateNewChat>>(t: T) -> Self {
    Update::NewChat(t.as_ref().clone())
  }

  pub fn new_chat_join_request<T: AsRef<UpdateNewChatJoinRequest>>(t: T) -> Self {
    Update::NewChatJoinRequest(t.as_ref().clone())
  }

  pub fn new_chosen_inline_result<T: AsRef<UpdateNewChosenInlineResult>>(t: T) -> Self {
    Update::NewChosenInlineResult(t.as_ref().clone())
  }

  pub fn new_custom_event<T: AsRef<UpdateNewCustomEvent>>(t: T) -> Self {
    Update::NewCustomEvent(t.as_ref().clone())
  }

  pub fn new_custom_query<T: AsRef<UpdateNewCustomQuery>>(t: T) -> Self {
    Update::NewCustomQuery(t.as_ref().clone())
  }

  pub fn new_group_call_message<T: AsRef<UpdateNewGroupCallMessage>>(t: T) -> Self {
    Update::NewGroupCallMessage(t.as_ref().clone())
  }

  pub fn new_group_call_paid_reaction<T: AsRef<UpdateNewGroupCallPaidReaction>>(t: T) -> Self {
    Update::NewGroupCallPaidReaction(t.as_ref().clone())
  }

  pub fn new_guest_query<T: AsRef<UpdateNewGuestQuery>>(t: T) -> Self {
    Update::NewGuestQuery(t.as_ref().clone())
  }

  pub fn new_inline_callback_query<T: AsRef<UpdateNewInlineCallbackQuery>>(t: T) -> Self {
    Update::NewInlineCallbackQuery(t.as_ref().clone())
  }

  pub fn new_inline_query<T: AsRef<UpdateNewInlineQuery>>(t: T) -> Self {
    Update::NewInlineQuery(t.as_ref().clone())
  }

  pub fn new_message<T: AsRef<UpdateNewMessage>>(t: T) -> Self {
    Update::NewMessage(t.as_ref().clone())
  }

  pub fn new_oauth_request<T: AsRef<UpdateNewOauthRequest>>(t: T) -> Self {
    Update::NewOauthRequest(t.as_ref().clone())
  }

  pub fn new_pre_checkout_query<T: AsRef<UpdateNewPreCheckoutQuery>>(t: T) -> Self {
    Update::NewPreCheckoutQuery(t.as_ref().clone())
  }

  pub fn new_shipping_query<T: AsRef<UpdateNewShippingQuery>>(t: T) -> Self {
    Update::NewShippingQuery(t.as_ref().clone())
  }

  pub fn notification<T: AsRef<UpdateNotification>>(t: T) -> Self {
    Update::Notification(t.as_ref().clone())
  }

  pub fn notification_group<T: AsRef<UpdateNotificationGroup>>(t: T) -> Self {
    Update::NotificationGroup(t.as_ref().clone())
  }

  pub fn option<T: AsRef<UpdateOption>>(t: T) -> Self {
    Update::Option(t.as_ref().clone())
  }

  pub fn owned_gram_count<T: AsRef<UpdateOwnedGramCount>>(t: T) -> Self {
    Update::OwnedGramCount(t.as_ref().clone())
  }

  pub fn owned_star_count<T: AsRef<UpdateOwnedStarCount>>(t: T) -> Self {
    Update::OwnedStarCount(t.as_ref().clone())
  }

  pub fn paid_media_purchased<T: AsRef<UpdatePaidMediaPurchased>>(t: T) -> Self {
    Update::PaidMediaPurchased(t.as_ref().clone())
  }

  pub fn pending_message<T: AsRef<UpdatePendingMessage>>(t: T) -> Self {
    Update::PendingMessage(t.as_ref().clone())
  }

  pub fn poll<T: AsRef<UpdatePoll>>(t: T) -> Self {
    Update::Poll(t.as_ref().clone())
  }

  pub fn poll_answer<T: AsRef<UpdatePollAnswer>>(t: T) -> Self {
    Update::PollAnswer(t.as_ref().clone())
  }

  pub fn profile_accent_colors<T: AsRef<UpdateProfileAccentColors>>(t: T) -> Self {
    Update::ProfileAccentColors(t.as_ref().clone())
  }

  pub fn quick_reply_shortcut<T: AsRef<UpdateQuickReplyShortcut>>(t: T) -> Self {
    Update::QuickReplyShortcut(t.as_ref().clone())
  }

  pub fn quick_reply_shortcut_deleted<T: AsRef<UpdateQuickReplyShortcutDeleted>>(t: T) -> Self {
    Update::QuickReplyShortcutDeleted(t.as_ref().clone())
  }

  pub fn quick_reply_shortcut_messages<T: AsRef<UpdateQuickReplyShortcutMessages>>(t: T) -> Self {
    Update::QuickReplyShortcutMessages(t.as_ref().clone())
  }

  pub fn quick_reply_shortcuts<T: AsRef<UpdateQuickReplyShortcuts>>(t: T) -> Self {
    Update::QuickReplyShortcuts(t.as_ref().clone())
  }

  pub fn reaction_notification_settings<T: AsRef<UpdateReactionNotificationSettings>>(
    t: T,
  ) -> Self {
    Update::ReactionNotificationSettings(t.as_ref().clone())
  }

  pub fn recent_stickers<T: AsRef<UpdateRecentStickers>>(t: T) -> Self {
    Update::RecentStickers(t.as_ref().clone())
  }

  pub fn saved_animations<T: AsRef<UpdateSavedAnimations>>(t: T) -> Self {
    Update::SavedAnimations(t.as_ref().clone())
  }

  pub fn saved_messages_tags<T: AsRef<UpdateSavedMessagesTags>>(t: T) -> Self {
    Update::SavedMessagesTags(t.as_ref().clone())
  }

  pub fn saved_messages_topic<T: AsRef<UpdateSavedMessagesTopic>>(t: T) -> Self {
    Update::SavedMessagesTopic(t.as_ref().clone())
  }

  pub fn saved_messages_topic_count<T: AsRef<UpdateSavedMessagesTopicCount>>(t: T) -> Self {
    Update::SavedMessagesTopicCount(t.as_ref().clone())
  }

  pub fn saved_notification_sounds<T: AsRef<UpdateSavedNotificationSounds>>(t: T) -> Self {
    Update::SavedNotificationSounds(t.as_ref().clone())
  }

  pub fn scope_notification_settings<T: AsRef<UpdateScopeNotificationSettings>>(t: T) -> Self {
    Update::ScopeNotificationSettings(t.as_ref().clone())
  }

  pub fn secret_chat<T: AsRef<UpdateSecretChat>>(t: T) -> Self {
    Update::SecretChat(t.as_ref().clone())
  }

  pub fn service_notification<T: AsRef<UpdateServiceNotification>>(t: T) -> Self {
    Update::ServiceNotification(t.as_ref().clone())
  }

  pub fn speech_recognition_trial<T: AsRef<UpdateSpeechRecognitionTrial>>(t: T) -> Self {
    Update::SpeechRecognitionTrial(t.as_ref().clone())
  }

  pub fn speed_limit_notification<T: AsRef<UpdateSpeedLimitNotification>>(t: T) -> Self {
    Update::SpeedLimitNotification(t.as_ref().clone())
  }

  pub fn stake_dice_state<T: AsRef<UpdateStakeDiceState>>(t: T) -> Self {
    Update::StakeDiceState(t.as_ref().clone())
  }

  pub fn star_revenue_status<T: AsRef<UpdateStarRevenueStatus>>(t: T) -> Self {
    Update::StarRevenueStatus(t.as_ref().clone())
  }

  pub fn sticker_set<T: AsRef<UpdateStickerSet>>(t: T) -> Self {
    Update::StickerSet(t.as_ref().clone())
  }

  pub fn stop_message_draft<T: AsRef<UpdateStopMessageDraft>>(t: T) -> Self {
    Update::StopMessageDraft(t.as_ref().clone())
  }

  pub fn story<T: AsRef<UpdateStory>>(t: T) -> Self {
    Update::Story(t.as_ref().clone())
  }

  pub fn story_deleted<T: AsRef<UpdateStoryDeleted>>(t: T) -> Self {
    Update::StoryDeleted(t.as_ref().clone())
  }

  pub fn story_list_chat_count<T: AsRef<UpdateStoryListChatCount>>(t: T) -> Self {
    Update::StoryListChatCount(t.as_ref().clone())
  }

  pub fn story_post_failed<T: AsRef<UpdateStoryPostFailed>>(t: T) -> Self {
    Update::StoryPostFailed(t.as_ref().clone())
  }

  pub fn story_post_succeeded<T: AsRef<UpdateStoryPostSucceeded>>(t: T) -> Self {
    Update::StoryPostSucceeded(t.as_ref().clone())
  }

  pub fn story_stealth_mode<T: AsRef<UpdateStoryStealthMode>>(t: T) -> Self {
    Update::StoryStealthMode(t.as_ref().clone())
  }

  pub fn suggested_actions<T: AsRef<UpdateSuggestedActions>>(t: T) -> Self {
    Update::SuggestedActions(t.as_ref().clone())
  }

  pub fn supergroup<T: AsRef<UpdateSupergroup>>(t: T) -> Self {
    Update::Supergroup(t.as_ref().clone())
  }

  pub fn supergroup_full_info<T: AsRef<UpdateSupergroupFullInfo>>(t: T) -> Self {
    Update::SupergroupFullInfo(t.as_ref().clone())
  }

  pub fn terms_of_service<T: AsRef<UpdateTermsOfService>>(t: T) -> Self {
    Update::TermsOfService(t.as_ref().clone())
  }

  pub fn text_composition_styles<T: AsRef<UpdateTextCompositionStyles>>(t: T) -> Self {
    Update::TextCompositionStyles(t.as_ref().clone())
  }

  pub fn ton_wallet_gasless_transfers_info<T: AsRef<UpdateTonWalletGaslessTransfersInfo>>(
    t: T,
  ) -> Self {
    Update::TonWalletGaslessTransfersInfo(t.as_ref().clone())
  }

  pub fn ton_wallet_state<T: AsRef<UpdateTonWalletState>>(t: T) -> Self {
    Update::TonWalletState(t.as_ref().clone())
  }

  pub fn ton_wallet_ton_connect_session<T: AsRef<UpdateTonWalletTonConnectSession>>(t: T) -> Self {
    Update::TonWalletTonConnectSession(t.as_ref().clone())
  }

  pub fn ton_wallet_ton_connect_session_disconnect_required<
    T: AsRef<UpdateTonWalletTonConnectSessionDisconnectRequired>,
  >(
    t: T,
  ) -> Self {
    Update::TonWalletTonConnectSessionDisconnectRequired(t.as_ref().clone())
  }

  pub fn topic_message_count<T: AsRef<UpdateTopicMessageCount>>(t: T) -> Self {
    Update::TopicMessageCount(t.as_ref().clone())
  }

  pub fn trending_sticker_sets<T: AsRef<UpdateTrendingStickerSets>>(t: T) -> Self {
    Update::TrendingStickerSets(t.as_ref().clone())
  }

  pub fn trusted_mini_app_bots<T: AsRef<UpdateTrustedMiniAppBots>>(t: T) -> Self {
    Update::TrustedMiniAppBots(t.as_ref().clone())
  }

  pub fn unconfirmed_session<T: AsRef<UpdateUnconfirmedSession>>(t: T) -> Self {
    Update::UnconfirmedSession(t.as_ref().clone())
  }

  pub fn unread_chat_count<T: AsRef<UpdateUnreadChatCount>>(t: T) -> Self {
    Update::UnreadChatCount(t.as_ref().clone())
  }

  pub fn unread_message_count<T: AsRef<UpdateUnreadMessageCount>>(t: T) -> Self {
    Update::UnreadMessageCount(t.as_ref().clone())
  }

  pub fn user<T: AsRef<UpdateUser>>(t: T) -> Self {
    Update::User(t.as_ref().clone())
  }

  pub fn user_full_info<T: AsRef<UpdateUserFullInfo>>(t: T) -> Self {
    Update::UserFullInfo(t.as_ref().clone())
  }

  pub fn user_privacy_setting_rules<T: AsRef<UpdateUserPrivacySettingRules>>(t: T) -> Self {
    Update::UserPrivacySettingRules(t.as_ref().clone())
  }

  pub fn user_status<T: AsRef<UpdateUserStatus>>(t: T) -> Self {
    Update::UserStatus(t.as_ref().clone())
  }

  pub fn user_subscription<T: AsRef<UpdateUserSubscription>>(t: T) -> Self {
    Update::UserSubscription(t.as_ref().clone())
  }

  pub fn video_published<T: AsRef<UpdateVideoPublished>>(t: T) -> Self {
    Update::VideoPublished(t.as_ref().clone())
  }

  pub fn web_app_message_sent<T: AsRef<UpdateWebAppMessageSent>>(t: T) -> Self {
    Update::WebAppMessageSent(t.as_ref().clone())
  }

  pub fn web_browser_settings<T: AsRef<UpdateWebBrowserSettings>>(t: T) -> Self {
    Update::WebBrowserSettings(t.as_ref().clone())
  }

  pub fn web_proxy_carrier_closed<T: AsRef<UpdateWebProxyCarrierClosed>>(t: T) -> Self {
    Update::WebProxyCarrierClosed(t.as_ref().clone())
  }

  pub fn web_proxy_carrier_data<T: AsRef<UpdateWebProxyCarrierData>>(t: T) -> Self {
    Update::WebProxyCarrierData(t.as_ref().clone())
  }

  pub fn web_proxy_carrier_opened<T: AsRef<UpdateWebProxyCarrierOpened>>(t: T) -> Self {
    Update::WebProxyCarrierOpened(t.as_ref().clone())
  }
}

impl AsRef<Update> for Update {
  fn as_ref(&self) -> &Update {
    self
  }
}

/// The list of supported accent colors has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateAccentColors {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Information about supported colors; colors with identifiers 0 (red), 1 (orange), 2 (purple/violet), 3 (green), 4 (cyan), 5 (blue), 6 (pink) must always be supported and aren't included in the list. The exact colors for the accent colors with identifiers 0-6 must be taken from the application theme
  colors: Vec<AccentColor>,
  /// The list of accent color identifiers, which can be set through setAccentColor and setChatAccentColor. The colors must be shown in the specified order
  available_accent_color_ids: Vec<i64>,
}

impl RObject for UpdateAccentColors {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateAccentColors"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateAccentColors {}

impl UpdateAccentColors {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateAccentColorsBuilder {
    let inner = UpdateAccentColors {
      td_name: "updateAccentColors".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateAccentColorsBuilder { inner }
  }

  pub fn colors(&self) -> &Vec<AccentColor> {
    &self.colors
  }

  pub fn available_accent_color_ids(&self) -> &Vec<i64> {
    &self.available_accent_color_ids
  }
}

#[doc(hidden)]
pub struct RTDUpdateAccentColorsBuilder {
  inner: UpdateAccentColors,
}

impl RTDUpdateAccentColorsBuilder {
  pub fn build(&self) -> UpdateAccentColors {
    self.inner.clone()
  }

  pub fn colors(&mut self, colors: Vec<AccentColor>) -> &mut Self {
    self.inner.colors = colors;
    self
  }

  pub fn available_accent_color_ids(&mut self, available_accent_color_ids: Vec<i64>) -> &mut Self {
    self.inner.available_accent_color_ids = available_accent_color_ids;
    self
  }
}

impl AsRef<UpdateAccentColors> for UpdateAccentColors {
  fn as_ref(&self) -> &UpdateAccentColors {
    self
  }
}

impl AsRef<UpdateAccentColors> for RTDUpdateAccentColorsBuilder {
  fn as_ref(&self) -> &UpdateAccentColors {
    &self.inner
  }
}

/// The list of active emoji reactions has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateActiveEmojiReactions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new list of active emoji reactions
  emojis: Vec<String>,
}

impl RObject for UpdateActiveEmojiReactions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateActiveEmojiReactions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateActiveEmojiReactions {}

impl UpdateActiveEmojiReactions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateActiveEmojiReactionsBuilder {
    let inner = UpdateActiveEmojiReactions {
      td_name: "updateActiveEmojiReactions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateActiveEmojiReactionsBuilder { inner }
  }

  pub fn emojis(&self) -> &Vec<String> {
    &self.emojis
  }
}

#[doc(hidden)]
pub struct RTDUpdateActiveEmojiReactionsBuilder {
  inner: UpdateActiveEmojiReactions,
}

impl RTDUpdateActiveEmojiReactionsBuilder {
  pub fn build(&self) -> UpdateActiveEmojiReactions {
    self.inner.clone()
  }

  pub fn emojis(&mut self, emojis: Vec<String>) -> &mut Self {
    self.inner.emojis = emojis;
    self
  }
}

impl AsRef<UpdateActiveEmojiReactions> for UpdateActiveEmojiReactions {
  fn as_ref(&self) -> &UpdateActiveEmojiReactions {
    self
  }
}

impl AsRef<UpdateActiveEmojiReactions> for RTDUpdateActiveEmojiReactionsBuilder {
  fn as_ref(&self) -> &UpdateActiveEmojiReactions {
    &self.inner
  }
}

/// The list of auctions in which the current user participates has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateActiveGiftAuctions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New states of the auctions
  states: Vec<GiftAuctionState>,
}

impl RObject for UpdateActiveGiftAuctions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateActiveGiftAuctions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateActiveGiftAuctions {}

impl UpdateActiveGiftAuctions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateActiveGiftAuctionsBuilder {
    let inner = UpdateActiveGiftAuctions {
      td_name: "updateActiveGiftAuctions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateActiveGiftAuctionsBuilder { inner }
  }

  pub fn states(&self) -> &Vec<GiftAuctionState> {
    &self.states
  }
}

#[doc(hidden)]
pub struct RTDUpdateActiveGiftAuctionsBuilder {
  inner: UpdateActiveGiftAuctions,
}

impl RTDUpdateActiveGiftAuctionsBuilder {
  pub fn build(&self) -> UpdateActiveGiftAuctions {
    self.inner.clone()
  }

  pub fn states(&mut self, states: Vec<GiftAuctionState>) -> &mut Self {
    self.inner.states = states;
    self
  }
}

impl AsRef<UpdateActiveGiftAuctions> for UpdateActiveGiftAuctions {
  fn as_ref(&self) -> &UpdateActiveGiftAuctions {
    self
  }
}

impl AsRef<UpdateActiveGiftAuctions> for RTDUpdateActiveGiftAuctionsBuilder {
  fn as_ref(&self) -> &UpdateActiveGiftAuctions {
    &self.inner
  }
}

/// The list of messages with active live location that need to be updated by the application has changed. The list is persistent across application restarts only if the message database is used
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateActiveLiveLocationMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The list of messages with active live locations
  messages: Vec<Message>,
}

impl RObject for UpdateActiveLiveLocationMessages {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateActiveLiveLocationMessages"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateActiveLiveLocationMessages {}

impl UpdateActiveLiveLocationMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateActiveLiveLocationMessagesBuilder {
    let inner = UpdateActiveLiveLocationMessages {
      td_name: "updateActiveLiveLocationMessages".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateActiveLiveLocationMessagesBuilder { inner }
  }

  pub fn messages(&self) -> &Vec<Message> {
    &self.messages
  }
}

#[doc(hidden)]
pub struct RTDUpdateActiveLiveLocationMessagesBuilder {
  inner: UpdateActiveLiveLocationMessages,
}

impl RTDUpdateActiveLiveLocationMessagesBuilder {
  pub fn build(&self) -> UpdateActiveLiveLocationMessages {
    self.inner.clone()
  }

  pub fn messages(&mut self, messages: Vec<Message>) -> &mut Self {
    self.inner.messages = messages;
    self
  }
}

impl AsRef<UpdateActiveLiveLocationMessages> for UpdateActiveLiveLocationMessages {
  fn as_ref(&self) -> &UpdateActiveLiveLocationMessages {
    self
  }
}

impl AsRef<UpdateActiveLiveLocationMessages> for RTDUpdateActiveLiveLocationMessagesBuilder {
  fn as_ref(&self) -> &UpdateActiveLiveLocationMessages {
    &self.inner
  }
}

/// Contains active notifications that were shown on previous application launches. This update is sent only if the message database is used. In that case it comes once before any updateNotification and updateNotificationGroup update
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateActiveNotifications"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateActiveNotifications {}

impl UpdateActiveNotifications {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateActiveNotificationsBuilder {
    let inner = UpdateActiveNotifications {
      td_name: "updateActiveNotifications".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateActiveNotificationsBuilder { inner }
  }

  pub fn groups(&self) -> &Vec<NotificationGroup> {
    &self.groups
  }
}

#[doc(hidden)]
pub struct RTDUpdateActiveNotificationsBuilder {
  inner: UpdateActiveNotifications,
}

impl RTDUpdateActiveNotificationsBuilder {
  pub fn build(&self) -> UpdateActiveNotifications {
    self.inner.clone()
  }

  pub fn groups(&mut self, groups: Vec<NotificationGroup>) -> &mut Self {
    self.inner.groups = groups;
    self
  }
}

impl AsRef<UpdateActiveNotifications> for UpdateActiveNotifications {
  fn as_ref(&self) -> &UpdateActiveNotifications {
    self
  }
}

impl AsRef<UpdateActiveNotifications> for RTDUpdateActiveNotificationsBuilder {
  fn as_ref(&self) -> &UpdateActiveNotifications {
    &self.inner
  }
}

/// The parameters for age verification of the current user's account have changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateAgeVerificationParameters {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Parameters for the age verification; may be null if age verification isn't needed
  parameters: Option<AgeVerificationParameters>,
}

impl RObject for UpdateAgeVerificationParameters {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateAgeVerificationParameters"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateAgeVerificationParameters {}

impl UpdateAgeVerificationParameters {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateAgeVerificationParametersBuilder {
    let inner = UpdateAgeVerificationParameters {
      td_name: "updateAgeVerificationParameters".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateAgeVerificationParametersBuilder { inner }
  }

  pub fn parameters(&self) -> &Option<AgeVerificationParameters> {
    &self.parameters
  }
}

#[doc(hidden)]
pub struct RTDUpdateAgeVerificationParametersBuilder {
  inner: UpdateAgeVerificationParameters,
}

impl RTDUpdateAgeVerificationParametersBuilder {
  pub fn build(&self) -> UpdateAgeVerificationParameters {
    self.inner.clone()
  }

  pub fn parameters<T: AsRef<AgeVerificationParameters>>(&mut self, parameters: T) -> &mut Self {
    self.inner.parameters = Some(parameters.as_ref().clone());
    self
  }
}

impl AsRef<UpdateAgeVerificationParameters> for UpdateAgeVerificationParameters {
  fn as_ref(&self) -> &UpdateAgeVerificationParameters {
    self
  }
}

impl AsRef<UpdateAgeVerificationParameters> for RTDUpdateAgeVerificationParametersBuilder {
  fn as_ref(&self) -> &UpdateAgeVerificationParameters {
    &self.inner
  }
}

/// Lists of suffixes allowed in secondary bot usernames, which can be added additionally to the main username if the bot's owner has a Telegram Premium subscription
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateAllowedBotUsernamePremiumSuffixes {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The list of allowed suffixes
  suffixes: Vec<String>,
}

impl RObject for UpdateAllowedBotUsernamePremiumSuffixes {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateAllowedBotUsernamePremiumSuffixes"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateAllowedBotUsernamePremiumSuffixes {}

impl UpdateAllowedBotUsernamePremiumSuffixes {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateAllowedBotUsernamePremiumSuffixesBuilder {
    let inner = UpdateAllowedBotUsernamePremiumSuffixes {
      td_name: "updateAllowedBotUsernamePremiumSuffixes".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateAllowedBotUsernamePremiumSuffixesBuilder { inner }
  }

  pub fn suffixes(&self) -> &Vec<String> {
    &self.suffixes
  }
}

#[doc(hidden)]
pub struct RTDUpdateAllowedBotUsernamePremiumSuffixesBuilder {
  inner: UpdateAllowedBotUsernamePremiumSuffixes,
}

impl RTDUpdateAllowedBotUsernamePremiumSuffixesBuilder {
  pub fn build(&self) -> UpdateAllowedBotUsernamePremiumSuffixes {
    self.inner.clone()
  }

  pub fn suffixes(&mut self, suffixes: Vec<String>) -> &mut Self {
    self.inner.suffixes = suffixes;
    self
  }
}

impl AsRef<UpdateAllowedBotUsernamePremiumSuffixes> for UpdateAllowedBotUsernamePremiumSuffixes {
  fn as_ref(&self) -> &UpdateAllowedBotUsernamePremiumSuffixes {
    self
  }
}

impl AsRef<UpdateAllowedBotUsernamePremiumSuffixes>
  for RTDUpdateAllowedBotUsernamePremiumSuffixesBuilder
{
  fn as_ref(&self) -> &UpdateAllowedBotUsernamePremiumSuffixes {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateAnimatedEmojiMessageClicked"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateAnimatedEmojiMessageClicked {}

impl UpdateAnimatedEmojiMessageClicked {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateAnimatedEmojiMessageClickedBuilder {
    let inner = UpdateAnimatedEmojiMessageClicked {
      td_name: "updateAnimatedEmojiMessageClicked".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateAnimatedEmojiMessageClickedBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }

  pub fn sticker(&self) -> &Sticker {
    &self.sticker
  }
}

#[doc(hidden)]
pub struct RTDUpdateAnimatedEmojiMessageClickedBuilder {
  inner: UpdateAnimatedEmojiMessageClicked,
}

impl RTDUpdateAnimatedEmojiMessageClickedBuilder {
  pub fn build(&self) -> UpdateAnimatedEmojiMessageClicked {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateAnimatedEmojiMessageClicked {
    self
  }
}

impl AsRef<UpdateAnimatedEmojiMessageClicked> for RTDUpdateAnimatedEmojiMessageClickedBuilder {
  fn as_ref(&self) -> &UpdateAnimatedEmojiMessageClicked {
    &self.inner
  }
}

/// The parameters of animation search through getOption("animation_search_bot_username") bot have changed
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateAnimationSearchParameters"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateAnimationSearchParameters {}

impl UpdateAnimationSearchParameters {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateAnimationSearchParametersBuilder {
    let inner = UpdateAnimationSearchParameters {
      td_name: "updateAnimationSearchParameters".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateAnimationSearchParametersBuilder { inner }
  }

  pub fn provider(&self) -> &String {
    &self.provider
  }

  pub fn emojis(&self) -> &Vec<String> {
    &self.emojis
  }
}

#[doc(hidden)]
pub struct RTDUpdateAnimationSearchParametersBuilder {
  inner: UpdateAnimationSearchParameters,
}

impl RTDUpdateAnimationSearchParametersBuilder {
  pub fn build(&self) -> UpdateAnimationSearchParameters {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateAnimationSearchParameters {
    self
  }
}

impl AsRef<UpdateAnimationSearchParameters> for RTDUpdateAnimationSearchParametersBuilder {
  fn as_ref(&self) -> &UpdateAnimationSearchParameters {
    &self.inner
  }
}

/// A request can't be completed unless reCAPTCHA verification is performed; for official mobile applications only. The method setApplicationVerificationToken must be called once the verification is completed or failed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateApplicationRecaptchaVerificationRequired {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier for the verification process
  verification_id: i64,
  /// The action for the check
  action: String,
  /// Identifier of the reCAPTCHA key
  recaptcha_key_id: String,
}

impl RObject for UpdateApplicationRecaptchaVerificationRequired {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateApplicationRecaptchaVerificationRequired"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateApplicationRecaptchaVerificationRequired {}

impl UpdateApplicationRecaptchaVerificationRequired {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateApplicationRecaptchaVerificationRequiredBuilder {
    let inner = UpdateApplicationRecaptchaVerificationRequired {
      td_name: "updateApplicationRecaptchaVerificationRequired".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateApplicationRecaptchaVerificationRequiredBuilder { inner }
  }

  pub fn verification_id(&self) -> i64 {
    self.verification_id
  }

  pub fn action(&self) -> &String {
    &self.action
  }

  pub fn recaptcha_key_id(&self) -> &String {
    &self.recaptcha_key_id
  }
}

#[doc(hidden)]
pub struct RTDUpdateApplicationRecaptchaVerificationRequiredBuilder {
  inner: UpdateApplicationRecaptchaVerificationRequired,
}

impl RTDUpdateApplicationRecaptchaVerificationRequiredBuilder {
  pub fn build(&self) -> UpdateApplicationRecaptchaVerificationRequired {
    self.inner.clone()
  }

  pub fn verification_id(&mut self, verification_id: i64) -> &mut Self {
    self.inner.verification_id = verification_id;
    self
  }

  pub fn action<T: AsRef<str>>(&mut self, action: T) -> &mut Self {
    self.inner.action = action.as_ref().to_string();
    self
  }

  pub fn recaptcha_key_id<T: AsRef<str>>(&mut self, recaptcha_key_id: T) -> &mut Self {
    self.inner.recaptcha_key_id = recaptcha_key_id.as_ref().to_string();
    self
  }
}

impl AsRef<UpdateApplicationRecaptchaVerificationRequired>
  for UpdateApplicationRecaptchaVerificationRequired
{
  fn as_ref(&self) -> &UpdateApplicationRecaptchaVerificationRequired {
    self
  }
}

impl AsRef<UpdateApplicationRecaptchaVerificationRequired>
  for RTDUpdateApplicationRecaptchaVerificationRequiredBuilder
{
  fn as_ref(&self) -> &UpdateApplicationRecaptchaVerificationRequired {
    &self.inner
  }
}

/// A request can't be completed unless application verification is performed; for official mobile applications only. The method setApplicationVerificationToken must be called once the verification is completed or failed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateApplicationVerificationRequired {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier for the verification process
  verification_id: i64,
  /// Unique base64url-encoded nonce for the classic Play Integrity verification (https://developer.android.com/google/play/integrity/classic) for Android, or a unique string to compare with verify_nonce field from a push notification for iOS
  nonce: String,
  /// Cloud project number to pass to the Play Integrity API on Android
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  cloud_project_number: isize,
}

impl RObject for UpdateApplicationVerificationRequired {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateApplicationVerificationRequired"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateApplicationVerificationRequired {}

impl UpdateApplicationVerificationRequired {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateApplicationVerificationRequiredBuilder {
    let inner = UpdateApplicationVerificationRequired {
      td_name: "updateApplicationVerificationRequired".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateApplicationVerificationRequiredBuilder { inner }
  }

  pub fn verification_id(&self) -> i64 {
    self.verification_id
  }

  pub fn nonce(&self) -> &String {
    &self.nonce
  }

  pub fn cloud_project_number(&self) -> isize {
    self.cloud_project_number
  }
}

#[doc(hidden)]
pub struct RTDUpdateApplicationVerificationRequiredBuilder {
  inner: UpdateApplicationVerificationRequired,
}

impl RTDUpdateApplicationVerificationRequiredBuilder {
  pub fn build(&self) -> UpdateApplicationVerificationRequired {
    self.inner.clone()
  }

  pub fn verification_id(&mut self, verification_id: i64) -> &mut Self {
    self.inner.verification_id = verification_id;
    self
  }

  pub fn nonce<T: AsRef<str>>(&mut self, nonce: T) -> &mut Self {
    self.inner.nonce = nonce.as_ref().to_string();
    self
  }

  pub fn cloud_project_number(&mut self, cloud_project_number: isize) -> &mut Self {
    self.inner.cloud_project_number = cloud_project_number;
    self
  }
}

impl AsRef<UpdateApplicationVerificationRequired> for UpdateApplicationVerificationRequired {
  fn as_ref(&self) -> &UpdateApplicationVerificationRequired {
    self
  }
}

impl AsRef<UpdateApplicationVerificationRequired>
  for RTDUpdateApplicationVerificationRequiredBuilder
{
  fn as_ref(&self) -> &UpdateApplicationVerificationRequired {
    &self.inner
  }
}

/// The list of bots added to attachment or side menu has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateAttachmentMenuBots {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new list of bots. The bots must not be shown on scheduled messages screen
  bots: Vec<AttachmentMenuBot>,
}

impl RObject for UpdateAttachmentMenuBots {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateAttachmentMenuBots"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateAttachmentMenuBots {}

impl UpdateAttachmentMenuBots {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateAttachmentMenuBotsBuilder {
    let inner = UpdateAttachmentMenuBots {
      td_name: "updateAttachmentMenuBots".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateAttachmentMenuBotsBuilder { inner }
  }

  pub fn bots(&self) -> &Vec<AttachmentMenuBot> {
    &self.bots
  }
}

#[doc(hidden)]
pub struct RTDUpdateAttachmentMenuBotsBuilder {
  inner: UpdateAttachmentMenuBots,
}

impl RTDUpdateAttachmentMenuBotsBuilder {
  pub fn build(&self) -> UpdateAttachmentMenuBots {
    self.inner.clone()
  }

  pub fn bots(&mut self, bots: Vec<AttachmentMenuBot>) -> &mut Self {
    self.inner.bots = bots;
    self
  }
}

impl AsRef<UpdateAttachmentMenuBots> for UpdateAttachmentMenuBots {
  fn as_ref(&self) -> &UpdateAttachmentMenuBots {
    self
  }
}

impl AsRef<UpdateAttachmentMenuBots> for RTDUpdateAttachmentMenuBotsBuilder {
  fn as_ref(&self) -> &UpdateAttachmentMenuBots {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateAuthorizationState"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateAuthorizationState {}

impl UpdateAuthorizationState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateAuthorizationStateBuilder {
    let inner = UpdateAuthorizationState {
      td_name: "updateAuthorizationState".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateAuthorizationStateBuilder { inner }
  }

  pub fn authorization_state(&self) -> &AuthorizationState {
    &self.authorization_state
  }
}

#[doc(hidden)]
pub struct RTDUpdateAuthorizationStateBuilder {
  inner: UpdateAuthorizationState,
}

impl RTDUpdateAuthorizationStateBuilder {
  pub fn build(&self) -> UpdateAuthorizationState {
    self.inner.clone()
  }

  pub fn authorization_state<T: AsRef<AuthorizationState>>(
    &mut self,
    authorization_state: T,
  ) -> &mut Self {
    self.inner.authorization_state = authorization_state.as_ref().clone();
    self
  }
}

impl AsRef<UpdateAuthorizationState> for UpdateAuthorizationState {
  fn as_ref(&self) -> &UpdateAuthorizationState {
    self
  }
}

impl AsRef<UpdateAuthorizationState> for RTDUpdateAuthorizationStateBuilder {
  fn as_ref(&self) -> &UpdateAuthorizationState {
    &self.inner
  }
}

/// Autosave settings for some type of chats were updated
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateAutosaveSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Type of chats for which autosave settings were updated
  scope: AutosaveSettingsScope,
  /// The new autosave settings; may be null if the settings are reset to default
  settings: Option<ScopeAutosaveSettings>,
}

impl RObject for UpdateAutosaveSettings {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateAutosaveSettings"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateAutosaveSettings {}

impl UpdateAutosaveSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateAutosaveSettingsBuilder {
    let inner = UpdateAutosaveSettings {
      td_name: "updateAutosaveSettings".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateAutosaveSettingsBuilder { inner }
  }

  pub fn scope(&self) -> &AutosaveSettingsScope {
    &self.scope
  }

  pub fn settings(&self) -> &Option<ScopeAutosaveSettings> {
    &self.settings
  }
}

#[doc(hidden)]
pub struct RTDUpdateAutosaveSettingsBuilder {
  inner: UpdateAutosaveSettings,
}

impl RTDUpdateAutosaveSettingsBuilder {
  pub fn build(&self) -> UpdateAutosaveSettings {
    self.inner.clone()
  }

  pub fn scope<T: AsRef<AutosaveSettingsScope>>(&mut self, scope: T) -> &mut Self {
    self.inner.scope = scope.as_ref().clone();
    self
  }

  pub fn settings<T: AsRef<ScopeAutosaveSettings>>(&mut self, settings: T) -> &mut Self {
    self.inner.settings = Some(settings.as_ref().clone());
    self
  }
}

impl AsRef<UpdateAutosaveSettings> for UpdateAutosaveSettings {
  fn as_ref(&self) -> &UpdateAutosaveSettings {
    self
  }
}

impl AsRef<UpdateAutosaveSettings> for RTDUpdateAutosaveSettingsBuilder {
  fn as_ref(&self) -> &UpdateAutosaveSettings {
    &self.inner
  }
}

/// The list of available message effects has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateAvailableMessageEffects {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new list of available message effects from emoji reactions
  reaction_effect_ids: Vec<isize>,
  /// The new list of available message effects from Premium stickers
  sticker_effect_ids: Vec<isize>,
}

impl RObject for UpdateAvailableMessageEffects {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateAvailableMessageEffects"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateAvailableMessageEffects {}

impl UpdateAvailableMessageEffects {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateAvailableMessageEffectsBuilder {
    let inner = UpdateAvailableMessageEffects {
      td_name: "updateAvailableMessageEffects".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateAvailableMessageEffectsBuilder { inner }
  }

  pub fn reaction_effect_ids(&self) -> &Vec<isize> {
    &self.reaction_effect_ids
  }

  pub fn sticker_effect_ids(&self) -> &Vec<isize> {
    &self.sticker_effect_ids
  }
}

#[doc(hidden)]
pub struct RTDUpdateAvailableMessageEffectsBuilder {
  inner: UpdateAvailableMessageEffects,
}

impl RTDUpdateAvailableMessageEffectsBuilder {
  pub fn build(&self) -> UpdateAvailableMessageEffects {
    self.inner.clone()
  }

  pub fn reaction_effect_ids(&mut self, reaction_effect_ids: Vec<isize>) -> &mut Self {
    self.inner.reaction_effect_ids = reaction_effect_ids;
    self
  }

  pub fn sticker_effect_ids(&mut self, sticker_effect_ids: Vec<isize>) -> &mut Self {
    self.inner.sticker_effect_ids = sticker_effect_ids;
    self
  }
}

impl AsRef<UpdateAvailableMessageEffects> for UpdateAvailableMessageEffects {
  fn as_ref(&self) -> &UpdateAvailableMessageEffects {
    self
  }
}

impl AsRef<UpdateAvailableMessageEffects> for RTDUpdateAvailableMessageEffectsBuilder {
  fn as_ref(&self) -> &UpdateAvailableMessageEffects {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateBasicGroup"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateBasicGroup {}

impl UpdateBasicGroup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateBasicGroupBuilder {
    let inner = UpdateBasicGroup {
      td_name: "updateBasicGroup".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateBasicGroupBuilder { inner }
  }

  pub fn basic_group(&self) -> &BasicGroup {
    &self.basic_group
  }
}

#[doc(hidden)]
pub struct RTDUpdateBasicGroupBuilder {
  inner: UpdateBasicGroup,
}

impl RTDUpdateBasicGroupBuilder {
  pub fn build(&self) -> UpdateBasicGroup {
    self.inner.clone()
  }

  pub fn basic_group<T: AsRef<BasicGroup>>(&mut self, basic_group: T) -> &mut Self {
    self.inner.basic_group = basic_group.as_ref().clone();
    self
  }
}

impl AsRef<UpdateBasicGroup> for UpdateBasicGroup {
  fn as_ref(&self) -> &UpdateBasicGroup {
    self
  }
}

impl AsRef<UpdateBasicGroup> for RTDUpdateBasicGroupBuilder {
  fn as_ref(&self) -> &UpdateBasicGroup {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateBasicGroupFullInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateBasicGroupFullInfo {}

impl UpdateBasicGroupFullInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateBasicGroupFullInfoBuilder {
    let inner = UpdateBasicGroupFullInfo {
      td_name: "updateBasicGroupFullInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateBasicGroupFullInfoBuilder { inner }
  }

  pub fn basic_group_id(&self) -> i64 {
    self.basic_group_id
  }

  pub fn basic_group_full_info(&self) -> &BasicGroupFullInfo {
    &self.basic_group_full_info
  }
}

#[doc(hidden)]
pub struct RTDUpdateBasicGroupFullInfoBuilder {
  inner: UpdateBasicGroupFullInfo,
}

impl RTDUpdateBasicGroupFullInfoBuilder {
  pub fn build(&self) -> UpdateBasicGroupFullInfo {
    self.inner.clone()
  }

  pub fn basic_group_id(&mut self, basic_group_id: i64) -> &mut Self {
    self.inner.basic_group_id = basic_group_id;
    self
  }

  pub fn basic_group_full_info<T: AsRef<BasicGroupFullInfo>>(
    &mut self,
    basic_group_full_info: T,
  ) -> &mut Self {
    self.inner.basic_group_full_info = basic_group_full_info.as_ref().clone();
    self
  }
}

impl AsRef<UpdateBasicGroupFullInfo> for UpdateBasicGroupFullInfo {
  fn as_ref(&self) -> &UpdateBasicGroupFullInfo {
    self
  }
}

impl AsRef<UpdateBasicGroupFullInfo> for RTDUpdateBasicGroupFullInfoBuilder {
  fn as_ref(&self) -> &UpdateBasicGroupFullInfo {
    &self.inner
  }
}

/// A business connection has changed; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateBusinessConnection {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New data about the connection
  connection: BusinessConnection,
}

impl RObject for UpdateBusinessConnection {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateBusinessConnection"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateBusinessConnection {}

impl UpdateBusinessConnection {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateBusinessConnectionBuilder {
    let inner = UpdateBusinessConnection {
      td_name: "updateBusinessConnection".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateBusinessConnectionBuilder { inner }
  }

  pub fn connection(&self) -> &BusinessConnection {
    &self.connection
  }
}

#[doc(hidden)]
pub struct RTDUpdateBusinessConnectionBuilder {
  inner: UpdateBusinessConnection,
}

impl RTDUpdateBusinessConnectionBuilder {
  pub fn build(&self) -> UpdateBusinessConnection {
    self.inner.clone()
  }

  pub fn connection<T: AsRef<BusinessConnection>>(&mut self, connection: T) -> &mut Self {
    self.inner.connection = connection.as_ref().clone();
    self
  }
}

impl AsRef<UpdateBusinessConnection> for UpdateBusinessConnection {
  fn as_ref(&self) -> &UpdateBusinessConnection {
    self
  }
}

impl AsRef<UpdateBusinessConnection> for RTDUpdateBusinessConnectionBuilder {
  fn as_ref(&self) -> &UpdateBusinessConnection {
    &self.inner
  }
}

/// A message in a business account was edited; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateBusinessMessageEdited {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the business connection
  connection_id: String,
  /// The edited message
  message: BusinessMessage,
}

impl RObject for UpdateBusinessMessageEdited {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateBusinessMessageEdited"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateBusinessMessageEdited {}

impl UpdateBusinessMessageEdited {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateBusinessMessageEditedBuilder {
    let inner = UpdateBusinessMessageEdited {
      td_name: "updateBusinessMessageEdited".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateBusinessMessageEditedBuilder { inner }
  }

  pub fn connection_id(&self) -> &String {
    &self.connection_id
  }

  pub fn message(&self) -> &BusinessMessage {
    &self.message
  }
}

#[doc(hidden)]
pub struct RTDUpdateBusinessMessageEditedBuilder {
  inner: UpdateBusinessMessageEdited,
}

impl RTDUpdateBusinessMessageEditedBuilder {
  pub fn build(&self) -> UpdateBusinessMessageEdited {
    self.inner.clone()
  }

  pub fn connection_id<T: AsRef<str>>(&mut self, connection_id: T) -> &mut Self {
    self.inner.connection_id = connection_id.as_ref().to_string();
    self
  }

  pub fn message<T: AsRef<BusinessMessage>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().clone();
    self
  }
}

impl AsRef<UpdateBusinessMessageEdited> for UpdateBusinessMessageEdited {
  fn as_ref(&self) -> &UpdateBusinessMessageEdited {
    self
  }
}

impl AsRef<UpdateBusinessMessageEdited> for RTDUpdateBusinessMessageEditedBuilder {
  fn as_ref(&self) -> &UpdateBusinessMessageEdited {
    &self.inner
  }
}

/// Messages in a business account were deleted; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateBusinessMessagesDeleted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the business connection
  connection_id: String,
  /// Identifier of a chat in the business account in which messages were deleted
  chat_id: i64,
  /// Unique message identifiers of the deleted messages
  message_ids: Vec<i64>,
}

impl RObject for UpdateBusinessMessagesDeleted {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateBusinessMessagesDeleted"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateBusinessMessagesDeleted {}

impl UpdateBusinessMessagesDeleted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateBusinessMessagesDeletedBuilder {
    let inner = UpdateBusinessMessagesDeleted {
      td_name: "updateBusinessMessagesDeleted".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateBusinessMessagesDeletedBuilder { inner }
  }

  pub fn connection_id(&self) -> &String {
    &self.connection_id
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_ids(&self) -> &Vec<i64> {
    &self.message_ids
  }
}

#[doc(hidden)]
pub struct RTDUpdateBusinessMessagesDeletedBuilder {
  inner: UpdateBusinessMessagesDeleted,
}

impl RTDUpdateBusinessMessagesDeletedBuilder {
  pub fn build(&self) -> UpdateBusinessMessagesDeleted {
    self.inner.clone()
  }

  pub fn connection_id<T: AsRef<str>>(&mut self, connection_id: T) -> &mut Self {
    self.inner.connection_id = connection_id.as_ref().to_string();
    self
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self {
    self.inner.message_ids = message_ids;
    self
  }
}

impl AsRef<UpdateBusinessMessagesDeleted> for UpdateBusinessMessagesDeleted {
  fn as_ref(&self) -> &UpdateBusinessMessagesDeleted {
    self
  }
}

impl AsRef<UpdateBusinessMessagesDeleted> for RTDUpdateBusinessMessagesDeletedBuilder {
  fn as_ref(&self) -> &UpdateBusinessMessagesDeleted {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateCall"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateCall {}

impl UpdateCall {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateCallBuilder {
    let inner = UpdateCall {
      td_name: "updateCall".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateCallBuilder { inner }
  }

  pub fn call(&self) -> &Call {
    &self.call
  }
}

#[doc(hidden)]
pub struct RTDUpdateCallBuilder {
  inner: UpdateCall,
}

impl RTDUpdateCallBuilder {
  pub fn build(&self) -> UpdateCall {
    self.inner.clone()
  }

  pub fn call<T: AsRef<Call>>(&mut self, call: T) -> &mut Self {
    self.inner.call = call.as_ref().clone();
    self
  }
}

impl AsRef<UpdateCall> for UpdateCall {
  fn as_ref(&self) -> &UpdateCall {
    self
  }
}

impl AsRef<UpdateCall> for RTDUpdateCallBuilder {
  fn as_ref(&self) -> &UpdateCall {
    &self.inner
  }
}

/// Chat accent colors have changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatAccentColors {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The new chat accent color identifier
  accent_color_id: i64,
  /// The new identifier of a custom emoji to be shown on the reply header and link preview background; 0 if none
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  background_custom_emoji_id: isize,
  /// Color scheme based on an upgraded gift to be used for the chat instead of accent_color_id and background_custom_emoji_id; may be null if none
  upgraded_gift_colors: Option<UpgradedGiftColors>,
  /// The new chat profile accent color identifier; 1 if none
  profile_accent_color_id: i64,
  /// The new identifier of a custom emoji to be shown on the profile background; 0 if none
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  profile_background_custom_emoji_id: isize,
}

impl RObject for UpdateChatAccentColors {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatAccentColors"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatAccentColors {}

impl UpdateChatAccentColors {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatAccentColorsBuilder {
    let inner = UpdateChatAccentColors {
      td_name: "updateChatAccentColors".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatAccentColorsBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
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
}

#[doc(hidden)]
pub struct RTDUpdateChatAccentColorsBuilder {
  inner: UpdateChatAccentColors,
}

impl RTDUpdateChatAccentColorsBuilder {
  pub fn build(&self) -> UpdateChatAccentColors {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
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
}

impl AsRef<UpdateChatAccentColors> for UpdateChatAccentColors {
  fn as_ref(&self) -> &UpdateChatAccentColors {
    self
  }
}

impl AsRef<UpdateChatAccentColors> for RTDUpdateChatAccentColorsBuilder {
  fn as_ref(&self) -> &UpdateChatAccentColors {
    &self.inner
  }
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
  /// Identifier of the specific topic in which the action was performed; may be null if none
  topic_id: Option<MessageTopic>,
  /// Identifier of a message sender performing the action
  sender_id: MessageSender,
  /// The action
  action: ChatAction,
}

impl RObject for UpdateChatAction {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatAction"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatAction {}

impl UpdateChatAction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatActionBuilder {
    let inner = UpdateChatAction {
      td_name: "updateChatAction".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatActionBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn topic_id(&self) -> &Option<MessageTopic> {
    &self.topic_id
  }

  pub fn sender_id(&self) -> &MessageSender {
    &self.sender_id
  }

  pub fn action(&self) -> &ChatAction {
    &self.action
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatActionBuilder {
  inner: UpdateChatAction,
}

impl RTDUpdateChatActionBuilder {
  pub fn build(&self) -> UpdateChatAction {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn topic_id<T: AsRef<MessageTopic>>(&mut self, topic_id: T) -> &mut Self {
    self.inner.topic_id = Some(topic_id.as_ref().clone());
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
  fn as_ref(&self) -> &UpdateChatAction {
    self
  }
}

impl AsRef<UpdateChatAction> for RTDUpdateChatActionBuilder {
  fn as_ref(&self) -> &UpdateChatAction {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatActionBar"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatActionBar {}

impl UpdateChatActionBar {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatActionBarBuilder {
    let inner = UpdateChatActionBar {
      td_name: "updateChatActionBar".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatActionBarBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn action_bar(&self) -> &Option<ChatActionBar> {
    &self.action_bar
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatActionBarBuilder {
  inner: UpdateChatActionBar,
}

impl RTDUpdateChatActionBarBuilder {
  pub fn build(&self) -> UpdateChatActionBar {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateChatActionBar {
    self
  }
}

impl AsRef<UpdateChatActionBar> for RTDUpdateChatActionBarBuilder {
  fn as_ref(&self) -> &UpdateChatActionBar {
    &self.inner
  }
}

/// The list of active stories posted by a specific chat has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatActiveStories {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new list of active stories
  active_stories: ChatActiveStories,
}

impl RObject for UpdateChatActiveStories {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatActiveStories"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatActiveStories {}

impl UpdateChatActiveStories {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatActiveStoriesBuilder {
    let inner = UpdateChatActiveStories {
      td_name: "updateChatActiveStories".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatActiveStoriesBuilder { inner }
  }

  pub fn active_stories(&self) -> &ChatActiveStories {
    &self.active_stories
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatActiveStoriesBuilder {
  inner: UpdateChatActiveStories,
}

impl RTDUpdateChatActiveStoriesBuilder {
  pub fn build(&self) -> UpdateChatActiveStories {
    self.inner.clone()
  }

  pub fn active_stories<T: AsRef<ChatActiveStories>>(&mut self, active_stories: T) -> &mut Self {
    self.inner.active_stories = active_stories.as_ref().clone();
    self
  }
}

impl AsRef<UpdateChatActiveStories> for UpdateChatActiveStories {
  fn as_ref(&self) -> &UpdateChatActiveStories {
    self
  }
}

impl AsRef<UpdateChatActiveStories> for RTDUpdateChatActiveStoriesBuilder {
  fn as_ref(&self) -> &UpdateChatActiveStories {
    &self.inner
  }
}

/// A chat was added to a chat list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatAddedToList {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The chat list to which the chat was added
  chat_list: ChatList,
}

impl RObject for UpdateChatAddedToList {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatAddedToList"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatAddedToList {}

impl UpdateChatAddedToList {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatAddedToListBuilder {
    let inner = UpdateChatAddedToList {
      td_name: "updateChatAddedToList".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatAddedToListBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn chat_list(&self) -> &ChatList {
    &self.chat_list
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatAddedToListBuilder {
  inner: UpdateChatAddedToList,
}

impl RTDUpdateChatAddedToListBuilder {
  pub fn build(&self) -> UpdateChatAddedToList {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn chat_list<T: AsRef<ChatList>>(&mut self, chat_list: T) -> &mut Self {
    self.inner.chat_list = chat_list.as_ref().clone();
    self
  }
}

impl AsRef<UpdateChatAddedToList> for UpdateChatAddedToList {
  fn as_ref(&self) -> &UpdateChatAddedToList {
    self
  }
}

impl AsRef<UpdateChatAddedToList> for RTDUpdateChatAddedToListBuilder {
  fn as_ref(&self) -> &UpdateChatAddedToList {
    &self.inner
  }
}

/// The chat available reactions were changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatAvailableReactions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The new reactions, available in the chat
  available_reactions: ChatAvailableReactions,
}

impl RObject for UpdateChatAvailableReactions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatAvailableReactions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatAvailableReactions {}

impl UpdateChatAvailableReactions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatAvailableReactionsBuilder {
    let inner = UpdateChatAvailableReactions {
      td_name: "updateChatAvailableReactions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatAvailableReactionsBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn available_reactions(&self) -> &ChatAvailableReactions {
    &self.available_reactions
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatAvailableReactionsBuilder {
  inner: UpdateChatAvailableReactions,
}

impl RTDUpdateChatAvailableReactionsBuilder {
  pub fn build(&self) -> UpdateChatAvailableReactions {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn available_reactions<T: AsRef<ChatAvailableReactions>>(
    &mut self,
    available_reactions: T,
  ) -> &mut Self {
    self.inner.available_reactions = available_reactions.as_ref().clone();
    self
  }
}

impl AsRef<UpdateChatAvailableReactions> for UpdateChatAvailableReactions {
  fn as_ref(&self) -> &UpdateChatAvailableReactions {
    self
  }
}

impl AsRef<UpdateChatAvailableReactions> for RTDUpdateChatAvailableReactionsBuilder {
  fn as_ref(&self) -> &UpdateChatAvailableReactions {
    &self.inner
  }
}

/// The chat background was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatBackground {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The new chat background; may be null if background was reset to default
  background: Option<ChatBackground>,
}

impl RObject for UpdateChatBackground {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatBackground"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatBackground {}

impl UpdateChatBackground {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatBackgroundBuilder {
    let inner = UpdateChatBackground {
      td_name: "updateChatBackground".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatBackgroundBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn background(&self) -> &Option<ChatBackground> {
    &self.background
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatBackgroundBuilder {
  inner: UpdateChatBackground,
}

impl RTDUpdateChatBackgroundBuilder {
  pub fn build(&self) -> UpdateChatBackground {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn background<T: AsRef<ChatBackground>>(&mut self, background: T) -> &mut Self {
    self.inner.background = Some(background.as_ref().clone());
    self
  }
}

impl AsRef<UpdateChatBackground> for UpdateChatBackground {
  fn as_ref(&self) -> &UpdateChatBackground {
    self
  }
}

impl AsRef<UpdateChatBackground> for RTDUpdateChatBackgroundBuilder {
  fn as_ref(&self) -> &UpdateChatBackground {
    &self.inner
  }
}

/// A chat was blocked or unblocked
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatBlockList {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Block list to which the chat is added; may be null if none
  block_list: Option<BlockList>,
}

impl RObject for UpdateChatBlockList {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatBlockList"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatBlockList {}

impl UpdateChatBlockList {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatBlockListBuilder {
    let inner = UpdateChatBlockList {
      td_name: "updateChatBlockList".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatBlockListBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn block_list(&self) -> &Option<BlockList> {
    &self.block_list
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatBlockListBuilder {
  inner: UpdateChatBlockList,
}

impl RTDUpdateChatBlockListBuilder {
  pub fn build(&self) -> UpdateChatBlockList {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn block_list<T: AsRef<BlockList>>(&mut self, block_list: T) -> &mut Self {
    self.inner.block_list = Some(block_list.as_ref().clone());
    self
  }
}

impl AsRef<UpdateChatBlockList> for UpdateChatBlockList {
  fn as_ref(&self) -> &UpdateChatBlockList {
    self
  }
}

impl AsRef<UpdateChatBlockList> for RTDUpdateChatBlockListBuilder {
  fn as_ref(&self) -> &UpdateChatBlockList {
    &self.inner
  }
}

/// A chat boost has changed; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatBoost {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New information about the boost
  boost: ChatBoost,
}

impl RObject for UpdateChatBoost {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatBoost"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatBoost {}

impl UpdateChatBoost {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatBoostBuilder {
    let inner = UpdateChatBoost {
      td_name: "updateChatBoost".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatBoostBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn boost(&self) -> &ChatBoost {
    &self.boost
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatBoostBuilder {
  inner: UpdateChatBoost,
}

impl RTDUpdateChatBoostBuilder {
  pub fn build(&self) -> UpdateChatBoost {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn boost<T: AsRef<ChatBoost>>(&mut self, boost: T) -> &mut Self {
    self.inner.boost = boost.as_ref().clone();
    self
  }
}

impl AsRef<UpdateChatBoost> for UpdateChatBoost {
  fn as_ref(&self) -> &UpdateChatBoost {
    self
  }
}

impl AsRef<UpdateChatBoost> for RTDUpdateChatBoostBuilder {
  fn as_ref(&self) -> &UpdateChatBoost {
    &self.inner
  }
}

/// The bar for managing business bot was changed in a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatBusinessBotManageBar {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The new value of the business bot manage bar; may be null
  business_bot_manage_bar: Option<BusinessBotManageBar>,
}

impl RObject for UpdateChatBusinessBotManageBar {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatBusinessBotManageBar"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatBusinessBotManageBar {}

impl UpdateChatBusinessBotManageBar {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatBusinessBotManageBarBuilder {
    let inner = UpdateChatBusinessBotManageBar {
      td_name: "updateChatBusinessBotManageBar".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatBusinessBotManageBarBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn business_bot_manage_bar(&self) -> &Option<BusinessBotManageBar> {
    &self.business_bot_manage_bar
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatBusinessBotManageBarBuilder {
  inner: UpdateChatBusinessBotManageBar,
}

impl RTDUpdateChatBusinessBotManageBarBuilder {
  pub fn build(&self) -> UpdateChatBusinessBotManageBar {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn business_bot_manage_bar<T: AsRef<BusinessBotManageBar>>(
    &mut self,
    business_bot_manage_bar: T,
  ) -> &mut Self {
    self.inner.business_bot_manage_bar = Some(business_bot_manage_bar.as_ref().clone());
    self
  }
}

impl AsRef<UpdateChatBusinessBotManageBar> for UpdateChatBusinessBotManageBar {
  fn as_ref(&self) -> &UpdateChatBusinessBotManageBar {
    self
  }
}

impl AsRef<UpdateChatBusinessBotManageBar> for RTDUpdateChatBusinessBotManageBarBuilder {
  fn as_ref(&self) -> &UpdateChatBusinessBotManageBar {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatDefaultDisableNotification"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatDefaultDisableNotification {}

impl UpdateChatDefaultDisableNotification {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatDefaultDisableNotificationBuilder {
    let inner = UpdateChatDefaultDisableNotification {
      td_name: "updateChatDefaultDisableNotification".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatDefaultDisableNotificationBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn default_disable_notification(&self) -> bool {
    self.default_disable_notification
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatDefaultDisableNotificationBuilder {
  inner: UpdateChatDefaultDisableNotification,
}

impl RTDUpdateChatDefaultDisableNotificationBuilder {
  pub fn build(&self) -> UpdateChatDefaultDisableNotification {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateChatDefaultDisableNotification {
    self
  }
}

impl AsRef<UpdateChatDefaultDisableNotification>
  for RTDUpdateChatDefaultDisableNotificationBuilder
{
  fn as_ref(&self) -> &UpdateChatDefaultDisableNotification {
    &self.inner
  }
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
  /// The new draft message; may be null if none
  draft_message: Option<DraftMessage>,
  /// The new chat positions in the chat lists
  positions: Vec<ChatPosition>,
}

impl RObject for UpdateChatDraftMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatDraftMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatDraftMessage {}

impl UpdateChatDraftMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatDraftMessageBuilder {
    let inner = UpdateChatDraftMessage {
      td_name: "updateChatDraftMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatDraftMessageBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn draft_message(&self) -> &Option<DraftMessage> {
    &self.draft_message
  }

  pub fn positions(&self) -> &Vec<ChatPosition> {
    &self.positions
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatDraftMessageBuilder {
  inner: UpdateChatDraftMessage,
}

impl RTDUpdateChatDraftMessageBuilder {
  pub fn build(&self) -> UpdateChatDraftMessage {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateChatDraftMessage {
    self
  }
}

impl AsRef<UpdateChatDraftMessage> for RTDUpdateChatDraftMessageBuilder {
  fn as_ref(&self) -> &UpdateChatDraftMessage {
    &self.inner
  }
}

/// Chat emoji status has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatEmojiStatus {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The new chat emoji status; may be null
  emoji_status: Option<EmojiStatus>,
}

impl RObject for UpdateChatEmojiStatus {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatEmojiStatus"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatEmojiStatus {}

impl UpdateChatEmojiStatus {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatEmojiStatusBuilder {
    let inner = UpdateChatEmojiStatus {
      td_name: "updateChatEmojiStatus".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatEmojiStatusBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn emoji_status(&self) -> &Option<EmojiStatus> {
    &self.emoji_status
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatEmojiStatusBuilder {
  inner: UpdateChatEmojiStatus,
}

impl RTDUpdateChatEmojiStatusBuilder {
  pub fn build(&self) -> UpdateChatEmojiStatus {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn emoji_status<T: AsRef<EmojiStatus>>(&mut self, emoji_status: T) -> &mut Self {
    self.inner.emoji_status = Some(emoji_status.as_ref().clone());
    self
  }
}

impl AsRef<UpdateChatEmojiStatus> for UpdateChatEmojiStatus {
  fn as_ref(&self) -> &UpdateChatEmojiStatus {
    self
  }
}

impl AsRef<UpdateChatEmojiStatus> for RTDUpdateChatEmojiStatusBuilder {
  fn as_ref(&self) -> &UpdateChatEmojiStatus {
    &self.inner
  }
}

/// The list of chat folders or a chat folder has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatFolders {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new list of chat folders
  chat_folders: Vec<ChatFolderInfo>,
  /// Position of the main chat list among chat folders, 0-based
  main_chat_list_position: i64,
  /// True, if folder tags are enabled
  are_tags_enabled: bool,
}

impl RObject for UpdateChatFolders {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatFolders"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatFolders {}

impl UpdateChatFolders {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatFoldersBuilder {
    let inner = UpdateChatFolders {
      td_name: "updateChatFolders".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatFoldersBuilder { inner }
  }

  pub fn chat_folders(&self) -> &Vec<ChatFolderInfo> {
    &self.chat_folders
  }

  pub fn main_chat_list_position(&self) -> i64 {
    self.main_chat_list_position
  }

  pub fn are_tags_enabled(&self) -> bool {
    self.are_tags_enabled
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatFoldersBuilder {
  inner: UpdateChatFolders,
}

impl RTDUpdateChatFoldersBuilder {
  pub fn build(&self) -> UpdateChatFolders {
    self.inner.clone()
  }

  pub fn chat_folders(&mut self, chat_folders: Vec<ChatFolderInfo>) -> &mut Self {
    self.inner.chat_folders = chat_folders;
    self
  }

  pub fn main_chat_list_position(&mut self, main_chat_list_position: i64) -> &mut Self {
    self.inner.main_chat_list_position = main_chat_list_position;
    self
  }

  pub fn are_tags_enabled(&mut self, are_tags_enabled: bool) -> &mut Self {
    self.inner.are_tags_enabled = are_tags_enabled;
    self
  }
}

impl AsRef<UpdateChatFolders> for UpdateChatFolders {
  fn as_ref(&self) -> &UpdateChatFolders {
    self
  }
}

impl AsRef<UpdateChatFolders> for RTDUpdateChatFoldersBuilder {
  fn as_ref(&self) -> &UpdateChatFolders {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatHasProtectedContent"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatHasProtectedContent {}

impl UpdateChatHasProtectedContent {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatHasProtectedContentBuilder {
    let inner = UpdateChatHasProtectedContent {
      td_name: "updateChatHasProtectedContent".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatHasProtectedContentBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn has_protected_content(&self) -> bool {
    self.has_protected_content
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatHasProtectedContentBuilder {
  inner: UpdateChatHasProtectedContent,
}

impl RTDUpdateChatHasProtectedContentBuilder {
  pub fn build(&self) -> UpdateChatHasProtectedContent {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateChatHasProtectedContent {
    self
  }
}

impl AsRef<UpdateChatHasProtectedContent> for RTDUpdateChatHasProtectedContentBuilder {
  fn as_ref(&self) -> &UpdateChatHasProtectedContent {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatHasScheduledMessages"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatHasScheduledMessages {}

impl UpdateChatHasScheduledMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatHasScheduledMessagesBuilder {
    let inner = UpdateChatHasScheduledMessages {
      td_name: "updateChatHasScheduledMessages".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatHasScheduledMessagesBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn has_scheduled_messages(&self) -> bool {
    self.has_scheduled_messages
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatHasScheduledMessagesBuilder {
  inner: UpdateChatHasScheduledMessages,
}

impl RTDUpdateChatHasScheduledMessagesBuilder {
  pub fn build(&self) -> UpdateChatHasScheduledMessages {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateChatHasScheduledMessages {
    self
  }
}

impl AsRef<UpdateChatHasScheduledMessages> for RTDUpdateChatHasScheduledMessagesBuilder {
  fn as_ref(&self) -> &UpdateChatHasScheduledMessages {
    &self.inner
  }
}

/// A chat's has_welcome_messages field has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatHasWelcomeMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New value of has_welcome_messages
  has_welcome_messages: bool,
}

impl RObject for UpdateChatHasWelcomeMessages {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatHasWelcomeMessages"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatHasWelcomeMessages {}

impl UpdateChatHasWelcomeMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatHasWelcomeMessagesBuilder {
    let inner = UpdateChatHasWelcomeMessages {
      td_name: "updateChatHasWelcomeMessages".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatHasWelcomeMessagesBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn has_welcome_messages(&self) -> bool {
    self.has_welcome_messages
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatHasWelcomeMessagesBuilder {
  inner: UpdateChatHasWelcomeMessages,
}

impl RTDUpdateChatHasWelcomeMessagesBuilder {
  pub fn build(&self) -> UpdateChatHasWelcomeMessages {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn has_welcome_messages(&mut self, has_welcome_messages: bool) -> &mut Self {
    self.inner.has_welcome_messages = has_welcome_messages;
    self
  }
}

impl AsRef<UpdateChatHasWelcomeMessages> for UpdateChatHasWelcomeMessages {
  fn as_ref(&self) -> &UpdateChatHasWelcomeMessages {
    self
  }
}

impl AsRef<UpdateChatHasWelcomeMessages> for RTDUpdateChatHasWelcomeMessagesBuilder {
  fn as_ref(&self) -> &UpdateChatHasWelcomeMessages {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatIsMarkedAsUnread"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatIsMarkedAsUnread {}

impl UpdateChatIsMarkedAsUnread {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatIsMarkedAsUnreadBuilder {
    let inner = UpdateChatIsMarkedAsUnread {
      td_name: "updateChatIsMarkedAsUnread".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatIsMarkedAsUnreadBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn is_marked_as_unread(&self) -> bool {
    self.is_marked_as_unread
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatIsMarkedAsUnreadBuilder {
  inner: UpdateChatIsMarkedAsUnread,
}

impl RTDUpdateChatIsMarkedAsUnreadBuilder {
  pub fn build(&self) -> UpdateChatIsMarkedAsUnread {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateChatIsMarkedAsUnread {
    self
  }
}

impl AsRef<UpdateChatIsMarkedAsUnread> for RTDUpdateChatIsMarkedAsUnreadBuilder {
  fn as_ref(&self) -> &UpdateChatIsMarkedAsUnread {
    &self.inner
  }
}

/// Translation of chat messages was enabled or disabled
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatIsTranslatable {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New value of is_translatable
  is_translatable: bool,
}

impl RObject for UpdateChatIsTranslatable {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatIsTranslatable"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatIsTranslatable {}

impl UpdateChatIsTranslatable {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatIsTranslatableBuilder {
    let inner = UpdateChatIsTranslatable {
      td_name: "updateChatIsTranslatable".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatIsTranslatableBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn is_translatable(&self) -> bool {
    self.is_translatable
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatIsTranslatableBuilder {
  inner: UpdateChatIsTranslatable,
}

impl RTDUpdateChatIsTranslatableBuilder {
  pub fn build(&self) -> UpdateChatIsTranslatable {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn is_translatable(&mut self, is_translatable: bool) -> &mut Self {
    self.inner.is_translatable = is_translatable;
    self
  }
}

impl AsRef<UpdateChatIsTranslatable> for UpdateChatIsTranslatable {
  fn as_ref(&self) -> &UpdateChatIsTranslatable {
    self
  }
}

impl AsRef<UpdateChatIsTranslatable> for RTDUpdateChatIsTranslatableBuilder {
  fn as_ref(&self) -> &UpdateChatIsTranslatable {
    &self.inner
  }
}

/// A join request from the user was completed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatJoinResult {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the join request query as received in chatJoinResultGuardBotApprovalRequired. If the corresponding Web App is still open, then it must be closed
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  query_id: isize,
  /// Identifier of the joined chat, or 0 if the request wasn't approved
  chat_id: i64,
  /// Result of the join
  result: ChatJoinRequestResult,
}

impl RObject for UpdateChatJoinResult {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatJoinResult"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatJoinResult {}

impl UpdateChatJoinResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatJoinResultBuilder {
    let inner = UpdateChatJoinResult {
      td_name: "updateChatJoinResult".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatJoinResultBuilder { inner }
  }

  pub fn query_id(&self) -> isize {
    self.query_id
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn result(&self) -> &ChatJoinRequestResult {
    &self.result
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatJoinResultBuilder {
  inner: UpdateChatJoinResult,
}

impl RTDUpdateChatJoinResultBuilder {
  pub fn build(&self) -> UpdateChatJoinResult {
    self.inner.clone()
  }

  pub fn query_id(&mut self, query_id: isize) -> &mut Self {
    self.inner.query_id = query_id;
    self
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn result<T: AsRef<ChatJoinRequestResult>>(&mut self, result: T) -> &mut Self {
    self.inner.result = result.as_ref().clone();
    self
  }
}

impl AsRef<UpdateChatJoinResult> for UpdateChatJoinResult {
  fn as_ref(&self) -> &UpdateChatJoinResult {
    self
  }
}

impl AsRef<UpdateChatJoinResult> for RTDUpdateChatJoinResultBuilder {
  fn as_ref(&self) -> &UpdateChatJoinResult {
    &self.inner
  }
}

/// The last message of a chat was changed
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
  /// The new last message in the chat; may be null if the last message became unknown. While the last message is unknown, new messages can be added to the chat without corresponding updateNewMessage update
  last_message: Option<Message>,
  /// The new chat positions in the chat lists
  positions: Option<Vec<ChatPosition>>,
}

impl RObject for UpdateChatLastMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatLastMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatLastMessage {}

impl UpdateChatLastMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatLastMessageBuilder {
    let inner = UpdateChatLastMessage {
      td_name: "updateChatLastMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatLastMessageBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn last_message(&self) -> &Option<Message> {
    &self.last_message
  }

  pub fn positions(&self) -> &Option<Vec<ChatPosition>> {
    &self.positions
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatLastMessageBuilder {
  inner: UpdateChatLastMessage,
}

impl RTDUpdateChatLastMessageBuilder {
  pub fn build(&self) -> UpdateChatLastMessage {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateChatLastMessage {
    self
  }
}

impl AsRef<UpdateChatLastMessage> for RTDUpdateChatLastMessageBuilder {
  fn as_ref(&self) -> &UpdateChatLastMessage {
    &self.inner
  }
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
  /// Point in time (Unix timestamp) when the user rights were changed
  date: i64,
  /// If user has joined the chat using an invite link, the invite link; may be null
  invite_link: Option<ChatInviteLink>,
  /// True, if the user has joined the chat after sending a join request and being approved by an administrator
  via_join_request: bool,
  /// True, if the user has joined the chat using an invite link for a chat folder
  via_chat_folder_invite_link: bool,
  /// Previous chat member
  old_chat_member: ChatMember,
  /// New chat member
  new_chat_member: ChatMember,
}

impl RObject for UpdateChatMember {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatMember"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatMember {}

impl UpdateChatMember {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatMemberBuilder {
    let inner = UpdateChatMember {
      td_name: "updateChatMember".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatMemberBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn actor_user_id(&self) -> i64 {
    self.actor_user_id
  }

  pub fn date(&self) -> i64 {
    self.date
  }

  pub fn invite_link(&self) -> &Option<ChatInviteLink> {
    &self.invite_link
  }

  pub fn via_join_request(&self) -> bool {
    self.via_join_request
  }

  pub fn via_chat_folder_invite_link(&self) -> bool {
    self.via_chat_folder_invite_link
  }

  pub fn old_chat_member(&self) -> &ChatMember {
    &self.old_chat_member
  }

  pub fn new_chat_member(&self) -> &ChatMember {
    &self.new_chat_member
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatMemberBuilder {
  inner: UpdateChatMember,
}

impl RTDUpdateChatMemberBuilder {
  pub fn build(&self) -> UpdateChatMember {
    self.inner.clone()
  }

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

  pub fn via_join_request(&mut self, via_join_request: bool) -> &mut Self {
    self.inner.via_join_request = via_join_request;
    self
  }

  pub fn via_chat_folder_invite_link(&mut self, via_chat_folder_invite_link: bool) -> &mut Self {
    self.inner.via_chat_folder_invite_link = via_chat_folder_invite_link;
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
  fn as_ref(&self) -> &UpdateChatMember {
    self
  }
}

impl AsRef<UpdateChatMember> for RTDUpdateChatMemberBuilder {
  fn as_ref(&self) -> &UpdateChatMember {
    &self.inner
  }
}

/// The message auto-delete or self-destruct timer setting for a chat was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatMessageAutoDeleteTime {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New value of message_auto_delete_time
  message_auto_delete_time: i64,
}

impl RObject for UpdateChatMessageAutoDeleteTime {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatMessageAutoDeleteTime"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatMessageAutoDeleteTime {}

impl UpdateChatMessageAutoDeleteTime {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatMessageAutoDeleteTimeBuilder {
    let inner = UpdateChatMessageAutoDeleteTime {
      td_name: "updateChatMessageAutoDeleteTime".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatMessageAutoDeleteTimeBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_auto_delete_time(&self) -> i64 {
    self.message_auto_delete_time
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatMessageAutoDeleteTimeBuilder {
  inner: UpdateChatMessageAutoDeleteTime,
}

impl RTDUpdateChatMessageAutoDeleteTimeBuilder {
  pub fn build(&self) -> UpdateChatMessageAutoDeleteTime {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn message_auto_delete_time(&mut self, message_auto_delete_time: i64) -> &mut Self {
    self.inner.message_auto_delete_time = message_auto_delete_time;
    self
  }
}

impl AsRef<UpdateChatMessageAutoDeleteTime> for UpdateChatMessageAutoDeleteTime {
  fn as_ref(&self) -> &UpdateChatMessageAutoDeleteTime {
    self
  }
}

impl AsRef<UpdateChatMessageAutoDeleteTime> for RTDUpdateChatMessageAutoDeleteTimeBuilder {
  fn as_ref(&self) -> &UpdateChatMessageAutoDeleteTime {
    &self.inner
  }
}

/// The message sender that is selected to send messages in a chat has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatMessageSender {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New value of message_sender_id; may be null if the user can't change message sender
  message_sender_id: Option<MessageSender>,
}

impl RObject for UpdateChatMessageSender {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatMessageSender"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatMessageSender {}

impl UpdateChatMessageSender {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatMessageSenderBuilder {
    let inner = UpdateChatMessageSender {
      td_name: "updateChatMessageSender".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatMessageSenderBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_sender_id(&self) -> &Option<MessageSender> {
    &self.message_sender_id
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatMessageSenderBuilder {
  inner: UpdateChatMessageSender,
}

impl RTDUpdateChatMessageSenderBuilder {
  pub fn build(&self) -> UpdateChatMessageSender {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn message_sender_id<T: AsRef<MessageSender>>(&mut self, message_sender_id: T) -> &mut Self {
    self.inner.message_sender_id = Some(message_sender_id.as_ref().clone());
    self
  }
}

impl AsRef<UpdateChatMessageSender> for UpdateChatMessageSender {
  fn as_ref(&self) -> &UpdateChatMessageSender {
    self
  }
}

impl AsRef<UpdateChatMessageSender> for RTDUpdateChatMessageSenderBuilder {
  fn as_ref(&self) -> &UpdateChatMessageSender {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatNotificationSettings"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatNotificationSettings {}

impl UpdateChatNotificationSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatNotificationSettingsBuilder {
    let inner = UpdateChatNotificationSettings {
      td_name: "updateChatNotificationSettings".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatNotificationSettingsBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn notification_settings(&self) -> &ChatNotificationSettings {
    &self.notification_settings
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatNotificationSettingsBuilder {
  inner: UpdateChatNotificationSettings,
}

impl RTDUpdateChatNotificationSettingsBuilder {
  pub fn build(&self) -> UpdateChatNotificationSettings {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn notification_settings<T: AsRef<ChatNotificationSettings>>(
    &mut self,
    notification_settings: T,
  ) -> &mut Self {
    self.inner.notification_settings = notification_settings.as_ref().clone();
    self
  }
}

impl AsRef<UpdateChatNotificationSettings> for UpdateChatNotificationSettings {
  fn as_ref(&self) -> &UpdateChatNotificationSettings {
    self
  }
}

impl AsRef<UpdateChatNotificationSettings> for RTDUpdateChatNotificationSettingsBuilder {
  fn as_ref(&self) -> &UpdateChatNotificationSettings {
    &self.inner
  }
}

/// The number of online group members has changed. This update with non-zero number of online group members is sent only for currently opened chats. There is no guarantee that it is sent just after the number of online users has changed
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatOnlineMemberCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatOnlineMemberCount {}

impl UpdateChatOnlineMemberCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatOnlineMemberCountBuilder {
    let inner = UpdateChatOnlineMemberCount {
      td_name: "updateChatOnlineMemberCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatOnlineMemberCountBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn online_member_count(&self) -> i64 {
    self.online_member_count
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatOnlineMemberCountBuilder {
  inner: UpdateChatOnlineMemberCount,
}

impl RTDUpdateChatOnlineMemberCountBuilder {
  pub fn build(&self) -> UpdateChatOnlineMemberCount {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateChatOnlineMemberCount {
    self
  }
}

impl AsRef<UpdateChatOnlineMemberCount> for RTDUpdateChatOnlineMemberCountBuilder {
  fn as_ref(&self) -> &UpdateChatOnlineMemberCount {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatPendingJoinRequests"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatPendingJoinRequests {}

impl UpdateChatPendingJoinRequests {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatPendingJoinRequestsBuilder {
    let inner = UpdateChatPendingJoinRequests {
      td_name: "updateChatPendingJoinRequests".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatPendingJoinRequestsBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn pending_join_requests(&self) -> &Option<ChatJoinRequestsInfo> {
    &self.pending_join_requests
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatPendingJoinRequestsBuilder {
  inner: UpdateChatPendingJoinRequests,
}

impl RTDUpdateChatPendingJoinRequestsBuilder {
  pub fn build(&self) -> UpdateChatPendingJoinRequests {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn pending_join_requests<T: AsRef<ChatJoinRequestsInfo>>(
    &mut self,
    pending_join_requests: T,
  ) -> &mut Self {
    self.inner.pending_join_requests = Some(pending_join_requests.as_ref().clone());
    self
  }
}

impl AsRef<UpdateChatPendingJoinRequests> for UpdateChatPendingJoinRequests {
  fn as_ref(&self) -> &UpdateChatPendingJoinRequests {
    self
  }
}

impl AsRef<UpdateChatPendingJoinRequests> for RTDUpdateChatPendingJoinRequestsBuilder {
  fn as_ref(&self) -> &UpdateChatPendingJoinRequests {
    &self.inner
  }
}

/// Chat permissions were changed
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatPermissions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatPermissions {}

impl UpdateChatPermissions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatPermissionsBuilder {
    let inner = UpdateChatPermissions {
      td_name: "updateChatPermissions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatPermissionsBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn permissions(&self) -> &ChatPermissions {
    &self.permissions
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatPermissionsBuilder {
  inner: UpdateChatPermissions,
}

impl RTDUpdateChatPermissionsBuilder {
  pub fn build(&self) -> UpdateChatPermissions {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateChatPermissions {
    self
  }
}

impl AsRef<UpdateChatPermissions> for RTDUpdateChatPermissionsBuilder {
  fn as_ref(&self) -> &UpdateChatPermissions {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatPhoto"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatPhoto {}

impl UpdateChatPhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatPhotoBuilder {
    let inner = UpdateChatPhoto {
      td_name: "updateChatPhoto".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatPhotoBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn photo(&self) -> &Option<ChatPhotoInfo> {
    &self.photo
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatPhotoBuilder {
  inner: UpdateChatPhoto,
}

impl RTDUpdateChatPhotoBuilder {
  pub fn build(&self) -> UpdateChatPhoto {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateChatPhoto {
    self
  }
}

impl AsRef<UpdateChatPhoto> for RTDUpdateChatPhotoBuilder {
  fn as_ref(&self) -> &UpdateChatPhoto {
    &self.inner
  }
}

/// The position of a chat in a chat list has changed. An updateChatLastMessage or updateChatDraftMessage update might be sent instead of the update
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatPosition"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatPosition {}

impl UpdateChatPosition {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatPositionBuilder {
    let inner = UpdateChatPosition {
      td_name: "updateChatPosition".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatPositionBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn position(&self) -> &ChatPosition {
    &self.position
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatPositionBuilder {
  inner: UpdateChatPosition,
}

impl RTDUpdateChatPositionBuilder {
  pub fn build(&self) -> UpdateChatPosition {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateChatPosition {
    self
  }
}

impl AsRef<UpdateChatPosition> for RTDUpdateChatPositionBuilder {
  fn as_ref(&self) -> &UpdateChatPosition {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatReadInbox"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatReadInbox {}

impl UpdateChatReadInbox {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatReadInboxBuilder {
    let inner = UpdateChatReadInbox {
      td_name: "updateChatReadInbox".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatReadInboxBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn last_read_inbox_message_id(&self) -> i64 {
    self.last_read_inbox_message_id
  }

  pub fn unread_count(&self) -> i64 {
    self.unread_count
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatReadInboxBuilder {
  inner: UpdateChatReadInbox,
}

impl RTDUpdateChatReadInboxBuilder {
  pub fn build(&self) -> UpdateChatReadInbox {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateChatReadInbox {
    self
  }
}

impl AsRef<UpdateChatReadInbox> for RTDUpdateChatReadInboxBuilder {
  fn as_ref(&self) -> &UpdateChatReadInbox {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatReadOutbox"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatReadOutbox {}

impl UpdateChatReadOutbox {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatReadOutboxBuilder {
    let inner = UpdateChatReadOutbox {
      td_name: "updateChatReadOutbox".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatReadOutboxBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn last_read_outbox_message_id(&self) -> i64 {
    self.last_read_outbox_message_id
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatReadOutboxBuilder {
  inner: UpdateChatReadOutbox,
}

impl RTDUpdateChatReadOutboxBuilder {
  pub fn build(&self) -> UpdateChatReadOutbox {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateChatReadOutbox {
    self
  }
}

impl AsRef<UpdateChatReadOutbox> for RTDUpdateChatReadOutboxBuilder {
  fn as_ref(&self) -> &UpdateChatReadOutbox {
    &self.inner
  }
}

/// A chat was removed from a chat list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatRemovedFromList {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The chat list from which the chat was removed
  chat_list: ChatList,
}

impl RObject for UpdateChatRemovedFromList {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatRemovedFromList"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatRemovedFromList {}

impl UpdateChatRemovedFromList {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatRemovedFromListBuilder {
    let inner = UpdateChatRemovedFromList {
      td_name: "updateChatRemovedFromList".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatRemovedFromListBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn chat_list(&self) -> &ChatList {
    &self.chat_list
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatRemovedFromListBuilder {
  inner: UpdateChatRemovedFromList,
}

impl RTDUpdateChatRemovedFromListBuilder {
  pub fn build(&self) -> UpdateChatRemovedFromList {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn chat_list<T: AsRef<ChatList>>(&mut self, chat_list: T) -> &mut Self {
    self.inner.chat_list = chat_list.as_ref().clone();
    self
  }
}

impl AsRef<UpdateChatRemovedFromList> for UpdateChatRemovedFromList {
  fn as_ref(&self) -> &UpdateChatRemovedFromList {
    self
  }
}

impl AsRef<UpdateChatRemovedFromList> for RTDUpdateChatRemovedFromListBuilder {
  fn as_ref(&self) -> &UpdateChatRemovedFromList {
    &self.inner
  }
}

/// The chat reply markup was changed
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
  /// The message from which the reply markup must be used; may be null if there is no default reply markup in the chat
  reply_markup_message: Option<Message>,
}

impl RObject for UpdateChatReplyMarkup {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatReplyMarkup"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatReplyMarkup {}

impl UpdateChatReplyMarkup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatReplyMarkupBuilder {
    let inner = UpdateChatReplyMarkup {
      td_name: "updateChatReplyMarkup".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatReplyMarkupBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn reply_markup_message(&self) -> &Option<Message> {
    &self.reply_markup_message
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatReplyMarkupBuilder {
  inner: UpdateChatReplyMarkup,
}

impl RTDUpdateChatReplyMarkupBuilder {
  pub fn build(&self) -> UpdateChatReplyMarkup {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn reply_markup_message<T: AsRef<Message>>(&mut self, reply_markup_message: T) -> &mut Self {
    self.inner.reply_markup_message = Some(reply_markup_message.as_ref().clone());
    self
  }
}

impl AsRef<UpdateChatReplyMarkup> for UpdateChatReplyMarkup {
  fn as_ref(&self) -> &UpdateChatReplyMarkup {
    self
  }
}

impl AsRef<UpdateChatReplyMarkup> for RTDUpdateChatReplyMarkupBuilder {
  fn as_ref(&self) -> &UpdateChatReplyMarkup {
    &self.inner
  }
}

/// The revenue earned from sponsored messages in a chat has changed. If chat revenue screen is opened, then getChatRevenueTransactions may be called to fetch new transactions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatRevenueAmount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat
  chat_id: i64,
  /// New amount of earned revenue
  revenue_amount: ChatRevenueAmount,
}

impl RObject for UpdateChatRevenueAmount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatRevenueAmount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatRevenueAmount {}

impl UpdateChatRevenueAmount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatRevenueAmountBuilder {
    let inner = UpdateChatRevenueAmount {
      td_name: "updateChatRevenueAmount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatRevenueAmountBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn revenue_amount(&self) -> &ChatRevenueAmount {
    &self.revenue_amount
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatRevenueAmountBuilder {
  inner: UpdateChatRevenueAmount,
}

impl RTDUpdateChatRevenueAmountBuilder {
  pub fn build(&self) -> UpdateChatRevenueAmount {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn revenue_amount<T: AsRef<ChatRevenueAmount>>(&mut self, revenue_amount: T) -> &mut Self {
    self.inner.revenue_amount = revenue_amount.as_ref().clone();
    self
  }
}

impl AsRef<UpdateChatRevenueAmount> for UpdateChatRevenueAmount {
  fn as_ref(&self) -> &UpdateChatRevenueAmount {
    self
  }
}

impl AsRef<UpdateChatRevenueAmount> for RTDUpdateChatRevenueAmountBuilder {
  fn as_ref(&self) -> &UpdateChatRevenueAmount {
    &self.inner
  }
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
  /// The new theme of the chat; may be null if theme was reset to default
  theme: Option<ChatTheme>,
}

impl RObject for UpdateChatTheme {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatTheme"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatTheme {}

impl UpdateChatTheme {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatThemeBuilder {
    let inner = UpdateChatTheme {
      td_name: "updateChatTheme".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatThemeBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn theme(&self) -> &Option<ChatTheme> {
    &self.theme
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatThemeBuilder {
  inner: UpdateChatTheme,
}

impl RTDUpdateChatThemeBuilder {
  pub fn build(&self) -> UpdateChatTheme {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn theme<T: AsRef<ChatTheme>>(&mut self, theme: T) -> &mut Self {
    self.inner.theme = Some(theme.as_ref().clone());
    self
  }
}

impl AsRef<UpdateChatTheme> for UpdateChatTheme {
  fn as_ref(&self) -> &UpdateChatTheme {
    self
  }
}

impl AsRef<UpdateChatTheme> for RTDUpdateChatThemeBuilder {
  fn as_ref(&self) -> &UpdateChatTheme {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatTitle"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatTitle {}

impl UpdateChatTitle {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatTitleBuilder {
    let inner = UpdateChatTitle {
      td_name: "updateChatTitle".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatTitleBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn title(&self) -> &String {
    &self.title
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatTitleBuilder {
  inner: UpdateChatTitle,
}

impl RTDUpdateChatTitleBuilder {
  pub fn build(&self) -> UpdateChatTitle {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateChatTitle {
    self
  }
}

impl AsRef<UpdateChatTitle> for RTDUpdateChatTitleBuilder {
  fn as_ref(&self) -> &UpdateChatTitle {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatUnreadMentionCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatUnreadMentionCount {}

impl UpdateChatUnreadMentionCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatUnreadMentionCountBuilder {
    let inner = UpdateChatUnreadMentionCount {
      td_name: "updateChatUnreadMentionCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatUnreadMentionCountBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn unread_mention_count(&self) -> i64 {
    self.unread_mention_count
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatUnreadMentionCountBuilder {
  inner: UpdateChatUnreadMentionCount,
}

impl RTDUpdateChatUnreadMentionCountBuilder {
  pub fn build(&self) -> UpdateChatUnreadMentionCount {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateChatUnreadMentionCount {
    self
  }
}

impl AsRef<UpdateChatUnreadMentionCount> for RTDUpdateChatUnreadMentionCountBuilder {
  fn as_ref(&self) -> &UpdateChatUnreadMentionCount {
    &self.inner
  }
}

/// The chat unread_poll_vote_count has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatUnreadPollVoteCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The number of messages with unread poll votes left in the chat
  unread_poll_vote_count: i64,
}

impl RObject for UpdateChatUnreadPollVoteCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatUnreadPollVoteCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatUnreadPollVoteCount {}

impl UpdateChatUnreadPollVoteCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatUnreadPollVoteCountBuilder {
    let inner = UpdateChatUnreadPollVoteCount {
      td_name: "updateChatUnreadPollVoteCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatUnreadPollVoteCountBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn unread_poll_vote_count(&self) -> i64 {
    self.unread_poll_vote_count
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatUnreadPollVoteCountBuilder {
  inner: UpdateChatUnreadPollVoteCount,
}

impl RTDUpdateChatUnreadPollVoteCountBuilder {
  pub fn build(&self) -> UpdateChatUnreadPollVoteCount {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn unread_poll_vote_count(&mut self, unread_poll_vote_count: i64) -> &mut Self {
    self.inner.unread_poll_vote_count = unread_poll_vote_count;
    self
  }
}

impl AsRef<UpdateChatUnreadPollVoteCount> for UpdateChatUnreadPollVoteCount {
  fn as_ref(&self) -> &UpdateChatUnreadPollVoteCount {
    self
  }
}

impl AsRef<UpdateChatUnreadPollVoteCount> for RTDUpdateChatUnreadPollVoteCountBuilder {
  fn as_ref(&self) -> &UpdateChatUnreadPollVoteCount {
    &self.inner
  }
}

/// The chat unread_reaction_count has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatUnreadReactionCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The number of messages with unread reactions left in the chat
  unread_reaction_count: i64,
}

impl RObject for UpdateChatUnreadReactionCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatUnreadReactionCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatUnreadReactionCount {}

impl UpdateChatUnreadReactionCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatUnreadReactionCountBuilder {
    let inner = UpdateChatUnreadReactionCount {
      td_name: "updateChatUnreadReactionCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatUnreadReactionCountBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn unread_reaction_count(&self) -> i64 {
    self.unread_reaction_count
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatUnreadReactionCountBuilder {
  inner: UpdateChatUnreadReactionCount,
}

impl RTDUpdateChatUnreadReactionCountBuilder {
  pub fn build(&self) -> UpdateChatUnreadReactionCount {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn unread_reaction_count(&mut self, unread_reaction_count: i64) -> &mut Self {
    self.inner.unread_reaction_count = unread_reaction_count;
    self
  }
}

impl AsRef<UpdateChatUnreadReactionCount> for UpdateChatUnreadReactionCount {
  fn as_ref(&self) -> &UpdateChatUnreadReactionCount {
    self
  }
}

impl AsRef<UpdateChatUnreadReactionCount> for RTDUpdateChatUnreadReactionCountBuilder {
  fn as_ref(&self) -> &UpdateChatUnreadReactionCount {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatVideoChat"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatVideoChat {}

impl UpdateChatVideoChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatVideoChatBuilder {
    let inner = UpdateChatVideoChat {
      td_name: "updateChatVideoChat".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatVideoChatBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn video_chat(&self) -> &VideoChat {
    &self.video_chat
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatVideoChatBuilder {
  inner: UpdateChatVideoChat,
}

impl RTDUpdateChatVideoChatBuilder {
  pub fn build(&self) -> UpdateChatVideoChat {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateChatVideoChat {
    self
  }
}

impl AsRef<UpdateChatVideoChat> for RTDUpdateChatVideoChatBuilder {
  fn as_ref(&self) -> &UpdateChatVideoChat {
    &self.inner
  }
}

/// A chat default appearance has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatViewAsTopics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New value of view_as_topics
  view_as_topics: bool,
}

impl RObject for UpdateChatViewAsTopics {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatViewAsTopics"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatViewAsTopics {}

impl UpdateChatViewAsTopics {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatViewAsTopicsBuilder {
    let inner = UpdateChatViewAsTopics {
      td_name: "updateChatViewAsTopics".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatViewAsTopicsBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn view_as_topics(&self) -> bool {
    self.view_as_topics
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatViewAsTopicsBuilder {
  inner: UpdateChatViewAsTopics,
}

impl RTDUpdateChatViewAsTopicsBuilder {
  pub fn build(&self) -> UpdateChatViewAsTopics {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn view_as_topics(&mut self, view_as_topics: bool) -> &mut Self {
    self.inner.view_as_topics = view_as_topics;
    self
  }
}

impl AsRef<UpdateChatViewAsTopics> for UpdateChatViewAsTopics {
  fn as_ref(&self) -> &UpdateChatViewAsTopics {
    self
  }
}

impl AsRef<UpdateChatViewAsTopics> for RTDUpdateChatViewAsTopicsBuilder {
  fn as_ref(&self) -> &UpdateChatViewAsTopics {
    &self.inner
  }
}

/// The list of welcome messages of a chat has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatWelcomeMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The identifier of the chat
  chat_id: i64,
  /// The new list of welcome messages of the chat in the order from the first to the last sent
  messages: Vec<WelcomeMessage>,
}

impl RObject for UpdateChatWelcomeMessages {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateChatWelcomeMessages"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateChatWelcomeMessages {}

impl UpdateChatWelcomeMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateChatWelcomeMessagesBuilder {
    let inner = UpdateChatWelcomeMessages {
      td_name: "updateChatWelcomeMessages".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateChatWelcomeMessagesBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn messages(&self) -> &Vec<WelcomeMessage> {
    &self.messages
  }
}

#[doc(hidden)]
pub struct RTDUpdateChatWelcomeMessagesBuilder {
  inner: UpdateChatWelcomeMessages,
}

impl RTDUpdateChatWelcomeMessagesBuilder {
  pub fn build(&self) -> UpdateChatWelcomeMessages {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn messages(&mut self, messages: Vec<WelcomeMessage>) -> &mut Self {
    self.inner.messages = messages;
    self
  }
}

impl AsRef<UpdateChatWelcomeMessages> for UpdateChatWelcomeMessages {
  fn as_ref(&self) -> &UpdateChatWelcomeMessages {
    self
  }
}

impl AsRef<UpdateChatWelcomeMessages> for RTDUpdateChatWelcomeMessagesBuilder {
  fn as_ref(&self) -> &UpdateChatWelcomeMessages {
    &self.inner
  }
}

/// Some data of a community has changed. This update is guaranteed to come before the community identifier is returned to the application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateCommunity {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New data about the community
  community: Community,
}

impl RObject for UpdateCommunity {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateCommunity"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateCommunity {}

impl UpdateCommunity {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateCommunityBuilder {
    let inner = UpdateCommunity {
      td_name: "updateCommunity".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateCommunityBuilder { inner }
  }

  pub fn community(&self) -> &Community {
    &self.community
  }
}

#[doc(hidden)]
pub struct RTDUpdateCommunityBuilder {
  inner: UpdateCommunity,
}

impl RTDUpdateCommunityBuilder {
  pub fn build(&self) -> UpdateCommunity {
    self.inner.clone()
  }

  pub fn community<T: AsRef<Community>>(&mut self, community: T) -> &mut Self {
    self.inner.community = community.as_ref().clone();
    self
  }
}

impl AsRef<UpdateCommunity> for UpdateCommunity {
  fn as_ref(&self) -> &UpdateCommunity {
    self
  }
}

impl AsRef<UpdateCommunity> for RTDUpdateCommunityBuilder {
  fn as_ref(&self) -> &UpdateCommunity {
    &self.inner
  }
}

/// Some data in communityFullInfo has been changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateCommunityFullInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the community
  community_id: i64,
  /// New full information about the community
  community_full_info: CommunityFullInfo,
}

impl RObject for UpdateCommunityFullInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateCommunityFullInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateCommunityFullInfo {}

impl UpdateCommunityFullInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateCommunityFullInfoBuilder {
    let inner = UpdateCommunityFullInfo {
      td_name: "updateCommunityFullInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateCommunityFullInfoBuilder { inner }
  }

  pub fn community_id(&self) -> i64 {
    self.community_id
  }

  pub fn community_full_info(&self) -> &CommunityFullInfo {
    &self.community_full_info
  }
}

#[doc(hidden)]
pub struct RTDUpdateCommunityFullInfoBuilder {
  inner: UpdateCommunityFullInfo,
}

impl RTDUpdateCommunityFullInfoBuilder {
  pub fn build(&self) -> UpdateCommunityFullInfo {
    self.inner.clone()
  }

  pub fn community_id(&mut self, community_id: i64) -> &mut Self {
    self.inner.community_id = community_id;
    self
  }

  pub fn community_full_info<T: AsRef<CommunityFullInfo>>(
    &mut self,
    community_full_info: T,
  ) -> &mut Self {
    self.inner.community_full_info = community_full_info.as_ref().clone();
    self
  }
}

impl AsRef<UpdateCommunityFullInfo> for UpdateCommunityFullInfo {
  fn as_ref(&self) -> &UpdateCommunityFullInfo {
    self
  }
}

impl AsRef<UpdateCommunityFullInfo> for RTDUpdateCommunityFullInfoBuilder {
  fn as_ref(&self) -> &UpdateCommunityFullInfo {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateConnectionState"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateConnectionState {}

impl UpdateConnectionState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateConnectionStateBuilder {
    let inner = UpdateConnectionState {
      td_name: "updateConnectionState".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateConnectionStateBuilder { inner }
  }

  pub fn state(&self) -> &ConnectionState {
    &self.state
  }
}

#[doc(hidden)]
pub struct RTDUpdateConnectionStateBuilder {
  inner: UpdateConnectionState,
}

impl RTDUpdateConnectionStateBuilder {
  pub fn build(&self) -> UpdateConnectionState {
    self.inner.clone()
  }

  pub fn state<T: AsRef<ConnectionState>>(&mut self, state: T) -> &mut Self {
    self.inner.state = state.as_ref().clone();
    self
  }
}

impl AsRef<UpdateConnectionState> for UpdateConnectionState {
  fn as_ref(&self) -> &UpdateConnectionState {
    self
  }
}

impl AsRef<UpdateConnectionState> for RTDUpdateConnectionStateBuilder {
  fn as_ref(&self) -> &UpdateConnectionState {
    &self.inner
  }
}

/// The list of contacts that had birthdays recently or will have birthday soon has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateContactCloseBirthdays {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of contact users with close birthday
  close_birthday_users: Vec<CloseBirthdayUser>,
}

impl RObject for UpdateContactCloseBirthdays {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateContactCloseBirthdays"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateContactCloseBirthdays {}

impl UpdateContactCloseBirthdays {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateContactCloseBirthdaysBuilder {
    let inner = UpdateContactCloseBirthdays {
      td_name: "updateContactCloseBirthdays".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateContactCloseBirthdaysBuilder { inner }
  }

  pub fn close_birthday_users(&self) -> &Vec<CloseBirthdayUser> {
    &self.close_birthday_users
  }
}

#[doc(hidden)]
pub struct RTDUpdateContactCloseBirthdaysBuilder {
  inner: UpdateContactCloseBirthdays,
}

impl RTDUpdateContactCloseBirthdaysBuilder {
  pub fn build(&self) -> UpdateContactCloseBirthdays {
    self.inner.clone()
  }

  pub fn close_birthday_users(
    &mut self,
    close_birthday_users: Vec<CloseBirthdayUser>,
  ) -> &mut Self {
    self.inner.close_birthday_users = close_birthday_users;
    self
  }
}

impl AsRef<UpdateContactCloseBirthdays> for UpdateContactCloseBirthdays {
  fn as_ref(&self) -> &UpdateContactCloseBirthdays {
    self
  }
}

impl AsRef<UpdateContactCloseBirthdays> for RTDUpdateContactCloseBirthdaysBuilder {
  fn as_ref(&self) -> &UpdateContactCloseBirthdays {
    &self.inner
  }
}

/// The default background has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateDefaultBackground {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if default background for dark theme has changed
  for_dark_theme: bool,
  /// The new default background; may be null
  background: Option<Background>,
}

impl RObject for UpdateDefaultBackground {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateDefaultBackground"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateDefaultBackground {}

impl UpdateDefaultBackground {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateDefaultBackgroundBuilder {
    let inner = UpdateDefaultBackground {
      td_name: "updateDefaultBackground".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateDefaultBackgroundBuilder { inner }
  }

  pub fn for_dark_theme(&self) -> bool {
    self.for_dark_theme
  }

  pub fn background(&self) -> &Option<Background> {
    &self.background
  }
}

#[doc(hidden)]
pub struct RTDUpdateDefaultBackgroundBuilder {
  inner: UpdateDefaultBackground,
}

impl RTDUpdateDefaultBackgroundBuilder {
  pub fn build(&self) -> UpdateDefaultBackground {
    self.inner.clone()
  }

  pub fn for_dark_theme(&mut self, for_dark_theme: bool) -> &mut Self {
    self.inner.for_dark_theme = for_dark_theme;
    self
  }

  pub fn background<T: AsRef<Background>>(&mut self, background: T) -> &mut Self {
    self.inner.background = Some(background.as_ref().clone());
    self
  }
}

impl AsRef<UpdateDefaultBackground> for UpdateDefaultBackground {
  fn as_ref(&self) -> &UpdateDefaultBackground {
    self
  }
}

impl AsRef<UpdateDefaultBackground> for RTDUpdateDefaultBackgroundBuilder {
  fn as_ref(&self) -> &UpdateDefaultBackground {
    &self.inner
  }
}

/// The type of default paid reaction has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateDefaultPaidReactionType {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new type of the default paid reaction
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: PaidReactionType,
}

impl RObject for UpdateDefaultPaidReactionType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateDefaultPaidReactionType"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateDefaultPaidReactionType {}

impl UpdateDefaultPaidReactionType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateDefaultPaidReactionTypeBuilder {
    let inner = UpdateDefaultPaidReactionType {
      td_name: "updateDefaultPaidReactionType".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateDefaultPaidReactionTypeBuilder { inner }
  }

  pub fn type_(&self) -> &PaidReactionType {
    &self.type_
  }
}

#[doc(hidden)]
pub struct RTDUpdateDefaultPaidReactionTypeBuilder {
  inner: UpdateDefaultPaidReactionType,
}

impl RTDUpdateDefaultPaidReactionTypeBuilder {
  pub fn build(&self) -> UpdateDefaultPaidReactionType {
    self.inner.clone()
  }

  pub fn type_<T: AsRef<PaidReactionType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }
}

impl AsRef<UpdateDefaultPaidReactionType> for UpdateDefaultPaidReactionType {
  fn as_ref(&self) -> &UpdateDefaultPaidReactionType {
    self
  }
}

impl AsRef<UpdateDefaultPaidReactionType> for RTDUpdateDefaultPaidReactionTypeBuilder {
  fn as_ref(&self) -> &UpdateDefaultPaidReactionType {
    &self.inner
  }
}

/// The type of default reaction has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateDefaultReactionType {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new type of the default reaction
  reaction_type: ReactionType,
}

impl RObject for UpdateDefaultReactionType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateDefaultReactionType"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateDefaultReactionType {}

impl UpdateDefaultReactionType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateDefaultReactionTypeBuilder {
    let inner = UpdateDefaultReactionType {
      td_name: "updateDefaultReactionType".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateDefaultReactionTypeBuilder { inner }
  }

  pub fn reaction_type(&self) -> &ReactionType {
    &self.reaction_type
  }
}

#[doc(hidden)]
pub struct RTDUpdateDefaultReactionTypeBuilder {
  inner: UpdateDefaultReactionType,
}

impl RTDUpdateDefaultReactionTypeBuilder {
  pub fn build(&self) -> UpdateDefaultReactionType {
    self.inner.clone()
  }

  pub fn reaction_type<T: AsRef<ReactionType>>(&mut self, reaction_type: T) -> &mut Self {
    self.inner.reaction_type = reaction_type.as_ref().clone();
    self
  }
}

impl AsRef<UpdateDefaultReactionType> for UpdateDefaultReactionType {
  fn as_ref(&self) -> &UpdateDefaultReactionType {
    self
  }
}

impl AsRef<UpdateDefaultReactionType> for RTDUpdateDefaultReactionTypeBuilder {
  fn as_ref(&self) -> &UpdateDefaultReactionType {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateDeleteMessages"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateDeleteMessages {}

impl UpdateDeleteMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateDeleteMessagesBuilder {
    let inner = UpdateDeleteMessages {
      td_name: "updateDeleteMessages".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateDeleteMessagesBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_ids(&self) -> &Vec<i64> {
    &self.message_ids
  }

  pub fn is_permanent(&self) -> bool {
    self.is_permanent
  }

  pub fn from_cache(&self) -> bool {
    self.from_cache
  }
}

#[doc(hidden)]
pub struct RTDUpdateDeleteMessagesBuilder {
  inner: UpdateDeleteMessages,
}

impl RTDUpdateDeleteMessagesBuilder {
  pub fn build(&self) -> UpdateDeleteMessages {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateDeleteMessages {
    self
  }
}

impl AsRef<UpdateDeleteMessages> for RTDUpdateDeleteMessagesBuilder {
  fn as_ref(&self) -> &UpdateDeleteMessages {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateDiceEmojis"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateDiceEmojis {}

impl UpdateDiceEmojis {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateDiceEmojisBuilder {
    let inner = UpdateDiceEmojis {
      td_name: "updateDiceEmojis".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateDiceEmojisBuilder { inner }
  }

  pub fn emojis(&self) -> &Vec<String> {
    &self.emojis
  }
}

#[doc(hidden)]
pub struct RTDUpdateDiceEmojisBuilder {
  inner: UpdateDiceEmojis,
}

impl RTDUpdateDiceEmojisBuilder {
  pub fn build(&self) -> UpdateDiceEmojis {
    self.inner.clone()
  }

  pub fn emojis(&mut self, emojis: Vec<String>) -> &mut Self {
    self.inner.emojis = emojis;
    self
  }
}

impl AsRef<UpdateDiceEmojis> for UpdateDiceEmojis {
  fn as_ref(&self) -> &UpdateDiceEmojis {
    self
  }
}

impl AsRef<UpdateDiceEmojis> for RTDUpdateDiceEmojisBuilder {
  fn as_ref(&self) -> &UpdateDiceEmojis {
    &self.inner
  }
}

/// Basic information about a topic in a channel direct messages chat administered by the current user has changed. This update is guaranteed to come before the topic identifier is returned to the application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateDirectMessagesChatTopic {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New data about the topic
  topic: DirectMessagesChatTopic,
}

impl RObject for UpdateDirectMessagesChatTopic {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateDirectMessagesChatTopic"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateDirectMessagesChatTopic {}

impl UpdateDirectMessagesChatTopic {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateDirectMessagesChatTopicBuilder {
    let inner = UpdateDirectMessagesChatTopic {
      td_name: "updateDirectMessagesChatTopic".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateDirectMessagesChatTopicBuilder { inner }
  }

  pub fn topic(&self) -> &DirectMessagesChatTopic {
    &self.topic
  }
}

#[doc(hidden)]
pub struct RTDUpdateDirectMessagesChatTopicBuilder {
  inner: UpdateDirectMessagesChatTopic,
}

impl RTDUpdateDirectMessagesChatTopicBuilder {
  pub fn build(&self) -> UpdateDirectMessagesChatTopic {
    self.inner.clone()
  }

  pub fn topic<T: AsRef<DirectMessagesChatTopic>>(&mut self, topic: T) -> &mut Self {
    self.inner.topic = topic.as_ref().clone();
    self
  }
}

impl AsRef<UpdateDirectMessagesChatTopic> for UpdateDirectMessagesChatTopic {
  fn as_ref(&self) -> &UpdateDirectMessagesChatTopic {
    self
  }
}

impl AsRef<UpdateDirectMessagesChatTopic> for RTDUpdateDirectMessagesChatTopicBuilder {
  fn as_ref(&self) -> &UpdateDirectMessagesChatTopic {
    &self.inner
  }
}

/// The list of available emoji chat themes has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateEmojiChatThemes {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new list of emoji chat themes
  chat_themes: Vec<EmojiChatTheme>,
}

impl RObject for UpdateEmojiChatThemes {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateEmojiChatThemes"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateEmojiChatThemes {}

impl UpdateEmojiChatThemes {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateEmojiChatThemesBuilder {
    let inner = UpdateEmojiChatThemes {
      td_name: "updateEmojiChatThemes".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateEmojiChatThemesBuilder { inner }
  }

  pub fn chat_themes(&self) -> &Vec<EmojiChatTheme> {
    &self.chat_themes
  }
}

#[doc(hidden)]
pub struct RTDUpdateEmojiChatThemesBuilder {
  inner: UpdateEmojiChatThemes,
}

impl RTDUpdateEmojiChatThemesBuilder {
  pub fn build(&self) -> UpdateEmojiChatThemes {
    self.inner.clone()
  }

  pub fn chat_themes(&mut self, chat_themes: Vec<EmojiChatTheme>) -> &mut Self {
    self.inner.chat_themes = chat_themes;
    self
  }
}

impl AsRef<UpdateEmojiChatThemes> for UpdateEmojiChatThemes {
  fn as_ref(&self) -> &UpdateEmojiChatThemes {
    self
  }
}

impl AsRef<UpdateEmojiChatThemes> for RTDUpdateEmojiChatThemesBuilder {
  fn as_ref(&self) -> &UpdateEmojiChatThemes {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateFavoriteStickers"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateFavoriteStickers {}

impl UpdateFavoriteStickers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateFavoriteStickersBuilder {
    let inner = UpdateFavoriteStickers {
      td_name: "updateFavoriteStickers".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateFavoriteStickersBuilder { inner }
  }

  pub fn sticker_ids(&self) -> &Vec<i64> {
    &self.sticker_ids
  }
}

#[doc(hidden)]
pub struct RTDUpdateFavoriteStickersBuilder {
  inner: UpdateFavoriteStickers,
}

impl RTDUpdateFavoriteStickersBuilder {
  pub fn build(&self) -> UpdateFavoriteStickers {
    self.inner.clone()
  }

  pub fn sticker_ids(&mut self, sticker_ids: Vec<i64>) -> &mut Self {
    self.inner.sticker_ids = sticker_ids;
    self
  }
}

impl AsRef<UpdateFavoriteStickers> for UpdateFavoriteStickers {
  fn as_ref(&self) -> &UpdateFavoriteStickers {
    self
  }
}

impl AsRef<UpdateFavoriteStickers> for RTDUpdateFavoriteStickersBuilder {
  fn as_ref(&self) -> &UpdateFavoriteStickers {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateFile"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateFile {}

impl UpdateFile {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateFileBuilder {
    let inner = UpdateFile {
      td_name: "updateFile".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateFileBuilder { inner }
  }

  pub fn file(&self) -> &File {
    &self.file
  }
}

#[doc(hidden)]
pub struct RTDUpdateFileBuilder {
  inner: UpdateFile,
}

impl RTDUpdateFileBuilder {
  pub fn build(&self) -> UpdateFile {
    self.inner.clone()
  }

  pub fn file<T: AsRef<File>>(&mut self, file: T) -> &mut Self {
    self.inner.file = file.as_ref().clone();
    self
  }
}

impl AsRef<UpdateFile> for UpdateFile {
  fn as_ref(&self) -> &UpdateFile {
    self
  }
}

impl AsRef<UpdateFile> for RTDUpdateFileBuilder {
  fn as_ref(&self) -> &UpdateFile {
    &self.inner
  }
}

/// A file was added to the file download list. This update is sent only after file download list is loaded for the first time
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateFileAddedToDownloads {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The added file download
  file_download: FileDownload,
  /// New number of being downloaded and recently downloaded files found
  counts: DownloadedFileCounts,
}

impl RObject for UpdateFileAddedToDownloads {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateFileAddedToDownloads"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateFileAddedToDownloads {}

impl UpdateFileAddedToDownloads {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateFileAddedToDownloadsBuilder {
    let inner = UpdateFileAddedToDownloads {
      td_name: "updateFileAddedToDownloads".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateFileAddedToDownloadsBuilder { inner }
  }

  pub fn file_download(&self) -> &FileDownload {
    &self.file_download
  }

  pub fn counts(&self) -> &DownloadedFileCounts {
    &self.counts
  }
}

#[doc(hidden)]
pub struct RTDUpdateFileAddedToDownloadsBuilder {
  inner: UpdateFileAddedToDownloads,
}

impl RTDUpdateFileAddedToDownloadsBuilder {
  pub fn build(&self) -> UpdateFileAddedToDownloads {
    self.inner.clone()
  }

  pub fn file_download<T: AsRef<FileDownload>>(&mut self, file_download: T) -> &mut Self {
    self.inner.file_download = file_download.as_ref().clone();
    self
  }

  pub fn counts<T: AsRef<DownloadedFileCounts>>(&mut self, counts: T) -> &mut Self {
    self.inner.counts = counts.as_ref().clone();
    self
  }
}

impl AsRef<UpdateFileAddedToDownloads> for UpdateFileAddedToDownloads {
  fn as_ref(&self) -> &UpdateFileAddedToDownloads {
    self
  }
}

impl AsRef<UpdateFileAddedToDownloads> for RTDUpdateFileAddedToDownloadsBuilder {
  fn as_ref(&self) -> &UpdateFileAddedToDownloads {
    &self.inner
  }
}

/// A file download was changed. This update is sent only after file download list is loaded for the first time
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateFileDownload {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// File identifier
  file_id: i64,
  /// Point in time (Unix timestamp) when the file downloading was completed; 0 if the file downloading isn't completed
  complete_date: i64,
  /// True, if downloading of the file is paused
  is_paused: bool,
  /// New number of being downloaded and recently downloaded files found
  counts: DownloadedFileCounts,
}

impl RObject for UpdateFileDownload {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateFileDownload"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateFileDownload {}

impl UpdateFileDownload {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateFileDownloadBuilder {
    let inner = UpdateFileDownload {
      td_name: "updateFileDownload".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateFileDownloadBuilder { inner }
  }

  pub fn file_id(&self) -> i64 {
    self.file_id
  }

  pub fn complete_date(&self) -> i64 {
    self.complete_date
  }

  pub fn is_paused(&self) -> bool {
    self.is_paused
  }

  pub fn counts(&self) -> &DownloadedFileCounts {
    &self.counts
  }
}

#[doc(hidden)]
pub struct RTDUpdateFileDownloadBuilder {
  inner: UpdateFileDownload,
}

impl RTDUpdateFileDownloadBuilder {
  pub fn build(&self) -> UpdateFileDownload {
    self.inner.clone()
  }

  pub fn file_id(&mut self, file_id: i64) -> &mut Self {
    self.inner.file_id = file_id;
    self
  }

  pub fn complete_date(&mut self, complete_date: i64) -> &mut Self {
    self.inner.complete_date = complete_date;
    self
  }

  pub fn is_paused(&mut self, is_paused: bool) -> &mut Self {
    self.inner.is_paused = is_paused;
    self
  }

  pub fn counts<T: AsRef<DownloadedFileCounts>>(&mut self, counts: T) -> &mut Self {
    self.inner.counts = counts.as_ref().clone();
    self
  }
}

impl AsRef<UpdateFileDownload> for UpdateFileDownload {
  fn as_ref(&self) -> &UpdateFileDownload {
    self
  }
}

impl AsRef<UpdateFileDownload> for RTDUpdateFileDownloadBuilder {
  fn as_ref(&self) -> &UpdateFileDownload {
    &self.inner
  }
}

/// The state of the file download list has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateFileDownloads {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Total size of files in the file download list, in bytes
  total_size: i64,
  /// Total number of files in the file download list
  total_count: i64,
  /// Total downloaded size of files in the file download list, in bytes
  downloaded_size: i64,
}

impl RObject for UpdateFileDownloads {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateFileDownloads"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateFileDownloads {}

impl UpdateFileDownloads {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateFileDownloadsBuilder {
    let inner = UpdateFileDownloads {
      td_name: "updateFileDownloads".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateFileDownloadsBuilder { inner }
  }

  pub fn total_size(&self) -> i64 {
    self.total_size
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }

  pub fn downloaded_size(&self) -> i64 {
    self.downloaded_size
  }
}

#[doc(hidden)]
pub struct RTDUpdateFileDownloadsBuilder {
  inner: UpdateFileDownloads,
}

impl RTDUpdateFileDownloadsBuilder {
  pub fn build(&self) -> UpdateFileDownloads {
    self.inner.clone()
  }

  pub fn total_size(&mut self, total_size: i64) -> &mut Self {
    self.inner.total_size = total_size;
    self
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

  pub fn downloaded_size(&mut self, downloaded_size: i64) -> &mut Self {
    self.inner.downloaded_size = downloaded_size;
    self
  }
}

impl AsRef<UpdateFileDownloads> for UpdateFileDownloads {
  fn as_ref(&self) -> &UpdateFileDownloads {
    self
  }
}

impl AsRef<UpdateFileDownloads> for RTDUpdateFileDownloadsBuilder {
  fn as_ref(&self) -> &UpdateFileDownloads {
    &self.inner
  }
}

/// The file generation process needs to be started by the application. Use setFileGenerationProgress and finishFileGeneration to generate the file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateFileGenerationStart {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier for the generation process
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  generation_id: isize,
  /// The original path specified by the application in inputFileGenerated
  original_path: String,
  /// The path to a file that must be created and where the new file must be generated by the application. If the application has no access to the path, it can use writeGeneratedFilePart to generate the file
  destination_path: String,
  /// If the conversion is "#url#", then original_path contains an HTTP/HTTPS URL of a file that must be downloaded by the application. Otherwise, this is the conversion specified by the application in inputFileGenerated
  conversion: String,
}

impl RObject for UpdateFileGenerationStart {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateFileGenerationStart"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateFileGenerationStart {}

impl UpdateFileGenerationStart {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateFileGenerationStartBuilder {
    let inner = UpdateFileGenerationStart {
      td_name: "updateFileGenerationStart".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateFileGenerationStartBuilder { inner }
  }

  pub fn generation_id(&self) -> isize {
    self.generation_id
  }

  pub fn original_path(&self) -> &String {
    &self.original_path
  }

  pub fn destination_path(&self) -> &String {
    &self.destination_path
  }

  pub fn conversion(&self) -> &String {
    &self.conversion
  }
}

#[doc(hidden)]
pub struct RTDUpdateFileGenerationStartBuilder {
  inner: UpdateFileGenerationStart,
}

impl RTDUpdateFileGenerationStartBuilder {
  pub fn build(&self) -> UpdateFileGenerationStart {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateFileGenerationStart {
    self
  }
}

impl AsRef<UpdateFileGenerationStart> for RTDUpdateFileGenerationStartBuilder {
  fn as_ref(&self) -> &UpdateFileGenerationStart {
    &self.inner
  }
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
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  generation_id: isize,
}

impl RObject for UpdateFileGenerationStop {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateFileGenerationStop"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateFileGenerationStop {}

impl UpdateFileGenerationStop {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateFileGenerationStopBuilder {
    let inner = UpdateFileGenerationStop {
      td_name: "updateFileGenerationStop".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateFileGenerationStopBuilder { inner }
  }

  pub fn generation_id(&self) -> isize {
    self.generation_id
  }
}

#[doc(hidden)]
pub struct RTDUpdateFileGenerationStopBuilder {
  inner: UpdateFileGenerationStop,
}

impl RTDUpdateFileGenerationStopBuilder {
  pub fn build(&self) -> UpdateFileGenerationStop {
    self.inner.clone()
  }

  pub fn generation_id(&mut self, generation_id: isize) -> &mut Self {
    self.inner.generation_id = generation_id;
    self
  }
}

impl AsRef<UpdateFileGenerationStop> for UpdateFileGenerationStop {
  fn as_ref(&self) -> &UpdateFileGenerationStop {
    self
  }
}

impl AsRef<UpdateFileGenerationStop> for RTDUpdateFileGenerationStopBuilder {
  fn as_ref(&self) -> &UpdateFileGenerationStop {
    &self.inner
  }
}

/// A file was removed from the file download list. This update is sent only after file download list is loaded for the first time
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateFileRemovedFromDownloads {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// File identifier
  file_id: i64,
  /// New number of being downloaded and recently downloaded files found
  counts: DownloadedFileCounts,
}

impl RObject for UpdateFileRemovedFromDownloads {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateFileRemovedFromDownloads"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateFileRemovedFromDownloads {}

impl UpdateFileRemovedFromDownloads {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateFileRemovedFromDownloadsBuilder {
    let inner = UpdateFileRemovedFromDownloads {
      td_name: "updateFileRemovedFromDownloads".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateFileRemovedFromDownloadsBuilder { inner }
  }

  pub fn file_id(&self) -> i64 {
    self.file_id
  }

  pub fn counts(&self) -> &DownloadedFileCounts {
    &self.counts
  }
}

#[doc(hidden)]
pub struct RTDUpdateFileRemovedFromDownloadsBuilder {
  inner: UpdateFileRemovedFromDownloads,
}

impl RTDUpdateFileRemovedFromDownloadsBuilder {
  pub fn build(&self) -> UpdateFileRemovedFromDownloads {
    self.inner.clone()
  }

  pub fn file_id(&mut self, file_id: i64) -> &mut Self {
    self.inner.file_id = file_id;
    self
  }

  pub fn counts<T: AsRef<DownloadedFileCounts>>(&mut self, counts: T) -> &mut Self {
    self.inner.counts = counts.as_ref().clone();
    self
  }
}

impl AsRef<UpdateFileRemovedFromDownloads> for UpdateFileRemovedFromDownloads {
  fn as_ref(&self) -> &UpdateFileRemovedFromDownloads {
    self
  }
}

impl AsRef<UpdateFileRemovedFromDownloads> for RTDUpdateFileRemovedFromDownloadsBuilder {
  fn as_ref(&self) -> &UpdateFileRemovedFromDownloads {
    &self.inner
  }
}

/// Information about a topic in a forum chat was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateForumTopic {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Forum topic identifier of the topic
  forum_topic_id: i64,
  /// True, if the topic is pinned in the topic list
  is_pinned: bool,
  /// Identifier of the last read incoming message
  last_read_inbox_message_id: i64,
  /// Identifier of the last read outgoing message
  last_read_outbox_message_id: i64,
  /// Number of unread messages with a mention/reply in the topic
  unread_mention_count: i64,
  /// Number of messages with unread reactions in the topic
  unread_reaction_count: i64,
  /// Number of messages with unread poll votes in the topic
  unread_poll_vote_count: i64,
  /// Notification settings for the topic
  notification_settings: ChatNotificationSettings,
  /// A draft of a message in the topic; may be null if none
  draft_message: Option<DraftMessage>,
}

impl RObject for UpdateForumTopic {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateForumTopic"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateForumTopic {}

impl UpdateForumTopic {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateForumTopicBuilder {
    let inner = UpdateForumTopic {
      td_name: "updateForumTopic".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateForumTopicBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn forum_topic_id(&self) -> i64 {
    self.forum_topic_id
  }

  pub fn is_pinned(&self) -> bool {
    self.is_pinned
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

  pub fn draft_message(&self) -> &Option<DraftMessage> {
    &self.draft_message
  }
}

#[doc(hidden)]
pub struct RTDUpdateForumTopicBuilder {
  inner: UpdateForumTopic,
}

impl RTDUpdateForumTopicBuilder {
  pub fn build(&self) -> UpdateForumTopic {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn forum_topic_id(&mut self, forum_topic_id: i64) -> &mut Self {
    self.inner.forum_topic_id = forum_topic_id;
    self
  }

  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
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

  pub fn draft_message<T: AsRef<DraftMessage>>(&mut self, draft_message: T) -> &mut Self {
    self.inner.draft_message = Some(draft_message.as_ref().clone());
    self
  }
}

impl AsRef<UpdateForumTopic> for UpdateForumTopic {
  fn as_ref(&self) -> &UpdateForumTopic {
    self
  }
}

impl AsRef<UpdateForumTopic> for RTDUpdateForumTopicBuilder {
  fn as_ref(&self) -> &UpdateForumTopic {
    &self.inner
  }
}

/// Basic information about a topic in a forum chat was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateForumTopicInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New information about the topic
  info: ForumTopicInfo,
}

impl RObject for UpdateForumTopicInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateForumTopicInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateForumTopicInfo {}

impl UpdateForumTopicInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateForumTopicInfoBuilder {
    let inner = UpdateForumTopicInfo {
      td_name: "updateForumTopicInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateForumTopicInfoBuilder { inner }
  }

  pub fn info(&self) -> &ForumTopicInfo {
    &self.info
  }
}

#[doc(hidden)]
pub struct RTDUpdateForumTopicInfoBuilder {
  inner: UpdateForumTopicInfo,
}

impl RTDUpdateForumTopicInfoBuilder {
  pub fn build(&self) -> UpdateForumTopicInfo {
    self.inner.clone()
  }

  pub fn info<T: AsRef<ForumTopicInfo>>(&mut self, info: T) -> &mut Self {
    self.inner.info = info.as_ref().clone();
    self
  }
}

impl AsRef<UpdateForumTopicInfo> for UpdateForumTopicInfo {
  fn as_ref(&self) -> &UpdateForumTopicInfo {
    self
  }
}

impl AsRef<UpdateForumTopicInfo> for RTDUpdateForumTopicInfoBuilder {
  fn as_ref(&self) -> &UpdateForumTopicInfo {
    &self.inner
  }
}

/// The freeze state of the current user's account has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateFreezeState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the account is frozen
  is_frozen: bool,
  /// Point in time (Unix timestamp) when the account was frozen; 0 if the account isn't frozen
  freezing_date: i64,
  /// Point in time (Unix timestamp) when the account will be deleted and can't be unfrozen; 0 if the account isn't frozen
  deletion_date: i64,
  /// The link to open to send an appeal to unfreeze the account
  appeal_link: String,
}

impl RObject for UpdateFreezeState {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateFreezeState"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateFreezeState {}

impl UpdateFreezeState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateFreezeStateBuilder {
    let inner = UpdateFreezeState {
      td_name: "updateFreezeState".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateFreezeStateBuilder { inner }
  }

  pub fn is_frozen(&self) -> bool {
    self.is_frozen
  }

  pub fn freezing_date(&self) -> i64 {
    self.freezing_date
  }

  pub fn deletion_date(&self) -> i64 {
    self.deletion_date
  }

  pub fn appeal_link(&self) -> &String {
    &self.appeal_link
  }
}

#[doc(hidden)]
pub struct RTDUpdateFreezeStateBuilder {
  inner: UpdateFreezeState,
}

impl RTDUpdateFreezeStateBuilder {
  pub fn build(&self) -> UpdateFreezeState {
    self.inner.clone()
  }

  pub fn is_frozen(&mut self, is_frozen: bool) -> &mut Self {
    self.inner.is_frozen = is_frozen;
    self
  }

  pub fn freezing_date(&mut self, freezing_date: i64) -> &mut Self {
    self.inner.freezing_date = freezing_date;
    self
  }

  pub fn deletion_date(&mut self, deletion_date: i64) -> &mut Self {
    self.inner.deletion_date = deletion_date;
    self
  }

  pub fn appeal_link<T: AsRef<str>>(&mut self, appeal_link: T) -> &mut Self {
    self.inner.appeal_link = appeal_link.as_ref().to_string();
    self
  }
}

impl AsRef<UpdateFreezeState> for UpdateFreezeState {
  fn as_ref(&self) -> &UpdateFreezeState {
    self
  }
}

impl AsRef<UpdateFreezeState> for RTDUpdateFreezeStateBuilder {
  fn as_ref(&self) -> &UpdateFreezeState {
    &self.inner
  }
}

/// State of a gift auction was updated
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateGiftAuctionState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New state of the auction
  state: GiftAuctionState,
}

impl RObject for UpdateGiftAuctionState {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateGiftAuctionState"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateGiftAuctionState {}

impl UpdateGiftAuctionState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateGiftAuctionStateBuilder {
    let inner = UpdateGiftAuctionState {
      td_name: "updateGiftAuctionState".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateGiftAuctionStateBuilder { inner }
  }

  pub fn state(&self) -> &GiftAuctionState {
    &self.state
  }
}

#[doc(hidden)]
pub struct RTDUpdateGiftAuctionStateBuilder {
  inner: UpdateGiftAuctionState,
}

impl RTDUpdateGiftAuctionStateBuilder {
  pub fn build(&self) -> UpdateGiftAuctionState {
    self.inner.clone()
  }

  pub fn state<T: AsRef<GiftAuctionState>>(&mut self, state: T) -> &mut Self {
    self.inner.state = state.as_ref().clone();
    self
  }
}

impl AsRef<UpdateGiftAuctionState> for UpdateGiftAuctionState {
  fn as_ref(&self) -> &UpdateGiftAuctionState {
    self
  }
}

impl AsRef<UpdateGiftAuctionState> for RTDUpdateGiftAuctionStateBuilder {
  fn as_ref(&self) -> &UpdateGiftAuctionState {
    &self.inner
  }
}

/// The TON Gram revenue earned by the current user has changed. If Gram transaction screen of the chat is opened, then getTonTransactions may be called to fetch new transactions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateGramRevenueStatus {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New Gram revenue status
  status: GramRevenueStatus,
}

impl RObject for UpdateGramRevenueStatus {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateGramRevenueStatus"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateGramRevenueStatus {}

impl UpdateGramRevenueStatus {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateGramRevenueStatusBuilder {
    let inner = UpdateGramRevenueStatus {
      td_name: "updateGramRevenueStatus".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateGramRevenueStatusBuilder { inner }
  }

  pub fn status(&self) -> &GramRevenueStatus {
    &self.status
  }
}

#[doc(hidden)]
pub struct RTDUpdateGramRevenueStatusBuilder {
  inner: UpdateGramRevenueStatus,
}

impl RTDUpdateGramRevenueStatusBuilder {
  pub fn build(&self) -> UpdateGramRevenueStatus {
    self.inner.clone()
  }

  pub fn status<T: AsRef<GramRevenueStatus>>(&mut self, status: T) -> &mut Self {
    self.inner.status = status.as_ref().clone();
    self
  }
}

impl AsRef<UpdateGramRevenueStatus> for UpdateGramRevenueStatus {
  fn as_ref(&self) -> &UpdateGramRevenueStatus {
    self
  }
}

impl AsRef<UpdateGramRevenueStatus> for RTDUpdateGramRevenueStatusBuilder {
  fn as_ref(&self) -> &UpdateGramRevenueStatus {
    &self.inner
  }
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
  /// New data about the group call
  group_call: GroupCall,
}

impl RObject for UpdateGroupCall {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateGroupCall"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateGroupCall {}

impl UpdateGroupCall {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateGroupCallBuilder {
    let inner = UpdateGroupCall {
      td_name: "updateGroupCall".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateGroupCallBuilder { inner }
  }

  pub fn group_call(&self) -> &GroupCall {
    &self.group_call
  }
}

#[doc(hidden)]
pub struct RTDUpdateGroupCallBuilder {
  inner: UpdateGroupCall,
}

impl RTDUpdateGroupCallBuilder {
  pub fn build(&self) -> UpdateGroupCall {
    self.inner.clone()
  }

  pub fn group_call<T: AsRef<GroupCall>>(&mut self, group_call: T) -> &mut Self {
    self.inner.group_call = group_call.as_ref().clone();
    self
  }
}

impl AsRef<UpdateGroupCall> for UpdateGroupCall {
  fn as_ref(&self) -> &UpdateGroupCall {
    self
  }
}

impl AsRef<UpdateGroupCall> for RTDUpdateGroupCallBuilder {
  fn as_ref(&self) -> &UpdateGroupCall {
    &self.inner
  }
}

/// The levels of live story group call messages have changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateGroupCallMessageLevels {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New description of the levels in decreasing order of groupCallMessageLevel.min_star_count
  levels: Vec<GroupCallMessageLevel>,
}

impl RObject for UpdateGroupCallMessageLevels {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateGroupCallMessageLevels"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateGroupCallMessageLevels {}

impl UpdateGroupCallMessageLevels {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateGroupCallMessageLevelsBuilder {
    let inner = UpdateGroupCallMessageLevels {
      td_name: "updateGroupCallMessageLevels".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateGroupCallMessageLevelsBuilder { inner }
  }

  pub fn levels(&self) -> &Vec<GroupCallMessageLevel> {
    &self.levels
  }
}

#[doc(hidden)]
pub struct RTDUpdateGroupCallMessageLevelsBuilder {
  inner: UpdateGroupCallMessageLevels,
}

impl RTDUpdateGroupCallMessageLevelsBuilder {
  pub fn build(&self) -> UpdateGroupCallMessageLevels {
    self.inner.clone()
  }

  pub fn levels(&mut self, levels: Vec<GroupCallMessageLevel>) -> &mut Self {
    self.inner.levels = levels;
    self
  }
}

impl AsRef<UpdateGroupCallMessageLevels> for UpdateGroupCallMessageLevels {
  fn as_ref(&self) -> &UpdateGroupCallMessageLevels {
    self
  }
}

impl AsRef<UpdateGroupCallMessageLevels> for RTDUpdateGroupCallMessageLevelsBuilder {
  fn as_ref(&self) -> &UpdateGroupCallMessageLevels {
    &self.inner
  }
}

/// A group call message failed to send
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateGroupCallMessageSendFailed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the group call
  group_call_id: i64,
  /// Message identifier
  message_id: i64,
  /// The cause of the message sending failure
  error: Error,
}

impl RObject for UpdateGroupCallMessageSendFailed {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateGroupCallMessageSendFailed"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateGroupCallMessageSendFailed {}

impl UpdateGroupCallMessageSendFailed {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateGroupCallMessageSendFailedBuilder {
    let inner = UpdateGroupCallMessageSendFailed {
      td_name: "updateGroupCallMessageSendFailed".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateGroupCallMessageSendFailedBuilder { inner }
  }

  pub fn group_call_id(&self) -> i64 {
    self.group_call_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }

  pub fn error(&self) -> &Error {
    &self.error
  }
}

#[doc(hidden)]
pub struct RTDUpdateGroupCallMessageSendFailedBuilder {
  inner: UpdateGroupCallMessageSendFailed,
}

impl RTDUpdateGroupCallMessageSendFailedBuilder {
  pub fn build(&self) -> UpdateGroupCallMessageSendFailed {
    self.inner.clone()
  }

  pub fn group_call_id(&mut self, group_call_id: i64) -> &mut Self {
    self.inner.group_call_id = group_call_id;
    self
  }

  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

  pub fn error<T: AsRef<Error>>(&mut self, error: T) -> &mut Self {
    self.inner.error = error.as_ref().clone();
    self
  }
}

impl AsRef<UpdateGroupCallMessageSendFailed> for UpdateGroupCallMessageSendFailed {
  fn as_ref(&self) -> &UpdateGroupCallMessageSendFailed {
    self
  }
}

impl AsRef<UpdateGroupCallMessageSendFailed> for RTDUpdateGroupCallMessageSendFailedBuilder {
  fn as_ref(&self) -> &UpdateGroupCallMessageSendFailed {
    &self.inner
  }
}

/// Some group call messages were deleted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateGroupCallMessagesDeleted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the group call
  group_call_id: i64,
  /// Identifiers of the deleted messages
  message_ids: Vec<i64>,
}

impl RObject for UpdateGroupCallMessagesDeleted {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateGroupCallMessagesDeleted"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateGroupCallMessagesDeleted {}

impl UpdateGroupCallMessagesDeleted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateGroupCallMessagesDeletedBuilder {
    let inner = UpdateGroupCallMessagesDeleted {
      td_name: "updateGroupCallMessagesDeleted".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateGroupCallMessagesDeletedBuilder { inner }
  }

  pub fn group_call_id(&self) -> i64 {
    self.group_call_id
  }

  pub fn message_ids(&self) -> &Vec<i64> {
    &self.message_ids
  }
}

#[doc(hidden)]
pub struct RTDUpdateGroupCallMessagesDeletedBuilder {
  inner: UpdateGroupCallMessagesDeleted,
}

impl RTDUpdateGroupCallMessagesDeletedBuilder {
  pub fn build(&self) -> UpdateGroupCallMessagesDeleted {
    self.inner.clone()
  }

  pub fn group_call_id(&mut self, group_call_id: i64) -> &mut Self {
    self.inner.group_call_id = group_call_id;
    self
  }

  pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self {
    self.inner.message_ids = message_ids;
    self
  }
}

impl AsRef<UpdateGroupCallMessagesDeleted> for UpdateGroupCallMessagesDeleted {
  fn as_ref(&self) -> &UpdateGroupCallMessagesDeleted {
    self
  }
}

impl AsRef<UpdateGroupCallMessagesDeleted> for RTDUpdateGroupCallMessagesDeletedBuilder {
  fn as_ref(&self) -> &UpdateGroupCallMessagesDeleted {
    &self.inner
  }
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
  /// Identifier of the group call
  group_call_id: i64,
  /// New data about the participant
  participant: GroupCallParticipant,
}

impl RObject for UpdateGroupCallParticipant {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateGroupCallParticipant"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateGroupCallParticipant {}

impl UpdateGroupCallParticipant {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateGroupCallParticipantBuilder {
    let inner = UpdateGroupCallParticipant {
      td_name: "updateGroupCallParticipant".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateGroupCallParticipantBuilder { inner }
  }

  pub fn group_call_id(&self) -> i64 {
    self.group_call_id
  }

  pub fn participant(&self) -> &GroupCallParticipant {
    &self.participant
  }
}

#[doc(hidden)]
pub struct RTDUpdateGroupCallParticipantBuilder {
  inner: UpdateGroupCallParticipant,
}

impl RTDUpdateGroupCallParticipantBuilder {
  pub fn build(&self) -> UpdateGroupCallParticipant {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateGroupCallParticipant {
    self
  }
}

impl AsRef<UpdateGroupCallParticipant> for RTDUpdateGroupCallParticipantBuilder {
  fn as_ref(&self) -> &UpdateGroupCallParticipant {
    &self.inner
  }
}

/// The list of group call participants that can send and receive encrypted call data has changed; for group calls not bound to a chat only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateGroupCallParticipants {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the group call
  group_call_id: i64,
  /// New list of group call participant user identifiers. The identifiers may be invalid or the corresponding users may be unknown. The participants must be shown in the list of group call participants even if there is no information about them
  participant_user_ids: Vec<isize>,
}

impl RObject for UpdateGroupCallParticipants {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateGroupCallParticipants"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateGroupCallParticipants {}

impl UpdateGroupCallParticipants {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateGroupCallParticipantsBuilder {
    let inner = UpdateGroupCallParticipants {
      td_name: "updateGroupCallParticipants".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateGroupCallParticipantsBuilder { inner }
  }

  pub fn group_call_id(&self) -> i64 {
    self.group_call_id
  }

  pub fn participant_user_ids(&self) -> &Vec<isize> {
    &self.participant_user_ids
  }
}

#[doc(hidden)]
pub struct RTDUpdateGroupCallParticipantsBuilder {
  inner: UpdateGroupCallParticipants,
}

impl RTDUpdateGroupCallParticipantsBuilder {
  pub fn build(&self) -> UpdateGroupCallParticipants {
    self.inner.clone()
  }

  pub fn group_call_id(&mut self, group_call_id: i64) -> &mut Self {
    self.inner.group_call_id = group_call_id;
    self
  }

  pub fn participant_user_ids(&mut self, participant_user_ids: Vec<isize>) -> &mut Self {
    self.inner.participant_user_ids = participant_user_ids;
    self
  }
}

impl AsRef<UpdateGroupCallParticipants> for UpdateGroupCallParticipants {
  fn as_ref(&self) -> &UpdateGroupCallParticipants {
    self
  }
}

impl AsRef<UpdateGroupCallParticipants> for RTDUpdateGroupCallParticipantsBuilder {
  fn as_ref(&self) -> &UpdateGroupCallParticipants {
    &self.inner
  }
}

/// The verification state of an encrypted group call has changed; for group calls not bound to a chat only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateGroupCallVerificationState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the group call
  group_call_id: i64,
  /// The call state generation to which the emoji corresponds. If generation is different for two users, then their emoji may be also different
  generation: i64,
  /// Group call state fingerprint represented as 4 emoji; may be empty if the state isn't verified yet
  emojis: Vec<String>,
}

impl RObject for UpdateGroupCallVerificationState {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateGroupCallVerificationState"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateGroupCallVerificationState {}

impl UpdateGroupCallVerificationState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateGroupCallVerificationStateBuilder {
    let inner = UpdateGroupCallVerificationState {
      td_name: "updateGroupCallVerificationState".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateGroupCallVerificationStateBuilder { inner }
  }

  pub fn group_call_id(&self) -> i64 {
    self.group_call_id
  }

  pub fn generation(&self) -> i64 {
    self.generation
  }

  pub fn emojis(&self) -> &Vec<String> {
    &self.emojis
  }
}

#[doc(hidden)]
pub struct RTDUpdateGroupCallVerificationStateBuilder {
  inner: UpdateGroupCallVerificationState,
}

impl RTDUpdateGroupCallVerificationStateBuilder {
  pub fn build(&self) -> UpdateGroupCallVerificationState {
    self.inner.clone()
  }

  pub fn group_call_id(&mut self, group_call_id: i64) -> &mut Self {
    self.inner.group_call_id = group_call_id;
    self
  }

  pub fn generation(&mut self, generation: i64) -> &mut Self {
    self.inner.generation = generation;
    self
  }

  pub fn emojis(&mut self, emojis: Vec<String>) -> &mut Self {
    self.inner.emojis = emojis;
    self
  }
}

impl AsRef<UpdateGroupCallVerificationState> for UpdateGroupCallVerificationState {
  fn as_ref(&self) -> &UpdateGroupCallVerificationState {
    self
  }
}

impl AsRef<UpdateGroupCallVerificationState> for RTDUpdateGroupCallVerificationStateBuilder {
  fn as_ref(&self) -> &UpdateGroupCallVerificationState {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateHavePendingNotifications"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateHavePendingNotifications {}

impl UpdateHavePendingNotifications {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateHavePendingNotificationsBuilder {
    let inner = UpdateHavePendingNotifications {
      td_name: "updateHavePendingNotifications".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateHavePendingNotificationsBuilder { inner }
  }

  pub fn have_delayed_notifications(&self) -> bool {
    self.have_delayed_notifications
  }

  pub fn have_unreceived_notifications(&self) -> bool {
    self.have_unreceived_notifications
  }
}

#[doc(hidden)]
pub struct RTDUpdateHavePendingNotificationsBuilder {
  inner: UpdateHavePendingNotifications,
}

impl RTDUpdateHavePendingNotificationsBuilder {
  pub fn build(&self) -> UpdateHavePendingNotifications {
    self.inner.clone()
  }

  pub fn have_delayed_notifications(&mut self, have_delayed_notifications: bool) -> &mut Self {
    self.inner.have_delayed_notifications = have_delayed_notifications;
    self
  }

  pub fn have_unreceived_notifications(
    &mut self,
    have_unreceived_notifications: bool,
  ) -> &mut Self {
    self.inner.have_unreceived_notifications = have_unreceived_notifications;
    self
  }
}

impl AsRef<UpdateHavePendingNotifications> for UpdateHavePendingNotifications {
  fn as_ref(&self) -> &UpdateHavePendingNotifications {
    self
  }
}

impl AsRef<UpdateHavePendingNotifications> for RTDUpdateHavePendingNotificationsBuilder {
  fn as_ref(&self) -> &UpdateHavePendingNotifications {
    &self.inner
  }
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
  /// Type of the affected stickers
  sticker_type: StickerType,
  /// The new list of installed ordinary sticker sets
  sticker_set_ids: Vec<isize>,
}

impl RObject for UpdateInstalledStickerSets {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateInstalledStickerSets"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateInstalledStickerSets {}

impl UpdateInstalledStickerSets {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateInstalledStickerSetsBuilder {
    let inner = UpdateInstalledStickerSets {
      td_name: "updateInstalledStickerSets".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateInstalledStickerSetsBuilder { inner }
  }

  pub fn sticker_type(&self) -> &StickerType {
    &self.sticker_type
  }

  pub fn sticker_set_ids(&self) -> &Vec<isize> {
    &self.sticker_set_ids
  }
}

#[doc(hidden)]
pub struct RTDUpdateInstalledStickerSetsBuilder {
  inner: UpdateInstalledStickerSets,
}

impl RTDUpdateInstalledStickerSetsBuilder {
  pub fn build(&self) -> UpdateInstalledStickerSets {
    self.inner.clone()
  }

  pub fn sticker_type<T: AsRef<StickerType>>(&mut self, sticker_type: T) -> &mut Self {
    self.inner.sticker_type = sticker_type.as_ref().clone();
    self
  }

  pub fn sticker_set_ids(&mut self, sticker_set_ids: Vec<isize>) -> &mut Self {
    self.inner.sticker_set_ids = sticker_set_ids;
    self
  }
}

impl AsRef<UpdateInstalledStickerSets> for UpdateInstalledStickerSets {
  fn as_ref(&self) -> &UpdateInstalledStickerSets {
    self
  }
}

impl AsRef<UpdateInstalledStickerSets> for RTDUpdateInstalledStickerSetsBuilder {
  fn as_ref(&self) -> &UpdateInstalledStickerSets {
    &self.inner
  }
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
  /// List of changed language pack strings; empty if all strings have changed
  strings: Vec<LanguagePackString>,
}

impl RObject for UpdateLanguagePackStrings {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateLanguagePackStrings"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateLanguagePackStrings {}

impl UpdateLanguagePackStrings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateLanguagePackStringsBuilder {
    let inner = UpdateLanguagePackStrings {
      td_name: "updateLanguagePackStrings".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateLanguagePackStringsBuilder { inner }
  }

  pub fn localization_target(&self) -> &String {
    &self.localization_target
  }

  pub fn language_pack_id(&self) -> &String {
    &self.language_pack_id
  }

  pub fn strings(&self) -> &Vec<LanguagePackString> {
    &self.strings
  }
}

#[doc(hidden)]
pub struct RTDUpdateLanguagePackStringsBuilder {
  inner: UpdateLanguagePackStrings,
}

impl RTDUpdateLanguagePackStringsBuilder {
  pub fn build(&self) -> UpdateLanguagePackStrings {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateLanguagePackStrings {
    self
  }
}

impl AsRef<UpdateLanguagePackStrings> for RTDUpdateLanguagePackStringsBuilder {
  fn as_ref(&self) -> &UpdateLanguagePackStrings {
    &self.inner
  }
}

/// The list of top donors in live story group call has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateLiveStoryTopDonors {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the group call
  group_call_id: i64,
  /// New list of live story donors
  donors: LiveStoryDonors,
}

impl RObject for UpdateLiveStoryTopDonors {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateLiveStoryTopDonors"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateLiveStoryTopDonors {}

impl UpdateLiveStoryTopDonors {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateLiveStoryTopDonorsBuilder {
    let inner = UpdateLiveStoryTopDonors {
      td_name: "updateLiveStoryTopDonors".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateLiveStoryTopDonorsBuilder { inner }
  }

  pub fn group_call_id(&self) -> i64 {
    self.group_call_id
  }

  pub fn donors(&self) -> &LiveStoryDonors {
    &self.donors
  }
}

#[doc(hidden)]
pub struct RTDUpdateLiveStoryTopDonorsBuilder {
  inner: UpdateLiveStoryTopDonors,
}

impl RTDUpdateLiveStoryTopDonorsBuilder {
  pub fn build(&self) -> UpdateLiveStoryTopDonors {
    self.inner.clone()
  }

  pub fn group_call_id(&mut self, group_call_id: i64) -> &mut Self {
    self.inner.group_call_id = group_call_id;
    self
  }

  pub fn donors<T: AsRef<LiveStoryDonors>>(&mut self, donors: T) -> &mut Self {
    self.inner.donors = donors.as_ref().clone();
    self
  }
}

impl AsRef<UpdateLiveStoryTopDonors> for UpdateLiveStoryTopDonors {
  fn as_ref(&self) -> &UpdateLiveStoryTopDonors {
    self
  }
}

impl AsRef<UpdateLiveStoryTopDonors> for RTDUpdateLiveStoryTopDonorsBuilder {
  fn as_ref(&self) -> &UpdateLiveStoryTopDonors {
    &self.inner
  }
}

/// A bot that can be managed by the current bot was created or updated; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateManagedBot {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user who created the bot
  user_id: i64,
  /// Identifier of the created managed bot
  bot_user_id: i64,
}

impl RObject for UpdateManagedBot {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateManagedBot"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateManagedBot {}

impl UpdateManagedBot {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateManagedBotBuilder {
    let inner = UpdateManagedBot {
      td_name: "updateManagedBot".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateManagedBotBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn bot_user_id(&self) -> i64 {
    self.bot_user_id
  }
}

#[doc(hidden)]
pub struct RTDUpdateManagedBotBuilder {
  inner: UpdateManagedBot,
}

impl RTDUpdateManagedBotBuilder {
  pub fn build(&self) -> UpdateManagedBot {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
    self.inner.bot_user_id = bot_user_id;
    self
  }
}

impl AsRef<UpdateManagedBot> for UpdateManagedBot {
  fn as_ref(&self) -> &UpdateManagedBot {
    self
  }
}

impl AsRef<UpdateManagedBot> for RTDUpdateManagedBotBuilder {
  fn as_ref(&self) -> &UpdateManagedBot {
    &self.inner
  }
}

/// Unread votes were added or removed from a poll message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageContainsUnreadPollVotes {
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
  /// True, if the message is a poll message with unread votes
  contains_unread_poll_votes: bool,
  /// The new number of messages with unread poll votes in the chat
  unread_poll_vote_count: i64,
}

impl RObject for UpdateMessageContainsUnreadPollVotes {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateMessageContainsUnreadPollVotes"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateMessageContainsUnreadPollVotes {}

impl UpdateMessageContainsUnreadPollVotes {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateMessageContainsUnreadPollVotesBuilder {
    let inner = UpdateMessageContainsUnreadPollVotes {
      td_name: "updateMessageContainsUnreadPollVotes".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateMessageContainsUnreadPollVotesBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }

  pub fn contains_unread_poll_votes(&self) -> bool {
    self.contains_unread_poll_votes
  }

  pub fn unread_poll_vote_count(&self) -> i64 {
    self.unread_poll_vote_count
  }
}

#[doc(hidden)]
pub struct RTDUpdateMessageContainsUnreadPollVotesBuilder {
  inner: UpdateMessageContainsUnreadPollVotes,
}

impl RTDUpdateMessageContainsUnreadPollVotesBuilder {
  pub fn build(&self) -> UpdateMessageContainsUnreadPollVotes {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

  pub fn contains_unread_poll_votes(&mut self, contains_unread_poll_votes: bool) -> &mut Self {
    self.inner.contains_unread_poll_votes = contains_unread_poll_votes;
    self
  }

  pub fn unread_poll_vote_count(&mut self, unread_poll_vote_count: i64) -> &mut Self {
    self.inner.unread_poll_vote_count = unread_poll_vote_count;
    self
  }
}

impl AsRef<UpdateMessageContainsUnreadPollVotes> for UpdateMessageContainsUnreadPollVotes {
  fn as_ref(&self) -> &UpdateMessageContainsUnreadPollVotes {
    self
  }
}

impl AsRef<UpdateMessageContainsUnreadPollVotes>
  for RTDUpdateMessageContainsUnreadPollVotesBuilder
{
  fn as_ref(&self) -> &UpdateMessageContainsUnreadPollVotes {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateMessageContent"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateMessageContent {}

impl UpdateMessageContent {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateMessageContentBuilder {
    let inner = UpdateMessageContent {
      td_name: "updateMessageContent".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateMessageContentBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }

  pub fn new_content(&self) -> &MessageContent {
    &self.new_content
  }
}

#[doc(hidden)]
pub struct RTDUpdateMessageContentBuilder {
  inner: UpdateMessageContent,
}

impl RTDUpdateMessageContentBuilder {
  pub fn build(&self) -> UpdateMessageContent {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateMessageContent {
    self
  }
}

impl AsRef<UpdateMessageContent> for RTDUpdateMessageContentBuilder {
  fn as_ref(&self) -> &UpdateMessageContent {
    &self.inner
  }
}

/// The message content was opened. Updates voice note messages to "listened", video note messages to "viewed" and starts the self-destruct timer
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateMessageContentOpened"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateMessageContentOpened {}

impl UpdateMessageContentOpened {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateMessageContentOpenedBuilder {
    let inner = UpdateMessageContentOpened {
      td_name: "updateMessageContentOpened".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateMessageContentOpenedBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }
}

#[doc(hidden)]
pub struct RTDUpdateMessageContentOpenedBuilder {
  inner: UpdateMessageContentOpened,
}

impl RTDUpdateMessageContentOpenedBuilder {
  pub fn build(&self) -> UpdateMessageContentOpened {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateMessageContentOpened {
    self
  }
}

impl AsRef<UpdateMessageContentOpened> for RTDUpdateMessageContentOpenedBuilder {
  fn as_ref(&self) -> &UpdateMessageContentOpened {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateMessageEdited"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateMessageEdited {}

impl UpdateMessageEdited {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateMessageEditedBuilder {
    let inner = UpdateMessageEdited {
      td_name: "updateMessageEdited".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateMessageEditedBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }

  pub fn edit_date(&self) -> i64 {
    self.edit_date
  }

  pub fn reply_markup(&self) -> &Option<ReplyMarkup> {
    &self.reply_markup
  }
}

#[doc(hidden)]
pub struct RTDUpdateMessageEditedBuilder {
  inner: UpdateMessageEdited,
}

impl RTDUpdateMessageEditedBuilder {
  pub fn build(&self) -> UpdateMessageEdited {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateMessageEdited {
    self
  }
}

impl AsRef<UpdateMessageEdited> for RTDUpdateMessageEditedBuilder {
  fn as_ref(&self) -> &UpdateMessageEdited {
    &self.inner
  }
}

/// The message ephemeral content has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageEphemeralContent {
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
  /// New ephemeral content of the message; may be null if none
  ephemeral_content: Option<EphemeralMessageContent>,
}

impl RObject for UpdateMessageEphemeralContent {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateMessageEphemeralContent"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateMessageEphemeralContent {}

impl UpdateMessageEphemeralContent {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateMessageEphemeralContentBuilder {
    let inner = UpdateMessageEphemeralContent {
      td_name: "updateMessageEphemeralContent".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateMessageEphemeralContentBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }

  pub fn ephemeral_content(&self) -> &Option<EphemeralMessageContent> {
    &self.ephemeral_content
  }
}

#[doc(hidden)]
pub struct RTDUpdateMessageEphemeralContentBuilder {
  inner: UpdateMessageEphemeralContent,
}

impl RTDUpdateMessageEphemeralContentBuilder {
  pub fn build(&self) -> UpdateMessageEphemeralContent {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

  pub fn ephemeral_content<T: AsRef<EphemeralMessageContent>>(
    &mut self,
    ephemeral_content: T,
  ) -> &mut Self {
    self.inner.ephemeral_content = Some(ephemeral_content.as_ref().clone());
    self
  }
}

impl AsRef<UpdateMessageEphemeralContent> for UpdateMessageEphemeralContent {
  fn as_ref(&self) -> &UpdateMessageEphemeralContent {
    self
  }
}

impl AsRef<UpdateMessageEphemeralContent> for RTDUpdateMessageEphemeralContentBuilder {
  fn as_ref(&self) -> &UpdateMessageEphemeralContent {
    &self.inner
  }
}

/// A fact-check added to a message was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageFactCheck {
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
  /// The new fact-check
  fact_check: FactCheck,
}

impl RObject for UpdateMessageFactCheck {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateMessageFactCheck"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateMessageFactCheck {}

impl UpdateMessageFactCheck {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateMessageFactCheckBuilder {
    let inner = UpdateMessageFactCheck {
      td_name: "updateMessageFactCheck".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateMessageFactCheckBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }

  pub fn fact_check(&self) -> &FactCheck {
    &self.fact_check
  }
}

#[doc(hidden)]
pub struct RTDUpdateMessageFactCheckBuilder {
  inner: UpdateMessageFactCheck,
}

impl RTDUpdateMessageFactCheckBuilder {
  pub fn build(&self) -> UpdateMessageFactCheck {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

  pub fn fact_check<T: AsRef<FactCheck>>(&mut self, fact_check: T) -> &mut Self {
    self.inner.fact_check = fact_check.as_ref().clone();
    self
  }
}

impl AsRef<UpdateMessageFactCheck> for UpdateMessageFactCheck {
  fn as_ref(&self) -> &UpdateMessageFactCheck {
    self
  }
}

impl AsRef<UpdateMessageFactCheck> for RTDUpdateMessageFactCheckBuilder {
  fn as_ref(&self) -> &UpdateMessageFactCheck {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateMessageInteractionInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateMessageInteractionInfo {}

impl UpdateMessageInteractionInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateMessageInteractionInfoBuilder {
    let inner = UpdateMessageInteractionInfo {
      td_name: "updateMessageInteractionInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateMessageInteractionInfoBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }

  pub fn interaction_info(&self) -> &Option<MessageInteractionInfo> {
    &self.interaction_info
  }
}

#[doc(hidden)]
pub struct RTDUpdateMessageInteractionInfoBuilder {
  inner: UpdateMessageInteractionInfo,
}

impl RTDUpdateMessageInteractionInfoBuilder {
  pub fn build(&self) -> UpdateMessageInteractionInfo {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

  pub fn interaction_info<T: AsRef<MessageInteractionInfo>>(
    &mut self,
    interaction_info: T,
  ) -> &mut Self {
    self.inner.interaction_info = Some(interaction_info.as_ref().clone());
    self
  }
}

impl AsRef<UpdateMessageInteractionInfo> for UpdateMessageInteractionInfo {
  fn as_ref(&self) -> &UpdateMessageInteractionInfo {
    self
  }
}

impl AsRef<UpdateMessageInteractionInfo> for RTDUpdateMessageInteractionInfoBuilder {
  fn as_ref(&self) -> &UpdateMessageInteractionInfo {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateMessageIsPinned"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateMessageIsPinned {}

impl UpdateMessageIsPinned {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateMessageIsPinnedBuilder {
    let inner = UpdateMessageIsPinned {
      td_name: "updateMessageIsPinned".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateMessageIsPinnedBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }

  pub fn is_pinned(&self) -> bool {
    self.is_pinned
  }
}

#[doc(hidden)]
pub struct RTDUpdateMessageIsPinnedBuilder {
  inner: UpdateMessageIsPinned,
}

impl RTDUpdateMessageIsPinnedBuilder {
  pub fn build(&self) -> UpdateMessageIsPinned {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateMessageIsPinned {
    self
  }
}

impl AsRef<UpdateMessageIsPinned> for RTDUpdateMessageIsPinnedBuilder {
  fn as_ref(&self) -> &UpdateMessageIsPinned {
    &self.inner
  }
}

/// A message with a live location was viewed. When the update is received, the application is expected to update the live location
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateMessageLiveLocationViewed"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateMessageLiveLocationViewed {}

impl UpdateMessageLiveLocationViewed {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateMessageLiveLocationViewedBuilder {
    let inner = UpdateMessageLiveLocationViewed {
      td_name: "updateMessageLiveLocationViewed".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateMessageLiveLocationViewedBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }
}

#[doc(hidden)]
pub struct RTDUpdateMessageLiveLocationViewedBuilder {
  inner: UpdateMessageLiveLocationViewed,
}

impl RTDUpdateMessageLiveLocationViewedBuilder {
  pub fn build(&self) -> UpdateMessageLiveLocationViewed {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateMessageLiveLocationViewed {
    self
  }
}

impl AsRef<UpdateMessageLiveLocationViewed> for RTDUpdateMessageLiveLocationViewedBuilder {
  fn as_ref(&self) -> &UpdateMessageLiveLocationViewed {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateMessageMentionRead"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateMessageMentionRead {}

impl UpdateMessageMentionRead {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateMessageMentionReadBuilder {
    let inner = UpdateMessageMentionRead {
      td_name: "updateMessageMentionRead".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateMessageMentionReadBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }

  pub fn unread_mention_count(&self) -> i64 {
    self.unread_mention_count
  }
}

#[doc(hidden)]
pub struct RTDUpdateMessageMentionReadBuilder {
  inner: UpdateMessageMentionRead,
}

impl RTDUpdateMessageMentionReadBuilder {
  pub fn build(&self) -> UpdateMessageMentionRead {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateMessageMentionRead {
    self
  }
}

impl AsRef<UpdateMessageMentionRead> for RTDUpdateMessageMentionReadBuilder {
  fn as_ref(&self) -> &UpdateMessageMentionRead {
    &self.inner
  }
}

/// User changed its reactions on a message with public reactions; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageReaction {
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
  /// Identifier of the user or chat that changed reactions
  actor_id: MessageSender,
  /// Point in time (Unix timestamp) when the reactions were changed
  date: i64,
  /// Old list of chosen reactions
  old_reaction_types: Vec<ReactionType>,
  /// New list of chosen reactions
  new_reaction_types: Vec<ReactionType>,
}

impl RObject for UpdateMessageReaction {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateMessageReaction"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateMessageReaction {}

impl UpdateMessageReaction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateMessageReactionBuilder {
    let inner = UpdateMessageReaction {
      td_name: "updateMessageReaction".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateMessageReactionBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }

  pub fn actor_id(&self) -> &MessageSender {
    &self.actor_id
  }

  pub fn date(&self) -> i64 {
    self.date
  }

  pub fn old_reaction_types(&self) -> &Vec<ReactionType> {
    &self.old_reaction_types
  }

  pub fn new_reaction_types(&self) -> &Vec<ReactionType> {
    &self.new_reaction_types
  }
}

#[doc(hidden)]
pub struct RTDUpdateMessageReactionBuilder {
  inner: UpdateMessageReaction,
}

impl RTDUpdateMessageReactionBuilder {
  pub fn build(&self) -> UpdateMessageReaction {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

  pub fn actor_id<T: AsRef<MessageSender>>(&mut self, actor_id: T) -> &mut Self {
    self.inner.actor_id = actor_id.as_ref().clone();
    self
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

  pub fn old_reaction_types(&mut self, old_reaction_types: Vec<ReactionType>) -> &mut Self {
    self.inner.old_reaction_types = old_reaction_types;
    self
  }

  pub fn new_reaction_types(&mut self, new_reaction_types: Vec<ReactionType>) -> &mut Self {
    self.inner.new_reaction_types = new_reaction_types;
    self
  }
}

impl AsRef<UpdateMessageReaction> for UpdateMessageReaction {
  fn as_ref(&self) -> &UpdateMessageReaction {
    self
  }
}

impl AsRef<UpdateMessageReaction> for RTDUpdateMessageReactionBuilder {
  fn as_ref(&self) -> &UpdateMessageReaction {
    &self.inner
  }
}

/// Reactions added to a message with anonymous reactions have changed; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageReactions {
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
  /// Point in time (Unix timestamp) when the reactions were changed
  date: i64,
  /// The list of reactions added to the message
  reactions: Vec<MessageReaction>,
}

impl RObject for UpdateMessageReactions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateMessageReactions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateMessageReactions {}

impl UpdateMessageReactions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateMessageReactionsBuilder {
    let inner = UpdateMessageReactions {
      td_name: "updateMessageReactions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateMessageReactionsBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }

  pub fn date(&self) -> i64 {
    self.date
  }

  pub fn reactions(&self) -> &Vec<MessageReaction> {
    &self.reactions
  }
}

#[doc(hidden)]
pub struct RTDUpdateMessageReactionsBuilder {
  inner: UpdateMessageReactions,
}

impl RTDUpdateMessageReactionsBuilder {
  pub fn build(&self) -> UpdateMessageReactions {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

  pub fn reactions(&mut self, reactions: Vec<MessageReaction>) -> &mut Self {
    self.inner.reactions = reactions;
    self
  }
}

impl AsRef<UpdateMessageReactions> for UpdateMessageReactions {
  fn as_ref(&self) -> &UpdateMessageReactions {
    self
  }
}

impl AsRef<UpdateMessageReactions> for RTDUpdateMessageReactionsBuilder {
  fn as_ref(&self) -> &UpdateMessageReactions {
    &self.inner
  }
}

/// A request to send a message has reached the Telegram server. This doesn't mean that the message will be sent successfully. This update is sent only if the option "use_quick_ack" is set to true. This update may be sent multiple times for the same message
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateMessageSendAcknowledged"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateMessageSendAcknowledged {}

impl UpdateMessageSendAcknowledged {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateMessageSendAcknowledgedBuilder {
    let inner = UpdateMessageSendAcknowledged {
      td_name: "updateMessageSendAcknowledged".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateMessageSendAcknowledgedBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }
}

#[doc(hidden)]
pub struct RTDUpdateMessageSendAcknowledgedBuilder {
  inner: UpdateMessageSendAcknowledged,
}

impl RTDUpdateMessageSendAcknowledgedBuilder {
  pub fn build(&self) -> UpdateMessageSendAcknowledged {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateMessageSendAcknowledged {
    self
  }
}

impl AsRef<UpdateMessageSendAcknowledged> for RTDUpdateMessageSendAcknowledgedBuilder {
  fn as_ref(&self) -> &UpdateMessageSendAcknowledged {
    &self.inner
  }
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
  /// The cause of the message sending failure
  error: Error,
}

impl RObject for UpdateMessageSendFailed {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateMessageSendFailed"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateMessageSendFailed {}

impl UpdateMessageSendFailed {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateMessageSendFailedBuilder {
    let inner = UpdateMessageSendFailed {
      td_name: "updateMessageSendFailed".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateMessageSendFailedBuilder { inner }
  }

  pub fn message(&self) -> &Message {
    &self.message
  }

  pub fn old_message_id(&self) -> i64 {
    self.old_message_id
  }

  pub fn error(&self) -> &Error {
    &self.error
  }
}

#[doc(hidden)]
pub struct RTDUpdateMessageSendFailedBuilder {
  inner: UpdateMessageSendFailed,
}

impl RTDUpdateMessageSendFailedBuilder {
  pub fn build(&self) -> UpdateMessageSendFailed {
    self.inner.clone()
  }

  pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().clone();
    self
  }

  pub fn old_message_id(&mut self, old_message_id: i64) -> &mut Self {
    self.inner.old_message_id = old_message_id;
    self
  }

  pub fn error<T: AsRef<Error>>(&mut self, error: T) -> &mut Self {
    self.inner.error = error.as_ref().clone();
    self
  }
}

impl AsRef<UpdateMessageSendFailed> for UpdateMessageSendFailed {
  fn as_ref(&self) -> &UpdateMessageSendFailed {
    self
  }
}

impl AsRef<UpdateMessageSendFailed> for RTDUpdateMessageSendFailedBuilder {
  fn as_ref(&self) -> &UpdateMessageSendFailed {
    &self.inner
  }
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
  /// The sent message. Almost any field of the new message can be different from the corresponding field of the original message. For example, the field scheduling_state may change, making the message scheduled, or non-scheduled
  message: Message,
  /// The previous temporary message identifier
  old_message_id: i64,
}

impl RObject for UpdateMessageSendSucceeded {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateMessageSendSucceeded"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateMessageSendSucceeded {}

impl UpdateMessageSendSucceeded {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateMessageSendSucceededBuilder {
    let inner = UpdateMessageSendSucceeded {
      td_name: "updateMessageSendSucceeded".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateMessageSendSucceededBuilder { inner }
  }

  pub fn message(&self) -> &Message {
    &self.message
  }

  pub fn old_message_id(&self) -> i64 {
    self.old_message_id
  }
}

#[doc(hidden)]
pub struct RTDUpdateMessageSendSucceededBuilder {
  inner: UpdateMessageSendSucceeded,
}

impl RTDUpdateMessageSendSucceededBuilder {
  pub fn build(&self) -> UpdateMessageSendSucceeded {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateMessageSendSucceeded {
    self
  }
}

impl AsRef<UpdateMessageSendSucceeded> for RTDUpdateMessageSendSucceededBuilder {
  fn as_ref(&self) -> &UpdateMessageSendSucceeded {
    &self.inner
  }
}

/// Information about suggested post of a message was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageSuggestedPostInfo {
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
  /// The new information about the suggested post
  suggested_post_info: SuggestedPostInfo,
}

impl RObject for UpdateMessageSuggestedPostInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateMessageSuggestedPostInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateMessageSuggestedPostInfo {}

impl UpdateMessageSuggestedPostInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateMessageSuggestedPostInfoBuilder {
    let inner = UpdateMessageSuggestedPostInfo {
      td_name: "updateMessageSuggestedPostInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateMessageSuggestedPostInfoBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }

  pub fn suggested_post_info(&self) -> &SuggestedPostInfo {
    &self.suggested_post_info
  }
}

#[doc(hidden)]
pub struct RTDUpdateMessageSuggestedPostInfoBuilder {
  inner: UpdateMessageSuggestedPostInfo,
}

impl RTDUpdateMessageSuggestedPostInfoBuilder {
  pub fn build(&self) -> UpdateMessageSuggestedPostInfo {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

  pub fn suggested_post_info<T: AsRef<SuggestedPostInfo>>(
    &mut self,
    suggested_post_info: T,
  ) -> &mut Self {
    self.inner.suggested_post_info = suggested_post_info.as_ref().clone();
    self
  }
}

impl AsRef<UpdateMessageSuggestedPostInfo> for UpdateMessageSuggestedPostInfo {
  fn as_ref(&self) -> &UpdateMessageSuggestedPostInfo {
    self
  }
}

impl AsRef<UpdateMessageSuggestedPostInfo> for RTDUpdateMessageSuggestedPostInfoBuilder {
  fn as_ref(&self) -> &UpdateMessageSuggestedPostInfo {
    &self.inner
  }
}

/// The list of unread reactions added to a message was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageUnreadReactions {
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
  /// The new list of unread reactions
  unread_reactions: Vec<UnreadReaction>,
  /// The new number of messages with unread reactions in the chat
  unread_reaction_count: i64,
}

impl RObject for UpdateMessageUnreadReactions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateMessageUnreadReactions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateMessageUnreadReactions {}

impl UpdateMessageUnreadReactions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateMessageUnreadReactionsBuilder {
    let inner = UpdateMessageUnreadReactions {
      td_name: "updateMessageUnreadReactions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateMessageUnreadReactionsBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }

  pub fn unread_reactions(&self) -> &Vec<UnreadReaction> {
    &self.unread_reactions
  }

  pub fn unread_reaction_count(&self) -> i64 {
    self.unread_reaction_count
  }
}

#[doc(hidden)]
pub struct RTDUpdateMessageUnreadReactionsBuilder {
  inner: UpdateMessageUnreadReactions,
}

impl RTDUpdateMessageUnreadReactionsBuilder {
  pub fn build(&self) -> UpdateMessageUnreadReactions {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

  pub fn unread_reactions(&mut self, unread_reactions: Vec<UnreadReaction>) -> &mut Self {
    self.inner.unread_reactions = unread_reactions;
    self
  }

  pub fn unread_reaction_count(&mut self, unread_reaction_count: i64) -> &mut Self {
    self.inner.unread_reaction_count = unread_reaction_count;
    self
  }
}

impl AsRef<UpdateMessageUnreadReactions> for UpdateMessageUnreadReactions {
  fn as_ref(&self) -> &UpdateMessageUnreadReactions {
    self
  }
}

impl AsRef<UpdateMessageUnreadReactions> for RTDUpdateMessageUnreadReactionsBuilder {
  fn as_ref(&self) -> &UpdateMessageUnreadReactions {
    &self.inner
  }
}

/// A new incoming callback query from a business message; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewBusinessCallbackQuery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique query identifier
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  id: isize,
  /// Identifier of the user who sent the query
  sender_user_id: i64,
  /// Unique identifier of the business connection
  connection_id: String,
  /// The message from the business account from which the query originated
  message: BusinessMessage,
  /// An identifier uniquely corresponding to the chat a message was sent to
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  chat_instance: isize,
  /// Query payload
  payload: CallbackQueryPayload,
}

impl RObject for UpdateNewBusinessCallbackQuery {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateNewBusinessCallbackQuery"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateNewBusinessCallbackQuery {}

impl UpdateNewBusinessCallbackQuery {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateNewBusinessCallbackQueryBuilder {
    let inner = UpdateNewBusinessCallbackQuery {
      td_name: "updateNewBusinessCallbackQuery".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateNewBusinessCallbackQueryBuilder { inner }
  }

  pub fn id(&self) -> isize {
    self.id
  }

  pub fn sender_user_id(&self) -> i64 {
    self.sender_user_id
  }

  pub fn connection_id(&self) -> &String {
    &self.connection_id
  }

  pub fn message(&self) -> &BusinessMessage {
    &self.message
  }

  pub fn chat_instance(&self) -> isize {
    self.chat_instance
  }

  pub fn payload(&self) -> &CallbackQueryPayload {
    &self.payload
  }
}

#[doc(hidden)]
pub struct RTDUpdateNewBusinessCallbackQueryBuilder {
  inner: UpdateNewBusinessCallbackQuery,
}

impl RTDUpdateNewBusinessCallbackQueryBuilder {
  pub fn build(&self) -> UpdateNewBusinessCallbackQuery {
    self.inner.clone()
  }

  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn sender_user_id(&mut self, sender_user_id: i64) -> &mut Self {
    self.inner.sender_user_id = sender_user_id;
    self
  }

  pub fn connection_id<T: AsRef<str>>(&mut self, connection_id: T) -> &mut Self {
    self.inner.connection_id = connection_id.as_ref().to_string();
    self
  }

  pub fn message<T: AsRef<BusinessMessage>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().clone();
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

impl AsRef<UpdateNewBusinessCallbackQuery> for UpdateNewBusinessCallbackQuery {
  fn as_ref(&self) -> &UpdateNewBusinessCallbackQuery {
    self
  }
}

impl AsRef<UpdateNewBusinessCallbackQuery> for RTDUpdateNewBusinessCallbackQueryBuilder {
  fn as_ref(&self) -> &UpdateNewBusinessCallbackQuery {
    &self.inner
  }
}

/// A new message was added to a business account; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewBusinessMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the business connection
  connection_id: String,
  /// The new message
  message: BusinessMessage,
}

impl RObject for UpdateNewBusinessMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateNewBusinessMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateNewBusinessMessage {}

impl UpdateNewBusinessMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateNewBusinessMessageBuilder {
    let inner = UpdateNewBusinessMessage {
      td_name: "updateNewBusinessMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateNewBusinessMessageBuilder { inner }
  }

  pub fn connection_id(&self) -> &String {
    &self.connection_id
  }

  pub fn message(&self) -> &BusinessMessage {
    &self.message
  }
}

#[doc(hidden)]
pub struct RTDUpdateNewBusinessMessageBuilder {
  inner: UpdateNewBusinessMessage,
}

impl RTDUpdateNewBusinessMessageBuilder {
  pub fn build(&self) -> UpdateNewBusinessMessage {
    self.inner.clone()
  }

  pub fn connection_id<T: AsRef<str>>(&mut self, connection_id: T) -> &mut Self {
    self.inner.connection_id = connection_id.as_ref().to_string();
    self
  }

  pub fn message<T: AsRef<BusinessMessage>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().clone();
    self
  }
}

impl AsRef<UpdateNewBusinessMessage> for UpdateNewBusinessMessage {
  fn as_ref(&self) -> &UpdateNewBusinessMessage {
    self
  }
}

impl AsRef<UpdateNewBusinessMessage> for RTDUpdateNewBusinessMessageBuilder {
  fn as_ref(&self) -> &UpdateNewBusinessMessage {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateNewCallSignalingData"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateNewCallSignalingData {}

impl UpdateNewCallSignalingData {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateNewCallSignalingDataBuilder {
    let inner = UpdateNewCallSignalingData {
      td_name: "updateNewCallSignalingData".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateNewCallSignalingDataBuilder { inner }
  }

  pub fn call_id(&self) -> i64 {
    self.call_id
  }

  pub fn data(&self) -> &String {
    &self.data
  }
}

#[doc(hidden)]
pub struct RTDUpdateNewCallSignalingDataBuilder {
  inner: UpdateNewCallSignalingData,
}

impl RTDUpdateNewCallSignalingDataBuilder {
  pub fn build(&self) -> UpdateNewCallSignalingData {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateNewCallSignalingData {
    self
  }
}

impl AsRef<UpdateNewCallSignalingData> for RTDUpdateNewCallSignalingDataBuilder {
  fn as_ref(&self) -> &UpdateNewCallSignalingData {
    &self.inner
  }
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
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  id: isize,
  /// Identifier of the user who sent the query
  sender_user_id: i64,
  /// Identifier of the chat where the query was sent
  chat_id: i64,
  /// Identifier of the message from which the query originated
  message_id: i64,
  /// Identifier that uniquely corresponds to the chat to which the message was sent
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  chat_instance: isize,
  /// Query payload
  payload: CallbackQueryPayload,
}

impl RObject for UpdateNewCallbackQuery {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateNewCallbackQuery"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateNewCallbackQuery {}

impl UpdateNewCallbackQuery {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateNewCallbackQueryBuilder {
    let inner = UpdateNewCallbackQuery {
      td_name: "updateNewCallbackQuery".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateNewCallbackQueryBuilder { inner }
  }

  pub fn id(&self) -> isize {
    self.id
  }

  pub fn sender_user_id(&self) -> i64 {
    self.sender_user_id
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }

  pub fn chat_instance(&self) -> isize {
    self.chat_instance
  }

  pub fn payload(&self) -> &CallbackQueryPayload {
    &self.payload
  }
}

#[doc(hidden)]
pub struct RTDUpdateNewCallbackQueryBuilder {
  inner: UpdateNewCallbackQuery,
}

impl RTDUpdateNewCallbackQueryBuilder {
  pub fn build(&self) -> UpdateNewCallbackQuery {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateNewCallbackQuery {
    self
  }
}

impl AsRef<UpdateNewCallbackQuery> for RTDUpdateNewCallbackQueryBuilder {
  fn as_ref(&self) -> &UpdateNewCallbackQuery {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateNewChat"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateNewChat {}

impl UpdateNewChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateNewChatBuilder {
    let inner = UpdateNewChat {
      td_name: "updateNewChat".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateNewChatBuilder { inner }
  }

  pub fn chat(&self) -> &Chat {
    &self.chat
  }
}

#[doc(hidden)]
pub struct RTDUpdateNewChatBuilder {
  inner: UpdateNewChat,
}

impl RTDUpdateNewChatBuilder {
  pub fn build(&self) -> UpdateNewChat {
    self.inner.clone()
  }

  pub fn chat<T: AsRef<Chat>>(&mut self, chat: T) -> &mut Self {
    self.inner.chat = chat.as_ref().clone();
    self
  }
}

impl AsRef<UpdateNewChat> for UpdateNewChat {
  fn as_ref(&self) -> &UpdateNewChat {
    self
  }
}

impl AsRef<UpdateNewChat> for RTDUpdateNewChatBuilder {
  fn as_ref(&self) -> &UpdateNewChat {
    &self.inner
  }
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
  /// Chat identifier of the private chat with the user
  user_chat_id: i64,
  /// The invite link, which was used to send join request; may be null
  invite_link: Option<ChatInviteLink>,
  /// Identifier of the join request query, which can be used in answerChatJoinRequestQuery; 0 if none
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  query_id: isize,
}

impl RObject for UpdateNewChatJoinRequest {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateNewChatJoinRequest"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateNewChatJoinRequest {}

impl UpdateNewChatJoinRequest {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateNewChatJoinRequestBuilder {
    let inner = UpdateNewChatJoinRequest {
      td_name: "updateNewChatJoinRequest".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateNewChatJoinRequestBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn request(&self) -> &ChatJoinRequest {
    &self.request
  }

  pub fn user_chat_id(&self) -> i64 {
    self.user_chat_id
  }

  pub fn invite_link(&self) -> &Option<ChatInviteLink> {
    &self.invite_link
  }

  pub fn query_id(&self) -> isize {
    self.query_id
  }
}

#[doc(hidden)]
pub struct RTDUpdateNewChatJoinRequestBuilder {
  inner: UpdateNewChatJoinRequest,
}

impl RTDUpdateNewChatJoinRequestBuilder {
  pub fn build(&self) -> UpdateNewChatJoinRequest {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn request<T: AsRef<ChatJoinRequest>>(&mut self, request: T) -> &mut Self {
    self.inner.request = request.as_ref().clone();
    self
  }

  pub fn user_chat_id(&mut self, user_chat_id: i64) -> &mut Self {
    self.inner.user_chat_id = user_chat_id;
    self
  }

  pub fn invite_link<T: AsRef<ChatInviteLink>>(&mut self, invite_link: T) -> &mut Self {
    self.inner.invite_link = Some(invite_link.as_ref().clone());
    self
  }

  pub fn query_id(&mut self, query_id: isize) -> &mut Self {
    self.inner.query_id = query_id;
    self
  }
}

impl AsRef<UpdateNewChatJoinRequest> for UpdateNewChatJoinRequest {
  fn as_ref(&self) -> &UpdateNewChatJoinRequest {
    self
  }
}

impl AsRef<UpdateNewChatJoinRequest> for RTDUpdateNewChatJoinRequestBuilder {
  fn as_ref(&self) -> &UpdateNewChatJoinRequest {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateNewChosenInlineResult"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateNewChosenInlineResult {}

impl UpdateNewChosenInlineResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateNewChosenInlineResultBuilder {
    let inner = UpdateNewChosenInlineResult {
      td_name: "updateNewChosenInlineResult".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateNewChosenInlineResultBuilder { inner }
  }

  pub fn sender_user_id(&self) -> i64 {
    self.sender_user_id
  }

  pub fn user_location(&self) -> &Option<Location> {
    &self.user_location
  }

  pub fn query(&self) -> &String {
    &self.query
  }

  pub fn result_id(&self) -> &String {
    &self.result_id
  }

  pub fn inline_message_id(&self) -> &String {
    &self.inline_message_id
  }
}

#[doc(hidden)]
pub struct RTDUpdateNewChosenInlineResultBuilder {
  inner: UpdateNewChosenInlineResult,
}

impl RTDUpdateNewChosenInlineResultBuilder {
  pub fn build(&self) -> UpdateNewChosenInlineResult {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateNewChosenInlineResult {
    self
  }
}

impl AsRef<UpdateNewChosenInlineResult> for RTDUpdateNewChosenInlineResultBuilder {
  fn as_ref(&self) -> &UpdateNewChosenInlineResult {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateNewCustomEvent"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateNewCustomEvent {}

impl UpdateNewCustomEvent {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateNewCustomEventBuilder {
    let inner = UpdateNewCustomEvent {
      td_name: "updateNewCustomEvent".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateNewCustomEventBuilder { inner }
  }

  pub fn event(&self) -> &String {
    &self.event
  }
}

#[doc(hidden)]
pub struct RTDUpdateNewCustomEventBuilder {
  inner: UpdateNewCustomEvent,
}

impl RTDUpdateNewCustomEventBuilder {
  pub fn build(&self) -> UpdateNewCustomEvent {
    self.inner.clone()
  }

  pub fn event<T: AsRef<str>>(&mut self, event: T) -> &mut Self {
    self.inner.event = event.as_ref().to_string();
    self
  }
}

impl AsRef<UpdateNewCustomEvent> for UpdateNewCustomEvent {
  fn as_ref(&self) -> &UpdateNewCustomEvent {
    self
  }
}

impl AsRef<UpdateNewCustomEvent> for RTDUpdateNewCustomEventBuilder {
  fn as_ref(&self) -> &UpdateNewCustomEvent {
    &self.inner
  }
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
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  id: isize,
  /// JSON-serialized query data
  data: String,
  /// Query timeout
  timeout: i64,
}

impl RObject for UpdateNewCustomQuery {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateNewCustomQuery"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateNewCustomQuery {}

impl UpdateNewCustomQuery {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateNewCustomQueryBuilder {
    let inner = UpdateNewCustomQuery {
      td_name: "updateNewCustomQuery".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateNewCustomQueryBuilder { inner }
  }

  pub fn id(&self) -> isize {
    self.id
  }

  pub fn data(&self) -> &String {
    &self.data
  }

  pub fn timeout(&self) -> i64 {
    self.timeout
  }
}

#[doc(hidden)]
pub struct RTDUpdateNewCustomQueryBuilder {
  inner: UpdateNewCustomQuery,
}

impl RTDUpdateNewCustomQueryBuilder {
  pub fn build(&self) -> UpdateNewCustomQuery {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateNewCustomQuery {
    self
  }
}

impl AsRef<UpdateNewCustomQuery> for RTDUpdateNewCustomQueryBuilder {
  fn as_ref(&self) -> &UpdateNewCustomQuery {
    &self.inner
  }
}

/// A new message was received in a group call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewGroupCallMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the group call
  group_call_id: i64,
  /// The message
  message: GroupCallMessage,
}

impl RObject for UpdateNewGroupCallMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateNewGroupCallMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateNewGroupCallMessage {}

impl UpdateNewGroupCallMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateNewGroupCallMessageBuilder {
    let inner = UpdateNewGroupCallMessage {
      td_name: "updateNewGroupCallMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateNewGroupCallMessageBuilder { inner }
  }

  pub fn group_call_id(&self) -> i64 {
    self.group_call_id
  }

  pub fn message(&self) -> &GroupCallMessage {
    &self.message
  }
}

#[doc(hidden)]
pub struct RTDUpdateNewGroupCallMessageBuilder {
  inner: UpdateNewGroupCallMessage,
}

impl RTDUpdateNewGroupCallMessageBuilder {
  pub fn build(&self) -> UpdateNewGroupCallMessage {
    self.inner.clone()
  }

  pub fn group_call_id(&mut self, group_call_id: i64) -> &mut Self {
    self.inner.group_call_id = group_call_id;
    self
  }

  pub fn message<T: AsRef<GroupCallMessage>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().clone();
    self
  }
}

impl AsRef<UpdateNewGroupCallMessage> for UpdateNewGroupCallMessage {
  fn as_ref(&self) -> &UpdateNewGroupCallMessage {
    self
  }
}

impl AsRef<UpdateNewGroupCallMessage> for RTDUpdateNewGroupCallMessageBuilder {
  fn as_ref(&self) -> &UpdateNewGroupCallMessage {
    &self.inner
  }
}

/// A new paid reaction was received in a live story group call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewGroupCallPaidReaction {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the group call
  group_call_id: i64,
  /// Identifier of the sender of the reaction
  sender_id: MessageSender,
  /// The number of Telegram Stars that were paid to send the reaction
  star_count: i64,
}

impl RObject for UpdateNewGroupCallPaidReaction {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateNewGroupCallPaidReaction"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateNewGroupCallPaidReaction {}

impl UpdateNewGroupCallPaidReaction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateNewGroupCallPaidReactionBuilder {
    let inner = UpdateNewGroupCallPaidReaction {
      td_name: "updateNewGroupCallPaidReaction".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateNewGroupCallPaidReactionBuilder { inner }
  }

  pub fn group_call_id(&self) -> i64 {
    self.group_call_id
  }

  pub fn sender_id(&self) -> &MessageSender {
    &self.sender_id
  }

  pub fn star_count(&self) -> i64 {
    self.star_count
  }
}

#[doc(hidden)]
pub struct RTDUpdateNewGroupCallPaidReactionBuilder {
  inner: UpdateNewGroupCallPaidReaction,
}

impl RTDUpdateNewGroupCallPaidReactionBuilder {
  pub fn build(&self) -> UpdateNewGroupCallPaidReaction {
    self.inner.clone()
  }

  pub fn group_call_id(&mut self, group_call_id: i64) -> &mut Self {
    self.inner.group_call_id = group_call_id;
    self
  }

  pub fn sender_id<T: AsRef<MessageSender>>(&mut self, sender_id: T) -> &mut Self {
    self.inner.sender_id = sender_id.as_ref().clone();
    self
  }

  pub fn star_count(&mut self, star_count: i64) -> &mut Self {
    self.inner.star_count = star_count;
    self
  }
}

impl AsRef<UpdateNewGroupCallPaidReaction> for UpdateNewGroupCallPaidReaction {
  fn as_ref(&self) -> &UpdateNewGroupCallPaidReaction {
    self
  }
}

impl AsRef<UpdateNewGroupCallPaidReaction> for RTDUpdateNewGroupCallPaidReactionBuilder {
  fn as_ref(&self) -> &UpdateNewGroupCallPaidReaction {
    &self.inner
  }
}

/// A new incoming guest query; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewGuestQuery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique query identifier
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  id: isize,
  /// The message with the query
  message: Message,
  /// The list of reference messages
  reference_messages: Vec<Message>,
}

impl RObject for UpdateNewGuestQuery {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateNewGuestQuery"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateNewGuestQuery {}

impl UpdateNewGuestQuery {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateNewGuestQueryBuilder {
    let inner = UpdateNewGuestQuery {
      td_name: "updateNewGuestQuery".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateNewGuestQueryBuilder { inner }
  }

  pub fn id(&self) -> isize {
    self.id
  }

  pub fn message(&self) -> &Message {
    &self.message
  }

  pub fn reference_messages(&self) -> &Vec<Message> {
    &self.reference_messages
  }
}

#[doc(hidden)]
pub struct RTDUpdateNewGuestQueryBuilder {
  inner: UpdateNewGuestQuery,
}

impl RTDUpdateNewGuestQueryBuilder {
  pub fn build(&self) -> UpdateNewGuestQuery {
    self.inner.clone()
  }

  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().clone();
    self
  }

  pub fn reference_messages(&mut self, reference_messages: Vec<Message>) -> &mut Self {
    self.inner.reference_messages = reference_messages;
    self
  }
}

impl AsRef<UpdateNewGuestQuery> for UpdateNewGuestQuery {
  fn as_ref(&self) -> &UpdateNewGuestQuery {
    self
  }
}

impl AsRef<UpdateNewGuestQuery> for RTDUpdateNewGuestQueryBuilder {
  fn as_ref(&self) -> &UpdateNewGuestQuery {
    &self.inner
  }
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
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  id: isize,
  /// Identifier of the user who sent the query
  sender_user_id: i64,
  /// Identifier of the inline message from which the query originated
  inline_message_id: String,
  /// An identifier uniquely corresponding to the chat a message was sent to
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  chat_instance: isize,
  /// Query payload
  payload: CallbackQueryPayload,
}

impl RObject for UpdateNewInlineCallbackQuery {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateNewInlineCallbackQuery"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateNewInlineCallbackQuery {}

impl UpdateNewInlineCallbackQuery {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateNewInlineCallbackQueryBuilder {
    let inner = UpdateNewInlineCallbackQuery {
      td_name: "updateNewInlineCallbackQuery".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateNewInlineCallbackQueryBuilder { inner }
  }

  pub fn id(&self) -> isize {
    self.id
  }

  pub fn sender_user_id(&self) -> i64 {
    self.sender_user_id
  }

  pub fn inline_message_id(&self) -> &String {
    &self.inline_message_id
  }

  pub fn chat_instance(&self) -> isize {
    self.chat_instance
  }

  pub fn payload(&self) -> &CallbackQueryPayload {
    &self.payload
  }
}

#[doc(hidden)]
pub struct RTDUpdateNewInlineCallbackQueryBuilder {
  inner: UpdateNewInlineCallbackQuery,
}

impl RTDUpdateNewInlineCallbackQueryBuilder {
  pub fn build(&self) -> UpdateNewInlineCallbackQuery {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateNewInlineCallbackQuery {
    self
  }
}

impl AsRef<UpdateNewInlineCallbackQuery> for RTDUpdateNewInlineCallbackQueryBuilder {
  fn as_ref(&self) -> &UpdateNewInlineCallbackQuery {
    &self.inner
  }
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
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  id: isize,
  /// Identifier of the user who sent the query
  sender_user_id: i64,
  /// User location; may be null
  user_location: Option<Location>,
  /// The type of the chat from which the query originated; may be null if unknown
  chat_type: Option<ChatType>,
  /// Text of the query
  query: String,
  /// Offset of the first entry to return
  offset: String,
}

impl RObject for UpdateNewInlineQuery {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateNewInlineQuery"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateNewInlineQuery {}

impl UpdateNewInlineQuery {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateNewInlineQueryBuilder {
    let inner = UpdateNewInlineQuery {
      td_name: "updateNewInlineQuery".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateNewInlineQueryBuilder { inner }
  }

  pub fn id(&self) -> isize {
    self.id
  }

  pub fn sender_user_id(&self) -> i64 {
    self.sender_user_id
  }

  pub fn user_location(&self) -> &Option<Location> {
    &self.user_location
  }

  pub fn chat_type(&self) -> &Option<ChatType> {
    &self.chat_type
  }

  pub fn query(&self) -> &String {
    &self.query
  }

  pub fn offset(&self) -> &String {
    &self.offset
  }
}

#[doc(hidden)]
pub struct RTDUpdateNewInlineQueryBuilder {
  inner: UpdateNewInlineQuery,
}

impl RTDUpdateNewInlineQueryBuilder {
  pub fn build(&self) -> UpdateNewInlineQuery {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateNewInlineQuery {
    self
  }
}

impl AsRef<UpdateNewInlineQuery> for RTDUpdateNewInlineQueryBuilder {
  fn as_ref(&self) -> &UpdateNewInlineQuery {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateNewMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateNewMessage {}

impl UpdateNewMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateNewMessageBuilder {
    let inner = UpdateNewMessage {
      td_name: "updateNewMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateNewMessageBuilder { inner }
  }

  pub fn message(&self) -> &Message {
    &self.message
  }
}

#[doc(hidden)]
pub struct RTDUpdateNewMessageBuilder {
  inner: UpdateNewMessage,
}

impl RTDUpdateNewMessageBuilder {
  pub fn build(&self) -> UpdateNewMessage {
    self.inner.clone()
  }

  pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().clone();
    self
  }
}

impl AsRef<UpdateNewMessage> for UpdateNewMessage {
  fn as_ref(&self) -> &UpdateNewMessage {
    self
  }
}

impl AsRef<UpdateNewMessage> for RTDUpdateNewMessageBuilder {
  fn as_ref(&self) -> &UpdateNewMessage {
    &self.inner
  }
}

/// An OAuth authorization request was received
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewOauthRequest {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A domain of the URL where the user authorizes
  domain: String,
  /// Human-readable description of a country and a region from which the authorization is performed, based on the IP address
  location: String,
  /// The URL to pass to getOauthLinkInfo; the link is valid for 60 seconds
  url: String,
}

impl RObject for UpdateNewOauthRequest {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateNewOauthRequest"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateNewOauthRequest {}

impl UpdateNewOauthRequest {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateNewOauthRequestBuilder {
    let inner = UpdateNewOauthRequest {
      td_name: "updateNewOauthRequest".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateNewOauthRequestBuilder { inner }
  }

  pub fn domain(&self) -> &String {
    &self.domain
  }

  pub fn location(&self) -> &String {
    &self.location
  }

  pub fn url(&self) -> &String {
    &self.url
  }
}

#[doc(hidden)]
pub struct RTDUpdateNewOauthRequestBuilder {
  inner: UpdateNewOauthRequest,
}

impl RTDUpdateNewOauthRequestBuilder {
  pub fn build(&self) -> UpdateNewOauthRequest {
    self.inner.clone()
  }

  pub fn domain<T: AsRef<str>>(&mut self, domain: T) -> &mut Self {
    self.inner.domain = domain.as_ref().to_string();
    self
  }

  pub fn location<T: AsRef<str>>(&mut self, location: T) -> &mut Self {
    self.inner.location = location.as_ref().to_string();
    self
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }
}

impl AsRef<UpdateNewOauthRequest> for UpdateNewOauthRequest {
  fn as_ref(&self) -> &UpdateNewOauthRequest {
    self
  }
}

impl AsRef<UpdateNewOauthRequest> for RTDUpdateNewOauthRequestBuilder {
  fn as_ref(&self) -> &UpdateNewOauthRequest {
    &self.inner
  }
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
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateNewPreCheckoutQuery"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateNewPreCheckoutQuery {}

impl UpdateNewPreCheckoutQuery {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateNewPreCheckoutQueryBuilder {
    let inner = UpdateNewPreCheckoutQuery {
      td_name: "updateNewPreCheckoutQuery".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateNewPreCheckoutQueryBuilder { inner }
  }

  pub fn id(&self) -> isize {
    self.id
  }

  pub fn sender_user_id(&self) -> i64 {
    self.sender_user_id
  }

  pub fn currency(&self) -> &String {
    &self.currency
  }

  pub fn total_amount(&self) -> i64 {
    self.total_amount
  }

  pub fn invoice_payload(&self) -> &String {
    &self.invoice_payload
  }

  pub fn shipping_option_id(&self) -> &String {
    &self.shipping_option_id
  }

  pub fn order_info(&self) -> &Option<OrderInfo> {
    &self.order_info
  }
}

#[doc(hidden)]
pub struct RTDUpdateNewPreCheckoutQueryBuilder {
  inner: UpdateNewPreCheckoutQuery,
}

impl RTDUpdateNewPreCheckoutQueryBuilder {
  pub fn build(&self) -> UpdateNewPreCheckoutQuery {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateNewPreCheckoutQuery {
    self
  }
}

impl AsRef<UpdateNewPreCheckoutQuery> for RTDUpdateNewPreCheckoutQueryBuilder {
  fn as_ref(&self) -> &UpdateNewPreCheckoutQuery {
    &self.inner
  }
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
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  id: isize,
  /// Identifier of the user who sent the query
  sender_user_id: i64,
  /// Invoice payload
  invoice_payload: String,
  /// User shipping address
  shipping_address: Address,
}

impl RObject for UpdateNewShippingQuery {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateNewShippingQuery"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateNewShippingQuery {}

impl UpdateNewShippingQuery {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateNewShippingQueryBuilder {
    let inner = UpdateNewShippingQuery {
      td_name: "updateNewShippingQuery".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateNewShippingQueryBuilder { inner }
  }

  pub fn id(&self) -> isize {
    self.id
  }

  pub fn sender_user_id(&self) -> i64 {
    self.sender_user_id
  }

  pub fn invoice_payload(&self) -> &String {
    &self.invoice_payload
  }

  pub fn shipping_address(&self) -> &Address {
    &self.shipping_address
  }
}

#[doc(hidden)]
pub struct RTDUpdateNewShippingQueryBuilder {
  inner: UpdateNewShippingQuery,
}

impl RTDUpdateNewShippingQueryBuilder {
  pub fn build(&self) -> UpdateNewShippingQuery {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateNewShippingQuery {
    self
  }
}

impl AsRef<UpdateNewShippingQuery> for RTDUpdateNewShippingQueryBuilder {
  fn as_ref(&self) -> &UpdateNewShippingQuery {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateNotification"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateNotification {}

impl UpdateNotification {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateNotificationBuilder {
    let inner = UpdateNotification {
      td_name: "updateNotification".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateNotificationBuilder { inner }
  }

  pub fn notification_group_id(&self) -> i64 {
    self.notification_group_id
  }

  pub fn notification(&self) -> &Notification {
    &self.notification
  }
}

#[doc(hidden)]
pub struct RTDUpdateNotificationBuilder {
  inner: UpdateNotification,
}

impl RTDUpdateNotificationBuilder {
  pub fn build(&self) -> UpdateNotification {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateNotification {
    self
  }
}

impl AsRef<UpdateNotification> for RTDUpdateNotificationBuilder {
  fn as_ref(&self) -> &UpdateNotification {
    &self.inner
  }
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
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: NotificationGroupType,
  /// Identifier of a chat to which all notifications in the group belong
  chat_id: i64,
  /// Chat identifier, which notification settings must be applied to the added notifications
  notification_settings_chat_id: i64,
  /// Identifier of the notification sound to be played; 0 if sound is disabled
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  notification_sound_id: isize,
  /// Total number of unread notifications in the group, can be bigger than number of active notifications
  total_count: i64,
  /// List of added group notifications, sorted by notification identifier
  added_notifications: Vec<Notification>,
  /// Identifiers of removed group notifications, sorted by notification identifier
  removed_notification_ids: Vec<i64>,
}

impl RObject for UpdateNotificationGroup {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateNotificationGroup"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateNotificationGroup {}

impl UpdateNotificationGroup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateNotificationGroupBuilder {
    let inner = UpdateNotificationGroup {
      td_name: "updateNotificationGroup".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateNotificationGroupBuilder { inner }
  }

  pub fn notification_group_id(&self) -> i64 {
    self.notification_group_id
  }

  pub fn type_(&self) -> &NotificationGroupType {
    &self.type_
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn notification_settings_chat_id(&self) -> i64 {
    self.notification_settings_chat_id
  }

  pub fn notification_sound_id(&self) -> isize {
    self.notification_sound_id
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }

  pub fn added_notifications(&self) -> &Vec<Notification> {
    &self.added_notifications
  }

  pub fn removed_notification_ids(&self) -> &Vec<i64> {
    &self.removed_notification_ids
  }
}

#[doc(hidden)]
pub struct RTDUpdateNotificationGroupBuilder {
  inner: UpdateNotificationGroup,
}

impl RTDUpdateNotificationGroupBuilder {
  pub fn build(&self) -> UpdateNotificationGroup {
    self.inner.clone()
  }

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

  pub fn notification_sound_id(&mut self, notification_sound_id: isize) -> &mut Self {
    self.inner.notification_sound_id = notification_sound_id;
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
  fn as_ref(&self) -> &UpdateNotificationGroup {
    self
  }
}

impl AsRef<UpdateNotificationGroup> for RTDUpdateNotificationGroupBuilder {
  fn as_ref(&self) -> &UpdateNotificationGroup {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateOption"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateOption {}

impl UpdateOption {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateOptionBuilder {
    let inner = UpdateOption {
      td_name: "updateOption".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateOptionBuilder { inner }
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn value(&self) -> &OptionValue {
    &self.value
  }
}

#[doc(hidden)]
pub struct RTDUpdateOptionBuilder {
  inner: UpdateOption,
}

impl RTDUpdateOptionBuilder {
  pub fn build(&self) -> UpdateOption {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateOption {
    self
  }
}

impl AsRef<UpdateOption> for RTDUpdateOptionBuilder {
  fn as_ref(&self) -> &UpdateOption {
    &self.inner
  }
}

/// The number of TON Grams owned by the current user has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateOwnedGramCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new amount of owned Grams; in the smallest units of the cryptocurrency
  gram_amount: i64,
}

impl RObject for UpdateOwnedGramCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateOwnedGramCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateOwnedGramCount {}

impl UpdateOwnedGramCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateOwnedGramCountBuilder {
    let inner = UpdateOwnedGramCount {
      td_name: "updateOwnedGramCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateOwnedGramCountBuilder { inner }
  }

  pub fn gram_amount(&self) -> i64 {
    self.gram_amount
  }
}

#[doc(hidden)]
pub struct RTDUpdateOwnedGramCountBuilder {
  inner: UpdateOwnedGramCount,
}

impl RTDUpdateOwnedGramCountBuilder {
  pub fn build(&self) -> UpdateOwnedGramCount {
    self.inner.clone()
  }

  pub fn gram_amount(&mut self, gram_amount: i64) -> &mut Self {
    self.inner.gram_amount = gram_amount;
    self
  }
}

impl AsRef<UpdateOwnedGramCount> for UpdateOwnedGramCount {
  fn as_ref(&self) -> &UpdateOwnedGramCount {
    self
  }
}

impl AsRef<UpdateOwnedGramCount> for RTDUpdateOwnedGramCountBuilder {
  fn as_ref(&self) -> &UpdateOwnedGramCount {
    &self.inner
  }
}

/// The number of Telegram Stars owned by the current user has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateOwnedStarCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new amount of owned Telegram Stars
  star_amount: StarAmount,
}

impl RObject for UpdateOwnedStarCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateOwnedStarCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateOwnedStarCount {}

impl UpdateOwnedStarCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateOwnedStarCountBuilder {
    let inner = UpdateOwnedStarCount {
      td_name: "updateOwnedStarCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateOwnedStarCountBuilder { inner }
  }

  pub fn star_amount(&self) -> &StarAmount {
    &self.star_amount
  }
}

#[doc(hidden)]
pub struct RTDUpdateOwnedStarCountBuilder {
  inner: UpdateOwnedStarCount,
}

impl RTDUpdateOwnedStarCountBuilder {
  pub fn build(&self) -> UpdateOwnedStarCount {
    self.inner.clone()
  }

  pub fn star_amount<T: AsRef<StarAmount>>(&mut self, star_amount: T) -> &mut Self {
    self.inner.star_amount = star_amount.as_ref().clone();
    self
  }
}

impl AsRef<UpdateOwnedStarCount> for UpdateOwnedStarCount {
  fn as_ref(&self) -> &UpdateOwnedStarCount {
    self
  }
}

impl AsRef<UpdateOwnedStarCount> for RTDUpdateOwnedStarCountBuilder {
  fn as_ref(&self) -> &UpdateOwnedStarCount {
    &self.inner
  }
}

/// Paid media were purchased by a user; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdatePaidMediaPurchased {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier
  user_id: i64,
  /// Bot-specified payload for the paid media
  payload: String,
}

impl RObject for UpdatePaidMediaPurchased {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updatePaidMediaPurchased"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdatePaidMediaPurchased {}

impl UpdatePaidMediaPurchased {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdatePaidMediaPurchasedBuilder {
    let inner = UpdatePaidMediaPurchased {
      td_name: "updatePaidMediaPurchased".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdatePaidMediaPurchasedBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn payload(&self) -> &String {
    &self.payload
  }
}

#[doc(hidden)]
pub struct RTDUpdatePaidMediaPurchasedBuilder {
  inner: UpdatePaidMediaPurchased,
}

impl RTDUpdatePaidMediaPurchasedBuilder {
  pub fn build(&self) -> UpdatePaidMediaPurchased {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn payload<T: AsRef<str>>(&mut self, payload: T) -> &mut Self {
    self.inner.payload = payload.as_ref().to_string();
    self
  }
}

impl AsRef<UpdatePaidMediaPurchased> for UpdatePaidMediaPurchased {
  fn as_ref(&self) -> &UpdatePaidMediaPurchased {
    self
  }
}

impl AsRef<UpdatePaidMediaPurchased> for RTDUpdatePaidMediaPurchasedBuilder {
  fn as_ref(&self) -> &UpdatePaidMediaPurchased {
    &self.inner
  }
}

/// A new pending text or rich message was received in a chat with a bot. The message must be shown in the chat for at most getOption("pending_text_message_period") seconds, replace any other pending message with the same draft_id with animation, and be deleted whenever any incoming message or a pending message with another draft_id is received in the message thread
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdatePendingMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The forum topic identifier in which the message will be sent; 0 if none
  forum_topic_id: i64,
  /// Unique identifier of the message draft within the message thread
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  draft_id: isize,
  /// True, if a button that calls stopPendingMessage to stop further message generation must be shown
  can_stop: bool,
  /// True, if the pending message must not be automatically deleted when the user presses the Stop button
  keep_on_stop: bool,
  /// Content of the message; always of the type messageText or messageRichMessage
  content: MessageContent,
}

impl RObject for UpdatePendingMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updatePendingMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdatePendingMessage {}

impl UpdatePendingMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdatePendingMessageBuilder {
    let inner = UpdatePendingMessage {
      td_name: "updatePendingMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdatePendingMessageBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn forum_topic_id(&self) -> i64 {
    self.forum_topic_id
  }

  pub fn draft_id(&self) -> isize {
    self.draft_id
  }

  pub fn can_stop(&self) -> bool {
    self.can_stop
  }

  pub fn keep_on_stop(&self) -> bool {
    self.keep_on_stop
  }

  pub fn content(&self) -> &MessageContent {
    &self.content
  }
}

#[doc(hidden)]
pub struct RTDUpdatePendingMessageBuilder {
  inner: UpdatePendingMessage,
}

impl RTDUpdatePendingMessageBuilder {
  pub fn build(&self) -> UpdatePendingMessage {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn forum_topic_id(&mut self, forum_topic_id: i64) -> &mut Self {
    self.inner.forum_topic_id = forum_topic_id;
    self
  }

  pub fn draft_id(&mut self, draft_id: isize) -> &mut Self {
    self.inner.draft_id = draft_id;
    self
  }

  pub fn can_stop(&mut self, can_stop: bool) -> &mut Self {
    self.inner.can_stop = can_stop;
    self
  }

  pub fn keep_on_stop(&mut self, keep_on_stop: bool) -> &mut Self {
    self.inner.keep_on_stop = keep_on_stop;
    self
  }

  pub fn content<T: AsRef<MessageContent>>(&mut self, content: T) -> &mut Self {
    self.inner.content = content.as_ref().clone();
    self
  }
}

impl AsRef<UpdatePendingMessage> for UpdatePendingMessage {
  fn as_ref(&self) -> &UpdatePendingMessage {
    self
  }
}

impl AsRef<UpdatePendingMessage> for RTDUpdatePendingMessageBuilder {
  fn as_ref(&self) -> &UpdatePendingMessage {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updatePoll"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdatePoll {}

impl UpdatePoll {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdatePollBuilder {
    let inner = UpdatePoll {
      td_name: "updatePoll".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdatePollBuilder { inner }
  }

  pub fn poll(&self) -> &Poll {
    &self.poll
  }
}

#[doc(hidden)]
pub struct RTDUpdatePollBuilder {
  inner: UpdatePoll,
}

impl RTDUpdatePollBuilder {
  pub fn build(&self) -> UpdatePoll {
    self.inner.clone()
  }

  pub fn poll<T: AsRef<Poll>>(&mut self, poll: T) -> &mut Self {
    self.inner.poll = poll.as_ref().clone();
    self
  }
}

impl AsRef<UpdatePoll> for UpdatePoll {
  fn as_ref(&self) -> &UpdatePoll {
    self
  }
}

impl AsRef<UpdatePoll> for RTDUpdatePollBuilder {
  fn as_ref(&self) -> &UpdatePoll {
    &self.inner
  }
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
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  poll_id: isize,
  /// Identifier of the message sender that changed the answer to the poll
  voter_id: MessageSender,
  /// Unique identifiers of answer options, that were chosen by the user
  option_ids: Vec<String>,
  /// 0-based identifiers of answer options, that were chosen by the user
  option_positions: Vec<i64>,
}

impl RObject for UpdatePollAnswer {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updatePollAnswer"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdatePollAnswer {}

impl UpdatePollAnswer {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdatePollAnswerBuilder {
    let inner = UpdatePollAnswer {
      td_name: "updatePollAnswer".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdatePollAnswerBuilder { inner }
  }

  pub fn poll_id(&self) -> isize {
    self.poll_id
  }

  pub fn voter_id(&self) -> &MessageSender {
    &self.voter_id
  }

  pub fn option_ids(&self) -> &Vec<String> {
    &self.option_ids
  }

  pub fn option_positions(&self) -> &Vec<i64> {
    &self.option_positions
  }
}

#[doc(hidden)]
pub struct RTDUpdatePollAnswerBuilder {
  inner: UpdatePollAnswer,
}

impl RTDUpdatePollAnswerBuilder {
  pub fn build(&self) -> UpdatePollAnswer {
    self.inner.clone()
  }

  pub fn poll_id(&mut self, poll_id: isize) -> &mut Self {
    self.inner.poll_id = poll_id;
    self
  }

  pub fn voter_id<T: AsRef<MessageSender>>(&mut self, voter_id: T) -> &mut Self {
    self.inner.voter_id = voter_id.as_ref().clone();
    self
  }

  pub fn option_ids(&mut self, option_ids: Vec<String>) -> &mut Self {
    self.inner.option_ids = option_ids;
    self
  }

  pub fn option_positions(&mut self, option_positions: Vec<i64>) -> &mut Self {
    self.inner.option_positions = option_positions;
    self
  }
}

impl AsRef<UpdatePollAnswer> for UpdatePollAnswer {
  fn as_ref(&self) -> &UpdatePollAnswer {
    self
  }
}

impl AsRef<UpdatePollAnswer> for RTDUpdatePollAnswerBuilder {
  fn as_ref(&self) -> &UpdatePollAnswer {
    &self.inner
  }
}

/// The list of supported accent colors for user profiles has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateProfileAccentColors {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Information about supported colors
  colors: Vec<ProfileAccentColor>,
  /// The list of accent color identifiers, which can be set through setProfileAccentColor and setChatProfileAccentColor. The colors must be shown in the specified order
  available_accent_color_ids: Vec<i64>,
}

impl RObject for UpdateProfileAccentColors {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateProfileAccentColors"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateProfileAccentColors {}

impl UpdateProfileAccentColors {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateProfileAccentColorsBuilder {
    let inner = UpdateProfileAccentColors {
      td_name: "updateProfileAccentColors".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateProfileAccentColorsBuilder { inner }
  }

  pub fn colors(&self) -> &Vec<ProfileAccentColor> {
    &self.colors
  }

  pub fn available_accent_color_ids(&self) -> &Vec<i64> {
    &self.available_accent_color_ids
  }
}

#[doc(hidden)]
pub struct RTDUpdateProfileAccentColorsBuilder {
  inner: UpdateProfileAccentColors,
}

impl RTDUpdateProfileAccentColorsBuilder {
  pub fn build(&self) -> UpdateProfileAccentColors {
    self.inner.clone()
  }

  pub fn colors(&mut self, colors: Vec<ProfileAccentColor>) -> &mut Self {
    self.inner.colors = colors;
    self
  }

  pub fn available_accent_color_ids(&mut self, available_accent_color_ids: Vec<i64>) -> &mut Self {
    self.inner.available_accent_color_ids = available_accent_color_ids;
    self
  }
}

impl AsRef<UpdateProfileAccentColors> for UpdateProfileAccentColors {
  fn as_ref(&self) -> &UpdateProfileAccentColors {
    self
  }
}

impl AsRef<UpdateProfileAccentColors> for RTDUpdateProfileAccentColorsBuilder {
  fn as_ref(&self) -> &UpdateProfileAccentColors {
    &self.inner
  }
}

/// Basic information about a quick reply shortcut has changed. This update is guaranteed to come before the quick shortcut name is returned to the application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateQuickReplyShortcut {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New data about the shortcut
  shortcut: QuickReplyShortcut,
}

impl RObject for UpdateQuickReplyShortcut {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateQuickReplyShortcut"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateQuickReplyShortcut {}

impl UpdateQuickReplyShortcut {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateQuickReplyShortcutBuilder {
    let inner = UpdateQuickReplyShortcut {
      td_name: "updateQuickReplyShortcut".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateQuickReplyShortcutBuilder { inner }
  }

  pub fn shortcut(&self) -> &QuickReplyShortcut {
    &self.shortcut
  }
}

#[doc(hidden)]
pub struct RTDUpdateQuickReplyShortcutBuilder {
  inner: UpdateQuickReplyShortcut,
}

impl RTDUpdateQuickReplyShortcutBuilder {
  pub fn build(&self) -> UpdateQuickReplyShortcut {
    self.inner.clone()
  }

  pub fn shortcut<T: AsRef<QuickReplyShortcut>>(&mut self, shortcut: T) -> &mut Self {
    self.inner.shortcut = shortcut.as_ref().clone();
    self
  }
}

impl AsRef<UpdateQuickReplyShortcut> for UpdateQuickReplyShortcut {
  fn as_ref(&self) -> &UpdateQuickReplyShortcut {
    self
  }
}

impl AsRef<UpdateQuickReplyShortcut> for RTDUpdateQuickReplyShortcutBuilder {
  fn as_ref(&self) -> &UpdateQuickReplyShortcut {
    &self.inner
  }
}

/// A quick reply shortcut and all its messages were deleted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateQuickReplyShortcutDeleted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The identifier of the deleted shortcut
  shortcut_id: i64,
}

impl RObject for UpdateQuickReplyShortcutDeleted {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateQuickReplyShortcutDeleted"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateQuickReplyShortcutDeleted {}

impl UpdateQuickReplyShortcutDeleted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateQuickReplyShortcutDeletedBuilder {
    let inner = UpdateQuickReplyShortcutDeleted {
      td_name: "updateQuickReplyShortcutDeleted".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateQuickReplyShortcutDeletedBuilder { inner }
  }

  pub fn shortcut_id(&self) -> i64 {
    self.shortcut_id
  }
}

#[doc(hidden)]
pub struct RTDUpdateQuickReplyShortcutDeletedBuilder {
  inner: UpdateQuickReplyShortcutDeleted,
}

impl RTDUpdateQuickReplyShortcutDeletedBuilder {
  pub fn build(&self) -> UpdateQuickReplyShortcutDeleted {
    self.inner.clone()
  }

  pub fn shortcut_id(&mut self, shortcut_id: i64) -> &mut Self {
    self.inner.shortcut_id = shortcut_id;
    self
  }
}

impl AsRef<UpdateQuickReplyShortcutDeleted> for UpdateQuickReplyShortcutDeleted {
  fn as_ref(&self) -> &UpdateQuickReplyShortcutDeleted {
    self
  }
}

impl AsRef<UpdateQuickReplyShortcutDeleted> for RTDUpdateQuickReplyShortcutDeletedBuilder {
  fn as_ref(&self) -> &UpdateQuickReplyShortcutDeleted {
    &self.inner
  }
}

/// The list of quick reply shortcut messages has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateQuickReplyShortcutMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The identifier of the shortcut
  shortcut_id: i64,
  /// The new list of quick reply messages for the shortcut in order from the first to the last sent
  messages: Vec<QuickReplyMessage>,
}

impl RObject for UpdateQuickReplyShortcutMessages {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateQuickReplyShortcutMessages"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateQuickReplyShortcutMessages {}

impl UpdateQuickReplyShortcutMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateQuickReplyShortcutMessagesBuilder {
    let inner = UpdateQuickReplyShortcutMessages {
      td_name: "updateQuickReplyShortcutMessages".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateQuickReplyShortcutMessagesBuilder { inner }
  }

  pub fn shortcut_id(&self) -> i64 {
    self.shortcut_id
  }

  pub fn messages(&self) -> &Vec<QuickReplyMessage> {
    &self.messages
  }
}

#[doc(hidden)]
pub struct RTDUpdateQuickReplyShortcutMessagesBuilder {
  inner: UpdateQuickReplyShortcutMessages,
}

impl RTDUpdateQuickReplyShortcutMessagesBuilder {
  pub fn build(&self) -> UpdateQuickReplyShortcutMessages {
    self.inner.clone()
  }

  pub fn shortcut_id(&mut self, shortcut_id: i64) -> &mut Self {
    self.inner.shortcut_id = shortcut_id;
    self
  }

  pub fn messages(&mut self, messages: Vec<QuickReplyMessage>) -> &mut Self {
    self.inner.messages = messages;
    self
  }
}

impl AsRef<UpdateQuickReplyShortcutMessages> for UpdateQuickReplyShortcutMessages {
  fn as_ref(&self) -> &UpdateQuickReplyShortcutMessages {
    self
  }
}

impl AsRef<UpdateQuickReplyShortcutMessages> for RTDUpdateQuickReplyShortcutMessagesBuilder {
  fn as_ref(&self) -> &UpdateQuickReplyShortcutMessages {
    &self.inner
  }
}

/// The list of quick reply shortcuts has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateQuickReplyShortcuts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new list of identifiers of quick reply shortcuts
  shortcut_ids: Vec<i64>,
}

impl RObject for UpdateQuickReplyShortcuts {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateQuickReplyShortcuts"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateQuickReplyShortcuts {}

impl UpdateQuickReplyShortcuts {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateQuickReplyShortcutsBuilder {
    let inner = UpdateQuickReplyShortcuts {
      td_name: "updateQuickReplyShortcuts".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateQuickReplyShortcutsBuilder { inner }
  }

  pub fn shortcut_ids(&self) -> &Vec<i64> {
    &self.shortcut_ids
  }
}

#[doc(hidden)]
pub struct RTDUpdateQuickReplyShortcutsBuilder {
  inner: UpdateQuickReplyShortcuts,
}

impl RTDUpdateQuickReplyShortcutsBuilder {
  pub fn build(&self) -> UpdateQuickReplyShortcuts {
    self.inner.clone()
  }

  pub fn shortcut_ids(&mut self, shortcut_ids: Vec<i64>) -> &mut Self {
    self.inner.shortcut_ids = shortcut_ids;
    self
  }
}

impl AsRef<UpdateQuickReplyShortcuts> for UpdateQuickReplyShortcuts {
  fn as_ref(&self) -> &UpdateQuickReplyShortcuts {
    self
  }
}

impl AsRef<UpdateQuickReplyShortcuts> for RTDUpdateQuickReplyShortcutsBuilder {
  fn as_ref(&self) -> &UpdateQuickReplyShortcuts {
    &self.inner
  }
}

/// Notification settings for reactions were updated
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateReactionNotificationSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new notification settings
  notification_settings: ReactionNotificationSettings,
}

impl RObject for UpdateReactionNotificationSettings {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateReactionNotificationSettings"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateReactionNotificationSettings {}

impl UpdateReactionNotificationSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateReactionNotificationSettingsBuilder {
    let inner = UpdateReactionNotificationSettings {
      td_name: "updateReactionNotificationSettings".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateReactionNotificationSettingsBuilder { inner }
  }

  pub fn notification_settings(&self) -> &ReactionNotificationSettings {
    &self.notification_settings
  }
}

#[doc(hidden)]
pub struct RTDUpdateReactionNotificationSettingsBuilder {
  inner: UpdateReactionNotificationSettings,
}

impl RTDUpdateReactionNotificationSettingsBuilder {
  pub fn build(&self) -> UpdateReactionNotificationSettings {
    self.inner.clone()
  }

  pub fn notification_settings<T: AsRef<ReactionNotificationSettings>>(
    &mut self,
    notification_settings: T,
  ) -> &mut Self {
    self.inner.notification_settings = notification_settings.as_ref().clone();
    self
  }
}

impl AsRef<UpdateReactionNotificationSettings> for UpdateReactionNotificationSettings {
  fn as_ref(&self) -> &UpdateReactionNotificationSettings {
    self
  }
}

impl AsRef<UpdateReactionNotificationSettings> for RTDUpdateReactionNotificationSettingsBuilder {
  fn as_ref(&self) -> &UpdateReactionNotificationSettings {
    &self.inner
  }
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
  /// True, if the list of stickers attached to photo or video files was updated; otherwise, the list of sent stickers is updated
  is_attached: bool,
  /// The new list of file identifiers of recently used stickers
  sticker_ids: Vec<i64>,
}

impl RObject for UpdateRecentStickers {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateRecentStickers"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateRecentStickers {}

impl UpdateRecentStickers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateRecentStickersBuilder {
    let inner = UpdateRecentStickers {
      td_name: "updateRecentStickers".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateRecentStickersBuilder { inner }
  }

  pub fn is_attached(&self) -> bool {
    self.is_attached
  }

  pub fn sticker_ids(&self) -> &Vec<i64> {
    &self.sticker_ids
  }
}

#[doc(hidden)]
pub struct RTDUpdateRecentStickersBuilder {
  inner: UpdateRecentStickers,
}

impl RTDUpdateRecentStickersBuilder {
  pub fn build(&self) -> UpdateRecentStickers {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateRecentStickers {
    self
  }
}

impl AsRef<UpdateRecentStickers> for RTDUpdateRecentStickersBuilder {
  fn as_ref(&self) -> &UpdateRecentStickers {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateSavedAnimations"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateSavedAnimations {}

impl UpdateSavedAnimations {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateSavedAnimationsBuilder {
    let inner = UpdateSavedAnimations {
      td_name: "updateSavedAnimations".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateSavedAnimationsBuilder { inner }
  }

  pub fn animation_ids(&self) -> &Vec<i64> {
    &self.animation_ids
  }
}

#[doc(hidden)]
pub struct RTDUpdateSavedAnimationsBuilder {
  inner: UpdateSavedAnimations,
}

impl RTDUpdateSavedAnimationsBuilder {
  pub fn build(&self) -> UpdateSavedAnimations {
    self.inner.clone()
  }

  pub fn animation_ids(&mut self, animation_ids: Vec<i64>) -> &mut Self {
    self.inner.animation_ids = animation_ids;
    self
  }
}

impl AsRef<UpdateSavedAnimations> for UpdateSavedAnimations {
  fn as_ref(&self) -> &UpdateSavedAnimations {
    self
  }
}

impl AsRef<UpdateSavedAnimations> for RTDUpdateSavedAnimationsBuilder {
  fn as_ref(&self) -> &UpdateSavedAnimations {
    &self.inner
  }
}

/// Tags used in Saved Messages or a Saved Messages topic have changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSavedMessagesTags {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of Saved Messages topic which tags were changed; 0 if tags for the whole chat have changed
  saved_messages_topic_id: i64,
  /// The new tags
  tags: SavedMessagesTags,
}

impl RObject for UpdateSavedMessagesTags {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateSavedMessagesTags"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateSavedMessagesTags {}

impl UpdateSavedMessagesTags {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateSavedMessagesTagsBuilder {
    let inner = UpdateSavedMessagesTags {
      td_name: "updateSavedMessagesTags".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateSavedMessagesTagsBuilder { inner }
  }

  pub fn saved_messages_topic_id(&self) -> i64 {
    self.saved_messages_topic_id
  }

  pub fn tags(&self) -> &SavedMessagesTags {
    &self.tags
  }
}

#[doc(hidden)]
pub struct RTDUpdateSavedMessagesTagsBuilder {
  inner: UpdateSavedMessagesTags,
}

impl RTDUpdateSavedMessagesTagsBuilder {
  pub fn build(&self) -> UpdateSavedMessagesTags {
    self.inner.clone()
  }

  pub fn saved_messages_topic_id(&mut self, saved_messages_topic_id: i64) -> &mut Self {
    self.inner.saved_messages_topic_id = saved_messages_topic_id;
    self
  }

  pub fn tags<T: AsRef<SavedMessagesTags>>(&mut self, tags: T) -> &mut Self {
    self.inner.tags = tags.as_ref().clone();
    self
  }
}

impl AsRef<UpdateSavedMessagesTags> for UpdateSavedMessagesTags {
  fn as_ref(&self) -> &UpdateSavedMessagesTags {
    self
  }
}

impl AsRef<UpdateSavedMessagesTags> for RTDUpdateSavedMessagesTagsBuilder {
  fn as_ref(&self) -> &UpdateSavedMessagesTags {
    &self.inner
  }
}

/// Basic information about a Saved Messages topic has changed. This update is guaranteed to come before the topic identifier is returned to the application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSavedMessagesTopic {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New data about the topic
  topic: SavedMessagesTopic,
}

impl RObject for UpdateSavedMessagesTopic {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateSavedMessagesTopic"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateSavedMessagesTopic {}

impl UpdateSavedMessagesTopic {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateSavedMessagesTopicBuilder {
    let inner = UpdateSavedMessagesTopic {
      td_name: "updateSavedMessagesTopic".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateSavedMessagesTopicBuilder { inner }
  }

  pub fn topic(&self) -> &SavedMessagesTopic {
    &self.topic
  }
}

#[doc(hidden)]
pub struct RTDUpdateSavedMessagesTopicBuilder {
  inner: UpdateSavedMessagesTopic,
}

impl RTDUpdateSavedMessagesTopicBuilder {
  pub fn build(&self) -> UpdateSavedMessagesTopic {
    self.inner.clone()
  }

  pub fn topic<T: AsRef<SavedMessagesTopic>>(&mut self, topic: T) -> &mut Self {
    self.inner.topic = topic.as_ref().clone();
    self
  }
}

impl AsRef<UpdateSavedMessagesTopic> for UpdateSavedMessagesTopic {
  fn as_ref(&self) -> &UpdateSavedMessagesTopic {
    self
  }
}

impl AsRef<UpdateSavedMessagesTopic> for RTDUpdateSavedMessagesTopicBuilder {
  fn as_ref(&self) -> &UpdateSavedMessagesTopic {
    &self.inner
  }
}

/// Number of Saved Messages topics has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSavedMessagesTopicCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Approximate total number of Saved Messages topics
  topic_count: i64,
}

impl RObject for UpdateSavedMessagesTopicCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateSavedMessagesTopicCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateSavedMessagesTopicCount {}

impl UpdateSavedMessagesTopicCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateSavedMessagesTopicCountBuilder {
    let inner = UpdateSavedMessagesTopicCount {
      td_name: "updateSavedMessagesTopicCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateSavedMessagesTopicCountBuilder { inner }
  }

  pub fn topic_count(&self) -> i64 {
    self.topic_count
  }
}

#[doc(hidden)]
pub struct RTDUpdateSavedMessagesTopicCountBuilder {
  inner: UpdateSavedMessagesTopicCount,
}

impl RTDUpdateSavedMessagesTopicCountBuilder {
  pub fn build(&self) -> UpdateSavedMessagesTopicCount {
    self.inner.clone()
  }

  pub fn topic_count(&mut self, topic_count: i64) -> &mut Self {
    self.inner.topic_count = topic_count;
    self
  }
}

impl AsRef<UpdateSavedMessagesTopicCount> for UpdateSavedMessagesTopicCount {
  fn as_ref(&self) -> &UpdateSavedMessagesTopicCount {
    self
  }
}

impl AsRef<UpdateSavedMessagesTopicCount> for RTDUpdateSavedMessagesTopicCountBuilder {
  fn as_ref(&self) -> &UpdateSavedMessagesTopicCount {
    &self.inner
  }
}

/// The list of saved notification sounds was updated. This update may not be sent until information about a notification sound was requested for the first time
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSavedNotificationSounds {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new list of identifiers of saved notification sounds
  notification_sound_ids: Vec<isize>,
}

impl RObject for UpdateSavedNotificationSounds {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateSavedNotificationSounds"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateSavedNotificationSounds {}

impl UpdateSavedNotificationSounds {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateSavedNotificationSoundsBuilder {
    let inner = UpdateSavedNotificationSounds {
      td_name: "updateSavedNotificationSounds".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateSavedNotificationSoundsBuilder { inner }
  }

  pub fn notification_sound_ids(&self) -> &Vec<isize> {
    &self.notification_sound_ids
  }
}

#[doc(hidden)]
pub struct RTDUpdateSavedNotificationSoundsBuilder {
  inner: UpdateSavedNotificationSounds,
}

impl RTDUpdateSavedNotificationSoundsBuilder {
  pub fn build(&self) -> UpdateSavedNotificationSounds {
    self.inner.clone()
  }

  pub fn notification_sound_ids(&mut self, notification_sound_ids: Vec<isize>) -> &mut Self {
    self.inner.notification_sound_ids = notification_sound_ids;
    self
  }
}

impl AsRef<UpdateSavedNotificationSounds> for UpdateSavedNotificationSounds {
  fn as_ref(&self) -> &UpdateSavedNotificationSounds {
    self
  }
}

impl AsRef<UpdateSavedNotificationSounds> for RTDUpdateSavedNotificationSoundsBuilder {
  fn as_ref(&self) -> &UpdateSavedNotificationSounds {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateScopeNotificationSettings"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateScopeNotificationSettings {}

impl UpdateScopeNotificationSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateScopeNotificationSettingsBuilder {
    let inner = UpdateScopeNotificationSettings {
      td_name: "updateScopeNotificationSettings".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateScopeNotificationSettingsBuilder { inner }
  }

  pub fn scope(&self) -> &NotificationSettingsScope {
    &self.scope
  }

  pub fn notification_settings(&self) -> &ScopeNotificationSettings {
    &self.notification_settings
  }
}

#[doc(hidden)]
pub struct RTDUpdateScopeNotificationSettingsBuilder {
  inner: UpdateScopeNotificationSettings,
}

impl RTDUpdateScopeNotificationSettingsBuilder {
  pub fn build(&self) -> UpdateScopeNotificationSettings {
    self.inner.clone()
  }

  pub fn scope<T: AsRef<NotificationSettingsScope>>(&mut self, scope: T) -> &mut Self {
    self.inner.scope = scope.as_ref().clone();
    self
  }

  pub fn notification_settings<T: AsRef<ScopeNotificationSettings>>(
    &mut self,
    notification_settings: T,
  ) -> &mut Self {
    self.inner.notification_settings = notification_settings.as_ref().clone();
    self
  }
}

impl AsRef<UpdateScopeNotificationSettings> for UpdateScopeNotificationSettings {
  fn as_ref(&self) -> &UpdateScopeNotificationSettings {
    self
  }
}

impl AsRef<UpdateScopeNotificationSettings> for RTDUpdateScopeNotificationSettingsBuilder {
  fn as_ref(&self) -> &UpdateScopeNotificationSettings {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateSecretChat"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateSecretChat {}

impl UpdateSecretChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateSecretChatBuilder {
    let inner = UpdateSecretChat {
      td_name: "updateSecretChat".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateSecretChatBuilder { inner }
  }

  pub fn secret_chat(&self) -> &SecretChat {
    &self.secret_chat
  }
}

#[doc(hidden)]
pub struct RTDUpdateSecretChatBuilder {
  inner: UpdateSecretChat,
}

impl RTDUpdateSecretChatBuilder {
  pub fn build(&self) -> UpdateSecretChat {
    self.inner.clone()
  }

  pub fn secret_chat<T: AsRef<SecretChat>>(&mut self, secret_chat: T) -> &mut Self {
    self.inner.secret_chat = secret_chat.as_ref().clone();
    self
  }
}

impl AsRef<UpdateSecretChat> for UpdateSecretChat {
  fn as_ref(&self) -> &UpdateSecretChat {
    self
  }
}

impl AsRef<UpdateSecretChat> for RTDUpdateSecretChatBuilder {
  fn as_ref(&self) -> &UpdateSecretChat {
    &self.inner
  }
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
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: String,
  /// Notification content
  content: MessageContent,
}

impl RObject for UpdateServiceNotification {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateServiceNotification"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateServiceNotification {}

impl UpdateServiceNotification {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateServiceNotificationBuilder {
    let inner = UpdateServiceNotification {
      td_name: "updateServiceNotification".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateServiceNotificationBuilder { inner }
  }

  pub fn type_(&self) -> &String {
    &self.type_
  }

  pub fn content(&self) -> &MessageContent {
    &self.content
  }
}

#[doc(hidden)]
pub struct RTDUpdateServiceNotificationBuilder {
  inner: UpdateServiceNotification,
}

impl RTDUpdateServiceNotificationBuilder {
  pub fn build(&self) -> UpdateServiceNotification {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateServiceNotification {
    self
  }
}

impl AsRef<UpdateServiceNotification> for RTDUpdateServiceNotificationBuilder {
  fn as_ref(&self) -> &UpdateServiceNotification {
    &self.inner
  }
}

/// The parameters of speech recognition without Telegram Premium subscription have changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSpeechRecognitionTrial {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The maximum allowed duration of media for speech recognition without Telegram Premium subscription, in seconds
  max_media_duration: i64,
  /// The total number of allowed speech recognitions per week; 0 if none
  weekly_count: i64,
  /// Number of left speech recognition attempts this week
  left_count: i64,
  /// Point in time (Unix timestamp) when the weekly number of tries will reset; 0 if unknown
  next_reset_date: i64,
}

impl RObject for UpdateSpeechRecognitionTrial {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateSpeechRecognitionTrial"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateSpeechRecognitionTrial {}

impl UpdateSpeechRecognitionTrial {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateSpeechRecognitionTrialBuilder {
    let inner = UpdateSpeechRecognitionTrial {
      td_name: "updateSpeechRecognitionTrial".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateSpeechRecognitionTrialBuilder { inner }
  }

  pub fn max_media_duration(&self) -> i64 {
    self.max_media_duration
  }

  pub fn weekly_count(&self) -> i64 {
    self.weekly_count
  }

  pub fn left_count(&self) -> i64 {
    self.left_count
  }

  pub fn next_reset_date(&self) -> i64 {
    self.next_reset_date
  }
}

#[doc(hidden)]
pub struct RTDUpdateSpeechRecognitionTrialBuilder {
  inner: UpdateSpeechRecognitionTrial,
}

impl RTDUpdateSpeechRecognitionTrialBuilder {
  pub fn build(&self) -> UpdateSpeechRecognitionTrial {
    self.inner.clone()
  }

  pub fn max_media_duration(&mut self, max_media_duration: i64) -> &mut Self {
    self.inner.max_media_duration = max_media_duration;
    self
  }

  pub fn weekly_count(&mut self, weekly_count: i64) -> &mut Self {
    self.inner.weekly_count = weekly_count;
    self
  }

  pub fn left_count(&mut self, left_count: i64) -> &mut Self {
    self.inner.left_count = left_count;
    self
  }

  pub fn next_reset_date(&mut self, next_reset_date: i64) -> &mut Self {
    self.inner.next_reset_date = next_reset_date;
    self
  }
}

impl AsRef<UpdateSpeechRecognitionTrial> for UpdateSpeechRecognitionTrial {
  fn as_ref(&self) -> &UpdateSpeechRecognitionTrial {
    self
  }
}

impl AsRef<UpdateSpeechRecognitionTrial> for RTDUpdateSpeechRecognitionTrialBuilder {
  fn as_ref(&self) -> &UpdateSpeechRecognitionTrial {
    &self.inner
  }
}

/// Download or upload file speed for the user was limited, but it can be restored by subscription to Telegram Premium. The notification can be postponed until a file being downloaded or uploaded is visible to the user. Use getOption("premium_download_speedup") or getOption("premium_upload_speedup") to get expected speedup after subscription to Telegram Premium
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSpeedLimitNotification {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if upload speed was limited; false, if download speed was limited
  is_upload: bool,
}

impl RObject for UpdateSpeedLimitNotification {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateSpeedLimitNotification"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateSpeedLimitNotification {}

impl UpdateSpeedLimitNotification {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateSpeedLimitNotificationBuilder {
    let inner = UpdateSpeedLimitNotification {
      td_name: "updateSpeedLimitNotification".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateSpeedLimitNotificationBuilder { inner }
  }

  pub fn is_upload(&self) -> bool {
    self.is_upload
  }
}

#[doc(hidden)]
pub struct RTDUpdateSpeedLimitNotificationBuilder {
  inner: UpdateSpeedLimitNotification,
}

impl RTDUpdateSpeedLimitNotificationBuilder {
  pub fn build(&self) -> UpdateSpeedLimitNotification {
    self.inner.clone()
  }

  pub fn is_upload(&mut self, is_upload: bool) -> &mut Self {
    self.inner.is_upload = is_upload;
    self
  }
}

impl AsRef<UpdateSpeedLimitNotification> for UpdateSpeedLimitNotification {
  fn as_ref(&self) -> &UpdateSpeedLimitNotification {
    self
  }
}

impl AsRef<UpdateSpeedLimitNotification> for RTDUpdateSpeedLimitNotificationBuilder {
  fn as_ref(&self) -> &UpdateSpeedLimitNotification {
    &self.inner
  }
}

/// The stake dice state has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateStakeDiceState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new state. The state can be used only if it was received recently enough. Otherwise, a new state must be requested using getStakeDiceState
  state: StakeDiceState,
}

impl RObject for UpdateStakeDiceState {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateStakeDiceState"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateStakeDiceState {}

impl UpdateStakeDiceState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateStakeDiceStateBuilder {
    let inner = UpdateStakeDiceState {
      td_name: "updateStakeDiceState".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateStakeDiceStateBuilder { inner }
  }

  pub fn state(&self) -> &StakeDiceState {
    &self.state
  }
}

#[doc(hidden)]
pub struct RTDUpdateStakeDiceStateBuilder {
  inner: UpdateStakeDiceState,
}

impl RTDUpdateStakeDiceStateBuilder {
  pub fn build(&self) -> UpdateStakeDiceState {
    self.inner.clone()
  }

  pub fn state<T: AsRef<StakeDiceState>>(&mut self, state: T) -> &mut Self {
    self.inner.state = state.as_ref().clone();
    self
  }
}

impl AsRef<UpdateStakeDiceState> for UpdateStakeDiceState {
  fn as_ref(&self) -> &UpdateStakeDiceState {
    self
  }
}

impl AsRef<UpdateStakeDiceState> for RTDUpdateStakeDiceStateBuilder {
  fn as_ref(&self) -> &UpdateStakeDiceState {
    &self.inner
  }
}

/// The Telegram Star revenue earned by a user or a chat has changed. If Telegram Star transaction screen of the chat is opened, then getStarTransactions may be called to fetch new transactions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateStarRevenueStatus {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the owner of the Telegram Stars
  owner_id: MessageSender,
  /// New Telegram Star revenue status
  status: StarRevenueStatus,
}

impl RObject for UpdateStarRevenueStatus {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateStarRevenueStatus"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateStarRevenueStatus {}

impl UpdateStarRevenueStatus {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateStarRevenueStatusBuilder {
    let inner = UpdateStarRevenueStatus {
      td_name: "updateStarRevenueStatus".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateStarRevenueStatusBuilder { inner }
  }

  pub fn owner_id(&self) -> &MessageSender {
    &self.owner_id
  }

  pub fn status(&self) -> &StarRevenueStatus {
    &self.status
  }
}

#[doc(hidden)]
pub struct RTDUpdateStarRevenueStatusBuilder {
  inner: UpdateStarRevenueStatus,
}

impl RTDUpdateStarRevenueStatusBuilder {
  pub fn build(&self) -> UpdateStarRevenueStatus {
    self.inner.clone()
  }

  pub fn owner_id<T: AsRef<MessageSender>>(&mut self, owner_id: T) -> &mut Self {
    self.inner.owner_id = owner_id.as_ref().clone();
    self
  }

  pub fn status<T: AsRef<StarRevenueStatus>>(&mut self, status: T) -> &mut Self {
    self.inner.status = status.as_ref().clone();
    self
  }
}

impl AsRef<UpdateStarRevenueStatus> for UpdateStarRevenueStatus {
  fn as_ref(&self) -> &UpdateStarRevenueStatus {
    self
  }
}

impl AsRef<UpdateStarRevenueStatus> for RTDUpdateStarRevenueStatusBuilder {
  fn as_ref(&self) -> &UpdateStarRevenueStatus {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateStickerSet"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateStickerSet {}

impl UpdateStickerSet {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateStickerSetBuilder {
    let inner = UpdateStickerSet {
      td_name: "updateStickerSet".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateStickerSetBuilder { inner }
  }

  pub fn sticker_set(&self) -> &StickerSet {
    &self.sticker_set
  }
}

#[doc(hidden)]
pub struct RTDUpdateStickerSetBuilder {
  inner: UpdateStickerSet,
}

impl RTDUpdateStickerSetBuilder {
  pub fn build(&self) -> UpdateStickerSet {
    self.inner.clone()
  }

  pub fn sticker_set<T: AsRef<StickerSet>>(&mut self, sticker_set: T) -> &mut Self {
    self.inner.sticker_set = sticker_set.as_ref().clone();
    self
  }
}

impl AsRef<UpdateStickerSet> for UpdateStickerSet {
  fn as_ref(&self) -> &UpdateStickerSet {
    self
  }
}

impl AsRef<UpdateStickerSet> for RTDUpdateStickerSetBuilder {
  fn as_ref(&self) -> &UpdateStickerSet {
    &self.inner
  }
}

/// A message draft generation was stopped by the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateStopMessageDraft {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The forum topic identifier of the message draft
  forum_topic_id: i64,
  /// Identifier of the message draft within the message thread
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  draft_id: isize,
}

impl RObject for UpdateStopMessageDraft {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateStopMessageDraft"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateStopMessageDraft {}

impl UpdateStopMessageDraft {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateStopMessageDraftBuilder {
    let inner = UpdateStopMessageDraft {
      td_name: "updateStopMessageDraft".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateStopMessageDraftBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn forum_topic_id(&self) -> i64 {
    self.forum_topic_id
  }

  pub fn draft_id(&self) -> isize {
    self.draft_id
  }
}

#[doc(hidden)]
pub struct RTDUpdateStopMessageDraftBuilder {
  inner: UpdateStopMessageDraft,
}

impl RTDUpdateStopMessageDraftBuilder {
  pub fn build(&self) -> UpdateStopMessageDraft {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn forum_topic_id(&mut self, forum_topic_id: i64) -> &mut Self {
    self.inner.forum_topic_id = forum_topic_id;
    self
  }

  pub fn draft_id(&mut self, draft_id: isize) -> &mut Self {
    self.inner.draft_id = draft_id;
    self
  }
}

impl AsRef<UpdateStopMessageDraft> for UpdateStopMessageDraft {
  fn as_ref(&self) -> &UpdateStopMessageDraft {
    self
  }
}

impl AsRef<UpdateStopMessageDraft> for RTDUpdateStopMessageDraftBuilder {
  fn as_ref(&self) -> &UpdateStopMessageDraft {
    &self.inner
  }
}

/// A story was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateStory {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new information about the story
  story: Story,
}

impl RObject for UpdateStory {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateStory"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateStory {}

impl UpdateStory {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateStoryBuilder {
    let inner = UpdateStory {
      td_name: "updateStory".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateStoryBuilder { inner }
  }

  pub fn story(&self) -> &Story {
    &self.story
  }
}

#[doc(hidden)]
pub struct RTDUpdateStoryBuilder {
  inner: UpdateStory,
}

impl RTDUpdateStoryBuilder {
  pub fn build(&self) -> UpdateStory {
    self.inner.clone()
  }

  pub fn story<T: AsRef<Story>>(&mut self, story: T) -> &mut Self {
    self.inner.story = story.as_ref().clone();
    self
  }
}

impl AsRef<UpdateStory> for UpdateStory {
  fn as_ref(&self) -> &UpdateStory {
    self
  }
}

impl AsRef<UpdateStory> for RTDUpdateStoryBuilder {
  fn as_ref(&self) -> &UpdateStory {
    &self.inner
  }
}

/// A story became inaccessible
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateStoryDeleted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat that posted the story
  story_poster_chat_id: i64,
  /// Story identifier
  story_id: i64,
}

impl RObject for UpdateStoryDeleted {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateStoryDeleted"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateStoryDeleted {}

impl UpdateStoryDeleted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateStoryDeletedBuilder {
    let inner = UpdateStoryDeleted {
      td_name: "updateStoryDeleted".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateStoryDeletedBuilder { inner }
  }

  pub fn story_poster_chat_id(&self) -> i64 {
    self.story_poster_chat_id
  }

  pub fn story_id(&self) -> i64 {
    self.story_id
  }
}

#[doc(hidden)]
pub struct RTDUpdateStoryDeletedBuilder {
  inner: UpdateStoryDeleted,
}

impl RTDUpdateStoryDeletedBuilder {
  pub fn build(&self) -> UpdateStoryDeleted {
    self.inner.clone()
  }

  pub fn story_poster_chat_id(&mut self, story_poster_chat_id: i64) -> &mut Self {
    self.inner.story_poster_chat_id = story_poster_chat_id;
    self
  }

  pub fn story_id(&mut self, story_id: i64) -> &mut Self {
    self.inner.story_id = story_id;
    self
  }
}

impl AsRef<UpdateStoryDeleted> for UpdateStoryDeleted {
  fn as_ref(&self) -> &UpdateStoryDeleted {
    self
  }
}

impl AsRef<UpdateStoryDeleted> for RTDUpdateStoryDeletedBuilder {
  fn as_ref(&self) -> &UpdateStoryDeleted {
    &self.inner
  }
}

/// Number of chats in a story list has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateStoryListChatCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The story list
  story_list: StoryList,
  /// Approximate total number of chats with active stories in the list
  chat_count: i64,
}

impl RObject for UpdateStoryListChatCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateStoryListChatCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateStoryListChatCount {}

impl UpdateStoryListChatCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateStoryListChatCountBuilder {
    let inner = UpdateStoryListChatCount {
      td_name: "updateStoryListChatCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateStoryListChatCountBuilder { inner }
  }

  pub fn story_list(&self) -> &StoryList {
    &self.story_list
  }

  pub fn chat_count(&self) -> i64 {
    self.chat_count
  }
}

#[doc(hidden)]
pub struct RTDUpdateStoryListChatCountBuilder {
  inner: UpdateStoryListChatCount,
}

impl RTDUpdateStoryListChatCountBuilder {
  pub fn build(&self) -> UpdateStoryListChatCount {
    self.inner.clone()
  }

  pub fn story_list<T: AsRef<StoryList>>(&mut self, story_list: T) -> &mut Self {
    self.inner.story_list = story_list.as_ref().clone();
    self
  }

  pub fn chat_count(&mut self, chat_count: i64) -> &mut Self {
    self.inner.chat_count = chat_count;
    self
  }
}

impl AsRef<UpdateStoryListChatCount> for UpdateStoryListChatCount {
  fn as_ref(&self) -> &UpdateStoryListChatCount {
    self
  }
}

impl AsRef<UpdateStoryListChatCount> for RTDUpdateStoryListChatCountBuilder {
  fn as_ref(&self) -> &UpdateStoryListChatCount {
    &self.inner
  }
}

/// A story failed to post. If the story posting is canceled, then updateStoryDeleted will be received instead of this update
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateStoryPostFailed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The failed to post story
  story: Story,
  /// The cause of the story posting failure
  error: Error,
  /// Type of the error; may be null if unknown
  error_type: Option<CanPostStoryResult>,
}

impl RObject for UpdateStoryPostFailed {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateStoryPostFailed"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateStoryPostFailed {}

impl UpdateStoryPostFailed {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateStoryPostFailedBuilder {
    let inner = UpdateStoryPostFailed {
      td_name: "updateStoryPostFailed".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateStoryPostFailedBuilder { inner }
  }

  pub fn story(&self) -> &Story {
    &self.story
  }

  pub fn error(&self) -> &Error {
    &self.error
  }

  pub fn error_type(&self) -> &Option<CanPostStoryResult> {
    &self.error_type
  }
}

#[doc(hidden)]
pub struct RTDUpdateStoryPostFailedBuilder {
  inner: UpdateStoryPostFailed,
}

impl RTDUpdateStoryPostFailedBuilder {
  pub fn build(&self) -> UpdateStoryPostFailed {
    self.inner.clone()
  }

  pub fn story<T: AsRef<Story>>(&mut self, story: T) -> &mut Self {
    self.inner.story = story.as_ref().clone();
    self
  }

  pub fn error<T: AsRef<Error>>(&mut self, error: T) -> &mut Self {
    self.inner.error = error.as_ref().clone();
    self
  }

  pub fn error_type<T: AsRef<CanPostStoryResult>>(&mut self, error_type: T) -> &mut Self {
    self.inner.error_type = Some(error_type.as_ref().clone());
    self
  }
}

impl AsRef<UpdateStoryPostFailed> for UpdateStoryPostFailed {
  fn as_ref(&self) -> &UpdateStoryPostFailed {
    self
  }
}

impl AsRef<UpdateStoryPostFailed> for RTDUpdateStoryPostFailedBuilder {
  fn as_ref(&self) -> &UpdateStoryPostFailed {
    &self.inner
  }
}

/// A story has been successfully posted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateStoryPostSucceeded {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The posted story
  story: Story,
  /// The previous temporary story identifier
  old_story_id: i64,
}

impl RObject for UpdateStoryPostSucceeded {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateStoryPostSucceeded"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateStoryPostSucceeded {}

impl UpdateStoryPostSucceeded {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateStoryPostSucceededBuilder {
    let inner = UpdateStoryPostSucceeded {
      td_name: "updateStoryPostSucceeded".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateStoryPostSucceededBuilder { inner }
  }

  pub fn story(&self) -> &Story {
    &self.story
  }

  pub fn old_story_id(&self) -> i64 {
    self.old_story_id
  }
}

#[doc(hidden)]
pub struct RTDUpdateStoryPostSucceededBuilder {
  inner: UpdateStoryPostSucceeded,
}

impl RTDUpdateStoryPostSucceededBuilder {
  pub fn build(&self) -> UpdateStoryPostSucceeded {
    self.inner.clone()
  }

  pub fn story<T: AsRef<Story>>(&mut self, story: T) -> &mut Self {
    self.inner.story = story.as_ref().clone();
    self
  }

  pub fn old_story_id(&mut self, old_story_id: i64) -> &mut Self {
    self.inner.old_story_id = old_story_id;
    self
  }
}

impl AsRef<UpdateStoryPostSucceeded> for UpdateStoryPostSucceeded {
  fn as_ref(&self) -> &UpdateStoryPostSucceeded {
    self
  }
}

impl AsRef<UpdateStoryPostSucceeded> for RTDUpdateStoryPostSucceededBuilder {
  fn as_ref(&self) -> &UpdateStoryPostSucceeded {
    &self.inner
  }
}

/// Story stealth mode settings have changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateStoryStealthMode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Point in time (Unix timestamp) until stealth mode is active; 0 if it is disabled
  active_until_date: i64,
  /// Point in time (Unix timestamp) when stealth mode can be enabled again; 0 if there is no active cooldown
  cooldown_until_date: i64,
}

impl RObject for UpdateStoryStealthMode {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateStoryStealthMode"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateStoryStealthMode {}

impl UpdateStoryStealthMode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateStoryStealthModeBuilder {
    let inner = UpdateStoryStealthMode {
      td_name: "updateStoryStealthMode".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateStoryStealthModeBuilder { inner }
  }

  pub fn active_until_date(&self) -> i64 {
    self.active_until_date
  }

  pub fn cooldown_until_date(&self) -> i64 {
    self.cooldown_until_date
  }
}

#[doc(hidden)]
pub struct RTDUpdateStoryStealthModeBuilder {
  inner: UpdateStoryStealthMode,
}

impl RTDUpdateStoryStealthModeBuilder {
  pub fn build(&self) -> UpdateStoryStealthMode {
    self.inner.clone()
  }

  pub fn active_until_date(&mut self, active_until_date: i64) -> &mut Self {
    self.inner.active_until_date = active_until_date;
    self
  }

  pub fn cooldown_until_date(&mut self, cooldown_until_date: i64) -> &mut Self {
    self.inner.cooldown_until_date = cooldown_until_date;
    self
  }
}

impl AsRef<UpdateStoryStealthMode> for UpdateStoryStealthMode {
  fn as_ref(&self) -> &UpdateStoryStealthMode {
    self
  }
}

impl AsRef<UpdateStoryStealthMode> for RTDUpdateStoryStealthModeBuilder {
  fn as_ref(&self) -> &UpdateStoryStealthMode {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateSuggestedActions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateSuggestedActions {}

impl UpdateSuggestedActions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateSuggestedActionsBuilder {
    let inner = UpdateSuggestedActions {
      td_name: "updateSuggestedActions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateSuggestedActionsBuilder { inner }
  }

  pub fn added_actions(&self) -> &Vec<SuggestedAction> {
    &self.added_actions
  }

  pub fn removed_actions(&self) -> &Vec<SuggestedAction> {
    &self.removed_actions
  }
}

#[doc(hidden)]
pub struct RTDUpdateSuggestedActionsBuilder {
  inner: UpdateSuggestedActions,
}

impl RTDUpdateSuggestedActionsBuilder {
  pub fn build(&self) -> UpdateSuggestedActions {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateSuggestedActions {
    self
  }
}

impl AsRef<UpdateSuggestedActions> for RTDUpdateSuggestedActionsBuilder {
  fn as_ref(&self) -> &UpdateSuggestedActions {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateSupergroup"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateSupergroup {}

impl UpdateSupergroup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateSupergroupBuilder {
    let inner = UpdateSupergroup {
      td_name: "updateSupergroup".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateSupergroupBuilder { inner }
  }

  pub fn supergroup(&self) -> &Supergroup {
    &self.supergroup
  }
}

#[doc(hidden)]
pub struct RTDUpdateSupergroupBuilder {
  inner: UpdateSupergroup,
}

impl RTDUpdateSupergroupBuilder {
  pub fn build(&self) -> UpdateSupergroup {
    self.inner.clone()
  }

  pub fn supergroup<T: AsRef<Supergroup>>(&mut self, supergroup: T) -> &mut Self {
    self.inner.supergroup = supergroup.as_ref().clone();
    self
  }
}

impl AsRef<UpdateSupergroup> for UpdateSupergroup {
  fn as_ref(&self) -> &UpdateSupergroup {
    self
  }
}

impl AsRef<UpdateSupergroup> for RTDUpdateSupergroupBuilder {
  fn as_ref(&self) -> &UpdateSupergroup {
    &self.inner
  }
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
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  supergroup_id: i64,
  /// New full information about the supergroup
  supergroup_full_info: SupergroupFullInfo,
}

impl RObject for UpdateSupergroupFullInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateSupergroupFullInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateSupergroupFullInfo {}

impl UpdateSupergroupFullInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateSupergroupFullInfoBuilder {
    let inner = UpdateSupergroupFullInfo {
      td_name: "updateSupergroupFullInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateSupergroupFullInfoBuilder { inner }
  }

  pub fn supergroup_id(&self) -> i64 {
    self.supergroup_id
  }

  pub fn supergroup_full_info(&self) -> &SupergroupFullInfo {
    &self.supergroup_full_info
  }
}

#[doc(hidden)]
pub struct RTDUpdateSupergroupFullInfoBuilder {
  inner: UpdateSupergroupFullInfo,
}

impl RTDUpdateSupergroupFullInfoBuilder {
  pub fn build(&self) -> UpdateSupergroupFullInfo {
    self.inner.clone()
  }

  pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
    self.inner.supergroup_id = supergroup_id;
    self
  }

  pub fn supergroup_full_info<T: AsRef<SupergroupFullInfo>>(
    &mut self,
    supergroup_full_info: T,
  ) -> &mut Self {
    self.inner.supergroup_full_info = supergroup_full_info.as_ref().clone();
    self
  }
}

impl AsRef<UpdateSupergroupFullInfo> for UpdateSupergroupFullInfo {
  fn as_ref(&self) -> &UpdateSupergroupFullInfo {
    self
  }
}

impl AsRef<UpdateSupergroupFullInfo> for RTDUpdateSupergroupFullInfoBuilder {
  fn as_ref(&self) -> &UpdateSupergroupFullInfo {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateTermsOfService"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateTermsOfService {}

impl UpdateTermsOfService {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateTermsOfServiceBuilder {
    let inner = UpdateTermsOfService {
      td_name: "updateTermsOfService".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateTermsOfServiceBuilder { inner }
  }

  pub fn terms_of_service_id(&self) -> &String {
    &self.terms_of_service_id
  }

  pub fn terms_of_service(&self) -> &TermsOfService {
    &self.terms_of_service
  }
}

#[doc(hidden)]
pub struct RTDUpdateTermsOfServiceBuilder {
  inner: UpdateTermsOfService,
}

impl RTDUpdateTermsOfServiceBuilder {
  pub fn build(&self) -> UpdateTermsOfService {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateTermsOfService {
    self
  }
}

impl AsRef<UpdateTermsOfService> for RTDUpdateTermsOfServiceBuilder {
  fn as_ref(&self) -> &UpdateTermsOfService {
    &self.inner
  }
}

/// The styles supported for text composition have changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateTextCompositionStyles {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new list of supported styles
  styles: Vec<TextCompositionStyle>,
}

impl RObject for UpdateTextCompositionStyles {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateTextCompositionStyles"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateTextCompositionStyles {}

impl UpdateTextCompositionStyles {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateTextCompositionStylesBuilder {
    let inner = UpdateTextCompositionStyles {
      td_name: "updateTextCompositionStyles".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateTextCompositionStylesBuilder { inner }
  }

  pub fn styles(&self) -> &Vec<TextCompositionStyle> {
    &self.styles
  }
}

#[doc(hidden)]
pub struct RTDUpdateTextCompositionStylesBuilder {
  inner: UpdateTextCompositionStyles,
}

impl RTDUpdateTextCompositionStylesBuilder {
  pub fn build(&self) -> UpdateTextCompositionStyles {
    self.inner.clone()
  }

  pub fn styles(&mut self, styles: Vec<TextCompositionStyle>) -> &mut Self {
    self.inner.styles = styles;
    self
  }
}

impl AsRef<UpdateTextCompositionStyles> for UpdateTextCompositionStyles {
  fn as_ref(&self) -> &UpdateTextCompositionStyles {
    self
  }
}

impl AsRef<UpdateTextCompositionStyles> for RTDUpdateTextCompositionStylesBuilder {
  fn as_ref(&self) -> &UpdateTextCompositionStyles {
    &self.inner
  }
}

/// The information about gasless transfers has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateTonWalletGaslessTransfersInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new information about gasless transfers
  info: TonWalletGaslessTransfersInfo,
}

impl RObject for UpdateTonWalletGaslessTransfersInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateTonWalletGaslessTransfersInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateTonWalletGaslessTransfersInfo {}

impl UpdateTonWalletGaslessTransfersInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateTonWalletGaslessTransfersInfoBuilder {
    let inner = UpdateTonWalletGaslessTransfersInfo {
      td_name: "updateTonWalletGaslessTransfersInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateTonWalletGaslessTransfersInfoBuilder { inner }
  }

  pub fn info(&self) -> &TonWalletGaslessTransfersInfo {
    &self.info
  }
}

#[doc(hidden)]
pub struct RTDUpdateTonWalletGaslessTransfersInfoBuilder {
  inner: UpdateTonWalletGaslessTransfersInfo,
}

impl RTDUpdateTonWalletGaslessTransfersInfoBuilder {
  pub fn build(&self) -> UpdateTonWalletGaslessTransfersInfo {
    self.inner.clone()
  }

  pub fn info<T: AsRef<TonWalletGaslessTransfersInfo>>(&mut self, info: T) -> &mut Self {
    self.inner.info = info.as_ref().clone();
    self
  }
}

impl AsRef<UpdateTonWalletGaslessTransfersInfo> for UpdateTonWalletGaslessTransfersInfo {
  fn as_ref(&self) -> &UpdateTonWalletGaslessTransfersInfo {
    self
  }
}

impl AsRef<UpdateTonWalletGaslessTransfersInfo> for RTDUpdateTonWalletGaslessTransfersInfoBuilder {
  fn as_ref(&self) -> &UpdateTonWalletGaslessTransfersInfo {
    &self.inner
  }
}

/// The state of the TON wallet of the current user has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateTonWalletState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new state of the wallet
  state: TonWalletState,
}

impl RObject for UpdateTonWalletState {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateTonWalletState"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateTonWalletState {}

impl UpdateTonWalletState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateTonWalletStateBuilder {
    let inner = UpdateTonWalletState {
      td_name: "updateTonWalletState".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateTonWalletStateBuilder { inner }
  }

  pub fn state(&self) -> &TonWalletState {
    &self.state
  }
}

#[doc(hidden)]
pub struct RTDUpdateTonWalletStateBuilder {
  inner: UpdateTonWalletState,
}

impl RTDUpdateTonWalletStateBuilder {
  pub fn build(&self) -> UpdateTonWalletState {
    self.inner.clone()
  }

  pub fn state<T: AsRef<TonWalletState>>(&mut self, state: T) -> &mut Self {
    self.inner.state = state.as_ref().clone();
    self
  }
}

impl AsRef<UpdateTonWalletState> for UpdateTonWalletState {
  fn as_ref(&self) -> &UpdateTonWalletState {
    self
  }
}

impl AsRef<UpdateTonWalletState> for RTDUpdateTonWalletStateBuilder {
  fn as_ref(&self) -> &UpdateTonWalletState {
    &self.inner
  }
}

/// A session of a TON Connect has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateTonWalletTonConnectSession {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new state of the session
  session: TonConnectSession,
}

impl RObject for UpdateTonWalletTonConnectSession {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateTonWalletTonConnectSession"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateTonWalletTonConnectSession {}

impl UpdateTonWalletTonConnectSession {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateTonWalletTonConnectSessionBuilder {
    let inner = UpdateTonWalletTonConnectSession {
      td_name: "updateTonWalletTonConnectSession".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateTonWalletTonConnectSessionBuilder { inner }
  }

  pub fn session(&self) -> &TonConnectSession {
    &self.session
  }
}

#[doc(hidden)]
pub struct RTDUpdateTonWalletTonConnectSessionBuilder {
  inner: UpdateTonWalletTonConnectSession,
}

impl RTDUpdateTonWalletTonConnectSessionBuilder {
  pub fn build(&self) -> UpdateTonWalletTonConnectSession {
    self.inner.clone()
  }

  pub fn session<T: AsRef<TonConnectSession>>(&mut self, session: T) -> &mut Self {
    self.inner.session = session.as_ref().clone();
    self
  }
}

impl AsRef<UpdateTonWalletTonConnectSession> for UpdateTonWalletTonConnectSession {
  fn as_ref(&self) -> &UpdateTonWalletTonConnectSession {
    self
  }
}

impl AsRef<UpdateTonWalletTonConnectSession> for RTDUpdateTonWalletTonConnectSessionBuilder {
  fn as_ref(&self) -> &UpdateTonWalletTonConnectSession {
    &self.inner
  }
}

/// TON Connect sessions must be disconnected in a regular way
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateTonWalletTonConnectSessionDisconnectRequired {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifiers of the sessions
  session_ids: Vec<isize>,
}

impl RObject for UpdateTonWalletTonConnectSessionDisconnectRequired {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateTonWalletTonConnectSessionDisconnectRequired"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateTonWalletTonConnectSessionDisconnectRequired {}

impl UpdateTonWalletTonConnectSessionDisconnectRequired {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateTonWalletTonConnectSessionDisconnectRequiredBuilder {
    let inner = UpdateTonWalletTonConnectSessionDisconnectRequired {
      td_name: "updateTonWalletTonConnectSessionDisconnectRequired".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateTonWalletTonConnectSessionDisconnectRequiredBuilder { inner }
  }

  pub fn session_ids(&self) -> &Vec<isize> {
    &self.session_ids
  }
}

#[doc(hidden)]
pub struct RTDUpdateTonWalletTonConnectSessionDisconnectRequiredBuilder {
  inner: UpdateTonWalletTonConnectSessionDisconnectRequired,
}

impl RTDUpdateTonWalletTonConnectSessionDisconnectRequiredBuilder {
  pub fn build(&self) -> UpdateTonWalletTonConnectSessionDisconnectRequired {
    self.inner.clone()
  }

  pub fn session_ids(&mut self, session_ids: Vec<isize>) -> &mut Self {
    self.inner.session_ids = session_ids;
    self
  }
}

impl AsRef<UpdateTonWalletTonConnectSessionDisconnectRequired>
  for UpdateTonWalletTonConnectSessionDisconnectRequired
{
  fn as_ref(&self) -> &UpdateTonWalletTonConnectSessionDisconnectRequired {
    self
  }
}

impl AsRef<UpdateTonWalletTonConnectSessionDisconnectRequired>
  for RTDUpdateTonWalletTonConnectSessionDisconnectRequiredBuilder
{
  fn as_ref(&self) -> &UpdateTonWalletTonConnectSessionDisconnectRequired {
    &self.inner
  }
}

/// Number of messages in a topic has changed; for Saved Messages and channel direct messages chat topics only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateTopicMessageCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat in topic of which the number of messages has changed
  chat_id: i64,
  /// Identifier of the topic
  topic_id: MessageTopic,
  /// Approximate number of messages in the topic
  message_count: i64,
}

impl RObject for UpdateTopicMessageCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateTopicMessageCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateTopicMessageCount {}

impl UpdateTopicMessageCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateTopicMessageCountBuilder {
    let inner = UpdateTopicMessageCount {
      td_name: "updateTopicMessageCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateTopicMessageCountBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn topic_id(&self) -> &MessageTopic {
    &self.topic_id
  }

  pub fn message_count(&self) -> i64 {
    self.message_count
  }
}

#[doc(hidden)]
pub struct RTDUpdateTopicMessageCountBuilder {
  inner: UpdateTopicMessageCount,
}

impl RTDUpdateTopicMessageCountBuilder {
  pub fn build(&self) -> UpdateTopicMessageCount {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn topic_id<T: AsRef<MessageTopic>>(&mut self, topic_id: T) -> &mut Self {
    self.inner.topic_id = topic_id.as_ref().clone();
    self
  }

  pub fn message_count(&mut self, message_count: i64) -> &mut Self {
    self.inner.message_count = message_count;
    self
  }
}

impl AsRef<UpdateTopicMessageCount> for UpdateTopicMessageCount {
  fn as_ref(&self) -> &UpdateTopicMessageCount {
    self
  }
}

impl AsRef<UpdateTopicMessageCount> for RTDUpdateTopicMessageCountBuilder {
  fn as_ref(&self) -> &UpdateTopicMessageCount {
    &self.inner
  }
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
  /// Type of the affected stickers
  sticker_type: StickerType,
  /// The prefix of the list of trending sticker sets with the newest trending sticker sets
  sticker_sets: TrendingStickerSets,
}

impl RObject for UpdateTrendingStickerSets {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateTrendingStickerSets"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateTrendingStickerSets {}

impl UpdateTrendingStickerSets {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateTrendingStickerSetsBuilder {
    let inner = UpdateTrendingStickerSets {
      td_name: "updateTrendingStickerSets".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateTrendingStickerSetsBuilder { inner }
  }

  pub fn sticker_type(&self) -> &StickerType {
    &self.sticker_type
  }

  pub fn sticker_sets(&self) -> &TrendingStickerSets {
    &self.sticker_sets
  }
}

#[doc(hidden)]
pub struct RTDUpdateTrendingStickerSetsBuilder {
  inner: UpdateTrendingStickerSets,
}

impl RTDUpdateTrendingStickerSetsBuilder {
  pub fn build(&self) -> UpdateTrendingStickerSets {
    self.inner.clone()
  }

  pub fn sticker_type<T: AsRef<StickerType>>(&mut self, sticker_type: T) -> &mut Self {
    self.inner.sticker_type = sticker_type.as_ref().clone();
    self
  }

  pub fn sticker_sets<T: AsRef<TrendingStickerSets>>(&mut self, sticker_sets: T) -> &mut Self {
    self.inner.sticker_sets = sticker_sets.as_ref().clone();
    self
  }
}

impl AsRef<UpdateTrendingStickerSets> for UpdateTrendingStickerSets {
  fn as_ref(&self) -> &UpdateTrendingStickerSets {
    self
  }
}

impl AsRef<UpdateTrendingStickerSets> for RTDUpdateTrendingStickerSetsBuilder {
  fn as_ref(&self) -> &UpdateTrendingStickerSets {
    &self.inner
  }
}

/// Lists of bots which Mini Apps must be allowed to read text from clipboard and must be opened without a warning
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateTrustedMiniAppBots {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of user identifiers of the bots; the corresponding users may not be sent using updateUser updates and may not be accessible
  bot_user_ids: Vec<i64>,
}

impl RObject for UpdateTrustedMiniAppBots {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateTrustedMiniAppBots"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateTrustedMiniAppBots {}

impl UpdateTrustedMiniAppBots {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateTrustedMiniAppBotsBuilder {
    let inner = UpdateTrustedMiniAppBots {
      td_name: "updateTrustedMiniAppBots".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateTrustedMiniAppBotsBuilder { inner }
  }

  pub fn bot_user_ids(&self) -> &Vec<i64> {
    &self.bot_user_ids
  }
}

#[doc(hidden)]
pub struct RTDUpdateTrustedMiniAppBotsBuilder {
  inner: UpdateTrustedMiniAppBots,
}

impl RTDUpdateTrustedMiniAppBotsBuilder {
  pub fn build(&self) -> UpdateTrustedMiniAppBots {
    self.inner.clone()
  }

  pub fn bot_user_ids(&mut self, bot_user_ids: Vec<i64>) -> &mut Self {
    self.inner.bot_user_ids = bot_user_ids;
    self
  }
}

impl AsRef<UpdateTrustedMiniAppBots> for UpdateTrustedMiniAppBots {
  fn as_ref(&self) -> &UpdateTrustedMiniAppBots {
    self
  }
}

impl AsRef<UpdateTrustedMiniAppBots> for RTDUpdateTrustedMiniAppBotsBuilder {
  fn as_ref(&self) -> &UpdateTrustedMiniAppBots {
    &self.inner
  }
}

/// The first unconfirmed session has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUnconfirmedSession {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The unconfirmed session; may be null if none
  session: Option<UnconfirmedSession>,
  /// The total number of unconfirmed sessions
  unconfirmed_session_count: i64,
}

impl RObject for UpdateUnconfirmedSession {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateUnconfirmedSession"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateUnconfirmedSession {}

impl UpdateUnconfirmedSession {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateUnconfirmedSessionBuilder {
    let inner = UpdateUnconfirmedSession {
      td_name: "updateUnconfirmedSession".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateUnconfirmedSessionBuilder { inner }
  }

  pub fn session(&self) -> &Option<UnconfirmedSession> {
    &self.session
  }

  pub fn unconfirmed_session_count(&self) -> i64 {
    self.unconfirmed_session_count
  }
}

#[doc(hidden)]
pub struct RTDUpdateUnconfirmedSessionBuilder {
  inner: UpdateUnconfirmedSession,
}

impl RTDUpdateUnconfirmedSessionBuilder {
  pub fn build(&self) -> UpdateUnconfirmedSession {
    self.inner.clone()
  }

  pub fn session<T: AsRef<UnconfirmedSession>>(&mut self, session: T) -> &mut Self {
    self.inner.session = Some(session.as_ref().clone());
    self
  }

  pub fn unconfirmed_session_count(&mut self, unconfirmed_session_count: i64) -> &mut Self {
    self.inner.unconfirmed_session_count = unconfirmed_session_count;
    self
  }
}

impl AsRef<UpdateUnconfirmedSession> for UpdateUnconfirmedSession {
  fn as_ref(&self) -> &UpdateUnconfirmedSession {
    self
  }
}

impl AsRef<UpdateUnconfirmedSession> for RTDUpdateUnconfirmedSessionBuilder {
  fn as_ref(&self) -> &UpdateUnconfirmedSession {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateUnreadChatCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateUnreadChatCount {}

impl UpdateUnreadChatCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateUnreadChatCountBuilder {
    let inner = UpdateUnreadChatCount {
      td_name: "updateUnreadChatCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateUnreadChatCountBuilder { inner }
  }

  pub fn chat_list(&self) -> &ChatList {
    &self.chat_list
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }

  pub fn unread_count(&self) -> i64 {
    self.unread_count
  }

  pub fn unread_unmuted_count(&self) -> i64 {
    self.unread_unmuted_count
  }

  pub fn marked_as_unread_count(&self) -> i64 {
    self.marked_as_unread_count
  }

  pub fn marked_as_unread_unmuted_count(&self) -> i64 {
    self.marked_as_unread_unmuted_count
  }
}

#[doc(hidden)]
pub struct RTDUpdateUnreadChatCountBuilder {
  inner: UpdateUnreadChatCount,
}

impl RTDUpdateUnreadChatCountBuilder {
  pub fn build(&self) -> UpdateUnreadChatCount {
    self.inner.clone()
  }

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

  pub fn marked_as_unread_unmuted_count(
    &mut self,
    marked_as_unread_unmuted_count: i64,
  ) -> &mut Self {
    self.inner.marked_as_unread_unmuted_count = marked_as_unread_unmuted_count;
    self
  }
}

impl AsRef<UpdateUnreadChatCount> for UpdateUnreadChatCount {
  fn as_ref(&self) -> &UpdateUnreadChatCount {
    self
  }
}

impl AsRef<UpdateUnreadChatCount> for RTDUpdateUnreadChatCountBuilder {
  fn as_ref(&self) -> &UpdateUnreadChatCount {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateUnreadMessageCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateUnreadMessageCount {}

impl UpdateUnreadMessageCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateUnreadMessageCountBuilder {
    let inner = UpdateUnreadMessageCount {
      td_name: "updateUnreadMessageCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateUnreadMessageCountBuilder { inner }
  }

  pub fn chat_list(&self) -> &ChatList {
    &self.chat_list
  }

  pub fn unread_count(&self) -> i64 {
    self.unread_count
  }

  pub fn unread_unmuted_count(&self) -> i64 {
    self.unread_unmuted_count
  }
}

#[doc(hidden)]
pub struct RTDUpdateUnreadMessageCountBuilder {
  inner: UpdateUnreadMessageCount,
}

impl RTDUpdateUnreadMessageCountBuilder {
  pub fn build(&self) -> UpdateUnreadMessageCount {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateUnreadMessageCount {
    self
  }
}

impl AsRef<UpdateUnreadMessageCount> for RTDUpdateUnreadMessageCountBuilder {
  fn as_ref(&self) -> &UpdateUnreadMessageCount {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateUser"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateUser {}

impl UpdateUser {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateUserBuilder {
    let inner = UpdateUser {
      td_name: "updateUser".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateUserBuilder { inner }
  }

  pub fn user(&self) -> &User {
    &self.user
  }
}

#[doc(hidden)]
pub struct RTDUpdateUserBuilder {
  inner: UpdateUser,
}

impl RTDUpdateUserBuilder {
  pub fn build(&self) -> UpdateUser {
    self.inner.clone()
  }

  pub fn user<T: AsRef<User>>(&mut self, user: T) -> &mut Self {
    self.inner.user = user.as_ref().clone();
    self
  }
}

impl AsRef<UpdateUser> for UpdateUser {
  fn as_ref(&self) -> &UpdateUser {
    self
  }
}

impl AsRef<UpdateUser> for RTDUpdateUserBuilder {
  fn as_ref(&self) -> &UpdateUser {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateUserFullInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateUserFullInfo {}

impl UpdateUserFullInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateUserFullInfoBuilder {
    let inner = UpdateUserFullInfo {
      td_name: "updateUserFullInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateUserFullInfoBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn user_full_info(&self) -> &UserFullInfo {
    &self.user_full_info
  }
}

#[doc(hidden)]
pub struct RTDUpdateUserFullInfoBuilder {
  inner: UpdateUserFullInfo,
}

impl RTDUpdateUserFullInfoBuilder {
  pub fn build(&self) -> UpdateUserFullInfo {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateUserFullInfo {
    self
  }
}

impl AsRef<UpdateUserFullInfo> for RTDUpdateUserFullInfoBuilder {
  fn as_ref(&self) -> &UpdateUserFullInfo {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateUserPrivacySettingRules"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateUserPrivacySettingRules {}

impl UpdateUserPrivacySettingRules {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateUserPrivacySettingRulesBuilder {
    let inner = UpdateUserPrivacySettingRules {
      td_name: "updateUserPrivacySettingRules".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateUserPrivacySettingRulesBuilder { inner }
  }

  pub fn setting(&self) -> &UserPrivacySetting {
    &self.setting
  }

  pub fn rules(&self) -> &UserPrivacySettingRules {
    &self.rules
  }
}

#[doc(hidden)]
pub struct RTDUpdateUserPrivacySettingRulesBuilder {
  inner: UpdateUserPrivacySettingRules,
}

impl RTDUpdateUserPrivacySettingRulesBuilder {
  pub fn build(&self) -> UpdateUserPrivacySettingRules {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateUserPrivacySettingRules {
    self
  }
}

impl AsRef<UpdateUserPrivacySettingRules> for RTDUpdateUserPrivacySettingRulesBuilder {
  fn as_ref(&self) -> &UpdateUserPrivacySettingRules {
    &self.inner
  }
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
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateUserStatus"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateUserStatus {}

impl UpdateUserStatus {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateUserStatusBuilder {
    let inner = UpdateUserStatus {
      td_name: "updateUserStatus".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateUserStatusBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn status(&self) -> &UserStatus {
    &self.status
  }
}

#[doc(hidden)]
pub struct RTDUpdateUserStatusBuilder {
  inner: UpdateUserStatus,
}

impl RTDUpdateUserStatusBuilder {
  pub fn build(&self) -> UpdateUserStatus {
    self.inner.clone()
  }

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
  fn as_ref(&self) -> &UpdateUserStatus {
    self
  }
}

impl AsRef<UpdateUserStatus> for RTDUpdateUserStatusBuilder {
  fn as_ref(&self) -> &UpdateUserStatus {
    &self.inner
  }
}

/// Subscription of a user to the bot was changed; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUserSubscription {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user
  user_id: i64,
  /// Bot-specified subscription invoice payload
  payload: String,
  /// True, if the subscription was canceled
  is_canceled: bool,
  /// True, if the subscription was restored
  is_restored: bool,
  /// True, if the payment for the subscription has failed
  is_payment_failed: bool,
}

impl RObject for UpdateUserSubscription {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateUserSubscription"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateUserSubscription {}

impl UpdateUserSubscription {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateUserSubscriptionBuilder {
    let inner = UpdateUserSubscription {
      td_name: "updateUserSubscription".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateUserSubscriptionBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn payload(&self) -> &String {
    &self.payload
  }

  pub fn is_canceled(&self) -> bool {
    self.is_canceled
  }

  pub fn is_restored(&self) -> bool {
    self.is_restored
  }

  pub fn is_payment_failed(&self) -> bool {
    self.is_payment_failed
  }
}

#[doc(hidden)]
pub struct RTDUpdateUserSubscriptionBuilder {
  inner: UpdateUserSubscription,
}

impl RTDUpdateUserSubscriptionBuilder {
  pub fn build(&self) -> UpdateUserSubscription {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn payload<T: AsRef<str>>(&mut self, payload: T) -> &mut Self {
    self.inner.payload = payload.as_ref().to_string();
    self
  }

  pub fn is_canceled(&mut self, is_canceled: bool) -> &mut Self {
    self.inner.is_canceled = is_canceled;
    self
  }

  pub fn is_restored(&mut self, is_restored: bool) -> &mut Self {
    self.inner.is_restored = is_restored;
    self
  }

  pub fn is_payment_failed(&mut self, is_payment_failed: bool) -> &mut Self {
    self.inner.is_payment_failed = is_payment_failed;
    self
  }
}

impl AsRef<UpdateUserSubscription> for UpdateUserSubscription {
  fn as_ref(&self) -> &UpdateUserSubscription {
    self
  }
}

impl AsRef<UpdateUserSubscription> for RTDUpdateUserSubscriptionBuilder {
  fn as_ref(&self) -> &UpdateUserSubscription {
    &self.inner
  }
}

/// An automatically scheduled message with video has been successfully sent after conversion
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateVideoPublished {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat with the message
  chat_id: i64,
  /// Identifier of the sent message
  message_id: i64,
}

impl RObject for UpdateVideoPublished {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateVideoPublished"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateVideoPublished {}

impl UpdateVideoPublished {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateVideoPublishedBuilder {
    let inner = UpdateVideoPublished {
      td_name: "updateVideoPublished".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateVideoPublishedBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }
}

#[doc(hidden)]
pub struct RTDUpdateVideoPublishedBuilder {
  inner: UpdateVideoPublished,
}

impl RTDUpdateVideoPublishedBuilder {
  pub fn build(&self) -> UpdateVideoPublished {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }
}

impl AsRef<UpdateVideoPublished> for UpdateVideoPublished {
  fn as_ref(&self) -> &UpdateVideoPublished {
    self
  }
}

impl AsRef<UpdateVideoPublished> for RTDUpdateVideoPublishedBuilder {
  fn as_ref(&self) -> &UpdateVideoPublished {
    &self.inner
  }
}

/// A message was sent by an opened Web App, so the Web App needs to be closed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateWebAppMessageSent {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of Web App launch
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  web_app_launch_id: isize,
}

impl RObject for UpdateWebAppMessageSent {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateWebAppMessageSent"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateWebAppMessageSent {}

impl UpdateWebAppMessageSent {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateWebAppMessageSentBuilder {
    let inner = UpdateWebAppMessageSent {
      td_name: "updateWebAppMessageSent".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateWebAppMessageSentBuilder { inner }
  }

  pub fn web_app_launch_id(&self) -> isize {
    self.web_app_launch_id
  }
}

#[doc(hidden)]
pub struct RTDUpdateWebAppMessageSentBuilder {
  inner: UpdateWebAppMessageSent,
}

impl RTDUpdateWebAppMessageSentBuilder {
  pub fn build(&self) -> UpdateWebAppMessageSent {
    self.inner.clone()
  }

  pub fn web_app_launch_id(&mut self, web_app_launch_id: isize) -> &mut Self {
    self.inner.web_app_launch_id = web_app_launch_id;
    self
  }
}

impl AsRef<UpdateWebAppMessageSent> for UpdateWebAppMessageSent {
  fn as_ref(&self) -> &UpdateWebAppMessageSent {
    self
  }
}

impl AsRef<UpdateWebAppMessageSent> for RTDUpdateWebAppMessageSentBuilder {
  fn as_ref(&self) -> &UpdateWebAppMessageSent {
    &self.inner
  }
}

/// Web browser settings have been updated
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateWebBrowserSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New settings
  settings: WebBrowserSettings,
}

impl RObject for UpdateWebBrowserSettings {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateWebBrowserSettings"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateWebBrowserSettings {}

impl UpdateWebBrowserSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateWebBrowserSettingsBuilder {
    let inner = UpdateWebBrowserSettings {
      td_name: "updateWebBrowserSettings".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateWebBrowserSettingsBuilder { inner }
  }

  pub fn settings(&self) -> &WebBrowserSettings {
    &self.settings
  }
}

#[doc(hidden)]
pub struct RTDUpdateWebBrowserSettingsBuilder {
  inner: UpdateWebBrowserSettings,
}

impl RTDUpdateWebBrowserSettingsBuilder {
  pub fn build(&self) -> UpdateWebBrowserSettings {
    self.inner.clone()
  }

  pub fn settings<T: AsRef<WebBrowserSettings>>(&mut self, settings: T) -> &mut Self {
    self.inner.settings = settings.as_ref().clone();
    self
  }
}

impl AsRef<UpdateWebBrowserSettings> for UpdateWebBrowserSettings {
  fn as_ref(&self) -> &UpdateWebBrowserSettings {
    self
  }
}

impl AsRef<UpdateWebBrowserSettings> for RTDUpdateWebBrowserSettingsBuilder {
  fn as_ref(&self) -> &UpdateWebBrowserSettings {
    &self.inner
  }
}

/// A web proxy carrier must be closed. If the page has passed any message to the bridge object, then the application must deliver the string {"t":"close"} to the page, so that it can end its relay session, and destroy the web view shortly after, for example, in 200 milliseconds; otherwise, the web view must be destroyed immediately. Messages from the carrier received after this update must be ignored
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateWebProxyCarrierClosed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the carrier
  carrier_id: i64,
}

impl RObject for UpdateWebProxyCarrierClosed {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateWebProxyCarrierClosed"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateWebProxyCarrierClosed {}

impl UpdateWebProxyCarrierClosed {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateWebProxyCarrierClosedBuilder {
    let inner = UpdateWebProxyCarrierClosed {
      td_name: "updateWebProxyCarrierClosed".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateWebProxyCarrierClosedBuilder { inner }
  }

  pub fn carrier_id(&self) -> i64 {
    self.carrier_id
  }
}

#[doc(hidden)]
pub struct RTDUpdateWebProxyCarrierClosedBuilder {
  inner: UpdateWebProxyCarrierClosed,
}

impl RTDUpdateWebProxyCarrierClosedBuilder {
  pub fn build(&self) -> UpdateWebProxyCarrierClosed {
    self.inner.clone()
  }

  pub fn carrier_id(&mut self, carrier_id: i64) -> &mut Self {
    self.inner.carrier_id = carrier_id;
    self
  }
}

impl AsRef<UpdateWebProxyCarrierClosed> for UpdateWebProxyCarrierClosed {
  fn as_ref(&self) -> &UpdateWebProxyCarrierClosed {
    self
  }
}

impl AsRef<UpdateWebProxyCarrierClosed> for RTDUpdateWebProxyCarrierClosedBuilder {
  fn as_ref(&self) -> &UpdateWebProxyCarrierClosed {
    &self.inner
  }
}

/// Data must be delivered as an ArrayBuffer to the page of a web proxy carrier through the onmessage function of the bridge object, in the order of the updates
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateWebProxyCarrierData {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the carrier
  carrier_id: i64,
  /// The data to deliver
  data: String,
}

impl RObject for UpdateWebProxyCarrierData {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateWebProxyCarrierData"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateWebProxyCarrierData {}

impl UpdateWebProxyCarrierData {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateWebProxyCarrierDataBuilder {
    let inner = UpdateWebProxyCarrierData {
      td_name: "updateWebProxyCarrierData".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateWebProxyCarrierDataBuilder { inner }
  }

  pub fn carrier_id(&self) -> i64 {
    self.carrier_id
  }

  pub fn data(&self) -> &String {
    &self.data
  }
}

#[doc(hidden)]
pub struct RTDUpdateWebProxyCarrierDataBuilder {
  inner: UpdateWebProxyCarrierData,
}

impl RTDUpdateWebProxyCarrierDataBuilder {
  pub fn build(&self) -> UpdateWebProxyCarrierData {
    self.inner.clone()
  }

  pub fn carrier_id(&mut self, carrier_id: i64) -> &mut Self {
    self.inner.carrier_id = carrier_id;
    self
  }

  pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
    self.inner.data = data.as_ref().to_string();
    self
  }
}

impl AsRef<UpdateWebProxyCarrierData> for UpdateWebProxyCarrierData {
  fn as_ref(&self) -> &UpdateWebProxyCarrierData {
    self
  }
}

impl AsRef<UpdateWebProxyCarrierData> for RTDUpdateWebProxyCarrierDataBuilder {
  fn as_ref(&self) -> &UpdateWebProxyCarrierData {
    &self.inner
  }
}

/// A web proxy carrier must be opened as described in webProxyCarrier. Messages passed by the page to the bridge object must be passed to processWebProxyCarrierMessage. Failures of the carrier must be reported with reportWebProxyCarrierError
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateWebProxyCarrierOpened {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The carrier
  carrier: WebProxyCarrier,
}

impl RObject for UpdateWebProxyCarrierOpened {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "updateWebProxyCarrierOpened"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpdate for UpdateWebProxyCarrierOpened {}

impl UpdateWebProxyCarrierOpened {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpdateWebProxyCarrierOpenedBuilder {
    let inner = UpdateWebProxyCarrierOpened {
      td_name: "updateWebProxyCarrierOpened".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpdateWebProxyCarrierOpenedBuilder { inner }
  }

  pub fn carrier(&self) -> &WebProxyCarrier {
    &self.carrier
  }
}

#[doc(hidden)]
pub struct RTDUpdateWebProxyCarrierOpenedBuilder {
  inner: UpdateWebProxyCarrierOpened,
}

impl RTDUpdateWebProxyCarrierOpenedBuilder {
  pub fn build(&self) -> UpdateWebProxyCarrierOpened {
    self.inner.clone()
  }

  pub fn carrier<T: AsRef<WebProxyCarrier>>(&mut self, carrier: T) -> &mut Self {
    self.inner.carrier = carrier.as_ref().clone();
    self
  }
}

impl AsRef<UpdateWebProxyCarrierOpened> for UpdateWebProxyCarrierOpened {
  fn as_ref(&self) -> &UpdateWebProxyCarrierOpened {
    self
  }
}

impl AsRef<UpdateWebProxyCarrierOpened> for RTDUpdateWebProxyCarrierOpenedBuilder {
  fn as_ref(&self) -> &UpdateWebProxyCarrierOpened {
    &self.inner
  }
}
