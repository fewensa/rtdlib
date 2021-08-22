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


///// tuple enum is field
//macro_rules! tuple_enum_is {
//  ($enum_name:ident, $field:ident) => {
//    |o: &$enum_name| {
//      if let $enum_name::$field(_) = o { true } else { false }
//    }
//  };
////  ($e:ident, $t:ident, $namespace:ident) => {
////    Box::new(|t: &$e| {
////      match t {
////        $namespace::$e::$t(_) => true,
////        _ => false
////      }
////    })
////  };
//}
//
//macro_rules! tuple_enum_on {
//  ($enum_name:ident, $field:ident, $fnc:expr) => {
//    |o: &$enum_name| {
//      if let $enum_name::$field(t) = o { $fnc(t) }
//    }
//  };
//}

pub fn detect_td_type<S: AsRef<str>>(json: S) -> Option<String> {
  let result: Result<serde_json::Value, serde_json::Error> = serde_json::from_str::<serde_json::Value>(json.as_ref());
  if let Err(_) = result { return None }
  let value = result.unwrap();
  value.as_object().map_or(None, |v| {
    v.get("@type").map_or(None, |t| t.as_str().map_or(None, |t| {
      Some(t.to_string())
    }))
  })
}

pub fn detect_td_type_and_extra<S: AsRef<str>>(json: S) -> (Option<String>, Option<String>) {
  let result: Result<serde_json::Value, serde_json::Error> = serde_json::from_str::<serde_json::Value>(json.as_ref());
  if let Err(_) = result { return (None, None) }
  let value = result.unwrap();
  let mut type_ = None;
  let mut extra = None;
  if let Some(map) = value.as_object() {
    map.get("@type").map(|v| v.as_str().map(|t| type_.replace(t.to_string())));
    map.get("@extra").map(|v| v.as_str().map(|t| extra.replace(t.to_string())));
  }
  (type_, extra)
}

pub fn from_json<'a, T>(json: &'a str) -> RTDResult<T> where T: serde::de::Deserialize<'a>, {
  Ok(serde_json::from_str(json.as_ref())?)
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


impl<'a, RObj: RObject> RObject for &'a RObj {
  fn td_name(&self) -> &'static str { (*self).td_name() }
  fn to_json(&self) -> RTDResult<String> { (*self).to_json() }
  fn extra(&self) -> Option<String> { (*self).extra() }
}

impl<'a, RObj: RObject> RObject for &'a mut RObj {
  fn td_name(&self) -> &'static str { (**self).td_name() }
  fn to_json(&self) -> RTDResult<String> { (**self).to_json() }
  fn extra(&self) -> Option<String> { (**self).extra() }
}


impl<'a, Fnc: RFunction> RFunction for &'a Fnc {}
impl<'a, Fnc: RFunction> RFunction for &'a mut Fnc {}


impl<'a, AUTHENTICATIONCODETYPE: TDAuthenticationCodeType> TDAuthenticationCodeType for &'a AUTHENTICATIONCODETYPE {}
impl<'a, AUTHENTICATIONCODETYPE: TDAuthenticationCodeType> TDAuthenticationCodeType for &'a mut AUTHENTICATIONCODETYPE {}

impl<'a, AUTHORIZATIONSTATE: TDAuthorizationState> TDAuthorizationState for &'a AUTHORIZATIONSTATE {}
impl<'a, AUTHORIZATIONSTATE: TDAuthorizationState> TDAuthorizationState for &'a mut AUTHORIZATIONSTATE {}

impl<'a, BACKGROUNDFILL: TDBackgroundFill> TDBackgroundFill for &'a BACKGROUNDFILL {}
impl<'a, BACKGROUNDFILL: TDBackgroundFill> TDBackgroundFill for &'a mut BACKGROUNDFILL {}

impl<'a, BACKGROUNDTYPE: TDBackgroundType> TDBackgroundType for &'a BACKGROUNDTYPE {}
impl<'a, BACKGROUNDTYPE: TDBackgroundType> TDBackgroundType for &'a mut BACKGROUNDTYPE {}

impl<'a, CALLDISCARDREASON: TDCallDiscardReason> TDCallDiscardReason for &'a CALLDISCARDREASON {}
impl<'a, CALLDISCARDREASON: TDCallDiscardReason> TDCallDiscardReason for &'a mut CALLDISCARDREASON {}

impl<'a, CALLPROBLEM: TDCallProblem> TDCallProblem for &'a CALLPROBLEM {}
impl<'a, CALLPROBLEM: TDCallProblem> TDCallProblem for &'a mut CALLPROBLEM {}

impl<'a, CALLSTATE: TDCallState> TDCallState for &'a CALLSTATE {}
impl<'a, CALLSTATE: TDCallState> TDCallState for &'a mut CALLSTATE {}

impl<'a, CALLBACKQUERYPAYLOAD: TDCallbackQueryPayload> TDCallbackQueryPayload for &'a CALLBACKQUERYPAYLOAD {}
impl<'a, CALLBACKQUERYPAYLOAD: TDCallbackQueryPayload> TDCallbackQueryPayload for &'a mut CALLBACKQUERYPAYLOAD {}

impl<'a, CANTRANSFEROWNERSHIPRESULT: TDCanTransferOwnershipResult> TDCanTransferOwnershipResult for &'a CANTRANSFEROWNERSHIPRESULT {}
impl<'a, CANTRANSFEROWNERSHIPRESULT: TDCanTransferOwnershipResult> TDCanTransferOwnershipResult for &'a mut CANTRANSFEROWNERSHIPRESULT {}

impl<'a, CHATACTION: TDChatAction> TDChatAction for &'a CHATACTION {}
impl<'a, CHATACTION: TDChatAction> TDChatAction for &'a mut CHATACTION {}

impl<'a, CHATACTIONBAR: TDChatActionBar> TDChatActionBar for &'a CHATACTIONBAR {}
impl<'a, CHATACTIONBAR: TDChatActionBar> TDChatActionBar for &'a mut CHATACTIONBAR {}

impl<'a, CHATEVENTACTION: TDChatEventAction> TDChatEventAction for &'a CHATEVENTACTION {}
impl<'a, CHATEVENTACTION: TDChatEventAction> TDChatEventAction for &'a mut CHATEVENTACTION {}

impl<'a, CHATLIST: TDChatList> TDChatList for &'a CHATLIST {}
impl<'a, CHATLIST: TDChatList> TDChatList for &'a mut CHATLIST {}

impl<'a, CHATMEMBERSTATUS: TDChatMemberStatus> TDChatMemberStatus for &'a CHATMEMBERSTATUS {}
impl<'a, CHATMEMBERSTATUS: TDChatMemberStatus> TDChatMemberStatus for &'a mut CHATMEMBERSTATUS {}

