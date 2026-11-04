use std::fmt::Debug;

use serde::de::{Deserialize, Deserializer};

use crate::errors::*;
use crate::types::*;

macro_rules! rtd_enum_deserialize {
  ($type_name:ident, $(($td_name:ident, $enum_item:ident));*;) => {
    // example json
    // {"@type":"authorizationStateWaitEncryptionKey","is_encrypted":false}
    |deserializer: D| -> Result<$type_name, D::Error> {
      let rtd_trait_value: serde_json::Value = Deserialize::deserialize(deserializer)?;
      // the `rtd_trait_value` variable type is &serde_json::Value, tdlib trait will return a object, convert this type to object `&Map<String, Value>`
      let rtd_trait_map = match rtd_trait_value.as_object() {
        Some(map) => map,
        None => return Err(
          D::Error::custom(format!(
            "{} is not the correct type", stringify!($type_name)
          ))
        ) // &format!("{} is not the correct type", stringify!($field))[..]
      };
      // get `@type` value, detect specific types
      let rtd_trait_type = match rtd_trait_map.get("@type") {
        // the `t` variable type is `serde_json::Value`, convert `t` to str
        Some(t) => match t.as_str() {
          Some(s) => s,
          None => return Err(
            D::Error::custom(format!(
              "{} -> @type is not the correct type", stringify!($type_name)
            ))
          ) // &format!("{} -> @type is not the correct type", stringify!($field))[..]
        },
        None => return Err(D::Error::custom(format!("unknown field {} -> @type", stringify!($type_name))))
      };

      let obj = match rtd_trait_type {
        $(
          stringify!($td_name) => $type_name::$enum_item(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(
              D::Error::custom(format!(
                "{} can't deserialize to {}::{}; {:?}",
                stringify!($td_name),
                stringify!($type_name),
                stringify!($enum_item),
                _e,
              ))
            )
          }),
        )*
        _ => return Err(D::Error::custom(format!("missing field {}", rtd_trait_type)))
      };
      Ok(obj)
    }
  }
}

pub fn detect_td_type<S: AsRef<str>>(json: S) -> Option<String> {
  let value: serde_json::Value = serde_json::from_str(json.as_ref()).ok()?;
  let obj = value.as_object()?;
  obj.get("@type")?.as_str().map(str::to_string)
}

pub fn detect_td_type_and_extra<S: AsRef<str>>(json: S) -> (Option<String>, Option<String>) {
  let Ok(value) = serde_json::from_str::<serde_json::Value>(json.as_ref()) else {
    return (None, None);
  };
  let Some(map) = value.as_object() else {
    return (None, None);
  };
  let type_ = map
    .get("@type")
    .and_then(serde_json::Value::as_str)
    .map(str::to_string);
  let extra = map
    .get("@extra")
    .and_then(serde_json::Value::as_str)
    .map(str::to_string);
  (type_, extra)
}

pub fn from_json<'a, T>(json: &'a str) -> RTDResult<T>
where
  T: serde::de::Deserialize<'a>,
{
  Ok(serde_json::from_str(json)?)
}

/// All tdlib type abstract class defined the same behavior
pub trait RObject: Debug {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str;
  #[doc(hidden)]
  fn extra(&self) -> Option<String>;
  /// Return td type to json string
  fn to_json(&self) -> RTDResult<String>;
}

pub trait RFunction: Debug + RObject {}

impl<RObj: RObject> RObject for &RObj {
  fn td_name(&self) -> &'static str {
    (*self).td_name()
  }
  fn to_json(&self) -> RTDResult<String> {
    (*self).to_json()
  }
  fn extra(&self) -> Option<String> {
    (*self).extra()
  }
}

impl<RObj: RObject> RObject for &mut RObj {
  fn td_name(&self) -> &'static str {
    (**self).td_name()
  }
  fn to_json(&self) -> RTDResult<String> {
    (**self).to_json()
  }
  fn extra(&self) -> Option<String> {
    (**self).extra()
  }
}

impl<Fnc: RFunction> RFunction for &Fnc {}
impl<Fnc: RFunction> RFunction for &mut Fnc {}

impl<AUTHENTICATIONCODETYPE: TDAuthenticationCodeType> TDAuthenticationCodeType
  for &AUTHENTICATIONCODETYPE
{
}
impl<AUTHENTICATIONCODETYPE: TDAuthenticationCodeType> TDAuthenticationCodeType
  for &mut AUTHENTICATIONCODETYPE
{
}

impl<AUTHORIZATIONSTATE: TDAuthorizationState> TDAuthorizationState for &AUTHORIZATIONSTATE {}
impl<AUTHORIZATIONSTATE: TDAuthorizationState> TDAuthorizationState for &mut AUTHORIZATIONSTATE {}

impl<BACKGROUNDFILL: TDBackgroundFill> TDBackgroundFill for &BACKGROUNDFILL {}
impl<BACKGROUNDFILL: TDBackgroundFill> TDBackgroundFill for &mut BACKGROUNDFILL {}

impl<BACKGROUNDTYPE: TDBackgroundType> TDBackgroundType for &BACKGROUNDTYPE {}
impl<BACKGROUNDTYPE: TDBackgroundType> TDBackgroundType for &mut BACKGROUNDTYPE {}

impl<BOTCOMMANDSCOPE: TDBotCommandScope> TDBotCommandScope for &BOTCOMMANDSCOPE {}
impl<BOTCOMMANDSCOPE: TDBotCommandScope> TDBotCommandScope for &mut BOTCOMMANDSCOPE {}

impl<CALLDISCARDREASON: TDCallDiscardReason> TDCallDiscardReason for &CALLDISCARDREASON {}
impl<CALLDISCARDREASON: TDCallDiscardReason> TDCallDiscardReason for &mut CALLDISCARDREASON {}

impl<CALLPROBLEM: TDCallProblem> TDCallProblem for &CALLPROBLEM {}
impl<CALLPROBLEM: TDCallProblem> TDCallProblem for &mut CALLPROBLEM {}

impl<CALLSERVERTYPE: TDCallServerType> TDCallServerType for &CALLSERVERTYPE {}
impl<CALLSERVERTYPE: TDCallServerType> TDCallServerType for &mut CALLSERVERTYPE {}

impl<CALLSTATE: TDCallState> TDCallState for &CALLSTATE {}
impl<CALLSTATE: TDCallState> TDCallState for &mut CALLSTATE {}

impl<CALLBACKQUERYPAYLOAD: TDCallbackQueryPayload> TDCallbackQueryPayload
  for &CALLBACKQUERYPAYLOAD
{
}
impl<CALLBACKQUERYPAYLOAD: TDCallbackQueryPayload> TDCallbackQueryPayload
  for &mut CALLBACKQUERYPAYLOAD
{
}

impl<CANTRANSFEROWNERSHIPRESULT: TDCanTransferOwnershipResult> TDCanTransferOwnershipResult
  for &CANTRANSFEROWNERSHIPRESULT
{
}
impl<CANTRANSFEROWNERSHIPRESULT: TDCanTransferOwnershipResult> TDCanTransferOwnershipResult
  for &mut CANTRANSFEROWNERSHIPRESULT
{
}

impl<CHATACTION: TDChatAction> TDChatAction for &CHATACTION {}
impl<CHATACTION: TDChatAction> TDChatAction for &mut CHATACTION {}

impl<CHATACTIONBAR: TDChatActionBar> TDChatActionBar for &CHATACTIONBAR {}
impl<CHATACTIONBAR: TDChatActionBar> TDChatActionBar for &mut CHATACTIONBAR {}

impl<CHATEVENTACTION: TDChatEventAction> TDChatEventAction for &CHATEVENTACTION {}
impl<CHATEVENTACTION: TDChatEventAction> TDChatEventAction for &mut CHATEVENTACTION {}

impl<CHATLIST: TDChatList> TDChatList for &CHATLIST {}
impl<CHATLIST: TDChatList> TDChatList for &mut CHATLIST {}

impl<CHATMEMBERSTATUS: TDChatMemberStatus> TDChatMemberStatus for &CHATMEMBERSTATUS {}
impl<CHATMEMBERSTATUS: TDChatMemberStatus> TDChatMemberStatus for &mut CHATMEMBERSTATUS {}

impl<CHATMEMBERSFILTER: TDChatMembersFilter> TDChatMembersFilter for &CHATMEMBERSFILTER {}
impl<CHATMEMBERSFILTER: TDChatMembersFilter> TDChatMembersFilter for &mut CHATMEMBERSFILTER {}

impl<CHATREPORTREASON: TDChatReportReason> TDChatReportReason for &CHATREPORTREASON {}
impl<CHATREPORTREASON: TDChatReportReason> TDChatReportReason for &mut CHATREPORTREASON {}

impl<CHATSOURCE: TDChatSource> TDChatSource for &CHATSOURCE {}
impl<CHATSOURCE: TDChatSource> TDChatSource for &mut CHATSOURCE {}

impl<CHATSTATISTICS: TDChatStatistics> TDChatStatistics for &CHATSTATISTICS {}
impl<CHATSTATISTICS: TDChatStatistics> TDChatStatistics for &mut CHATSTATISTICS {}

impl<CHATTYPE: TDChatType> TDChatType for &CHATTYPE {}
impl<CHATTYPE: TDChatType> TDChatType for &mut CHATTYPE {}

impl<CHECKCHATUSERNAMERESULT: TDCheckChatUsernameResult> TDCheckChatUsernameResult
  for &CHECKCHATUSERNAMERESULT
{
}
impl<CHECKCHATUSERNAMERESULT: TDCheckChatUsernameResult> TDCheckChatUsernameResult
  for &mut CHECKCHATUSERNAMERESULT
{
}

impl<CHECKSTICKERSETNAMERESULT: TDCheckStickerSetNameResult> TDCheckStickerSetNameResult
  for &CHECKSTICKERSETNAMERESULT
{
}
impl<CHECKSTICKERSETNAMERESULT: TDCheckStickerSetNameResult> TDCheckStickerSetNameResult
  for &mut CHECKSTICKERSETNAMERESULT
{
}

impl<CONNECTIONSTATE: TDConnectionState> TDConnectionState for &CONNECTIONSTATE {}
impl<CONNECTIONSTATE: TDConnectionState> TDConnectionState for &mut CONNECTIONSTATE {}

impl<DEVICETOKEN: TDDeviceToken> TDDeviceToken for &DEVICETOKEN {}
impl<DEVICETOKEN: TDDeviceToken> TDDeviceToken for &mut DEVICETOKEN {}

impl<DICESTICKERS: TDDiceStickers> TDDiceStickers for &DICESTICKERS {}
impl<DICESTICKERS: TDDiceStickers> TDDiceStickers for &mut DICESTICKERS {}

impl<FILETYPE: TDFileType> TDFileType for &FILETYPE {}
impl<FILETYPE: TDFileType> TDFileType for &mut FILETYPE {}

impl<GROUPCALLVIDEOQUALITY: TDGroupCallVideoQuality> TDGroupCallVideoQuality
  for &GROUPCALLVIDEOQUALITY
{
}
impl<GROUPCALLVIDEOQUALITY: TDGroupCallVideoQuality> TDGroupCallVideoQuality
  for &mut GROUPCALLVIDEOQUALITY
{
}

impl<INLINEKEYBOARDBUTTONTYPE: TDInlineKeyboardButtonType> TDInlineKeyboardButtonType
  for &INLINEKEYBOARDBUTTONTYPE
{
}
impl<INLINEKEYBOARDBUTTONTYPE: TDInlineKeyboardButtonType> TDInlineKeyboardButtonType
  for &mut INLINEKEYBOARDBUTTONTYPE
{
}

