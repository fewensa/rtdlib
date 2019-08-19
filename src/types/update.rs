
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Contains notifications about data changes. 
#[typetag::serde(tag = "@struct")]
pub trait Update: Object + RObject + Debug {}






impl Update {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<Update> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDUpdateType {
  UpdateActiveNotifications,
  UpdateAuthorizationState,
  UpdateBasicGroup,
  UpdateBasicGroupFullInfo,
  UpdateCall,
  UpdateChatDefaultDisableNotification,
  UpdateChatDraftMessage,
  UpdateChatIsMarkedAsUnread,
  UpdateChatIsPinned,
  UpdateChatIsSponsored,
  UpdateChatLastMessage,
  UpdateChatNotificationSettings,
  UpdateChatOnlineMemberCount,
  UpdateChatOrder,
  UpdateChatPhoto,
  UpdateChatPinnedMessage,
  UpdateChatReadInbox,
  UpdateChatReadOutbox,
  UpdateChatReplyMarkup,
  UpdateChatTitle,
  UpdateChatUnreadMentionCount,
  UpdateConnectionState,
  UpdateDeleteMessages,
  UpdateFavoriteStickers,
  UpdateFile,
  UpdateFileGenerationStart,
  UpdateFileGenerationStop,
  UpdateHavePendingNotifications,
  UpdateInstalledStickerSets,
  UpdateLanguagePackStrings,
  UpdateMessageContent,
  UpdateMessageContentOpened,
  UpdateMessageEdited,
  UpdateMessageMentionRead,
  UpdateMessageSendAcknowledged,
  UpdateMessageSendFailed,
  UpdateMessageSendSucceeded,
  UpdateMessageViews,
  UpdateNewCallbackQuery,
  UpdateNewChat,
  UpdateNewChosenInlineResult,
  UpdateNewCustomEvent,
  UpdateNewCustomQuery,
  UpdateNewInlineCallbackQuery,
  UpdateNewInlineQuery,
  UpdateNewMessage,
  UpdateNewPreCheckoutQuery,
  UpdateNewShippingQuery,
  UpdateNotification,
  UpdateNotificationGroup,
  UpdateOption,
  UpdatePoll,
  UpdateRecentStickers,
  UpdateSavedAnimations,
  UpdateScopeNotificationSettings,
  UpdateSecretChat,
  UpdateServiceNotification,
  UpdateSupergroup,
  UpdateSupergroupFullInfo,
  UpdateTermsOfService,
  UpdateTrendingStickerSets,
  UpdateUnreadChatCount,
  UpdateUnreadMessageCount,
  UpdateUser,
  UpdateUserChatAction,
  UpdateUserFullInfo,
  UpdateUserPrivacySettingRules,
  UpdateUserStatus,
  
}
impl RTDUpdateType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDUpdateType)(text.as_ref()) }
}