impl<'a, CHATMEMBERSFILTER: TDChatMembersFilter> TDChatMembersFilter for &'a CHATMEMBERSFILTER {}
impl<'a, CHATMEMBERSFILTER: TDChatMembersFilter> TDChatMembersFilter for &'a mut CHATMEMBERSFILTER {}

impl<'a, CHATREPORTREASON: TDChatReportReason> TDChatReportReason for &'a CHATREPORTREASON {}
impl<'a, CHATREPORTREASON: TDChatReportReason> TDChatReportReason for &'a mut CHATREPORTREASON {}

impl<'a, CHATTYPE: TDChatType> TDChatType for &'a CHATTYPE {}
impl<'a, CHATTYPE: TDChatType> TDChatType for &'a mut CHATTYPE {}

impl<'a, CHECKCHATUSERNAMERESULT: TDCheckChatUsernameResult> TDCheckChatUsernameResult for &'a CHECKCHATUSERNAMERESULT {}
impl<'a, CHECKCHATUSERNAMERESULT: TDCheckChatUsernameResult> TDCheckChatUsernameResult for &'a mut CHECKCHATUSERNAMERESULT {}

impl<'a, CONNECTIONSTATE: TDConnectionState> TDConnectionState for &'a CONNECTIONSTATE {}
impl<'a, CONNECTIONSTATE: TDConnectionState> TDConnectionState for &'a mut CONNECTIONSTATE {}

impl<'a, DEVICETOKEN: TDDeviceToken> TDDeviceToken for &'a DEVICETOKEN {}
impl<'a, DEVICETOKEN: TDDeviceToken> TDDeviceToken for &'a mut DEVICETOKEN {}

impl<'a, FILETYPE: TDFileType> TDFileType for &'a FILETYPE {}
impl<'a, FILETYPE: TDFileType> TDFileType for &'a mut FILETYPE {}

impl<'a, INLINEKEYBOARDBUTTONTYPE: TDInlineKeyboardButtonType> TDInlineKeyboardButtonType for &'a INLINEKEYBOARDBUTTONTYPE {}
impl<'a, INLINEKEYBOARDBUTTONTYPE: TDInlineKeyboardButtonType> TDInlineKeyboardButtonType for &'a mut INLINEKEYBOARDBUTTONTYPE {}

impl<'a, INLINEQUERYRESULT: TDInlineQueryResult> TDInlineQueryResult for &'a INLINEQUERYRESULT {}
impl<'a, INLINEQUERYRESULT: TDInlineQueryResult> TDInlineQueryResult for &'a mut INLINEQUERYRESULT {}

impl<'a, INPUTBACKGROUND: TDInputBackground> TDInputBackground for &'a INPUTBACKGROUND {}
impl<'a, INPUTBACKGROUND: TDInputBackground> TDInputBackground for &'a mut INPUTBACKGROUND {}

impl<'a, INPUTCREDENTIALS: TDInputCredentials> TDInputCredentials for &'a INPUTCREDENTIALS {}
impl<'a, INPUTCREDENTIALS: TDInputCredentials> TDInputCredentials for &'a mut INPUTCREDENTIALS {}

impl<'a, INPUTFILE: TDInputFile> TDInputFile for &'a INPUTFILE {}
impl<'a, INPUTFILE: TDInputFile> TDInputFile for &'a mut INPUTFILE {}

impl<'a, INPUTINLINEQUERYRESULT: TDInputInlineQueryResult> TDInputInlineQueryResult for &'a INPUTINLINEQUERYRESULT {}
impl<'a, INPUTINLINEQUERYRESULT: TDInputInlineQueryResult> TDInputInlineQueryResult for &'a mut INPUTINLINEQUERYRESULT {}

impl<'a, INPUTMESSAGECONTENT: TDInputMessageContent> TDInputMessageContent for &'a INPUTMESSAGECONTENT {}
impl<'a, INPUTMESSAGECONTENT: TDInputMessageContent> TDInputMessageContent for &'a mut INPUTMESSAGECONTENT {}

impl<'a, INPUTPASSPORTELEMENT: TDInputPassportElement> TDInputPassportElement for &'a INPUTPASSPORTELEMENT {}
impl<'a, INPUTPASSPORTELEMENT: TDInputPassportElement> TDInputPassportElement for &'a mut INPUTPASSPORTELEMENT {}

impl<'a, INPUTPASSPORTELEMENTERRORSOURCE: TDInputPassportElementErrorSource> TDInputPassportElementErrorSource for &'a INPUTPASSPORTELEMENTERRORSOURCE {}
impl<'a, INPUTPASSPORTELEMENTERRORSOURCE: TDInputPassportElementErrorSource> TDInputPassportElementErrorSource for &'a mut INPUTPASSPORTELEMENTERRORSOURCE {}

impl<'a, JSONVALUE: TDJsonValue> TDJsonValue for &'a JSONVALUE {}
impl<'a, JSONVALUE: TDJsonValue> TDJsonValue for &'a mut JSONVALUE {}

impl<'a, KEYBOARDBUTTONTYPE: TDKeyboardButtonType> TDKeyboardButtonType for &'a KEYBOARDBUTTONTYPE {}
impl<'a, KEYBOARDBUTTONTYPE: TDKeyboardButtonType> TDKeyboardButtonType for &'a mut KEYBOARDBUTTONTYPE {}

impl<'a, LANGUAGEPACKSTRINGVALUE: TDLanguagePackStringValue> TDLanguagePackStringValue for &'a LANGUAGEPACKSTRINGVALUE {}
impl<'a, LANGUAGEPACKSTRINGVALUE: TDLanguagePackStringValue> TDLanguagePackStringValue for &'a mut LANGUAGEPACKSTRINGVALUE {}

impl<'a, LOGSTREAM: TDLogStream> TDLogStream for &'a LOGSTREAM {}
impl<'a, LOGSTREAM: TDLogStream> TDLogStream for &'a mut LOGSTREAM {}

impl<'a, LOGINURLINFO: TDLoginUrlInfo> TDLoginUrlInfo for &'a LOGINURLINFO {}
impl<'a, LOGINURLINFO: TDLoginUrlInfo> TDLoginUrlInfo for &'a mut LOGINURLINFO {}

impl<'a, MASKPOINT: TDMaskPoint> TDMaskPoint for &'a MASKPOINT {}
impl<'a, MASKPOINT: TDMaskPoint> TDMaskPoint for &'a mut MASKPOINT {}

impl<'a, MESSAGECONTENT: TDMessageContent> TDMessageContent for &'a MESSAGECONTENT {}
impl<'a, MESSAGECONTENT: TDMessageContent> TDMessageContent for &'a mut MESSAGECONTENT {}

impl<'a, MESSAGEFORWARDORIGIN: TDMessageForwardOrigin> TDMessageForwardOrigin for &'a MESSAGEFORWARDORIGIN {}
impl<'a, MESSAGEFORWARDORIGIN: TDMessageForwardOrigin> TDMessageForwardOrigin for &'a mut MESSAGEFORWARDORIGIN {}