impl<INLINEQUERYRESULT: TDInlineQueryResult> TDInlineQueryResult for &INLINEQUERYRESULT {}
impl<INLINEQUERYRESULT: TDInlineQueryResult> TDInlineQueryResult for &mut INLINEQUERYRESULT {}

impl<INPUTBACKGROUND: TDInputBackground> TDInputBackground for &INPUTBACKGROUND {}
impl<INPUTBACKGROUND: TDInputBackground> TDInputBackground for &mut INPUTBACKGROUND {}

impl<INPUTCHATPHOTO: TDInputChatPhoto> TDInputChatPhoto for &INPUTCHATPHOTO {}
impl<INPUTCHATPHOTO: TDInputChatPhoto> TDInputChatPhoto for &mut INPUTCHATPHOTO {}

impl<INPUTCREDENTIALS: TDInputCredentials> TDInputCredentials for &INPUTCREDENTIALS {}
impl<INPUTCREDENTIALS: TDInputCredentials> TDInputCredentials for &mut INPUTCREDENTIALS {}

impl<INPUTFILE: TDInputFile> TDInputFile for &INPUTFILE {}
impl<INPUTFILE: TDInputFile> TDInputFile for &mut INPUTFILE {}

impl<INPUTINLINEQUERYRESULT: TDInputInlineQueryResult> TDInputInlineQueryResult
  for &INPUTINLINEQUERYRESULT
{
}
impl<INPUTINLINEQUERYRESULT: TDInputInlineQueryResult> TDInputInlineQueryResult
  for &mut INPUTINLINEQUERYRESULT
{
}

impl<INPUTMESSAGECONTENT: TDInputMessageContent> TDInputMessageContent for &INPUTMESSAGECONTENT {}
impl<INPUTMESSAGECONTENT: TDInputMessageContent> TDInputMessageContent
  for &mut INPUTMESSAGECONTENT
{
}

impl<INPUTPASSPORTELEMENT: TDInputPassportElement> TDInputPassportElement
  for &INPUTPASSPORTELEMENT
{
}
impl<INPUTPASSPORTELEMENT: TDInputPassportElement> TDInputPassportElement
  for &mut INPUTPASSPORTELEMENT
{
}

impl<INPUTPASSPORTELEMENTERRORSOURCE: TDInputPassportElementErrorSource>
  TDInputPassportElementErrorSource for &INPUTPASSPORTELEMENTERRORSOURCE
{
}
impl<INPUTPASSPORTELEMENTERRORSOURCE: TDInputPassportElementErrorSource>
  TDInputPassportElementErrorSource for &mut INPUTPASSPORTELEMENTERRORSOURCE
{
}

impl<INPUTSTICKER: TDInputSticker> TDInputSticker for &INPUTSTICKER {}
impl<INPUTSTICKER: TDInputSticker> TDInputSticker for &mut INPUTSTICKER {}

impl<INTERNALLINKTYPE: TDInternalLinkType> TDInternalLinkType for &INTERNALLINKTYPE {}
impl<INTERNALLINKTYPE: TDInternalLinkType> TDInternalLinkType for &mut INTERNALLINKTYPE {}

impl<JSONVALUE: TDJsonValue> TDJsonValue for &JSONVALUE {}
impl<JSONVALUE: TDJsonValue> TDJsonValue for &mut JSONVALUE {}

impl<KEYBOARDBUTTONTYPE: TDKeyboardButtonType> TDKeyboardButtonType for &KEYBOARDBUTTONTYPE {}
impl<KEYBOARDBUTTONTYPE: TDKeyboardButtonType> TDKeyboardButtonType for &mut KEYBOARDBUTTONTYPE {}

impl<LANGUAGEPACKSTRINGVALUE: TDLanguagePackStringValue> TDLanguagePackStringValue
  for &LANGUAGEPACKSTRINGVALUE
{
}
impl<LANGUAGEPACKSTRINGVALUE: TDLanguagePackStringValue> TDLanguagePackStringValue
  for &mut LANGUAGEPACKSTRINGVALUE
{
}

impl<LOGSTREAM: TDLogStream> TDLogStream for &LOGSTREAM {}
impl<LOGSTREAM: TDLogStream> TDLogStream for &mut LOGSTREAM {}

impl<LOGINURLINFO: TDLoginUrlInfo> TDLoginUrlInfo for &LOGINURLINFO {}
impl<LOGINURLINFO: TDLoginUrlInfo> TDLoginUrlInfo for &mut LOGINURLINFO {}

impl<MASKPOINT: TDMaskPoint> TDMaskPoint for &MASKPOINT {}
impl<MASKPOINT: TDMaskPoint> TDMaskPoint for &mut MASKPOINT {}

impl<MESSAGECONTENT: TDMessageContent> TDMessageContent for &MESSAGECONTENT {}
impl<MESSAGECONTENT: TDMessageContent> TDMessageContent for &mut MESSAGECONTENT {}

impl<MESSAGEFILETYPE: TDMessageFileType> TDMessageFileType for &MESSAGEFILETYPE {}
impl<MESSAGEFILETYPE: TDMessageFileType> TDMessageFileType for &mut MESSAGEFILETYPE {}

impl<MESSAGEFORWARDORIGIN: TDMessageForwardOrigin> TDMessageForwardOrigin
  for &MESSAGEFORWARDORIGIN
{
}
impl<MESSAGEFORWARDORIGIN: TDMessageForwardOrigin> TDMessageForwardOrigin
  for &mut MESSAGEFORWARDORIGIN
{
}

impl<MESSAGESCHEDULINGSTATE: TDMessageSchedulingState> TDMessageSchedulingState
  for &MESSAGESCHEDULINGSTATE
{
}
impl<MESSAGESCHEDULINGSTATE: TDMessageSchedulingState> TDMessageSchedulingState
  for &mut MESSAGESCHEDULINGSTATE
{
}

impl<MESSAGESENDER: TDMessageSender> TDMessageSender for &MESSAGESENDER {}
impl<MESSAGESENDER: TDMessageSender> TDMessageSender for &mut MESSAGESENDER {}

impl<MESSAGESENDINGSTATE: TDMessageSendingState> TDMessageSendingState for &MESSAGESENDINGSTATE {}
impl<MESSAGESENDINGSTATE: TDMessageSendingState> TDMessageSendingState
  for &mut MESSAGESENDINGSTATE
{
}

impl<NETWORKSTATISTICSENTRY: TDNetworkStatisticsEntry> TDNetworkStatisticsEntry
  for &NETWORKSTATISTICSENTRY
{
}
impl<NETWORKSTATISTICSENTRY: TDNetworkStatisticsEntry> TDNetworkStatisticsEntry
  for &mut NETWORKSTATISTICSENTRY
{
}

impl<NETWORKTYPE: TDNetworkType> TDNetworkType for &NETWORKTYPE {}
impl<NETWORKTYPE: TDNetworkType> TDNetworkType for &mut NETWORKTYPE {}

impl<NOTIFICATIONGROUPTYPE: TDNotificationGroupType> TDNotificationGroupType
  for &NOTIFICATIONGROUPTYPE
{
}
impl<NOTIFICATIONGROUPTYPE: TDNotificationGroupType> TDNotificationGroupType
  for &mut NOTIFICATIONGROUPTYPE
{
}

impl<NOTIFICATIONSETTINGSSCOPE: TDNotificationSettingsScope> TDNotificationSettingsScope
  for &NOTIFICATIONSETTINGSSCOPE
{
}
impl<NOTIFICATIONSETTINGSSCOPE: TDNotificationSettingsScope> TDNotificationSettingsScope
  for &mut NOTIFICATIONSETTINGSSCOPE
{
}

impl<NOTIFICATIONTYPE: TDNotificationType> TDNotificationType for &NOTIFICATIONTYPE {}
impl<NOTIFICATIONTYPE: TDNotificationType> TDNotificationType for &mut NOTIFICATIONTYPE {}

impl<OPTIONVALUE: TDOptionValue> TDOptionValue for &OPTIONVALUE {}
impl<OPTIONVALUE: TDOptionValue> TDOptionValue for &mut OPTIONVALUE {}

impl<PAGEBLOCK: TDPageBlock> TDPageBlock for &PAGEBLOCK {}
impl<PAGEBLOCK: TDPageBlock> TDPageBlock for &mut PAGEBLOCK {}

impl<PAGEBLOCKHORIZONTALALIGNMENT: TDPageBlockHorizontalAlignment> TDPageBlockHorizontalAlignment
  for &PAGEBLOCKHORIZONTALALIGNMENT
{
}
impl<PAGEBLOCKHORIZONTALALIGNMENT: TDPageBlockHorizontalAlignment> TDPageBlockHorizontalAlignment
  for &mut PAGEBLOCKHORIZONTALALIGNMENT
{
}

impl<PAGEBLOCKVERTICALALIGNMENT: TDPageBlockVerticalAlignment> TDPageBlockVerticalAlignment
  for &PAGEBLOCKVERTICALALIGNMENT
{
}
impl<PAGEBLOCKVERTICALALIGNMENT: TDPageBlockVerticalAlignment> TDPageBlockVerticalAlignment
  for &mut PAGEBLOCKVERTICALALIGNMENT
{
}

impl<PASSPORTELEMENT: TDPassportElement> TDPassportElement for &PASSPORTELEMENT {}
impl<PASSPORTELEMENT: TDPassportElement> TDPassportElement for &mut PASSPORTELEMENT {}

impl<PASSPORTELEMENTERRORSOURCE: TDPassportElementErrorSource> TDPassportElementErrorSource
  for &PASSPORTELEMENTERRORSOURCE
{
}
impl<PASSPORTELEMENTERRORSOURCE: TDPassportElementErrorSource> TDPassportElementErrorSource
  for &mut PASSPORTELEMENTERRORSOURCE
{
}

impl<PASSPORTELEMENTTYPE: TDPassportElementType> TDPassportElementType for &PASSPORTELEMENTTYPE {}
impl<PASSPORTELEMENTTYPE: TDPassportElementType> TDPassportElementType
  for &mut PASSPORTELEMENTTYPE
{
}

impl<POLLTYPE: TDPollType> TDPollType for &POLLTYPE {}
impl<POLLTYPE: TDPollType> TDPollType for &mut POLLTYPE {}

impl<PROXYTYPE: TDProxyType> TDProxyType for &PROXYTYPE {}
impl<PROXYTYPE: TDProxyType> TDProxyType for &mut PROXYTYPE {}

impl<PUBLICCHATTYPE: TDPublicChatType> TDPublicChatType for &PUBLICCHATTYPE {}
impl<PUBLICCHATTYPE: TDPublicChatType> TDPublicChatType for &mut PUBLICCHATTYPE {}

impl<PUSHMESSAGECONTENT: TDPushMessageContent> TDPushMessageContent for &PUSHMESSAGECONTENT {}
impl<PUSHMESSAGECONTENT: TDPushMessageContent> TDPushMessageContent for &mut PUSHMESSAGECONTENT {}

impl<REPLYMARKUP: TDReplyMarkup> TDReplyMarkup for &REPLYMARKUP {}
impl<REPLYMARKUP: TDReplyMarkup> TDReplyMarkup for &mut REPLYMARKUP {}

impl<RESETPASSWORDRESULT: TDResetPasswordResult> TDResetPasswordResult for &RESETPASSWORDRESULT {}
impl<RESETPASSWORDRESULT: TDResetPasswordResult> TDResetPasswordResult
  for &mut RESETPASSWORDRESULT
{
}

impl<RICHTEXT: TDRichText> TDRichText for &RICHTEXT {}
impl<RICHTEXT: TDRichText> TDRichText for &mut RICHTEXT {}

impl<SEARCHMESSAGESFILTER: TDSearchMessagesFilter> TDSearchMessagesFilter
  for &SEARCHMESSAGESFILTER
{
}
impl<SEARCHMESSAGESFILTER: TDSearchMessagesFilter> TDSearchMessagesFilter
  for &mut SEARCHMESSAGESFILTER
{
}

impl<SECRETCHATSTATE: TDSecretChatState> TDSecretChatState for &SECRETCHATSTATE {}
impl<SECRETCHATSTATE: TDSecretChatState> TDSecretChatState for &mut SECRETCHATSTATE {}

impl<STATISTICALGRAPH: TDStatisticalGraph> TDStatisticalGraph for &STATISTICALGRAPH {}
impl<STATISTICALGRAPH: TDStatisticalGraph> TDStatisticalGraph for &mut STATISTICALGRAPH {}

impl<SUGGESTEDACTION: TDSuggestedAction> TDSuggestedAction for &SUGGESTEDACTION {}
impl<SUGGESTEDACTION: TDSuggestedAction> TDSuggestedAction for &mut SUGGESTEDACTION {}

impl<SUPERGROUPMEMBERSFILTER: TDSupergroupMembersFilter> TDSupergroupMembersFilter
  for &SUPERGROUPMEMBERSFILTER
{
}
impl<SUPERGROUPMEMBERSFILTER: TDSupergroupMembersFilter> TDSupergroupMembersFilter
  for &mut SUPERGROUPMEMBERSFILTER
{
}

impl<TMEURLTYPE: TDTMeUrlType> TDTMeUrlType for &TMEURLTYPE {}
impl<TMEURLTYPE: TDTMeUrlType> TDTMeUrlType for &mut TMEURLTYPE {}

impl<TEXTENTITYTYPE: TDTextEntityType> TDTextEntityType for &TEXTENTITYTYPE {}
impl<TEXTENTITYTYPE: TDTextEntityType> TDTextEntityType for &mut TEXTENTITYTYPE {}

impl<TEXTPARSEMODE: TDTextParseMode> TDTextParseMode for &TEXTPARSEMODE {}
impl<TEXTPARSEMODE: TDTextParseMode> TDTextParseMode for &mut TEXTPARSEMODE {}

impl<THUMBNAILFORMAT: TDThumbnailFormat> TDThumbnailFormat for &THUMBNAILFORMAT {}
impl<THUMBNAILFORMAT: TDThumbnailFormat> TDThumbnailFormat for &mut THUMBNAILFORMAT {}

impl<TOPCHATCATEGORY: TDTopChatCategory> TDTopChatCategory for &TOPCHATCATEGORY {}
impl<TOPCHATCATEGORY: TDTopChatCategory> TDTopChatCategory for &mut TOPCHATCATEGORY {}

impl<UPDATE: TDUpdate> TDUpdate for &UPDATE {}
impl<UPDATE: TDUpdate> TDUpdate for &mut UPDATE {}

impl<USERPRIVACYSETTING: TDUserPrivacySetting> TDUserPrivacySetting for &USERPRIVACYSETTING {}
impl<USERPRIVACYSETTING: TDUserPrivacySetting> TDUserPrivacySetting for &mut USERPRIVACYSETTING {}

impl<USERPRIVACYSETTINGRULE: TDUserPrivacySettingRule> TDUserPrivacySettingRule
  for &USERPRIVACYSETTINGRULE
{
}
impl<USERPRIVACYSETTINGRULE: TDUserPrivacySettingRule> TDUserPrivacySettingRule
  for &mut USERPRIVACYSETTINGRULE
{
}

impl<USERSTATUS: TDUserStatus> TDUserStatus for &USERSTATUS {}
impl<USERSTATUS: TDUserStatus> TDUserStatus for &mut USERSTATUS {}

impl<USERTYPE: TDUserType> TDUserType for &USERTYPE {}
impl<USERTYPE: TDUserType> TDUserType for &mut USERTYPE {}

impl<VECTORPATHCOMMAND: TDVectorPathCommand> TDVectorPathCommand for &VECTORPATHCOMMAND {}
impl<VECTORPATHCOMMAND: TDVectorPathCommand> TDVectorPathCommand for &mut VECTORPATHCOMMAND {}

#[derive(Debug, Clone)]
pub enum TdType {
  TestUseUpdate(TestUseUpdate),
  UpdateActiveNotifications(UpdateActiveNotifications),
  UpdateAnimatedEmojiMessageClicked(UpdateAnimatedEmojiMessageClicked),
  UpdateAnimationSearchParameters(UpdateAnimationSearchParameters),
  UpdateAuthorizationState(UpdateAuthorizationState),
  UpdateBasicGroup(UpdateBasicGroup),
  UpdateBasicGroupFullInfo(UpdateBasicGroupFullInfo),
  UpdateCall(UpdateCall),
  UpdateChatAction(UpdateChatAction),
  UpdateChatActionBar(UpdateChatActionBar),
  UpdateChatDefaultDisableNotification(UpdateChatDefaultDisableNotification),
  UpdateChatDraftMessage(UpdateChatDraftMessage),
  UpdateChatFilters(UpdateChatFilters),
  UpdateChatHasProtectedContent(UpdateChatHasProtectedContent),
  UpdateChatHasScheduledMessages(UpdateChatHasScheduledMessages),
  UpdateChatIsBlocked(UpdateChatIsBlocked),
  UpdateChatIsMarkedAsUnread(UpdateChatIsMarkedAsUnread),
  UpdateChatLastMessage(UpdateChatLastMessage),
  UpdateChatMember(UpdateChatMember),
  UpdateChatMessageSender(UpdateChatMessageSender),
  UpdateChatMessageTtl(UpdateChatMessageTtl),
  UpdateChatNotificationSettings(UpdateChatNotificationSettings),
  UpdateChatOnlineMemberCount(UpdateChatOnlineMemberCount),
  UpdateChatPendingJoinRequests(UpdateChatPendingJoinRequests),
  UpdateChatPermissions(UpdateChatPermissions),
  UpdateChatPhoto(UpdateChatPhoto),
  UpdateChatPosition(UpdateChatPosition),
  UpdateChatReadInbox(UpdateChatReadInbox),
  UpdateChatReadOutbox(UpdateChatReadOutbox),
  UpdateChatReplyMarkup(UpdateChatReplyMarkup),
  UpdateChatTheme(UpdateChatTheme),
  UpdateChatThemes(UpdateChatThemes),
  UpdateChatTitle(UpdateChatTitle),
  UpdateChatUnreadMentionCount(UpdateChatUnreadMentionCount),
  UpdateChatVideoChat(UpdateChatVideoChat),
  UpdateConnectionState(UpdateConnectionState),
  UpdateDeleteMessages(UpdateDeleteMessages),
  UpdateDiceEmojis(UpdateDiceEmojis),
  UpdateFavoriteStickers(UpdateFavoriteStickers),
  UpdateFile(UpdateFile),
  UpdateFileGenerationStart(UpdateFileGenerationStart),
  UpdateFileGenerationStop(UpdateFileGenerationStop),
  UpdateGroupCall(UpdateGroupCall),
  UpdateGroupCallParticipant(UpdateGroupCallParticipant),
  UpdateHavePendingNotifications(UpdateHavePendingNotifications),
  UpdateInstalledStickerSets(UpdateInstalledStickerSets),
  UpdateLanguagePackStrings(UpdateLanguagePackStrings),
  UpdateMessageContent(UpdateMessageContent),
  UpdateMessageContentOpened(UpdateMessageContentOpened),
  UpdateMessageEdited(UpdateMessageEdited),
  UpdateMessageInteractionInfo(UpdateMessageInteractionInfo),
  UpdateMessageIsPinned(UpdateMessageIsPinned),
  UpdateMessageLiveLocationViewed(UpdateMessageLiveLocationViewed),
  UpdateMessageMentionRead(UpdateMessageMentionRead),
  UpdateMessageSendAcknowledged(UpdateMessageSendAcknowledged),
  UpdateMessageSendFailed(UpdateMessageSendFailed),
  UpdateMessageSendSucceeded(UpdateMessageSendSucceeded),
  UpdateNewCallSignalingData(UpdateNewCallSignalingData),
  UpdateNewCallbackQuery(UpdateNewCallbackQuery),
  UpdateNewChat(UpdateNewChat),
  UpdateNewChatJoinRequest(UpdateNewChatJoinRequest),
  UpdateNewChosenInlineResult(UpdateNewChosenInlineResult),
  UpdateNewCustomEvent(UpdateNewCustomEvent),
  UpdateNewCustomQuery(UpdateNewCustomQuery),
  UpdateNewInlineCallbackQuery(UpdateNewInlineCallbackQuery),
  UpdateNewInlineQuery(UpdateNewInlineQuery),
  UpdateNewMessage(UpdateNewMessage),
  UpdateNewPreCheckoutQuery(UpdateNewPreCheckoutQuery),
  UpdateNewShippingQuery(UpdateNewShippingQuery),
  UpdateNotification(UpdateNotification),
  UpdateNotificationGroup(UpdateNotificationGroup),
  UpdateOption(UpdateOption),
  UpdatePoll(UpdatePoll),
  UpdatePollAnswer(UpdatePollAnswer),
  UpdateRecentStickers(UpdateRecentStickers),
  UpdateSavedAnimations(UpdateSavedAnimations),
  UpdateScopeNotificationSettings(UpdateScopeNotificationSettings),
  UpdateSecretChat(UpdateSecretChat),
  UpdateSelectedBackground(UpdateSelectedBackground),
  UpdateServiceNotification(UpdateServiceNotification),
  UpdateStickerSet(UpdateStickerSet),
  UpdateSuggestedActions(UpdateSuggestedActions),
  UpdateSupergroup(UpdateSupergroup),
  UpdateSupergroupFullInfo(UpdateSupergroupFullInfo),
  UpdateTermsOfService(UpdateTermsOfService),
  UpdateTrendingStickerSets(UpdateTrendingStickerSets),
  UpdateUnreadChatCount(UpdateUnreadChatCount),
  UpdateUnreadMessageCount(UpdateUnreadMessageCount),
  UpdateUser(UpdateUser),
  UpdateUserFullInfo(UpdateUserFullInfo),
  UpdateUserPrivacySettingRules(UpdateUserPrivacySettingRules),
  UpdateUserStatus(UpdateUserStatus),
  UpdateUsersNearby(UpdateUsersNearby),