impl<'a, MESSAGESCHEDULINGSTATE: TDMessageSchedulingState> TDMessageSchedulingState for &'a MESSAGESCHEDULINGSTATE {}
impl<'a, MESSAGESCHEDULINGSTATE: TDMessageSchedulingState> TDMessageSchedulingState for &'a mut MESSAGESCHEDULINGSTATE {}

impl<'a, MESSAGESENDINGSTATE: TDMessageSendingState> TDMessageSendingState for &'a MESSAGESENDINGSTATE {}
impl<'a, MESSAGESENDINGSTATE: TDMessageSendingState> TDMessageSendingState for &'a mut MESSAGESENDINGSTATE {}

impl<'a, NETWORKSTATISTICSENTRY: TDNetworkStatisticsEntry> TDNetworkStatisticsEntry for &'a NETWORKSTATISTICSENTRY {}
impl<'a, NETWORKSTATISTICSENTRY: TDNetworkStatisticsEntry> TDNetworkStatisticsEntry for &'a mut NETWORKSTATISTICSENTRY {}

impl<'a, NETWORKTYPE: TDNetworkType> TDNetworkType for &'a NETWORKTYPE {}
impl<'a, NETWORKTYPE: TDNetworkType> TDNetworkType for &'a mut NETWORKTYPE {}

impl<'a, NOTIFICATIONGROUPTYPE: TDNotificationGroupType> TDNotificationGroupType for &'a NOTIFICATIONGROUPTYPE {}
impl<'a, NOTIFICATIONGROUPTYPE: TDNotificationGroupType> TDNotificationGroupType for &'a mut NOTIFICATIONGROUPTYPE {}

impl<'a, NOTIFICATIONSETTINGSSCOPE: TDNotificationSettingsScope> TDNotificationSettingsScope for &'a NOTIFICATIONSETTINGSSCOPE {}
impl<'a, NOTIFICATIONSETTINGSSCOPE: TDNotificationSettingsScope> TDNotificationSettingsScope for &'a mut NOTIFICATIONSETTINGSSCOPE {}

impl<'a, NOTIFICATIONTYPE: TDNotificationType> TDNotificationType for &'a NOTIFICATIONTYPE {}
impl<'a, NOTIFICATIONTYPE: TDNotificationType> TDNotificationType for &'a mut NOTIFICATIONTYPE {}

impl<'a, OPTIONVALUE: TDOptionValue> TDOptionValue for &'a OPTIONVALUE {}
impl<'a, OPTIONVALUE: TDOptionValue> TDOptionValue for &'a mut OPTIONVALUE {}

impl<'a, PAGEBLOCK: TDPageBlock> TDPageBlock for &'a PAGEBLOCK {}
impl<'a, PAGEBLOCK: TDPageBlock> TDPageBlock for &'a mut PAGEBLOCK {}

impl<'a, PAGEBLOCKHORIZONTALALIGNMENT: TDPageBlockHorizontalAlignment> TDPageBlockHorizontalAlignment for &'a PAGEBLOCKHORIZONTALALIGNMENT {}
impl<'a, PAGEBLOCKHORIZONTALALIGNMENT: TDPageBlockHorizontalAlignment> TDPageBlockHorizontalAlignment for &'a mut PAGEBLOCKHORIZONTALALIGNMENT {}

impl<'a, PAGEBLOCKVERTICALALIGNMENT: TDPageBlockVerticalAlignment> TDPageBlockVerticalAlignment for &'a PAGEBLOCKVERTICALALIGNMENT {}
impl<'a, PAGEBLOCKVERTICALALIGNMENT: TDPageBlockVerticalAlignment> TDPageBlockVerticalAlignment for &'a mut PAGEBLOCKVERTICALALIGNMENT {}

impl<'a, PASSPORTELEMENT: TDPassportElement> TDPassportElement for &'a PASSPORTELEMENT {}
impl<'a, PASSPORTELEMENT: TDPassportElement> TDPassportElement for &'a mut PASSPORTELEMENT {}

impl<'a, PASSPORTELEMENTERRORSOURCE: TDPassportElementErrorSource> TDPassportElementErrorSource for &'a PASSPORTELEMENTERRORSOURCE {}
impl<'a, PASSPORTELEMENTERRORSOURCE: TDPassportElementErrorSource> TDPassportElementErrorSource for &'a mut PASSPORTELEMENTERRORSOURCE {}

impl<'a, PASSPORTELEMENTTYPE: TDPassportElementType> TDPassportElementType for &'a PASSPORTELEMENTTYPE {}
impl<'a, PASSPORTELEMENTTYPE: TDPassportElementType> TDPassportElementType for &'a mut PASSPORTELEMENTTYPE {}

impl<'a, POLLTYPE: TDPollType> TDPollType for &'a POLLTYPE {}
impl<'a, POLLTYPE: TDPollType> TDPollType for &'a mut POLLTYPE {}

impl<'a, PROXYTYPE: TDProxyType> TDProxyType for &'a PROXYTYPE {}
impl<'a, PROXYTYPE: TDProxyType> TDProxyType for &'a mut PROXYTYPE {}

impl<'a, PUBLICCHATTYPE: TDPublicChatType> TDPublicChatType for &'a PUBLICCHATTYPE {}
impl<'a, PUBLICCHATTYPE: TDPublicChatType> TDPublicChatType for &'a mut PUBLICCHATTYPE {}

impl<'a, PUSHMESSAGECONTENT: TDPushMessageContent> TDPushMessageContent for &'a PUSHMESSAGECONTENT {}
impl<'a, PUSHMESSAGECONTENT: TDPushMessageContent> TDPushMessageContent for &'a mut PUSHMESSAGECONTENT {}

impl<'a, REPLYMARKUP: TDReplyMarkup> TDReplyMarkup for &'a REPLYMARKUP {}
impl<'a, REPLYMARKUP: TDReplyMarkup> TDReplyMarkup for &'a mut REPLYMARKUP {}

impl<'a, RICHTEXT: TDRichText> TDRichText for &'a RICHTEXT {}
impl<'a, RICHTEXT: TDRichText> TDRichText for &'a mut RICHTEXT {}

impl<'a, SEARCHMESSAGESFILTER: TDSearchMessagesFilter> TDSearchMessagesFilter for &'a SEARCHMESSAGESFILTER {}
impl<'a, SEARCHMESSAGESFILTER: TDSearchMessagesFilter> TDSearchMessagesFilter for &'a mut SEARCHMESSAGESFILTER {}

impl<'a, SECRETCHATSTATE: TDSecretChatState> TDSecretChatState for &'a SECRETCHATSTATE {}
impl<'a, SECRETCHATSTATE: TDSecretChatState> TDSecretChatState for &'a mut SECRETCHATSTATE {}

impl<'a, SUPERGROUPMEMBERSFILTER: TDSupergroupMembersFilter> TDSupergroupMembersFilter for &'a SUPERGROUPMEMBERSFILTER {}
impl<'a, SUPERGROUPMEMBERSFILTER: TDSupergroupMembersFilter> TDSupergroupMembersFilter for &'a mut SUPERGROUPMEMBERSFILTER {}

impl<'a, TMEURLTYPE: TDTMeUrlType> TDTMeUrlType for &'a TMEURLTYPE {}
impl<'a, TMEURLTYPE: TDTMeUrlType> TDTMeUrlType for &'a mut TMEURLTYPE {}

impl<'a, TEXTENTITYTYPE: TDTextEntityType> TDTextEntityType for &'a TEXTENTITYTYPE {}
impl<'a, TEXTENTITYTYPE: TDTextEntityType> TDTextEntityType for &'a mut TEXTENTITYTYPE {}

impl<'a, TEXTPARSEMODE: TDTextParseMode> TDTextParseMode for &'a TEXTPARSEMODE {}
impl<'a, TEXTPARSEMODE: TDTextParseMode> TDTextParseMode for &'a mut TEXTPARSEMODE {}

impl<'a, TOPCHATCATEGORY: TDTopChatCategory> TDTopChatCategory for &'a TOPCHATCATEGORY {}
impl<'a, TOPCHATCATEGORY: TDTopChatCategory> TDTopChatCategory for &'a mut TOPCHATCATEGORY {}

impl<'a, UPDATE: TDUpdate> TDUpdate for &'a UPDATE {}
impl<'a, UPDATE: TDUpdate> TDUpdate for &'a mut UPDATE {}

impl<'a, USERPRIVACYSETTING: TDUserPrivacySetting> TDUserPrivacySetting for &'a USERPRIVACYSETTING {}
impl<'a, USERPRIVACYSETTING: TDUserPrivacySetting> TDUserPrivacySetting for &'a mut USERPRIVACYSETTING {}

impl<'a, USERPRIVACYSETTINGRULE: TDUserPrivacySettingRule> TDUserPrivacySettingRule for &'a USERPRIVACYSETTINGRULE {}
impl<'a, USERPRIVACYSETTINGRULE: TDUserPrivacySettingRule> TDUserPrivacySettingRule for &'a mut USERPRIVACYSETTINGRULE {}

impl<'a, USERSTATUS: TDUserStatus> TDUserStatus for &'a USERSTATUS {}
impl<'a, USERSTATUS: TDUserStatus> TDUserStatus for &'a mut USERSTATUS {}

impl<'a, USERTYPE: TDUserType> TDUserType for &'a USERTYPE {}
impl<'a, USERTYPE: TDUserType> TDUserType for &'a mut USERTYPE {}


#[derive(Debug, Clone)]
pub enum TdType {
  TestUseUpdate(TestUseUpdate),
  UpdateActiveNotifications(UpdateActiveNotifications),
  UpdateAuthorizationState(UpdateAuthorizationState),
  UpdateBasicGroup(UpdateBasicGroup),
  UpdateBasicGroupFullInfo(UpdateBasicGroupFullInfo),
  UpdateCall(UpdateCall),
  UpdateChatActionBar(UpdateChatActionBar),
  UpdateChatChatList(UpdateChatChatList),
  UpdateChatDefaultDisableNotification(UpdateChatDefaultDisableNotification),
  UpdateChatDraftMessage(UpdateChatDraftMessage),
  UpdateChatHasScheduledMessages(UpdateChatHasScheduledMessages),
  UpdateChatIsMarkedAsUnread(UpdateChatIsMarkedAsUnread),
  UpdateChatIsPinned(UpdateChatIsPinned),
  UpdateChatIsSponsored(UpdateChatIsSponsored),
  UpdateChatLastMessage(UpdateChatLastMessage),
  UpdateChatNotificationSettings(UpdateChatNotificationSettings),
  UpdateChatOnlineMemberCount(UpdateChatOnlineMemberCount),
  UpdateChatOrder(UpdateChatOrder),
  UpdateChatPermissions(UpdateChatPermissions),
  UpdateChatPhoto(UpdateChatPhoto),
  UpdateChatPinnedMessage(UpdateChatPinnedMessage),
  UpdateChatReadInbox(UpdateChatReadInbox),
  UpdateChatReadOutbox(UpdateChatReadOutbox),
  UpdateChatReplyMarkup(UpdateChatReplyMarkup),
  UpdateChatTitle(UpdateChatTitle),
  UpdateChatUnreadMentionCount(UpdateChatUnreadMentionCount),
  UpdateConnectionState(UpdateConnectionState),
  UpdateDeleteMessages(UpdateDeleteMessages),
  UpdateFavoriteStickers(UpdateFavoriteStickers),
  UpdateFile(UpdateFile),
  UpdateFileGenerationStart(UpdateFileGenerationStart),
  UpdateFileGenerationStop(UpdateFileGenerationStop),
  UpdateHavePendingNotifications(UpdateHavePendingNotifications),
  UpdateInstalledStickerSets(UpdateInstalledStickerSets),
  UpdateLanguagePackStrings(UpdateLanguagePackStrings),
  UpdateMessageContent(UpdateMessageContent),
  UpdateMessageContentOpened(UpdateMessageContentOpened),
  UpdateMessageEdited(UpdateMessageEdited),
  UpdateMessageLiveLocationViewed(UpdateMessageLiveLocationViewed),
  UpdateMessageMentionRead(UpdateMessageMentionRead),
  UpdateMessageSendAcknowledged(UpdateMessageSendAcknowledged),
  UpdateMessageSendFailed(UpdateMessageSendFailed),
  UpdateMessageSendSucceeded(UpdateMessageSendSucceeded),
  UpdateMessageViews(UpdateMessageViews),
  UpdateNewCallbackQuery(UpdateNewCallbackQuery),
  UpdateNewChat(UpdateNewChat),
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
  UpdateSupergroup(UpdateSupergroup),
  UpdateSupergroupFullInfo(UpdateSupergroupFullInfo),
  UpdateTermsOfService(UpdateTermsOfService),
  UpdateTrendingStickerSets(UpdateTrendingStickerSets),
  UpdateUnreadChatCount(UpdateUnreadChatCount),
  UpdateUnreadMessageCount(UpdateUnreadMessageCount),
  UpdateUser(UpdateUser),
  UpdateUserChatAction(UpdateUserChatAction),
  UpdateUserFullInfo(UpdateUserFullInfo),
  UpdateUserPrivacySettingRules(UpdateUserPrivacySettingRules),
  UpdateUserStatus(UpdateUserStatus),
  UpdateUsersNearby(UpdateUsersNearby),