  AuthorizationState(AuthorizationState),
  CanTransferOwnershipResult(CanTransferOwnershipResult),
  ChatStatistics(ChatStatistics),
  CheckChatUsernameResult(CheckChatUsernameResult),
  CheckStickerSetNameResult(CheckStickerSetNameResult),
  InternalLinkType(InternalLinkType),
  JsonValue(JsonValue),
  LanguagePackStringValue(LanguagePackStringValue),
  LogStream(LogStream),
  LoginUrlInfo(LoginUrlInfo),
  MessageFileType(MessageFileType),
  OptionValue(OptionValue),
  PassportElement(PassportElement),
  ResetPasswordResult(ResetPasswordResult),
  StatisticalGraph(StatisticalGraph),
  Update(Update),
  AccountTtl(AccountTtl),
  AnimatedEmoji(AnimatedEmoji),
  Animations(Animations),
  AuthenticationCodeInfo(AuthenticationCodeInfo),
  AutoDownloadSettingsPresets(AutoDownloadSettingsPresets),
  Background(Background),
  Backgrounds(Backgrounds),
  BankCardInfo(BankCardInfo),
  BasicGroup(BasicGroup),
  BasicGroupFullInfo(BasicGroupFullInfo),
  BotCommands(BotCommands),
  CallId(CallId),
  CallbackQueryAnswer(CallbackQueryAnswer),
  Chat(Chat),
  ChatAdministrators(ChatAdministrators),
  ChatEvents(ChatEvents),
  ChatFilter(ChatFilter),
  ChatFilterInfo(ChatFilterInfo),
  ChatInviteLink(ChatInviteLink),
  ChatInviteLinkCounts(ChatInviteLinkCounts),
  ChatInviteLinkInfo(ChatInviteLinkInfo),
  ChatInviteLinkMembers(ChatInviteLinkMembers),
  ChatInviteLinks(ChatInviteLinks),
  ChatJoinRequests(ChatJoinRequests),
  ChatLists(ChatLists),
  ChatMember(ChatMember),
  ChatMembers(ChatMembers),
  ChatPhotos(ChatPhotos),
  Chats(Chats),
  ChatsNearby(ChatsNearby),
  ConnectedWebsites(ConnectedWebsites),
  Count(Count),
  Countries(Countries),
  CustomRequestResult(CustomRequestResult),
  DatabaseStatistics(DatabaseStatistics),
  DeepLinkInfo(DeepLinkInfo),
  EmailAddressAuthenticationCodeInfo(EmailAddressAuthenticationCodeInfo),
  Emojis(Emojis),
  Error(Error),
  File(File),
  FilePart(FilePart),
  FormattedText(FormattedText),
  FoundMessages(FoundMessages),
  GameHighScores(GameHighScores),
  GroupCall(GroupCall),
  GroupCallId(GroupCallId),
  Hashtags(Hashtags),
  HttpUrl(HttpUrl),
  ImportedContacts(ImportedContacts),
  InlineQueryResults(InlineQueryResults),
  LanguagePackInfo(LanguagePackInfo),
  LanguagePackStrings(LanguagePackStrings),
  LocalizationTargetInfo(LocalizationTargetInfo),
  LogTags(LogTags),
  LogVerbosityLevel(LogVerbosityLevel),
  Message(Message),
  MessageCalendar(MessageCalendar),
  MessageLink(MessageLink),
  MessageLinkInfo(MessageLinkInfo),
  MessagePositions(MessagePositions),
  MessageSenders(MessageSenders),
  MessageStatistics(MessageStatistics),
  MessageThreadInfo(MessageThreadInfo),
  Messages(Messages),
  NetworkStatistics(NetworkStatistics),
  Ok(Ok),
  OrderInfo(OrderInfo),
  PassportAuthorizationForm(PassportAuthorizationForm),
  PassportElements(PassportElements),
  PassportElementsWithErrors(PassportElementsWithErrors),
  PasswordState(PasswordState),
  PaymentForm(PaymentForm),
  PaymentReceipt(PaymentReceipt),
  PaymentResult(PaymentResult),
  PhoneNumberInfo(PhoneNumberInfo),
  Proxies(Proxies),
  Proxy(Proxy),
  PushReceiverId(PushReceiverId),
  RecommendedChatFilters(RecommendedChatFilters),
  RecoveryEmailAddress(RecoveryEmailAddress),
  ScopeNotificationSettings(ScopeNotificationSettings),
  Seconds(Seconds),
  SecretChat(SecretChat),
  Session(Session),
  Sessions(Sessions),
  SponsoredMessage(SponsoredMessage),
  Sticker(Sticker),
  StickerSet(StickerSet),
  StickerSets(StickerSets),
  Stickers(Stickers),
  StorageStatistics(StorageStatistics),
  StorageStatisticsFast(StorageStatisticsFast),
  Supergroup(Supergroup),
  SupergroupFullInfo(SupergroupFullInfo),
  TMeUrls(TMeUrls),
  TemporaryPasswordState(TemporaryPasswordState),
  TestBytes(TestBytes),
  TestInt(TestInt),
  TestString(TestString),
  TestVectorInt(TestVectorInt),
  TestVectorIntObject(TestVectorIntObject),
  TestVectorString(TestVectorString),
  TestVectorStringObject(TestVectorStringObject),
  Text(Text),
  TextEntities(TextEntities),
  Updates(Updates),
  User(User),
  UserFullInfo(UserFullInfo),
  UserPrivacySettingRules(UserPrivacySettingRules),
  Users(Users),
  ValidatedOrderInfo(ValidatedOrderInfo),
  WebPage(WebPage),
  WebPageInstantView(WebPageInstantView),
}
impl<'de> Deserialize<'de> for TdType {
  fn deserialize<D>(deserializer: D) -> Result<TdType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
         TdType,
     (testUseUpdate, TestUseUpdate);
     (updateActiveNotifications, UpdateActiveNotifications);
     (updateAnimatedEmojiMessageClicked, UpdateAnimatedEmojiMessageClicked);
     (updateAnimationSearchParameters, UpdateAnimationSearchParameters);
     (updateAuthorizationState, UpdateAuthorizationState);
     (updateBasicGroup, UpdateBasicGroup);
     (updateBasicGroupFullInfo, UpdateBasicGroupFullInfo);
     (updateCall, UpdateCall);
     (updateChatAction, UpdateChatAction);
     (updateChatActionBar, UpdateChatActionBar);
     (updateChatDefaultDisableNotification, UpdateChatDefaultDisableNotification);
     (updateChatDraftMessage, UpdateChatDraftMessage);
     (updateChatFilters, UpdateChatFilters);
     (updateChatHasProtectedContent, UpdateChatHasProtectedContent);
     (updateChatHasScheduledMessages, UpdateChatHasScheduledMessages);
     (updateChatIsBlocked, UpdateChatIsBlocked);
     (updateChatIsMarkedAsUnread, UpdateChatIsMarkedAsUnread);
     (updateChatLastMessage, UpdateChatLastMessage);
     (updateChatMember, UpdateChatMember);
     (updateChatMessageSender, UpdateChatMessageSender);
     (updateChatMessageTtl, UpdateChatMessageTtl);
     (updateChatNotificationSettings, UpdateChatNotificationSettings);
     (updateChatOnlineMemberCount, UpdateChatOnlineMemberCount);
     (updateChatPendingJoinRequests, UpdateChatPendingJoinRequests);
     (updateChatPermissions, UpdateChatPermissions);
     (updateChatPhoto, UpdateChatPhoto);
     (updateChatPosition, UpdateChatPosition);
     (updateChatReadInbox, UpdateChatReadInbox);
     (updateChatReadOutbox, UpdateChatReadOutbox);
     (updateChatReplyMarkup, UpdateChatReplyMarkup);
     (updateChatTheme, UpdateChatTheme);
     (updateChatThemes, UpdateChatThemes);
     (updateChatTitle, UpdateChatTitle);
     (updateChatUnreadMentionCount, UpdateChatUnreadMentionCount);
     (updateChatVideoChat, UpdateChatVideoChat);
     (updateConnectionState, UpdateConnectionState);
     (updateDeleteMessages, UpdateDeleteMessages);
     (updateDiceEmojis, UpdateDiceEmojis);
     (updateFavoriteStickers, UpdateFavoriteStickers);
     (updateFile, UpdateFile);
     (updateFileGenerationStart, UpdateFileGenerationStart);
     (updateFileGenerationStop, UpdateFileGenerationStop);
     (updateGroupCall, UpdateGroupCall);
     (updateGroupCallParticipant, UpdateGroupCallParticipant);
     (updateHavePendingNotifications, UpdateHavePendingNotifications);
     (updateInstalledStickerSets, UpdateInstalledStickerSets);
     (updateLanguagePackStrings, UpdateLanguagePackStrings);
     (updateMessageContent, UpdateMessageContent);
     (updateMessageContentOpened, UpdateMessageContentOpened);
     (updateMessageEdited, UpdateMessageEdited);
     (updateMessageInteractionInfo, UpdateMessageInteractionInfo);
     (updateMessageIsPinned, UpdateMessageIsPinned);
     (updateMessageLiveLocationViewed, UpdateMessageLiveLocationViewed);
     (updateMessageMentionRead, UpdateMessageMentionRead);
     (updateMessageSendAcknowledged, UpdateMessageSendAcknowledged);
     (updateMessageSendFailed, UpdateMessageSendFailed);
     (updateMessageSendSucceeded, UpdateMessageSendSucceeded);
     (updateNewCallSignalingData, UpdateNewCallSignalingData);
     (updateNewCallbackQuery, UpdateNewCallbackQuery);
     (updateNewChat, UpdateNewChat);
     (updateNewChatJoinRequest, UpdateNewChatJoinRequest);
     (updateNewChosenInlineResult, UpdateNewChosenInlineResult);
     (updateNewCustomEvent, UpdateNewCustomEvent);
     (updateNewCustomQuery, UpdateNewCustomQuery);
     (updateNewInlineCallbackQuery, UpdateNewInlineCallbackQuery);
     (updateNewInlineQuery, UpdateNewInlineQuery);
     (updateNewMessage, UpdateNewMessage);
     (updateNewPreCheckoutQuery, UpdateNewPreCheckoutQuery);
     (updateNewShippingQuery, UpdateNewShippingQuery);
     (updateNotification, UpdateNotification);
     (updateNotificationGroup, UpdateNotificationGroup);
     (updateOption, UpdateOption);
     (updatePoll, UpdatePoll);
     (updatePollAnswer, UpdatePollAnswer);
     (updateRecentStickers, UpdateRecentStickers);
     (updateSavedAnimations, UpdateSavedAnimations);
     (updateScopeNotificationSettings, UpdateScopeNotificationSettings);
     (updateSecretChat, UpdateSecretChat);
     (updateSelectedBackground, UpdateSelectedBackground);
     (updateServiceNotification, UpdateServiceNotification);
     (updateStickerSet, UpdateStickerSet);
     (updateSuggestedActions, UpdateSuggestedActions);
     (updateSupergroup, UpdateSupergroup);
     (updateSupergroupFullInfo, UpdateSupergroupFullInfo);
     (updateTermsOfService, UpdateTermsOfService);
     (updateTrendingStickerSets, UpdateTrendingStickerSets);
     (updateUnreadChatCount, UpdateUnreadChatCount);
     (updateUnreadMessageCount, UpdateUnreadMessageCount);
     (updateUser, UpdateUser);
     (updateUserFullInfo, UpdateUserFullInfo);
     (updateUserPrivacySettingRules, UpdateUserPrivacySettingRules);
     (updateUserStatus, UpdateUserStatus);
     (updateUsersNearby, UpdateUsersNearby);