  AuthorizationState(AuthorizationState),
  CanTransferOwnershipResult(CanTransferOwnershipResult),
  CheckChatUsernameResult(CheckChatUsernameResult),
  JsonValue(JsonValue),
  LanguagePackStringValue(LanguagePackStringValue),
  LogStream(LogStream),
  LoginUrlInfo(LoginUrlInfo),
  OptionValue(OptionValue),
  PassportElement(PassportElement),
  Update(Update),
  AccountTtl(AccountTtl),
  Animations(Animations),
  AuthenticationCodeInfo(AuthenticationCodeInfo),
  AutoDownloadSettingsPresets(AutoDownloadSettingsPresets),
  Background(Background),
  Backgrounds(Backgrounds),
  BasicGroup(BasicGroup),
  BasicGroupFullInfo(BasicGroupFullInfo),
  CallId(CallId),
  CallbackQueryAnswer(CallbackQueryAnswer),
  Chat(Chat),
  ChatAdministrators(ChatAdministrators),
  ChatEvents(ChatEvents),
  ChatInviteLink(ChatInviteLink),
  ChatInviteLinkInfo(ChatInviteLinkInfo),
  ChatMember(ChatMember),
  ChatMembers(ChatMembers),
  Chats(Chats),
  ChatsNearby(ChatsNearby),
  ConnectedWebsites(ConnectedWebsites),
  Count(Count),
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
  MessageLinkInfo(MessageLinkInfo),
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
  Proxies(Proxies),
  Proxy(Proxy),
  PublicMessageLink(PublicMessageLink),
  PushReceiverId(PushReceiverId),
  RecoveryEmailAddress(RecoveryEmailAddress),
  ScopeNotificationSettings(ScopeNotificationSettings),
  Seconds(Seconds),
  SecretChat(SecretChat),
  Session(Session),
  Sessions(Sessions),
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
  UserProfilePhotos(UserProfilePhotos),
  Users(Users),
  ValidatedOrderInfo(ValidatedOrderInfo),
  WebPage(WebPage),
  WebPageInstantView(WebPageInstantView),

}
impl<'de> Deserialize<'de> for TdType {
fn deserialize<D>(deserializer: D) -> Result<TdType, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      TdType,
  (testUseUpdate, TestUseUpdate);
  (updateActiveNotifications, UpdateActiveNotifications);
  (updateAuthorizationState, UpdateAuthorizationState);
  (updateBasicGroup, UpdateBasicGroup);
  (updateBasicGroupFullInfo, UpdateBasicGroupFullInfo);
  (updateCall, UpdateCall);
  (updateChatActionBar, UpdateChatActionBar);
  (updateChatChatList, UpdateChatChatList);
  (updateChatDefaultDisableNotification, UpdateChatDefaultDisableNotification);
  (updateChatDraftMessage, UpdateChatDraftMessage);
  (updateChatHasScheduledMessages, UpdateChatHasScheduledMessages);
  (updateChatIsMarkedAsUnread, UpdateChatIsMarkedAsUnread);
  (updateChatIsPinned, UpdateChatIsPinned);
  (updateChatIsSponsored, UpdateChatIsSponsored);
  (updateChatLastMessage, UpdateChatLastMessage);
  (updateChatNotificationSettings, UpdateChatNotificationSettings);
  (updateChatOnlineMemberCount, UpdateChatOnlineMemberCount);
  (updateChatOrder, UpdateChatOrder);
  (updateChatPermissions, UpdateChatPermissions);
  (updateChatPhoto, UpdateChatPhoto);
  (updateChatPinnedMessage, UpdateChatPinnedMessage);
  (updateChatReadInbox, UpdateChatReadInbox);
  (updateChatReadOutbox, UpdateChatReadOutbox);
  (updateChatReplyMarkup, UpdateChatReplyMarkup);
  (updateChatTitle, UpdateChatTitle);
  (updateChatUnreadMentionCount, UpdateChatUnreadMentionCount);
  (updateConnectionState, UpdateConnectionState);
  (updateDeleteMessages, UpdateDeleteMessages);
  (updateFavoriteStickers, UpdateFavoriteStickers);
  (updateFile, UpdateFile);
  (updateFileGenerationStart, UpdateFileGenerationStart);
  (updateFileGenerationStop, UpdateFileGenerationStop);
  (updateHavePendingNotifications, UpdateHavePendingNotifications);
  (updateInstalledStickerSets, UpdateInstalledStickerSets);
  (updateLanguagePackStrings, UpdateLanguagePackStrings);
  (updateMessageContent, UpdateMessageContent);
  (updateMessageContentOpened, UpdateMessageContentOpened);
  (updateMessageEdited, UpdateMessageEdited);
  (updateMessageLiveLocationViewed, UpdateMessageLiveLocationViewed);
  (updateMessageMentionRead, UpdateMessageMentionRead);
  (updateMessageSendAcknowledged, UpdateMessageSendAcknowledged);
  (updateMessageSendFailed, UpdateMessageSendFailed);
  (updateMessageSendSucceeded, UpdateMessageSendSucceeded);
  (updateMessageViews, UpdateMessageViews);
  (updateNewCallbackQuery, UpdateNewCallbackQuery);
  (updateNewChat, UpdateNewChat);
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
  (updateSupergroup, UpdateSupergroup);
  (updateSupergroupFullInfo, UpdateSupergroupFullInfo);
  (updateTermsOfService, UpdateTermsOfService);
  (updateTrendingStickerSets, UpdateTrendingStickerSets);
  (updateUnreadChatCount, UpdateUnreadChatCount);
  (updateUnreadMessageCount, UpdateUnreadMessageCount);
  (updateUser, UpdateUser);
  (updateUserChatAction, UpdateUserChatAction);
  (updateUserFullInfo, UpdateUserFullInfo);
  (updateUserPrivacySettingRules, UpdateUserPrivacySettingRules);
  (updateUserStatus, UpdateUserStatus);
  (updateUsersNearby, UpdateUsersNearby);

  (AuthorizationState, AuthorizationState);
  (CanTransferOwnershipResult, CanTransferOwnershipResult);
  (CheckChatUsernameResult, CheckChatUsernameResult);
  (JsonValue, JsonValue);
  (LanguagePackStringValue, LanguagePackStringValue);
  (LogStream, LogStream);
  (LoginUrlInfo, LoginUrlInfo);
  (OptionValue, OptionValue);
  (PassportElement, PassportElement);
  (Update, Update);
  (accountTtl, AccountTtl);
  (animations, Animations);
  (authenticationCodeInfo, AuthenticationCodeInfo);
  (autoDownloadSettingsPresets, AutoDownloadSettingsPresets);
  (background, Background);
  (backgrounds, Backgrounds);
  (basicGroup, BasicGroup);
  (basicGroupFullInfo, BasicGroupFullInfo);
  (callId, CallId);
  (callbackQueryAnswer, CallbackQueryAnswer);
  (chat, Chat);
  (chatAdministrators, ChatAdministrators);
  (chatEvents, ChatEvents);
  (chatInviteLink, ChatInviteLink);
  (chatInviteLinkInfo, ChatInviteLinkInfo);
  (chatMember, ChatMember);
  (chatMembers, ChatMembers);
  (chats, Chats);
  (chatsNearby, ChatsNearby);
  (connectedWebsites, ConnectedWebsites);
  (count, Count);
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
  (messageLinkInfo, MessageLinkInfo);
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
  (proxies, Proxies);
  (proxy, Proxy);
  (publicMessageLink, PublicMessageLink);
  (pushReceiverId, PushReceiverId);
  (recoveryEmailAddress, RecoveryEmailAddress);
  (scopeNotificationSettings, ScopeNotificationSettings);
  (seconds, Seconds);
  (secretChat, SecretChat);
  (session, Session);
  (sessions, Sessions);
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
  (userProfilePhotos, UserProfilePhotos);
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
      Self::UpdateAuthorizationState(value) => value.td_name(),
      Self::UpdateBasicGroup(value) => value.td_name(),
      Self::UpdateBasicGroupFullInfo(value) => value.td_name(),
      Self::UpdateCall(value) => value.td_name(),
      Self::UpdateChatActionBar(value) => value.td_name(),
      Self::UpdateChatChatList(value) => value.td_name(),
      Self::UpdateChatDefaultDisableNotification(value) => value.td_name(),
      Self::UpdateChatDraftMessage(value) => value.td_name(),
      Self::UpdateChatHasScheduledMessages(value) => value.td_name(),
      Self::UpdateChatIsMarkedAsUnread(value) => value.td_name(),
      Self::UpdateChatIsPinned(value) => value.td_name(),
      Self::UpdateChatIsSponsored(value) => value.td_name(),
      Self::UpdateChatLastMessage(value) => value.td_name(),
      Self::UpdateChatNotificationSettings(value) => value.td_name(),
      Self::UpdateChatOnlineMemberCount(value) => value.td_name(),
      Self::UpdateChatOrder(value) => value.td_name(),
      Self::UpdateChatPermissions(value) => value.td_name(),
      Self::UpdateChatPhoto(value) => value.td_name(),
      Self::UpdateChatPinnedMessage(value) => value.td_name(),
      Self::UpdateChatReadInbox(value) => value.td_name(),
      Self::UpdateChatReadOutbox(value) => value.td_name(),
      Self::UpdateChatReplyMarkup(value) => value.td_name(),
      Self::UpdateChatTitle(value) => value.td_name(),
      Self::UpdateChatUnreadMentionCount(value) => value.td_name(),
      Self::UpdateConnectionState(value) => value.td_name(),
      Self::UpdateDeleteMessages(value) => value.td_name(),
      Self::UpdateFavoriteStickers(value) => value.td_name(),
      Self::UpdateFile(value) => value.td_name(),
      Self::UpdateFileGenerationStart(value) => value.td_name(),
      Self::UpdateFileGenerationStop(value) => value.td_name(),
      Self::UpdateHavePendingNotifications(value) => value.td_name(),
      Self::UpdateInstalledStickerSets(value) => value.td_name(),
      Self::UpdateLanguagePackStrings(value) => value.td_name(),
      Self::UpdateMessageContent(value) => value.td_name(),
      Self::UpdateMessageContentOpened(value) => value.td_name(),
      Self::UpdateMessageEdited(value) => value.td_name(),
      Self::UpdateMessageLiveLocationViewed(value) => value.td_name(),
      Self::UpdateMessageMentionRead(value) => value.td_name(),
      Self::UpdateMessageSendAcknowledged(value) => value.td_name(),
      Self::UpdateMessageSendFailed(value) => value.td_name(),
      Self::UpdateMessageSendSucceeded(value) => value.td_name(),
      Self::UpdateMessageViews(value) => value.td_name(),
      Self::UpdateNewCallbackQuery(value) => value.td_name(),
      Self::UpdateNewChat(value) => value.td_name(),
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
      Self::UpdateSupergroup(value) => value.td_name(),
      Self::UpdateSupergroupFullInfo(value) => value.td_name(),
      Self::UpdateTermsOfService(value) => value.td_name(),
      Self::UpdateTrendingStickerSets(value) => value.td_name(),
      Self::UpdateUnreadChatCount(value) => value.td_name(),
      Self::UpdateUnreadMessageCount(value) => value.td_name(),
      Self::UpdateUser(value) => value.td_name(),
      Self::UpdateUserChatAction(value) => value.td_name(),
      Self::UpdateUserFullInfo(value) => value.td_name(),
      Self::UpdateUserPrivacySettingRules(value) => value.td_name(),
      Self::UpdateUserStatus(value) => value.td_name(),
      Self::UpdateUsersNearby(value) => value.td_name(),
    