     (AuthorizationState, AuthorizationState);
     (CanTransferOwnershipResult, CanTransferOwnershipResult);
     (ChatStatistics, ChatStatistics);
     (CheckChatUsernameResult, CheckChatUsernameResult);
     (CheckStickerSetNameResult, CheckStickerSetNameResult);
     (InternalLinkType, InternalLinkType);
     (JsonValue, JsonValue);
     (LanguagePackStringValue, LanguagePackStringValue);
     (LogStream, LogStream);
     (LoginUrlInfo, LoginUrlInfo);
     (MessageFileType, MessageFileType);
     (OptionValue, OptionValue);
     (PassportElement, PassportElement);
     (ResetPasswordResult, ResetPasswordResult);
     (StatisticalGraph, StatisticalGraph);
     (Update, Update);
     (accountTtl, AccountTtl);
     (animatedEmoji, AnimatedEmoji);
     (animations, Animations);
     (authenticationCodeInfo, AuthenticationCodeInfo);
     (autoDownloadSettingsPresets, AutoDownloadSettingsPresets);
     (background, Background);
     (backgrounds, Backgrounds);
     (bankCardInfo, BankCardInfo);
     (basicGroup, BasicGroup);
     (basicGroupFullInfo, BasicGroupFullInfo);
     (botCommands, BotCommands);
     (callId, CallId);
     (callbackQueryAnswer, CallbackQueryAnswer);
     (chat, Chat);
     (chatAdministrators, ChatAdministrators);
     (chatEvents, ChatEvents);
     (chatFilter, ChatFilter);
     (chatFilterInfo, ChatFilterInfo);
     (chatInviteLink, ChatInviteLink);
     (chatInviteLinkCounts, ChatInviteLinkCounts);
     (chatInviteLinkInfo, ChatInviteLinkInfo);
     (chatInviteLinkMembers, ChatInviteLinkMembers);
     (chatInviteLinks, ChatInviteLinks);
     (chatJoinRequests, ChatJoinRequests);
     (chatLists, ChatLists);
     (chatMember, ChatMember);
     (chatMembers, ChatMembers);
     (chatPhotos, ChatPhotos);
     (chats, Chats);
     (chatsNearby, ChatsNearby);
     (connectedWebsites, ConnectedWebsites);
     (count, Count);
     (countries, Countries);
     (customRequestResult, CustomRequestResult);
     (databaseStatistics, DatabaseStatistics);
     (deepLinkInfo, DeepLinkInfo);
     (emailAddressAuthenticationCodeInfo, EmailAddressAuthenticationCodeInfo);
     (emojis, Emojis);
     (error, Error);
     (file, File);
     (filePart, FilePart);
     (formattedText, FormattedText);
     (foundMessages, FoundMessages);
     (gameHighScores, GameHighScores);
     (groupCall, GroupCall);
     (groupCallId, GroupCallId);
     (hashtags, Hashtags);
     (httpUrl, HttpUrl);
     (importedContacts, ImportedContacts);
     (inlineQueryResults, InlineQueryResults);
     (languagePackInfo, LanguagePackInfo);
     (languagePackStrings, LanguagePackStrings);
     (localizationTargetInfo, LocalizationTargetInfo);
     (logTags, LogTags);
     (logVerbosityLevel, LogVerbosityLevel);
     (message, Message);
     (messageCalendar, MessageCalendar);
     (messageLink, MessageLink);
     (messageLinkInfo, MessageLinkInfo);
     (messagePositions, MessagePositions);
     (messageSenders, MessageSenders);
     (messageStatistics, MessageStatistics);
     (messageThreadInfo, MessageThreadInfo);
     (messages, Messages);
     (networkStatistics, NetworkStatistics);
     (ok, Ok);
     (orderInfo, OrderInfo);
     (passportAuthorizationForm, PassportAuthorizationForm);
     (passportElements, PassportElements);
     (passportElementsWithErrors, PassportElementsWithErrors);
     (passwordState, PasswordState);
     (paymentForm, PaymentForm);
     (paymentReceipt, PaymentReceipt);
     (paymentResult, PaymentResult);
     (phoneNumberInfo, PhoneNumberInfo);
     (proxies, Proxies);
     (proxy, Proxy);
     (pushReceiverId, PushReceiverId);
     (recommendedChatFilters, RecommendedChatFilters);
     (recoveryEmailAddress, RecoveryEmailAddress);
     (scopeNotificationSettings, ScopeNotificationSettings);
     (seconds, Seconds);
     (secretChat, SecretChat);
     (session, Session);
     (sessions, Sessions);
     (sponsoredMessage, SponsoredMessage);
     (sticker, Sticker);
     (stickerSet, StickerSet);
     (stickerSets, StickerSets);
     (stickers, Stickers);
     (storageStatistics, StorageStatistics);
     (storageStatisticsFast, StorageStatisticsFast);
     (supergroup, Supergroup);
     (supergroupFullInfo, SupergroupFullInfo);
     (tMeUrls, TMeUrls);
     (temporaryPasswordState, TemporaryPasswordState);
     (testBytes, TestBytes);
     (testInt, TestInt);
     (testString, TestString);
     (testVectorInt, TestVectorInt);
     (testVectorIntObject, TestVectorIntObject);
     (testVectorString, TestVectorString);
     (testVectorStringObject, TestVectorStringObject);
     (text, Text);
     (textEntities, TextEntities);
     (updates, Updates);
     (user, User);
     (userFullInfo, UserFullInfo);
     (userPrivacySettingRules, UserPrivacySettingRules);
     (users, Users);
     (validatedOrderInfo, ValidatedOrderInfo);
     (webPage, WebPage);
     (webPageInstantView, WebPageInstantView);

    )(deserializer)
  }
}