      Self::AuthorizationState(value) => value.td_name(),
      Self::CanTransferOwnershipResult(value) => value.td_name(),
      Self::CheckChatUsernameResult(value) => value.td_name(),
      Self::JsonValue(value) => value.td_name(),
      Self::LanguagePackStringValue(value) => value.td_name(),
      Self::LogStream(value) => value.td_name(),
      Self::LoginUrlInfo(value) => value.td_name(),
      Self::OptionValue(value) => value.td_name(),
      Self::PassportElement(value) => value.td_name(),
      Self::Update(value) => value.td_name(),
      Self::AccountTtl(value) => value.td_name(),
      Self::Animations(value) => value.td_name(),
      Self::AuthenticationCodeInfo(value) => value.td_name(),
      Self::AutoDownloadSettingsPresets(value) => value.td_name(),
      Self::Background(value) => value.td_name(),
      Self::Backgrounds(value) => value.td_name(),
      Self::BasicGroup(value) => value.td_name(),
      Self::BasicGroupFullInfo(value) => value.td_name(),
      Self::CallId(value) => value.td_name(),
      Self::CallbackQueryAnswer(value) => value.td_name(),
      Self::Chat(value) => value.td_name(),
      Self::ChatAdministrators(value) => value.td_name(),
      Self::ChatEvents(value) => value.td_name(),
      Self::ChatInviteLink(value) => value.td_name(),
      Self::ChatInviteLinkInfo(value) => value.td_name(),
      Self::ChatMember(value) => value.td_name(),
      Self::ChatMembers(value) => value.td_name(),
      Self::Chats(value) => value.td_name(),
      Self::ChatsNearby(value) => value.td_name(),
      Self::ConnectedWebsites(value) => value.td_name(),
      Self::Count(value) => value.td_name(),
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
      Self::MessageLinkInfo(value) => value.td_name(),
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
      Self::Proxies(value) => value.td_name(),
      Self::Proxy(value) => value.td_name(),
      Self::PublicMessageLink(value) => value.td_name(),
      Self::PushReceiverId(value) => value.td_name(),
      Self::RecoveryEmailAddress(value) => value.td_name(),
      Self::ScopeNotificationSettings(value) => value.td_name(),
      Self::Seconds(value) => value.td_name(),
      Self::SecretChat(value) => value.td_name(),
      Self::Session(value) => value.td_name(),
      Self::Sessions(value) => value.td_name(),
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
      Self::UserProfilePhotos(value) => value.td_name(),
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
        Self::UpdateAuthorizationState(value) => value.extra(),
        Self::UpdateBasicGroup(value) => value.extra(),
        Self::UpdateBasicGroupFullInfo(value) => value.extra(),
        Self::UpdateCall(value) => value.extra(),
        Self::UpdateChatActionBar(value) => value.extra(),
        Self::UpdateChatChatList(value) => value.extra(),
        Self::UpdateChatDefaultDisableNotification(value) => value.extra(),
        Self::UpdateChatDraftMessage(value) => value.extra(),
        Self::UpdateChatHasScheduledMessages(value) => value.extra(),
        Self::UpdateChatIsMarkedAsUnread(value) => value.extra(),
        Self::UpdateChatIsPinned(value) => value.extra(),
        Self::UpdateChatIsSponsored(value) => value.extra(),
        Self::UpdateChatLastMessage(value) => value.extra(),
        Self::UpdateChatNotificationSettings(value) => value.extra(),
        Self::UpdateChatOnlineMemberCount(value) => value.extra(),
        Self::UpdateChatOrder(value) => value.extra(),
        Self::UpdateChatPermissions(value) => value.extra(),
        Self::UpdateChatPhoto(value) => value.extra(),
        Self::UpdateChatPinnedMessage(value) => value.extra(),
        Self::UpdateChatReadInbox(value) => value.extra(),
        Self::UpdateChatReadOutbox(value) => value.extra(),
        Self::UpdateChatReplyMarkup(value) => value.extra(),
        Self::UpdateChatTitle(value) => value.extra(),
        Self::UpdateChatUnreadMentionCount(value) => value.extra(),
        Self::UpdateConnectionState(value) => value.extra(),
        Self::UpdateDeleteMessages(value) => value.extra(),
        Self::UpdateFavoriteStickers(value) => value.extra(),
        Self::UpdateFile(value) => value.extra(),
        Self::UpdateFileGenerationStart(value) => value.extra(),
        Self::UpdateFileGenerationStop(value) => value.extra(),
        Self::UpdateHavePendingNotifications(value) => value.extra(),
        Self::UpdateInstalledStickerSets(value) => value.extra(),
        Self::UpdateLanguagePackStrings(value) => value.extra(),
        Self::UpdateMessageContent(value) => value.extra(),
        Self::UpdateMessageContentOpened(value) => value.extra(),
        Self::UpdateMessageEdited(value) => value.extra(),
        Self::UpdateMessageLiveLocationViewed(value) => value.extra(),
        Self::UpdateMessageMentionRead(value) => value.extra(),
        Self::UpdateMessageSendAcknowledged(value) => value.extra(),
        Self::UpdateMessageSendFailed(value) => value.extra(),
        Self::UpdateMessageSendSucceeded(value) => value.extra(),
        Self::UpdateMessageViews(value) => value.extra(),
        Self::UpdateNewCallbackQuery(value) => value.extra(),
        Self::UpdateNewChat(value) => value.extra(),
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
        Self::UpdateSupergroup(value) => value.extra(),
        Self::UpdateSupergroupFullInfo(value) => value.extra(),
        Self::UpdateTermsOfService(value) => value.extra(),
        Self::UpdateTrendingStickerSets(value) => value.extra(),
        Self::UpdateUnreadChatCount(value) => value.extra(),
        Self::UpdateUnreadMessageCount(value) => value.extra(),
        Self::UpdateUser(value) => value.extra(),
        Self::UpdateUserChatAction(value) => value.extra(),
        Self::UpdateUserFullInfo(value) => value.extra(),
        Self::UpdateUserPrivacySettingRules(value) => value.extra(),
        Self::UpdateUserStatus(value) => value.extra(),
        Self::UpdateUsersNearby(value) => value.extra(),
      
        Self::AuthorizationState(value) => value.extra(),
        Self::CanTransferOwnershipResult(value) => value.extra(),
        Self::CheckChatUsernameResult(value) => value.extra(),
        Self::JsonValue(value) => value.extra(),
        Self::LanguagePackStringValue(value) => value.extra(),
        Self::LogStream(value) => value.extra(),
        Self::LoginUrlInfo(value) => value.extra(),
        Self::OptionValue(value) => value.extra(),
        Self::PassportElement(value) => value.extra(),
        Self::Update(value) => value.extra(),
        Self::AccountTtl(value) => value.extra(),
        Self::Animations(value) => value.extra(),
        Self::AuthenticationCodeInfo(value) => value.extra(),
        Self::AutoDownloadSettingsPresets(value) => value.extra(),
        Self::Background(value) => value.extra(),
        Self::Backgrounds(value) => value.extra(),
        Self::BasicGroup(value) => value.extra(),
        Self::BasicGroupFullInfo(value) => value.extra(),
        Self::CallId(value) => value.extra(),
        Self::CallbackQueryAnswer(value) => value.extra(),
        Self::Chat(value) => value.extra(),
        Self::ChatAdministrators(value) => value.extra(),
        Self::ChatEvents(value) => value.extra(),
        Self::ChatInviteLink(value) => value.extra(),
        Self::ChatInviteLinkInfo(value) => value.extra(),
        Self::ChatMember(value) => value.extra(),
        Self::ChatMembers(value) => value.extra(),
        Self::Chats(value) => value.extra(),
        Self::ChatsNearby(value) => value.extra(),
        Self::ConnectedWebsites(value) => value.extra(),
        Self::Count(value) => value.extra(),
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
        Self::MessageLinkInfo(value) => value.extra(),
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
        Self::Proxies(value) => value.extra(),
        Self::Proxy(value) => value.extra(),
        Self::PublicMessageLink(value) => value.extra(),
        Self::PushReceiverId(value) => value.extra(),
        Self::RecoveryEmailAddress(value) => value.extra(),
        Self::ScopeNotificationSettings(value) => value.extra(),
        Self::Seconds(value) => value.extra(),
        Self::SecretChat(value) => value.extra(),
        Self::Session(value) => value.extra(),
        Self::Sessions(value) => value.extra(),
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
        Self::UserProfilePhotos(value) => value.extra(),
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
        Self::UpdateAuthorizationState(value) => value.to_json(),
        Self::UpdateBasicGroup(value) => value.to_json(),
        Self::UpdateBasicGroupFullInfo(value) => value.to_json(),
        Self::UpdateCall(value) => value.to_json(),
        Self::UpdateChatActionBar(value) => value.to_json(),
        Self::UpdateChatChatList(value) => value.to_json(),
        Self::UpdateChatDefaultDisableNotification(value) => value.to_json(),
        Self::UpdateChatDraftMessage(value) => value.to_json(),
        Self::UpdateChatHasScheduledMessages(value) => value.to_json(),
        Self::UpdateChatIsMarkedAsUnread(value) => value.to_json(),
        Self::UpdateChatIsPinned(value) => value.to_json(),
        Self::UpdateChatIsSponsored(value) => value.to_json(),
        Self::UpdateChatLastMessage(value) => value.to_json(),
        Self::UpdateChatNotificationSettings(value) => value.to_json(),
        Self::UpdateChatOnlineMemberCount(value) => value.to_json(),
        Self::UpdateChatOrder(value) => value.to_json(),
        Self::UpdateChatPermissions(value) => value.to_json(),
        Self::UpdateChatPhoto(value) => value.to_json(),
        Self::UpdateChatPinnedMessage(value) => value.to_json(),
        Self::UpdateChatReadInbox(value) => value.to_json(),
        Self::UpdateChatReadOutbox(value) => value.to_json(),
        Self::UpdateChatReplyMarkup(value) => value.to_json(),
        Self::UpdateChatTitle(value) => value.to_json(),
        Self::UpdateChatUnreadMentionCount(value) => value.to_json(),
        Self::UpdateConnectionState(value) => value.to_json(),
        Self::UpdateDeleteMessages(value) => value.to_json(),
        Self::UpdateFavoriteStickers(value) => value.to_json(),
        Self::UpdateFile(value) => value.to_json(),
        Self::UpdateFileGenerationStart(value) => value.to_json(),
        Self::UpdateFileGenerationStop(value) => value.to_json(),
        Self::UpdateHavePendingNotifications(value) => value.to_json(),
        Self::UpdateInstalledStickerSets(value) => value.to_json(),
        Self::UpdateLanguagePackStrings(value) => value.to_json(),
        Self::UpdateMessageContent(value) => value.to_json(),
        Self::UpdateMessageContentOpened(value) => value.to_json(),
        Self::UpdateMessageEdited(value) => value.to_json(),
        Self::UpdateMessageLiveLocationViewed(value) => value.to_json(),
        Self::UpdateMessageMentionRead(value) => value.to_json(),
        Self::UpdateMessageSendAcknowledged(value) => value.to_json(),
        Self::UpdateMessageSendFailed(value) => value.to_json(),
        Self::UpdateMessageSendSucceeded(value) => value.to_json(),
        Self::UpdateMessageViews(value) => value.to_json(),
        Self::UpdateNewCallbackQuery(value) => value.to_json(),
        Self::UpdateNewChat(value) => value.to_json(),
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
        Self::UpdateSupergroup(value) => value.to_json(),
        Self::UpdateSupergroupFullInfo(value) => value.to_json(),
        Self::UpdateTermsOfService(value) => value.to_json(),
        Self::UpdateTrendingStickerSets(value) => value.to_json(),
        Self::UpdateUnreadChatCount(value) => value.to_json(),
        Self::UpdateUnreadMessageCount(value) => value.to_json(),
        Self::UpdateUser(value) => value.to_json(),
        Self::UpdateUserChatAction(value) => value.to_json(),
        Self::UpdateUserFullInfo(value) => value.to_json(),
        Self::UpdateUserPrivacySettingRules(value) => value.to_json(),
        Self::UpdateUserStatus(value) => value.to_json(),
        Self::UpdateUsersNearby(value) => value.to_json(),
      
        Self::AuthorizationState(value) => value.to_json(),
        Self::CanTransferOwnershipResult(value) => value.to_json(),
        Self::CheckChatUsernameResult(value) => value.to_json(),
        Self::JsonValue(value) => value.to_json(),
        Self::LanguagePackStringValue(value) => value.to_json(),
        Self::LogStream(value) => value.to_json(),
        Self::LoginUrlInfo(value) => value.to_json(),
        Self::OptionValue(value) => value.to_json(),
        Self::PassportElement(value) => value.to_json(),
        Self::Update(value) => value.to_json(),
        Self::AccountTtl(value) => value.to_json(),
        Self::Animations(value) => value.to_json(),
        Self::AuthenticationCodeInfo(value) => value.to_json(),
        Self::AutoDownloadSettingsPresets(value) => value.to_json(),
        Self::Background(value) => value.to_json(),
        Self::Backgrounds(value) => value.to_json(),
        Self::BasicGroup(value) => value.to_json(),
        Self::BasicGroupFullInfo(value) => value.to_json(),
        Self::CallId(value) => value.to_json(),
        Self::CallbackQueryAnswer(value) => value.to_json(),
        Self::Chat(value) => value.to_json(),
        Self::ChatAdministrators(value) => value.to_json(),
        Self::ChatEvents(value) => value.to_json(),
        Self::ChatInviteLink(value) => value.to_json(),
        Self::ChatInviteLinkInfo(value) => value.to_json(),
        Self::ChatMember(value) => value.to_json(),
        Self::ChatMembers(value) => value.to_json(),
        Self::Chats(value) => value.to_json(),
        Self::ChatsNearby(value) => value.to_json(),
        Self::ConnectedWebsites(value) => value.to_json(),
        Self::Count(value) => value.to_json(),
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
        Self::MessageLinkInfo(value) => value.to_json(),
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
        Self::Proxies(value) => value.to_json(),
        Self::Proxy(value) => value.to_json(),
        Self::PublicMessageLink(value) => value.to_json(),
        Self::PushReceiverId(value) => value.to_json(),
        Self::RecoveryEmailAddress(value) => value.to_json(),
        Self::ScopeNotificationSettings(value) => value.to_json(),
        Self::Seconds(value) => value.to_json(),
        Self::SecretChat(value) => value.to_json(),
        Self::Session(value) => value.to_json(),
        Self::Sessions(value) => value.to_json(),
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
        Self::UserProfilePhotos(value) => value.to_json(),
        Self::Users(value) => value.to_json(),
        Self::ValidatedOrderInfo(value) => value.to_json(),
        Self::WebPage(value) => value.to_json(),
        Self::WebPageInstantView(value) => value.to_json(),
      
    }
  }
}



#[cfg(test)]
mod tests {
  use crate::types::{TdType, from_json, UpdateAuthorizationState};

  #[test]
  fn test_deserialize_enum() {
    match from_json::<UpdateAuthorizationState>(r#"{"@type":"updateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters"}}"#) {
      Ok(t) => {},
      Err(e) => {panic!("{}", e)}
    };

    match from_json::<TdType>(r#"{"@type":"updateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters"}}"#) {
      Ok(t) => {
        match t {
          TdType::UpdateAuthorizationState(v) => {},
          _ => panic!("from_json failed: {:?}", t)
        }
      },
      Err(e) => {panic!("{}", e)}
    };
  }
}