impl RObject for TdType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      Self::TestUseUpdate(value) => value.td_name(),
      Self::UpdateActiveNotifications(value) => value.td_name(),
      Self::UpdateAnimatedEmojiMessageClicked(value) => value.td_name(),
      Self::UpdateAnimationSearchParameters(value) => value.td_name(),
      Self::UpdateAuthorizationState(value) => value.td_name(),
      Self::UpdateBasicGroup(value) => value.td_name(),
      Self::UpdateBasicGroupFullInfo(value) => value.td_name(),
      Self::UpdateCall(value) => value.td_name(),
      Self::UpdateChatAction(value) => value.td_name(),
      Self::UpdateChatActionBar(value) => value.td_name(),
      Self::UpdateChatDefaultDisableNotification(value) => value.td_name(),
      Self::UpdateChatDraftMessage(value) => value.td_name(),
      Self::UpdateChatFilters(value) => value.td_name(),
      Self::UpdateChatHasProtectedContent(value) => value.td_name(),
      Self::UpdateChatHasScheduledMessages(value) => value.td_name(),
      Self::UpdateChatIsBlocked(value) => value.td_name(),
      Self::UpdateChatIsMarkedAsUnread(value) => value.td_name(),
      Self::UpdateChatLastMessage(value) => value.td_name(),
      Self::UpdateChatMember(value) => value.td_name(),
      Self::UpdateChatMessageSender(value) => value.td_name(),
      Self::UpdateChatMessageTtl(value) => value.td_name(),
      Self::UpdateChatNotificationSettings(value) => value.td_name(),
      Self::UpdateChatOnlineMemberCount(value) => value.td_name(),
      Self::UpdateChatPendingJoinRequests(value) => value.td_name(),
      Self::UpdateChatPermissions(value) => value.td_name(),
      Self::UpdateChatPhoto(value) => value.td_name(),
      Self::UpdateChatPosition(value) => value.td_name(),
      Self::UpdateChatReadInbox(value) => value.td_name(),
      Self::UpdateChatReadOutbox(value) => value.td_name(),
      Self::UpdateChatReplyMarkup(value) => value.td_name(),
      Self::UpdateChatTheme(value) => value.td_name(),
      Self::UpdateChatThemes(value) => value.td_name(),
      Self::UpdateChatTitle(value) => value.td_name(),
      Self::UpdateChatUnreadMentionCount(value) => value.td_name(),
      Self::UpdateChatVideoChat(value) => value.td_name(),
      Self::UpdateConnectionState(value) => value.td_name(),
      Self::UpdateDeleteMessages(value) => value.td_name(),
      Self::UpdateDiceEmojis(value) => value.td_name(),
      Self::UpdateFavoriteStickers(value) => value.td_name(),
      Self::UpdateFile(value) => value.td_name(),
      Self::UpdateFileGenerationStart(value) => value.td_name(),
      Self::UpdateFileGenerationStop(value) => value.td_name(),
      Self::UpdateGroupCall(value) => value.td_name(),
      Self::UpdateGroupCallParticipant(value) => value.td_name(),
      Self::UpdateHavePendingNotifications(value) => value.td_name(),
      Self::UpdateInstalledStickerSets(value) => value.td_name(),
      Self::UpdateLanguagePackStrings(value) => value.td_name(),
      Self::UpdateMessageContent(value) => value.td_name(),
      Self::UpdateMessageContentOpened(value) => value.td_name(),
      Self::UpdateMessageEdited(value) => value.td_name(),
      Self::UpdateMessageInteractionInfo(value) => value.td_name(),
      Self::UpdateMessageIsPinned(value) => value.td_name(),
      Self::UpdateMessageLiveLocationViewed(value) => value.td_name(),
      Self::UpdateMessageMentionRead(value) => value.td_name(),
      Self::UpdateMessageSendAcknowledged(value) => value.td_name(),
      Self::UpdateMessageSendFailed(value) => value.td_name(),
      Self::UpdateMessageSendSucceeded(value) => value.td_name(),
      Self::UpdateNewCallSignalingData(value) => value.td_name(),
      Self::UpdateNewCallbackQuery(value) => value.td_name(),
      Self::UpdateNewChat(value) => value.td_name(),
      Self::UpdateNewChatJoinRequest(value) => value.td_name(),
      Self::UpdateNewChosenInlineResult(value) => value.td_name(),
      Self::UpdateNewCustomEvent(value) => value.td_name(),
      Self::UpdateNewCustomQuery(value) => value.td_name(),
      Self::UpdateNewInlineCallbackQuery(value) => value.td_name(),
      Self::UpdateNewInlineQuery(value) => value.td_name(),
      Self::UpdateNewMessage(value) => value.td_name(),
      Self::UpdateNewPreCheckoutQuery(value) => value.td_name(),
      Self::UpdateNewShippingQuery(value) => value.td_name(),
      Self::UpdateNotification(value) => value.td_name(),
      Self::UpdateNotificationGroup(value) => value.td_name(),
      Self::UpdateOption(value) => value.td_name(),
      Self::UpdatePoll(value) => value.td_name(),
      Self::UpdatePollAnswer(value) => value.td_name(),
      Self::UpdateRecentStickers(value) => value.td_name(),
      Self::UpdateSavedAnimations(value) => value.td_name(),
      Self::UpdateScopeNotificationSettings(value) => value.td_name(),
      Self::UpdateSecretChat(value) => value.td_name(),
      Self::UpdateSelectedBackground(value) => value.td_name(),
      Self::UpdateServiceNotification(value) => value.td_name(),
      Self::UpdateStickerSet(value) => value.td_name(),
      Self::UpdateSuggestedActions(value) => value.td_name(),
      Self::UpdateSupergroup(value) => value.td_name(),
      Self::UpdateSupergroupFullInfo(value) => value.td_name(),
      Self::UpdateTermsOfService(value) => value.td_name(),
      Self::UpdateTrendingStickerSets(value) => value.td_name(),
      Self::UpdateUnreadChatCount(value) => value.td_name(),
      Self::UpdateUnreadMessageCount(value) => value.td_name(),
      Self::UpdateUser(value) => value.td_name(),
      Self::UpdateUserFullInfo(value) => value.td_name(),
      Self::UpdateUserPrivacySettingRules(value) => value.td_name(),
      Self::UpdateUserStatus(value) => value.td_name(),
      Self::UpdateUsersNearby(value) => value.td_name(),

      Self::AuthorizationState(value) => value.td_name(),
      Self::CanTransferOwnershipResult(value) => value.td_name(),
      Self::ChatStatistics(value) => value.td_name(),
      Self::CheckChatUsernameResult(value) => value.td_name(),
      Self::CheckStickerSetNameResult(value) => value.td_name(),
      Self::InternalLinkType(value) => value.td_name(),
      Self::JsonValue(value) => value.td_name(),
      Self::LanguagePackStringValue(value) => value.td_name(),
      Self::LogStream(value) => value.td_name(),
      Self::LoginUrlInfo(value) => value.td_name(),
      Self::MessageFileType(value) => value.td_name(),
      Self::OptionValue(value) => value.td_name(),
      Self::PassportElement(value) => value.td_name(),
      Self::ResetPasswordResult(value) => value.td_name(),
      Self::StatisticalGraph(value) => value.td_name(),
      Self::Update(value) => value.td_name(),
      Self::AccountTtl(value) => value.td_name(),
      Self::AnimatedEmoji(value) => value.td_name(),
      Self::Animations(value) => value.td_name(),
      Self::AuthenticationCodeInfo(value) => value.td_name(),
      Self::AutoDownloadSettingsPresets(value) => value.td_name(),
      Self::Background(value) => value.td_name(),
      Self::Backgrounds(value) => value.td_name(),
      Self::BankCardInfo(value) => value.td_name(),
      Self::BasicGroup(value) => value.td_name(),
      Self::BasicGroupFullInfo(value) => value.td_name(),
      Self::BotCommands(value) => value.td_name(),
      Self::CallId(value) => value.td_name(),
      Self::CallbackQueryAnswer(value) => value.td_name(),
      Self::Chat(value) => value.td_name(),
      Self::ChatAdministrators(value) => value.td_name(),
      Self::ChatEvents(value) => value.td_name(),
      Self::ChatFilter(value) => value.td_name(),
      Self::ChatFilterInfo(value) => value.td_name(),
      Self::ChatInviteLink(value) => value.td_name(),
      Self::ChatInviteLinkCounts(value) => value.td_name(),
      Self::ChatInviteLinkInfo(value) => value.td_name(),
      Self::ChatInviteLinkMembers(value) => value.td_name(),
      Self::ChatInviteLinks(value) => value.td_name(),
      Self::ChatJoinRequests(value) => value.td_name(),
      Self::ChatLists(value) => value.td_name(),
      Self::ChatMember(value) => value.td_name(),
      Self::ChatMembers(value) => value.td_name(),
      Self::ChatPhotos(value) => value.td_name(),
      Self::Chats(value) => value.td_name(),
      Self::ChatsNearby(value) => value.td_name(),
      Self::ConnectedWebsites(value) => value.td_name(),
      Self::Count(value) => value.td_name(),
      Self::Countries(value) => value.td_name(),
      Self::CustomRequestResult(value) => value.td_name(),
      Self::DatabaseStatistics(value) => value.td_name(),
      Self::DeepLinkInfo(value) => value.td_name(),
      Self::EmailAddressAuthenticationCodeInfo(value) => value.td_name(),
      Self::Emojis(value) => value.td_name(),
      Self::Error(value) => value.td_name(),
      Self::File(value) => value.td_name(),
      Self::FilePart(value) => value.td_name(),
      Self::FormattedText(value) => value.td_name(),
      Self::FoundMessages(value) => value.td_name(),
      Self::GameHighScores(value) => value.td_name(),
      Self::GroupCall(value) => value.td_name(),
      Self::GroupCallId(value) => value.td_name(),
      Self::Hashtags(value) => value.td_name(),
      Self::HttpUrl(value) => value.td_name(),
      Self::ImportedContacts(value) => value.td_name(),
      Self::InlineQueryResults(value) => value.td_name(),
      Self::LanguagePackInfo(value) => value.td_name(),
      Self::LanguagePackStrings(value) => value.td_name(),
      Self::LocalizationTargetInfo(value) => value.td_name(),
      Self::LogTags(value) => value.td_name(),
      Self::LogVerbosityLevel(value) => value.td_name(),
      Self::Message(value) => value.td_name(),
      Self::MessageCalendar(value) => value.td_name(),
      Self::MessageLink(value) => value.td_name(),
      Self::MessageLinkInfo(value) => value.td_name(),
      Self::MessagePositions(value) => value.td_name(),
      Self::MessageSenders(value) => value.td_name(),
      Self::MessageStatistics(value) => value.td_name(),
      Self::MessageThreadInfo(value) => value.td_name(),
      Self::Messages(value) => value.td_name(),
      Self::NetworkStatistics(value) => value.td_name(),
      Self::Ok(value) => value.td_name(),
      Self::OrderInfo(value) => value.td_name(),
      Self::PassportAuthorizationForm(value) => value.td_name(),
      Self::PassportElements(value) => value.td_name(),
      Self::PassportElementsWithErrors(value) => value.td_name(),
      Self::PasswordState(value) => value.td_name(),
      Self::PaymentForm(value) => value.td_name(),
      Self::PaymentReceipt(value) => value.td_name(),
      Self::PaymentResult(value) => value.td_name(),
      Self::PhoneNumberInfo(value) => value.td_name(),
      Self::Proxies(value) => value.td_name(),
      Self::Proxy(value) => value.td_name(),
      Self::PushReceiverId(value) => value.td_name(),
      Self::RecommendedChatFilters(value) => value.td_name(),
      Self::RecoveryEmailAddress(value) => value.td_name(),
      Self::ScopeNotificationSettings(value) => value.td_name(),
      Self::Seconds(value) => value.td_name(),
      Self::SecretChat(value) => value.td_name(),
      Self::Session(value) => value.td_name(),
      Self::Sessions(value) => value.td_name(),
      Self::SponsoredMessage(value) => value.td_name(),
      Self::Sticker(value) => value.td_name(),
      Self::StickerSet(value) => value.td_name(),
      Self::StickerSets(value) => value.td_name(),
      Self::Stickers(value) => value.td_name(),
      Self::StorageStatistics(value) => value.td_name(),
      Self::StorageStatisticsFast(value) => value.td_name(),
      Self::Supergroup(value) => value.td_name(),
      Self::SupergroupFullInfo(value) => value.td_name(),
      Self::TMeUrls(value) => value.td_name(),
      Self::TemporaryPasswordState(value) => value.td_name(),
      Self::TestBytes(value) => value.td_name(),
      Self::TestInt(value) => value.td_name(),
      Self::TestString(value) => value.td_name(),
      Self::TestVectorInt(value) => value.td_name(),
      Self::TestVectorIntObject(value) => value.td_name(),
      Self::TestVectorString(value) => value.td_name(),
      Self::TestVectorStringObject(value) => value.td_name(),
      Self::Text(value) => value.td_name(),
      Self::TextEntities(value) => value.td_name(),
      Self::Updates(value) => value.td_name(),
      Self::User(value) => value.td_name(),
      Self::UserFullInfo(value) => value.td_name(),
      Self::UserPrivacySettingRules(value) => value.td_name(),
      Self::Users(value) => value.td_name(),
      Self::ValidatedOrderInfo(value) => value.td_name(),
      Self::WebPage(value) => value.td_name(),
      Self::WebPageInstantView(value) => value.td_name(),
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      Self::TestUseUpdate(value) => value.extra(),
      Self::UpdateActiveNotifications(value) => value.extra(),
      Self::UpdateAnimatedEmojiMessageClicked(value) => value.extra(),
      Self::UpdateAnimationSearchParameters(value) => value.extra(),
      Self::UpdateAuthorizationState(value) => value.extra(),
      Self::UpdateBasicGroup(value) => value.extra(),
      Self::UpdateBasicGroupFullInfo(value) => value.extra(),
      Self::UpdateCall(value) => value.extra(),
      Self::UpdateChatAction(value) => value.extra(),
      Self::UpdateChatActionBar(value) => value.extra(),
      Self::UpdateChatDefaultDisableNotification(value) => value.extra(),
      Self::UpdateChatDraftMessage(value) => value.extra(),
      Self::UpdateChatFilters(value) => value.extra(),
      Self::UpdateChatHasProtectedContent(value) => value.extra(),
      Self::UpdateChatHasScheduledMessages(value) => value.extra(),
      Self::UpdateChatIsBlocked(value) => value.extra(),
      Self::UpdateChatIsMarkedAsUnread(value) => value.extra(),
      Self::UpdateChatLastMessage(value) => value.extra(),
      Self::UpdateChatMember(value) => value.extra(),
      Self::UpdateChatMessageSender(value) => value.extra(),
      Self::UpdateChatMessageTtl(value) => value.extra(),
      Self::UpdateChatNotificationSettings(value) => value.extra(),
      Self::UpdateChatOnlineMemberCount(value) => value.extra(),
      Self::UpdateChatPendingJoinRequests(value) => value.extra(),
      Self::UpdateChatPermissions(value) => value.extra(),
      Self::UpdateChatPhoto(value) => value.extra(),
      Self::UpdateChatPosition(value) => value.extra(),
      Self::UpdateChatReadInbox(value) => value.extra(),
      Self::UpdateChatReadOutbox(value) => value.extra(),
      Self::UpdateChatReplyMarkup(value) => value.extra(),
      Self::UpdateChatTheme(value) => value.extra(),
      Self::UpdateChatThemes(value) => value.extra(),
      Self::UpdateChatTitle(value) => value.extra(),
      Self::UpdateChatUnreadMentionCount(value) => value.extra(),
      Self::UpdateChatVideoChat(value) => value.extra(),
      Self::UpdateConnectionState(value) => value.extra(),
      Self::UpdateDeleteMessages(value) => value.extra(),
      Self::UpdateDiceEmojis(value) => value.extra(),
      Self::UpdateFavoriteStickers(value) => value.extra(),
      Self::UpdateFile(value) => value.extra(),
      Self::UpdateFileGenerationStart(value) => value.extra(),
      Self::UpdateFileGenerationStop(value) => value.extra(),
      Self::UpdateGroupCall(value) => value.extra(),
      Self::UpdateGroupCallParticipant(value) => value.extra(),
      Self::UpdateHavePendingNotifications(value) => value.extra(),
      Self::UpdateInstalledStickerSets(value) => value.extra(),
      Self::UpdateLanguagePackStrings(value) => value.extra(),
      Self::UpdateMessageContent(value) => value.extra(),
      Self::UpdateMessageContentOpened(value) => value.extra(),
      Self::UpdateMessageEdited(value) => value.extra(),
      Self::UpdateMessageInteractionInfo(value) => value.extra(),
      Self::UpdateMessageIsPinned(value) => value.extra(),
      Self::UpdateMessageLiveLocationViewed(value) => value.extra(),
      Self::UpdateMessageMentionRead(value) => value.extra(),
      Self::UpdateMessageSendAcknowledged(value) => value.extra(),
      Self::UpdateMessageSendFailed(value) => value.extra(),
      Self::UpdateMessageSendSucceeded(value) => value.extra(),
      Self::UpdateNewCallSignalingData(value) => value.extra(),
      Self::UpdateNewCallbackQuery(value) => value.extra(),
      Self::UpdateNewChat(value) => value.extra(),
      Self::UpdateNewChatJoinRequest(value) => value.extra(),
      Self::UpdateNewChosenInlineResult(value) => value.extra(),
      Self::UpdateNewCustomEvent(value) => value.extra(),
      Self::UpdateNewCustomQuery(value) => value.extra(),
      Self::UpdateNewInlineCallbackQuery(value) => value.extra(),
      Self::UpdateNewInlineQuery(value) => value.extra(),
      Self::UpdateNewMessage(value) => value.extra(),
      Self::UpdateNewPreCheckoutQuery(value) => value.extra(),
      Self::UpdateNewShippingQuery(value) => value.extra(),
      Self::UpdateNotification(value) => value.extra(),
      Self::UpdateNotificationGroup(value) => value.extra(),
      Self::UpdateOption(value) => value.extra(),
      Self::UpdatePoll(value) => value.extra(),
      Self::UpdatePollAnswer(value) => value.extra(),
      Self::UpdateRecentStickers(value) => value.extra(),
      Self::UpdateSavedAnimations(value) => value.extra(),
      Self::UpdateScopeNotificationSettings(value) => value.extra(),
      Self::UpdateSecretChat(value) => value.extra(),
      Self::UpdateSelectedBackground(value) => value.extra(),
      Self::UpdateServiceNotification(value) => value.extra(),
      Self::UpdateStickerSet(value) => value.extra(),
      Self::UpdateSuggestedActions(value) => value.extra(),
      Self::UpdateSupergroup(value) => value.extra(),
      Self::UpdateSupergroupFullInfo(value) => value.extra(),
      Self::UpdateTermsOfService(value) => value.extra(),
      Self::UpdateTrendingStickerSets(value) => value.extra(),
      Self::UpdateUnreadChatCount(value) => value.extra(),
      Self::UpdateUnreadMessageCount(value) => value.extra(),
      Self::UpdateUser(value) => value.extra(),
      Self::UpdateUserFullInfo(value) => value.extra(),
      Self::UpdateUserPrivacySettingRules(value) => value.extra(),
      Self::UpdateUserStatus(value) => value.extra(),
      Self::UpdateUsersNearby(value) => value.extra(),

      Self::AuthorizationState(value) => value.extra(),
      Self::CanTransferOwnershipResult(value) => value.extra(),
      Self::ChatStatistics(value) => value.extra(),
      Self::CheckChatUsernameResult(value) => value.extra(),
      Self::CheckStickerSetNameResult(value) => value.extra(),
      Self::InternalLinkType(value) => value.extra(),
      Self::JsonValue(value) => value.extra(),
      Self::LanguagePackStringValue(value) => value.extra(),
      Self::LogStream(value) => value.extra(),
      Self::LoginUrlInfo(value) => value.extra(),
      Self::MessageFileType(value) => value.extra(),
      Self::OptionValue(value) => value.extra(),
      Self::PassportElement(value) => value.extra(),
      Self::ResetPasswordResult(value) => value.extra(),
      Self::StatisticalGraph(value) => value.extra(),
      Self::Update(value) => value.extra(),
      Self::AccountTtl(value) => value.extra(),
      Self::AnimatedEmoji(value) => value.extra(),
      Self::Animations(value) => value.extra(),
      Self::AuthenticationCodeInfo(value) => value.extra(),
      Self::AutoDownloadSettingsPresets(value) => value.extra(),
      Self::Background(value) => value.extra(),
      Self::Backgrounds(value) => value.extra(),
      Self::BankCardInfo(value) => value.extra(),
      Self::BasicGroup(value) => value.extra(),
      Self::BasicGroupFullInfo(value) => value.extra(),
      Self::BotCommands(value) => value.extra(),
      Self::CallId(value) => value.extra(),
      Self::CallbackQueryAnswer(value) => value.extra(),
      Self::Chat(value) => value.extra(),
      Self::ChatAdministrators(value) => value.extra(),
      Self::ChatEvents(value) => value.extra(),
      Self::ChatFilter(value) => value.extra(),
      Self::ChatFilterInfo(value) => value.extra(),
      Self::ChatInviteLink(value) => value.extra(),
      Self::ChatInviteLinkCounts(value) => value.extra(),
      Self::ChatInviteLinkInfo(value) => value.extra(),
      Self::ChatInviteLinkMembers(value) => value.extra(),
      Self::ChatInviteLinks(value) => value.extra(),
      Self::ChatJoinRequests(value) => value.extra(),
      Self::ChatLists(value) => value.extra(),
      Self::ChatMember(value) => value.extra(),
      Self::ChatMembers(value) => value.extra(),
      Self::ChatPhotos(value) => value.extra(),
      Self::Chats(value) => value.extra(),
      Self::ChatsNearby(value) => value.extra(),
      Self::ConnectedWebsites(value) => value.extra(),
      Self::Count(value) => value.extra(),
      Self::Countries(value) => value.extra(),
      Self::CustomRequestResult(value) => value.extra(),
      Self::DatabaseStatistics(value) => value.extra(),
      Self::DeepLinkInfo(value) => value.extra(),
      Self::EmailAddressAuthenticationCodeInfo(value) => value.extra(),
      Self::Emojis(value) => value.extra(),
      Self::Error(value) => value.extra(),
      Self::File(value) => value.extra(),
      Self::FilePart(value) => value.extra(),
      Self::FormattedText(value) => value.extra(),
      Self::FoundMessages(value) => value.extra(),
      Self::GameHighScores(value) => value.extra(),
      Self::GroupCall(value) => value.extra(),
      Self::GroupCallId(value) => value.extra(),
      Self::Hashtags(value) => value.extra(),
      Self::HttpUrl(value) => value.extra(),
      Self::ImportedContacts(value) => value.extra(),
      Self::InlineQueryResults(value) => value.extra(),
      Self::LanguagePackInfo(value) => value.extra(),
      Self::LanguagePackStrings(value) => value.extra(),
      Self::LocalizationTargetInfo(value) => value.extra(),
      Self::LogTags(value) => value.extra(),
      Self::LogVerbosityLevel(value) => value.extra(),
      Self::Message(value) => value.extra(),
      Self::MessageCalendar(value) => value.extra(),
      Self::MessageLink(value) => value.extra(),
      Self::MessageLinkInfo(value) => value.extra(),
      Self::MessagePositions(value) => value.extra(),
      Self::MessageSenders(value) => value.extra(),
      Self::MessageStatistics(value) => value.extra(),
      Self::MessageThreadInfo(value) => value.extra(),
      Self::Messages(value) => value.extra(),
      Self::NetworkStatistics(value) => value.extra(),
      Self::Ok(value) => value.extra(),
      Self::OrderInfo(value) => value.extra(),
      Self::PassportAuthorizationForm(value) => value.extra(),
      Self::PassportElements(value) => value.extra(),
      Self::PassportElementsWithErrors(value) => value.extra(),
      Self::PasswordState(value) => value.extra(),
      Self::PaymentForm(value) => value.extra(),
      Self::PaymentReceipt(value) => value.extra(),
      Self::PaymentResult(value) => value.extra(),
      Self::PhoneNumberInfo(value) => value.extra(),
      Self::Proxies(value) => value.extra(),
      Self::Proxy(value) => value.extra(),
      Self::PushReceiverId(value) => value.extra(),
      Self::RecommendedChatFilters(value) => value.extra(),
      Self::RecoveryEmailAddress(value) => value.extra(),
      Self::ScopeNotificationSettings(value) => value.extra(),
      Self::Seconds(value) => value.extra(),
      Self::SecretChat(value) => value.extra(),
      Self::Session(value) => value.extra(),
      Self::Sessions(value) => value.extra(),
      Self::SponsoredMessage(value) => value.extra(),
      Self::Sticker(value) => value.extra(),
      Self::StickerSet(value) => value.extra(),
      Self::StickerSets(value) => value.extra(),
      Self::Stickers(value) => value.extra(),
      Self::StorageStatistics(value) => value.extra(),
      Self::StorageStatisticsFast(value) => value.extra(),
      Self::Supergroup(value) => value.extra(),
      Self::SupergroupFullInfo(value) => value.extra(),
      Self::TMeUrls(value) => value.extra(),
      Self::TemporaryPasswordState(value) => value.extra(),
      Self::TestBytes(value) => value.extra(),
      Self::TestInt(value) => value.extra(),
      Self::TestString(value) => value.extra(),
      Self::TestVectorInt(value) => value.extra(),
      Self::TestVectorIntObject(value) => value.extra(),
      Self::TestVectorString(value) => value.extra(),
      Self::TestVectorStringObject(value) => value.extra(),
      Self::Text(value) => value.extra(),
      Self::TextEntities(value) => value.extra(),
      Self::Updates(value) => value.extra(),
      Self::User(value) => value.extra(),
      Self::UserFullInfo(value) => value.extra(),
      Self::UserPrivacySettingRules(value) => value.extra(),
      Self::Users(value) => value.extra(),
      Self::ValidatedOrderInfo(value) => value.extra(),
      Self::WebPage(value) => value.extra(),
      Self::WebPageInstantView(value) => value.extra(),
    }
  }
  /// Return td type to json string
  fn to_json(&self) -> RTDResult<String> {
    match self {
      Self::TestUseUpdate(value) => value.to_json(),
      Self::UpdateActiveNotifications(value) => value.to_json(),
      Self::UpdateAnimatedEmojiMessageClicked(value) => value.to_json(),
      Self::UpdateAnimationSearchParameters(value) => value.to_json(),
      Self::UpdateAuthorizationState(value) => value.to_json(),
      Self::UpdateBasicGroup(value) => value.to_json(),
      Self::UpdateBasicGroupFullInfo(value) => value.to_json(),
      Self::UpdateCall(value) => value.to_json(),
      Self::UpdateChatAction(value) => value.to_json(),
      Self::UpdateChatActionBar(value) => value.to_json(),
      Self::UpdateChatDefaultDisableNotification(value) => value.to_json(),
      Self::UpdateChatDraftMessage(value) => value.to_json(),
      Self::UpdateChatFilters(value) => value.to_json(),
      Self::UpdateChatHasProtectedContent(value) => value.to_json(),
      Self::UpdateChatHasScheduledMessages(value) => value.to_json(),
      Self::UpdateChatIsBlocked(value) => value.to_json(),
      Self::UpdateChatIsMarkedAsUnread(value) => value.to_json(),
      Self::UpdateChatLastMessage(value) => value.to_json(),
      Self::UpdateChatMember(value) => value.to_json(),
      Self::UpdateChatMessageSender(value) => value.to_json(),
      Self::UpdateChatMessageTtl(value) => value.to_json(),
      Self::UpdateChatNotificationSettings(value) => value.to_json(),
      Self::UpdateChatOnlineMemberCount(value) => value.to_json(),
      Self::UpdateChatPendingJoinRequests(value) => value.to_json(),
      Self::UpdateChatPermissions(value) => value.to_json(),
      Self::UpdateChatPhoto(value) => value.to_json(),
      Self::UpdateChatPosition(value) => value.to_json(),
      Self::UpdateChatReadInbox(value) => value.to_json(),
      Self::UpdateChatReadOutbox(value) => value.to_json(),
      Self::UpdateChatReplyMarkup(value) => value.to_json(),
      Self::UpdateChatTheme(value) => value.to_json(),
      Self::UpdateChatThemes(value) => value.to_json(),
      Self::UpdateChatTitle(value) => value.to_json(),
      Self::UpdateChatUnreadMentionCount(value) => value.to_json(),
      Self::UpdateChatVideoChat(value) => value.to_json(),
      Self::UpdateConnectionState(value) => value.to_json(),
      Self::UpdateDeleteMessages(value) => value.to_json(),
      Self::UpdateDiceEmojis(value) => value.to_json(),
      Self::UpdateFavoriteStickers(value) => value.to_json(),
      Self::UpdateFile(value) => value.to_json(),
      Self::UpdateFileGenerationStart(value) => value.to_json(),
      Self::UpdateFileGenerationStop(value) => value.to_json(),
      Self::UpdateGroupCall(value) => value.to_json(),
      Self::UpdateGroupCallParticipant(value) => value.to_json(),
      Self::UpdateHavePendingNotifications(value) => value.to_json(),
      Self::UpdateInstalledStickerSets(value) => value.to_json(),
      Self::UpdateLanguagePackStrings(value) => value.to_json(),
      Self::UpdateMessageContent(value) => value.to_json(),
      Self::UpdateMessageContentOpened(value) => value.to_json(),
      Self::UpdateMessageEdited(value) => value.to_json(),
      Self::UpdateMessageInteractionInfo(value) => value.to_json(),
      Self::UpdateMessageIsPinned(value) => value.to_json(),
      Self::UpdateMessageLiveLocationViewed(value) => value.to_json(),
      Self::UpdateMessageMentionRead(value) => value.to_json(),
      Self::UpdateMessageSendAcknowledged(value) => value.to_json(),
      Self::UpdateMessageSendFailed(value) => value.to_json(),
      Self::UpdateMessageSendSucceeded(value) => value.to_json(),
      Self::UpdateNewCallSignalingData(value) => value.to_json(),
      Self::UpdateNewCallbackQuery(value) => value.to_json(),
      Self::UpdateNewChat(value) => value.to_json(),
      Self::UpdateNewChatJoinRequest(value) => value.to_json(),
      Self::UpdateNewChosenInlineResult(value) => value.to_json(),
      Self::UpdateNewCustomEvent(value) => value.to_json(),
      Self::UpdateNewCustomQuery(value) => value.to_json(),
      Self::UpdateNewInlineCallbackQuery(value) => value.to_json(),
      Self::UpdateNewInlineQuery(value) => value.to_json(),
      Self::UpdateNewMessage(value) => value.to_json(),
      Self::UpdateNewPreCheckoutQuery(value) => value.to_json(),
      Self::UpdateNewShippingQuery(value) => value.to_json(),
      Self::UpdateNotification(value) => value.to_json(),
      Self::UpdateNotificationGroup(value) => value.to_json(),
      Self::UpdateOption(value) => value.to_json(),
      Self::UpdatePoll(value) => value.to_json(),
      Self::UpdatePollAnswer(value) => value.to_json(),
      Self::UpdateRecentStickers(value) => value.to_json(),
      Self::UpdateSavedAnimations(value) => value.to_json(),
      Self::UpdateScopeNotificationSettings(value) => value.to_json(),
      Self::UpdateSecretChat(value) => value.to_json(),
      Self::UpdateSelectedBackground(value) => value.to_json(),
      Self::UpdateServiceNotification(value) => value.to_json(),
      Self::UpdateStickerSet(value) => value.to_json(),
      Self::UpdateSuggestedActions(value) => value.to_json(),
      Self::UpdateSupergroup(value) => value.to_json(),
      Self::UpdateSupergroupFullInfo(value) => value.to_json(),
      Self::UpdateTermsOfService(value) => value.to_json(),
      Self::UpdateTrendingStickerSets(value) => value.to_json(),
      Self::UpdateUnreadChatCount(value) => value.to_json(),
      Self::UpdateUnreadMessageCount(value) => value.to_json(),
      Self::UpdateUser(value) => value.to_json(),
      Self::UpdateUserFullInfo(value) => value.to_json(),
      Self::UpdateUserPrivacySettingRules(value) => value.to_json(),
      Self::UpdateUserStatus(value) => value.to_json(),
      Self::UpdateUsersNearby(value) => value.to_json(),

      Self::AuthorizationState(value) => value.to_json(),
      Self::CanTransferOwnershipResult(value) => value.to_json(),
      Self::ChatStatistics(value) => value.to_json(),
      Self::CheckChatUsernameResult(value) => value.to_json(),
      Self::CheckStickerSetNameResult(value) => value.to_json(),
      Self::InternalLinkType(value) => value.to_json(),
      Self::JsonValue(value) => value.to_json(),
      Self::LanguagePackStringValue(value) => value.to_json(),
      Self::LogStream(value) => value.to_json(),
      Self::LoginUrlInfo(value) => value.to_json(),
      Self::MessageFileType(value) => value.to_json(),
      Self::OptionValue(value) => value.to_json(),
      Self::PassportElement(value) => value.to_json(),
      Self::ResetPasswordResult(value) => value.to_json(),
      Self::StatisticalGraph(value) => value.to_json(),
      Self::Update(value) => value.to_json(),
      Self::AccountTtl(value) => value.to_json(),
      Self::AnimatedEmoji(value) => value.to_json(),
      Self::Animations(value) => value.to_json(),
      Self::AuthenticationCodeInfo(value) => value.to_json(),
      Self::AutoDownloadSettingsPresets(value) => value.to_json(),
      Self::Background(value) => value.to_json(),
      Self::Backgrounds(value) => value.to_json(),
      Self::BankCardInfo(value) => value.to_json(),
      Self::BasicGroup(value) => value.to_json(),
      Self::BasicGroupFullInfo(value) => value.to_json(),
      Self::BotCommands(value) => value.to_json(),
      Self::CallId(value) => value.to_json(),
      Self::CallbackQueryAnswer(value) => value.to_json(),
      Self::Chat(value) => value.to_json(),
      Self::ChatAdministrators(value) => value.to_json(),
      Self::ChatEvents(value) => value.to_json(),
      Self::ChatFilter(value) => value.to_json(),
      Self::ChatFilterInfo(value) => value.to_json(),
      Self::ChatInviteLink(value) => value.to_json(),
      Self::ChatInviteLinkCounts(value) => value.to_json(),
      Self::ChatInviteLinkInfo(value) => value.to_json(),
      Self::ChatInviteLinkMembers(value) => value.to_json(),
      Self::ChatInviteLinks(value) => value.to_json(),
      Self::ChatJoinRequests(value) => value.to_json(),
      Self::ChatLists(value) => value.to_json(),
      Self::ChatMember(value) => value.to_json(),
      Self::ChatMembers(value) => value.to_json(),
      Self::ChatPhotos(value) => value.to_json(),
      Self::Chats(value) => value.to_json(),
      Self::ChatsNearby(value) => value.to_json(),
      Self::ConnectedWebsites(value) => value.to_json(),
      Self::Count(value) => value.to_json(),
      Self::Countries(value) => value.to_json(),
      Self::CustomRequestResult(value) => value.to_json(),
      Self::DatabaseStatistics(value) => value.to_json(),
      Self::DeepLinkInfo(value) => value.to_json(),
      Self::EmailAddressAuthenticationCodeInfo(value) => value.to_json(),
      Self::Emojis(value) => value.to_json(),
      Self::Error(value) => value.to_json(),
      Self::File(value) => value.to_json(),
      Self::FilePart(value) => value.to_json(),
      Self::FormattedText(value) => value.to_json(),
      Self::FoundMessages(value) => value.to_json(),
      Self::GameHighScores(value) => value.to_json(),
      Self::GroupCall(value) => value.to_json(),
      Self::GroupCallId(value) => value.to_json(),
      Self::Hashtags(value) => value.to_json(),
      Self::HttpUrl(value) => value.to_json(),
      Self::ImportedContacts(value) => value.to_json(),
      Self::InlineQueryResults(value) => value.to_json(),
      Self::LanguagePackInfo(value) => value.to_json(),
      Self::LanguagePackStrings(value) => value.to_json(),
      Self::LocalizationTargetInfo(value) => value.to_json(),
      Self::LogTags(value) => value.to_json(),
      Self::LogVerbosityLevel(value) => value.to_json(),
      Self::Message(value) => value.to_json(),
      Self::MessageCalendar(value) => value.to_json(),
      Self::MessageLink(value) => value.to_json(),
      Self::MessageLinkInfo(value) => value.to_json(),
      Self::MessagePositions(value) => value.to_json(),
      Self::MessageSenders(value) => value.to_json(),
      Self::MessageStatistics(value) => value.to_json(),
      Self::MessageThreadInfo(value) => value.to_json(),
      Self::Messages(value) => value.to_json(),
      Self::NetworkStatistics(value) => value.to_json(),
      Self::Ok(value) => value.to_json(),
      Self::OrderInfo(value) => value.to_json(),
      Self::PassportAuthorizationForm(value) => value.to_json(),
      Self::PassportElements(value) => value.to_json(),
      Self::PassportElementsWithErrors(value) => value.to_json(),
      Self::PasswordState(value) => value.to_json(),
      Self::PaymentForm(value) => value.to_json(),
      Self::PaymentReceipt(value) => value.to_json(),
      Self::PaymentResult(value) => value.to_json(),
      Self::PhoneNumberInfo(value) => value.to_json(),
      Self::Proxies(value) => value.to_json(),
      Self::Proxy(value) => value.to_json(),
      Self::PushReceiverId(value) => value.to_json(),
      Self::RecommendedChatFilters(value) => value.to_json(),
      Self::RecoveryEmailAddress(value) => value.to_json(),
      Self::ScopeNotificationSettings(value) => value.to_json(),
      Self::Seconds(value) => value.to_json(),
      Self::SecretChat(value) => value.to_json(),
      Self::Session(value) => value.to_json(),
      Self::Sessions(value) => value.to_json(),
      Self::SponsoredMessage(value) => value.to_json(),
      Self::Sticker(value) => value.to_json(),
      Self::StickerSet(value) => value.to_json(),
      Self::StickerSets(value) => value.to_json(),
      Self::Stickers(value) => value.to_json(),
      Self::StorageStatistics(value) => value.to_json(),
      Self::StorageStatisticsFast(value) => value.to_json(),
      Self::Supergroup(value) => value.to_json(),
      Self::SupergroupFullInfo(value) => value.to_json(),
      Self::TMeUrls(value) => value.to_json(),
      Self::TemporaryPasswordState(value) => value.to_json(),
      Self::TestBytes(value) => value.to_json(),
      Self::TestInt(value) => value.to_json(),
      Self::TestString(value) => value.to_json(),
      Self::TestVectorInt(value) => value.to_json(),
      Self::TestVectorIntObject(value) => value.to_json(),
      Self::TestVectorString(value) => value.to_json(),
      Self::TestVectorStringObject(value) => value.to_json(),
      Self::Text(value) => value.to_json(),
      Self::TextEntities(value) => value.to_json(),
      Self::Updates(value) => value.to_json(),
      Self::User(value) => value.to_json(),
      Self::UserFullInfo(value) => value.to_json(),
      Self::UserPrivacySettingRules(value) => value.to_json(),
      Self::Users(value) => value.to_json(),
      Self::ValidatedOrderInfo(value) => value.to_json(),
      Self::WebPage(value) => value.to_json(),
      Self::WebPageInstantView(value) => value.to_json(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::RObject;
  use crate::types::{from_json, TdType, UpdateAuthorizationState};

  const AUTH_JSON: &str = r#"{"@type":"updateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters"}}"#;

  #[test]
  fn test_deserialize_enum() {
    let value = from_json::<UpdateAuthorizationState>(AUTH_JSON).expect("deserialize update");
    assert_eq!(value.td_name(), "updateAuthorizationState");

    match from_json::<TdType>(AUTH_JSON) {
      Ok(TdType::UpdateAuthorizationState(_)) => {}
      Ok(other) => panic!("from_json failed: {:?}", other),
      Err(err) => panic!("{}", err),
    }
  }
}
