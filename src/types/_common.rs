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

impl<ACTIVESTORYSTATE: TDActiveStoryState> TDActiveStoryState for &ACTIVESTORYSTATE {}
impl<ACTIVESTORYSTATE: TDActiveStoryState> TDActiveStoryState for &mut ACTIVESTORYSTATE {}

impl<AFFILIATEPROGRAMSORTORDER: TDAffiliateProgramSortOrder> TDAffiliateProgramSortOrder
  for &AFFILIATEPROGRAMSORTORDER
{
}
impl<AFFILIATEPROGRAMSORTORDER: TDAffiliateProgramSortOrder> TDAffiliateProgramSortOrder
  for &mut AFFILIATEPROGRAMSORTORDER
{
}

impl<AFFILIATETYPE: TDAffiliateType> TDAffiliateType for &AFFILIATETYPE {}
impl<AFFILIATETYPE: TDAffiliateType> TDAffiliateType for &mut AFFILIATETYPE {}

impl<AUCTIONSTATE: TDAuctionState> TDAuctionState for &AUCTIONSTATE {}
impl<AUCTIONSTATE: TDAuctionState> TDAuctionState for &mut AUCTIONSTATE {}

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

impl<AUTOSAVESETTINGSSCOPE: TDAutosaveSettingsScope> TDAutosaveSettingsScope
  for &AUTOSAVESETTINGSSCOPE
{
}
impl<AUTOSAVESETTINGSSCOPE: TDAutosaveSettingsScope> TDAutosaveSettingsScope
  for &mut AUTOSAVESETTINGSSCOPE
{
}

impl<BACKGROUNDFILL: TDBackgroundFill> TDBackgroundFill for &BACKGROUNDFILL {}
impl<BACKGROUNDFILL: TDBackgroundFill> TDBackgroundFill for &mut BACKGROUNDFILL {}

impl<BACKGROUNDTYPE: TDBackgroundType> TDBackgroundType for &BACKGROUNDTYPE {}
impl<BACKGROUNDTYPE: TDBackgroundType> TDBackgroundType for &mut BACKGROUNDTYPE {}

impl<BLOCKLIST: TDBlockList> TDBlockList for &BLOCKLIST {}
impl<BLOCKLIST: TDBlockList> TDBlockList for &mut BLOCKLIST {}

impl<BOTCOMMANDSCOPE: TDBotCommandScope> TDBotCommandScope for &BOTCOMMANDSCOPE {}
impl<BOTCOMMANDSCOPE: TDBotCommandScope> TDBotCommandScope for &mut BOTCOMMANDSCOPE {}

impl<BOTWRITEACCESSALLOWREASON: TDBotWriteAccessAllowReason> TDBotWriteAccessAllowReason
  for &BOTWRITEACCESSALLOWREASON
{
}
impl<BOTWRITEACCESSALLOWREASON: TDBotWriteAccessAllowReason> TDBotWriteAccessAllowReason
  for &mut BOTWRITEACCESSALLOWREASON
{
}

impl<BUILTINTHEME: TDBuiltInTheme> TDBuiltInTheme for &BUILTINTHEME {}
impl<BUILTINTHEME: TDBuiltInTheme> TDBuiltInTheme for &mut BUILTINTHEME {}

impl<BUSINESSAWAYMESSAGESCHEDULE: TDBusinessAwayMessageSchedule> TDBusinessAwayMessageSchedule
  for &BUSINESSAWAYMESSAGESCHEDULE
{
}
impl<BUSINESSAWAYMESSAGESCHEDULE: TDBusinessAwayMessageSchedule> TDBusinessAwayMessageSchedule
  for &mut BUSINESSAWAYMESSAGESCHEDULE
{
}

impl<BUSINESSFEATURE: TDBusinessFeature> TDBusinessFeature for &BUSINESSFEATURE {}
impl<BUSINESSFEATURE: TDBusinessFeature> TDBusinessFeature for &mut BUSINESSFEATURE {}

impl<BUTTONSTYLE: TDButtonStyle> TDButtonStyle for &BUTTONSTYLE {}
impl<BUTTONSTYLE: TDButtonStyle> TDButtonStyle for &mut BUTTONSTYLE {}

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

impl<CANPOSTSTORYRESULT: TDCanPostStoryResult> TDCanPostStoryResult for &CANPOSTSTORYRESULT {}
impl<CANPOSTSTORYRESULT: TDCanPostStoryResult> TDCanPostStoryResult for &mut CANPOSTSTORYRESULT {}

impl<CANSENDGIFTRESULT: TDCanSendGiftResult> TDCanSendGiftResult for &CANSENDGIFTRESULT {}
impl<CANSENDGIFTRESULT: TDCanSendGiftResult> TDCanSendGiftResult for &mut CANSENDGIFTRESULT {}

impl<CANSENDMESSAGETOUSERRESULT: TDCanSendMessageToUserResult> TDCanSendMessageToUserResult
  for &CANSENDMESSAGETOUSERRESULT
{
}
impl<CANSENDMESSAGETOUSERRESULT: TDCanSendMessageToUserResult> TDCanSendMessageToUserResult
  for &mut CANSENDMESSAGETOUSERRESULT
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

impl<CHATAVAILABLEREACTIONS: TDChatAvailableReactions> TDChatAvailableReactions
  for &CHATAVAILABLEREACTIONS
{
}
impl<CHATAVAILABLEREACTIONS: TDChatAvailableReactions> TDChatAvailableReactions
  for &mut CHATAVAILABLEREACTIONS
{
}

impl<CHATBOOSTSOURCE: TDChatBoostSource> TDChatBoostSource for &CHATBOOSTSOURCE {}
impl<CHATBOOSTSOURCE: TDChatBoostSource> TDChatBoostSource for &mut CHATBOOSTSOURCE {}

impl<CHATEVENTACTION: TDChatEventAction> TDChatEventAction for &CHATEVENTACTION {}
impl<CHATEVENTACTION: TDChatEventAction> TDChatEventAction for &mut CHATEVENTACTION {}

impl<CHATJOINREQUESTRESULT: TDChatJoinRequestResult> TDChatJoinRequestResult
  for &CHATJOINREQUESTRESULT
{
}
impl<CHATJOINREQUESTRESULT: TDChatJoinRequestResult> TDChatJoinRequestResult
  for &mut CHATJOINREQUESTRESULT
{
}

impl<CHATJOINRESULT: TDChatJoinResult> TDChatJoinResult for &CHATJOINRESULT {}
impl<CHATJOINRESULT: TDChatJoinResult> TDChatJoinResult for &mut CHATJOINRESULT {}

impl<CHATLIST: TDChatList> TDChatList for &CHATLIST {}
impl<CHATLIST: TDChatList> TDChatList for &mut CHATLIST {}

impl<CHATMEMBERSTATUS: TDChatMemberStatus> TDChatMemberStatus for &CHATMEMBERSTATUS {}
impl<CHATMEMBERSTATUS: TDChatMemberStatus> TDChatMemberStatus for &mut CHATMEMBERSTATUS {}

impl<CHATMEMBERSFILTER: TDChatMembersFilter> TDChatMembersFilter for &CHATMEMBERSFILTER {}
impl<CHATMEMBERSFILTER: TDChatMembersFilter> TDChatMembersFilter for &mut CHATMEMBERSFILTER {}

impl<CHATPHOTOSTICKERTYPE: TDChatPhotoStickerType> TDChatPhotoStickerType
  for &CHATPHOTOSTICKERTYPE
{
}
impl<CHATPHOTOSTICKERTYPE: TDChatPhotoStickerType> TDChatPhotoStickerType
  for &mut CHATPHOTOSTICKERTYPE
{
}

impl<CHATREVENUETRANSACTIONTYPE: TDChatRevenueTransactionType> TDChatRevenueTransactionType
  for &CHATREVENUETRANSACTIONTYPE
{
}
impl<CHATREVENUETRANSACTIONTYPE: TDChatRevenueTransactionType> TDChatRevenueTransactionType
  for &mut CHATREVENUETRANSACTIONTYPE
{
}

impl<CHATSOURCE: TDChatSource> TDChatSource for &CHATSOURCE {}
impl<CHATSOURCE: TDChatSource> TDChatSource for &mut CHATSOURCE {}

impl<CHATSTATISTICS: TDChatStatistics> TDChatStatistics for &CHATSTATISTICS {}
impl<CHATSTATISTICS: TDChatStatistics> TDChatStatistics for &mut CHATSTATISTICS {}

impl<CHATSTATISTICSOBJECTTYPE: TDChatStatisticsObjectType> TDChatStatisticsObjectType
  for &CHATSTATISTICSOBJECTTYPE
{
}
impl<CHATSTATISTICSOBJECTTYPE: TDChatStatisticsObjectType> TDChatStatisticsObjectType
  for &mut CHATSTATISTICSOBJECTTYPE
{
}

impl<CHATTHEME: TDChatTheme> TDChatTheme for &CHATTHEME {}
impl<CHATTHEME: TDChatTheme> TDChatTheme for &mut CHATTHEME {}

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

impl<COLLECTIBLEITEMTYPE: TDCollectibleItemType> TDCollectibleItemType for &COLLECTIBLEITEMTYPE {}
impl<COLLECTIBLEITEMTYPE: TDCollectibleItemType> TDCollectibleItemType
  for &mut COLLECTIBLEITEMTYPE
{
}

impl<COMMUNITYMEMBERSTATUS: TDCommunityMemberStatus> TDCommunityMemberStatus
  for &COMMUNITYMEMBERSTATUS
{
}
impl<COMMUNITYMEMBERSTATUS: TDCommunityMemberStatus> TDCommunityMemberStatus
  for &mut COMMUNITYMEMBERSTATUS
{
}

impl<CONNECTIONSTATE: TDConnectionState> TDConnectionState for &CONNECTIONSTATE {}
impl<CONNECTIONSTATE: TDConnectionState> TDConnectionState for &mut CONNECTIONSTATE {}

impl<CRAFTGIFTRESULT: TDCraftGiftResult> TDCraftGiftResult for &CRAFTGIFTRESULT {}
impl<CRAFTGIFTRESULT: TDCraftGiftResult> TDCraftGiftResult for &mut CRAFTGIFTRESULT {}

impl<DATETIMEFORMATTINGTYPE: TDDateTimeFormattingType> TDDateTimeFormattingType
  for &DATETIMEFORMATTINGTYPE
{
}
impl<DATETIMEFORMATTINGTYPE: TDDateTimeFormattingType> TDDateTimeFormattingType
  for &mut DATETIMEFORMATTINGTYPE
{
}

impl<DATETIMEPARTPRECISION: TDDateTimePartPrecision> TDDateTimePartPrecision
  for &DATETIMEPARTPRECISION
{
}
impl<DATETIMEPARTPRECISION: TDDateTimePartPrecision> TDDateTimePartPrecision
  for &mut DATETIMEPARTPRECISION
{
}

impl<DEVICETOKEN: TDDeviceToken> TDDeviceToken for &DEVICETOKEN {}
impl<DEVICETOKEN: TDDeviceToken> TDDeviceToken for &mut DEVICETOKEN {}

impl<DICESTICKERS: TDDiceStickers> TDDiceStickers for &DICESTICKERS {}
impl<DICESTICKERS: TDDiceStickers> TDDiceStickers for &mut DICESTICKERS {}

impl<DIFFENTITYTYPE: TDDiffEntityType> TDDiffEntityType for &DIFFENTITYTYPE {}
impl<DIFFENTITYTYPE: TDDiffEntityType> TDDiffEntityType for &mut DIFFENTITYTYPE {}

impl<DRAFTMESSAGECONTENT: TDDraftMessageContent> TDDraftMessageContent for &DRAFTMESSAGECONTENT {}
impl<DRAFTMESSAGECONTENT: TDDraftMessageContent> TDDraftMessageContent
  for &mut DRAFTMESSAGECONTENT
{
}

impl<EMAILADDRESSAUTHENTICATION: TDEmailAddressAuthentication> TDEmailAddressAuthentication
  for &EMAILADDRESSAUTHENTICATION
{
}
impl<EMAILADDRESSAUTHENTICATION: TDEmailAddressAuthentication> TDEmailAddressAuthentication
  for &mut EMAILADDRESSAUTHENTICATION
{
}

impl<EMAILADDRESSRESETSTATE: TDEmailAddressResetState> TDEmailAddressResetState
  for &EMAILADDRESSRESETSTATE
{
}
impl<EMAILADDRESSRESETSTATE: TDEmailAddressResetState> TDEmailAddressResetState
  for &mut EMAILADDRESSRESETSTATE
{
}

impl<EMOJICATEGORYSOURCE: TDEmojiCategorySource> TDEmojiCategorySource for &EMOJICATEGORYSOURCE {}
impl<EMOJICATEGORYSOURCE: TDEmojiCategorySource> TDEmojiCategorySource
  for &mut EMOJICATEGORYSOURCE
{
}

impl<EMOJICATEGORYTYPE: TDEmojiCategoryType> TDEmojiCategoryType for &EMOJICATEGORYTYPE {}
impl<EMOJICATEGORYTYPE: TDEmojiCategoryType> TDEmojiCategoryType for &mut EMOJICATEGORYTYPE {}

impl<EMOJISTATUSTYPE: TDEmojiStatusType> TDEmojiStatusType for &EMOJISTATUSTYPE {}
impl<EMOJISTATUSTYPE: TDEmojiStatusType> TDEmojiStatusType for &mut EMOJISTATUSTYPE {}

impl<EMOJITYPE: TDEmojiType> TDEmojiType for &EMOJITYPE {}
impl<EMOJITYPE: TDEmojiType> TDEmojiType for &mut EMOJITYPE {}

impl<FILETYPE: TDFileType> TDFileType for &FILETYPE {}
impl<FILETYPE: TDFileType> TDFileType for &mut FILETYPE {}

impl<FIREBASEAUTHENTICATIONSETTINGS: TDFirebaseAuthenticationSettings>
  TDFirebaseAuthenticationSettings for &FIREBASEAUTHENTICATIONSETTINGS
{
}
impl<FIREBASEAUTHENTICATIONSETTINGS: TDFirebaseAuthenticationSettings>
  TDFirebaseAuthenticationSettings for &mut FIREBASEAUTHENTICATIONSETTINGS
{
}

impl<FIREBASEDEVICEVERIFICATIONPARAMETERS: TDFirebaseDeviceVerificationParameters>
  TDFirebaseDeviceVerificationParameters for &FIREBASEDEVICEVERIFICATIONPARAMETERS
{
}
impl<FIREBASEDEVICEVERIFICATIONPARAMETERS: TDFirebaseDeviceVerificationParameters>
  TDFirebaseDeviceVerificationParameters for &mut FIREBASEDEVICEVERIFICATIONPARAMETERS
{
}

impl<GIFTFORRESALEORDER: TDGiftForResaleOrder> TDGiftForResaleOrder for &GIFTFORRESALEORDER {}
impl<GIFTFORRESALEORDER: TDGiftForResaleOrder> TDGiftForResaleOrder for &mut GIFTFORRESALEORDER {}

impl<GIFTPURCHASEOFFERSTATE: TDGiftPurchaseOfferState> TDGiftPurchaseOfferState
  for &GIFTPURCHASEOFFERSTATE
{
}
impl<GIFTPURCHASEOFFERSTATE: TDGiftPurchaseOfferState> TDGiftPurchaseOfferState
  for &mut GIFTPURCHASEOFFERSTATE
{
}

impl<GIFTRESALEPRICE: TDGiftResalePrice> TDGiftResalePrice for &GIFTRESALEPRICE {}
impl<GIFTRESALEPRICE: TDGiftResalePrice> TDGiftResalePrice for &mut GIFTRESALEPRICE {}

impl<GIFTRESALERESULT: TDGiftResaleResult> TDGiftResaleResult for &GIFTRESALERESULT {}
impl<GIFTRESALERESULT: TDGiftResaleResult> TDGiftResaleResult for &mut GIFTRESALERESULT {}

impl<GIVEAWAYINFO: TDGiveawayInfo> TDGiveawayInfo for &GIVEAWAYINFO {}
impl<GIVEAWAYINFO: TDGiveawayInfo> TDGiveawayInfo for &mut GIVEAWAYINFO {}

impl<GIVEAWAYPARTICIPANTSTATUS: TDGiveawayParticipantStatus> TDGiveawayParticipantStatus
  for &GIVEAWAYPARTICIPANTSTATUS
{
}
impl<GIVEAWAYPARTICIPANTSTATUS: TDGiveawayParticipantStatus> TDGiveawayParticipantStatus
  for &mut GIVEAWAYPARTICIPANTSTATUS
{
}

impl<GIVEAWAYPRIZE: TDGiveawayPrize> TDGiveawayPrize for &GIVEAWAYPRIZE {}
impl<GIVEAWAYPRIZE: TDGiveawayPrize> TDGiveawayPrize for &mut GIVEAWAYPRIZE {}

impl<GROUPCALLDATACHANNEL: TDGroupCallDataChannel> TDGroupCallDataChannel
  for &GROUPCALLDATACHANNEL
{
}
impl<GROUPCALLDATACHANNEL: TDGroupCallDataChannel> TDGroupCallDataChannel
  for &mut GROUPCALLDATACHANNEL
{
}

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

impl<INLINEQUERYRESULTSBUTTONTYPE: TDInlineQueryResultsButtonType> TDInlineQueryResultsButtonType
  for &INLINEQUERYRESULTSBUTTONTYPE
{
}
impl<INLINEQUERYRESULTSBUTTONTYPE: TDInlineQueryResultsButtonType> TDInlineQueryResultsButtonType
  for &mut INLINEQUERYRESULTSBUTTONTYPE
{
}

impl<INPUTBACKGROUND: TDInputBackground> TDInputBackground for &INPUTBACKGROUND {}
impl<INPUTBACKGROUND: TDInputBackground> TDInputBackground for &mut INPUTBACKGROUND {}

impl<INPUTCALL: TDInputCall> TDInputCall for &INPUTCALL {}
impl<INPUTCALL: TDInputCall> TDInputCall for &mut INPUTCALL {}

impl<INPUTCHATPHOTO: TDInputChatPhoto> TDInputChatPhoto for &INPUTCHATPHOTO {}
impl<INPUTCHATPHOTO: TDInputChatPhoto> TDInputChatPhoto for &mut INPUTCHATPHOTO {}

impl<INPUTCHATTHEME: TDInputChatTheme> TDInputChatTheme for &INPUTCHATTHEME {}
impl<INPUTCHATTHEME: TDInputChatTheme> TDInputChatTheme for &mut INPUTCHATTHEME {}

impl<INPUTCREDENTIALS: TDInputCredentials> TDInputCredentials for &INPUTCREDENTIALS {}
impl<INPUTCREDENTIALS: TDInputCredentials> TDInputCredentials for &mut INPUTCREDENTIALS {}

impl<INPUTFILE: TDInputFile> TDInputFile for &INPUTFILE {}
impl<INPUTFILE: TDInputFile> TDInputFile for &mut INPUTFILE {}

impl<INPUTGROUPCALL: TDInputGroupCall> TDInputGroupCall for &INPUTGROUPCALL {}
impl<INPUTGROUPCALL: TDInputGroupCall> TDInputGroupCall for &mut INPUTGROUPCALL {}

impl<INPUTINLINEQUERYRESULT: TDInputInlineQueryResult> TDInputInlineQueryResult
  for &INPUTINLINEQUERYRESULT
{
}
impl<INPUTINLINEQUERYRESULT: TDInputInlineQueryResult> TDInputInlineQueryResult
  for &mut INPUTINLINEQUERYRESULT
{
}

impl<INPUTINVOICE: TDInputInvoice> TDInputInvoice for &INPUTINVOICE {}
impl<INPUTINVOICE: TDInputInvoice> TDInputInvoice for &mut INPUTINVOICE {}

impl<INPUTMESSAGECONTENT: TDInputMessageContent> TDInputMessageContent for &INPUTMESSAGECONTENT {}
impl<INPUTMESSAGECONTENT: TDInputMessageContent> TDInputMessageContent
  for &mut INPUTMESSAGECONTENT
{
}

impl<INPUTMESSAGEREPLYTO: TDInputMessageReplyTo> TDInputMessageReplyTo for &INPUTMESSAGEREPLYTO {}
impl<INPUTMESSAGEREPLYTO: TDInputMessageReplyTo> TDInputMessageReplyTo
  for &mut INPUTMESSAGEREPLYTO
{
}

impl<INPUTPAGEBLOCK: TDInputPageBlock> TDInputPageBlock for &INPUTPAGEBLOCK {}
impl<INPUTPAGEBLOCK: TDInputPageBlock> TDInputPageBlock for &mut INPUTPAGEBLOCK {}

impl<INPUTPAIDMEDIATYPE: TDInputPaidMediaType> TDInputPaidMediaType for &INPUTPAIDMEDIATYPE {}
impl<INPUTPAIDMEDIATYPE: TDInputPaidMediaType> TDInputPaidMediaType for &mut INPUTPAIDMEDIATYPE {}

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

impl<INPUTPOLLMEDIA: TDInputPollMedia> TDInputPollMedia for &INPUTPOLLMEDIA {}
impl<INPUTPOLLMEDIA: TDInputPollMedia> TDInputPollMedia for &mut INPUTPOLLMEDIA {}

impl<INPUTPOLLTYPE: TDInputPollType> TDInputPollType for &INPUTPOLLTYPE {}
impl<INPUTPOLLTYPE: TDInputPollType> TDInputPollType for &mut INPUTPOLLTYPE {}

impl<INPUTPROFILESHARETHEME: TDInputProfileShareTheme> TDInputProfileShareTheme
  for &INPUTPROFILESHARETHEME
{
}
impl<INPUTPROFILESHARETHEME: TDInputProfileShareTheme> TDInputProfileShareTheme
  for &mut INPUTPROFILESHARETHEME
{
}

impl<INPUTSTORYAREATYPE: TDInputStoryAreaType> TDInputStoryAreaType for &INPUTSTORYAREATYPE {}
impl<INPUTSTORYAREATYPE: TDInputStoryAreaType> TDInputStoryAreaType for &mut INPUTSTORYAREATYPE {}

impl<INPUTSTORYCONTENT: TDInputStoryContent> TDInputStoryContent for &INPUTSTORYCONTENT {}
impl<INPUTSTORYCONTENT: TDInputStoryContent> TDInputStoryContent for &mut INPUTSTORYCONTENT {}

impl<INTERNALLINKTYPE: TDInternalLinkType> TDInternalLinkType for &INTERNALLINKTYPE {}
impl<INTERNALLINKTYPE: TDInternalLinkType> TDInternalLinkType for &mut INTERNALLINKTYPE {}

impl<INVITEGROUPCALLPARTICIPANTRESULT: TDInviteGroupCallParticipantResult>
  TDInviteGroupCallParticipantResult for &INVITEGROUPCALLPARTICIPANTRESULT
{
}
impl<INVITEGROUPCALLPARTICIPANTRESULT: TDInviteGroupCallParticipantResult>
  TDInviteGroupCallParticipantResult for &mut INVITEGROUPCALLPARTICIPANTRESULT
{
}

impl<INVITELINKCHATTYPE: TDInviteLinkChatType> TDInviteLinkChatType for &INVITELINKCHATTYPE {}
impl<INVITELINKCHATTYPE: TDInviteLinkChatType> TDInviteLinkChatType for &mut INVITELINKCHATTYPE {}

impl<JSONVALUE: TDJsonValue> TDJsonValue for &JSONVALUE {}
impl<JSONVALUE: TDJsonValue> TDJsonValue for &mut JSONVALUE {}

impl<KEYBOARDBUTTONSOURCE: TDKeyboardButtonSource> TDKeyboardButtonSource
  for &KEYBOARDBUTTONSOURCE
{
}
impl<KEYBOARDBUTTONSOURCE: TDKeyboardButtonSource> TDKeyboardButtonSource
  for &mut KEYBOARDBUTTONSOURCE
{
}

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

impl<LINKPREVIEWALBUMMEDIA: TDLinkPreviewAlbumMedia> TDLinkPreviewAlbumMedia
  for &LINKPREVIEWALBUMMEDIA
{
}
impl<LINKPREVIEWALBUMMEDIA: TDLinkPreviewAlbumMedia> TDLinkPreviewAlbumMedia
  for &mut LINKPREVIEWALBUMMEDIA
{
}

impl<LINKPREVIEWTYPE: TDLinkPreviewType> TDLinkPreviewType for &LINKPREVIEWTYPE {}
impl<LINKPREVIEWTYPE: TDLinkPreviewType> TDLinkPreviewType for &mut LINKPREVIEWTYPE {}

impl<LOGSTREAM: TDLogStream> TDLogStream for &LOGSTREAM {}
impl<LOGSTREAM: TDLogStream> TDLogStream for &mut LOGSTREAM {}

impl<LOGINURLINFO: TDLoginUrlInfo> TDLoginUrlInfo for &LOGINURLINFO {}
impl<LOGINURLINFO: TDLoginUrlInfo> TDLoginUrlInfo for &mut LOGINURLINFO {}

impl<MASKPOINT: TDMaskPoint> TDMaskPoint for &MASKPOINT {}
impl<MASKPOINT: TDMaskPoint> TDMaskPoint for &mut MASKPOINT {}

impl<MESSAGECONTENT: TDMessageContent> TDMessageContent for &MESSAGECONTENT {}
impl<MESSAGECONTENT: TDMessageContent> TDMessageContent for &mut MESSAGECONTENT {}

impl<MESSAGEEFFECTTYPE: TDMessageEffectType> TDMessageEffectType for &MESSAGEEFFECTTYPE {}
impl<MESSAGEEFFECTTYPE: TDMessageEffectType> TDMessageEffectType for &mut MESSAGEEFFECTTYPE {}

impl<MESSAGEFILETYPE: TDMessageFileType> TDMessageFileType for &MESSAGEFILETYPE {}
impl<MESSAGEFILETYPE: TDMessageFileType> TDMessageFileType for &mut MESSAGEFILETYPE {}

impl<MESSAGEORIGIN: TDMessageOrigin> TDMessageOrigin for &MESSAGEORIGIN {}
impl<MESSAGEORIGIN: TDMessageOrigin> TDMessageOrigin for &mut MESSAGEORIGIN {}

impl<MESSAGEREADDATE: TDMessageReadDate> TDMessageReadDate for &MESSAGEREADDATE {}
impl<MESSAGEREADDATE: TDMessageReadDate> TDMessageReadDate for &mut MESSAGEREADDATE {}

impl<MESSAGEREPLYTO: TDMessageReplyTo> TDMessageReplyTo for &MESSAGEREPLYTO {}
impl<MESSAGEREPLYTO: TDMessageReplyTo> TDMessageReplyTo for &mut MESSAGEREPLYTO {}

impl<MESSAGESCHEDULINGSTATE: TDMessageSchedulingState> TDMessageSchedulingState
  for &MESSAGESCHEDULINGSTATE
{
}
impl<MESSAGESCHEDULINGSTATE: TDMessageSchedulingState> TDMessageSchedulingState
  for &mut MESSAGESCHEDULINGSTATE
{
}

impl<MESSAGESELFDESTRUCTTYPE: TDMessageSelfDestructType> TDMessageSelfDestructType
  for &MESSAGESELFDESTRUCTTYPE
{
}
impl<MESSAGESELFDESTRUCTTYPE: TDMessageSelfDestructType> TDMessageSelfDestructType
  for &mut MESSAGESELFDESTRUCTTYPE
{
}

impl<MESSAGESENDER: TDMessageSender> TDMessageSender for &MESSAGESENDER {}
impl<MESSAGESENDER: TDMessageSender> TDMessageSender for &mut MESSAGESENDER {}

impl<MESSAGESENDINGSTATE: TDMessageSendingState> TDMessageSendingState for &MESSAGESENDINGSTATE {}
impl<MESSAGESENDINGSTATE: TDMessageSendingState> TDMessageSendingState
  for &mut MESSAGESENDINGSTATE
{
}

impl<MESSAGESOURCE: TDMessageSource> TDMessageSource for &MESSAGESOURCE {}
impl<MESSAGESOURCE: TDMessageSource> TDMessageSource for &mut MESSAGESOURCE {}

impl<MESSAGETOPIC: TDMessageTopic> TDMessageTopic for &MESSAGETOPIC {}
impl<MESSAGETOPIC: TDMessageTopic> TDMessageTopic for &mut MESSAGETOPIC {}

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

impl<PAIDMEDIA: TDPaidMedia> TDPaidMedia for &PAIDMEDIA {}
impl<PAIDMEDIA: TDPaidMedia> TDPaidMedia for &mut PAIDMEDIA {}

impl<PAIDREACTIONTYPE: TDPaidReactionType> TDPaidReactionType for &PAIDREACTIONTYPE {}
impl<PAIDREACTIONTYPE: TDPaidReactionType> TDPaidReactionType for &mut PAIDREACTIONTYPE {}

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

impl<PAYMENTFORMTYPE: TDPaymentFormType> TDPaymentFormType for &PAYMENTFORMTYPE {}
impl<PAYMENTFORMTYPE: TDPaymentFormType> TDPaymentFormType for &mut PAYMENTFORMTYPE {}

impl<PAYMENTPROVIDER: TDPaymentProvider> TDPaymentProvider for &PAYMENTPROVIDER {}
impl<PAYMENTPROVIDER: TDPaymentProvider> TDPaymentProvider for &mut PAYMENTPROVIDER {}

impl<PAYMENTRECEIPTTYPE: TDPaymentReceiptType> TDPaymentReceiptType for &PAYMENTRECEIPTTYPE {}
impl<PAYMENTRECEIPTTYPE: TDPaymentReceiptType> TDPaymentReceiptType for &mut PAYMENTRECEIPTTYPE {}

impl<PHONENUMBERCODETYPE: TDPhoneNumberCodeType> TDPhoneNumberCodeType for &PHONENUMBERCODETYPE {}
impl<PHONENUMBERCODETYPE: TDPhoneNumberCodeType> TDPhoneNumberCodeType
  for &mut PHONENUMBERCODETYPE
{
}

impl<POLLMEDIA: TDPollMedia> TDPollMedia for &POLLMEDIA {}
impl<POLLMEDIA: TDPollMedia> TDPollMedia for &mut POLLMEDIA {}

impl<POLLTYPE: TDPollType> TDPollType for &POLLTYPE {}
impl<POLLTYPE: TDPollType> TDPollType for &mut POLLTYPE {}

impl<POLLVOTERESTRICTIONREASON: TDPollVoteRestrictionReason> TDPollVoteRestrictionReason
  for &POLLVOTERESTRICTIONREASON
{
}
impl<POLLVOTERESTRICTIONREASON: TDPollVoteRestrictionReason> TDPollVoteRestrictionReason
  for &mut POLLVOTERESTRICTIONREASON
{
}

impl<PREMIUMFEATURE: TDPremiumFeature> TDPremiumFeature for &PREMIUMFEATURE {}
impl<PREMIUMFEATURE: TDPremiumFeature> TDPremiumFeature for &mut PREMIUMFEATURE {}

impl<PREMIUMLIMITTYPE: TDPremiumLimitType> TDPremiumLimitType for &PREMIUMLIMITTYPE {}
impl<PREMIUMLIMITTYPE: TDPremiumLimitType> TDPremiumLimitType for &mut PREMIUMLIMITTYPE {}

impl<PREMIUMSOURCE: TDPremiumSource> TDPremiumSource for &PREMIUMSOURCE {}
impl<PREMIUMSOURCE: TDPremiumSource> TDPremiumSource for &mut PREMIUMSOURCE {}

impl<PREMIUMSTORYFEATURE: TDPremiumStoryFeature> TDPremiumStoryFeature for &PREMIUMSTORYFEATURE {}
impl<PREMIUMSTORYFEATURE: TDPremiumStoryFeature> TDPremiumStoryFeature
  for &mut PREMIUMSTORYFEATURE
{
}

impl<PROFILESHARETHEME: TDProfileShareTheme> TDProfileShareTheme for &PROFILESHARETHEME {}
impl<PROFILESHARETHEME: TDProfileShareTheme> TDProfileShareTheme for &mut PROFILESHARETHEME {}

impl<PROFILETAB: TDProfileTab> TDProfileTab for &PROFILETAB {}
impl<PROFILETAB: TDProfileTab> TDProfileTab for &mut PROFILETAB {}

impl<PROXYTYPE: TDProxyType> TDProxyType for &PROXYTYPE {}
impl<PROXYTYPE: TDProxyType> TDProxyType for &mut PROXYTYPE {}

impl<PUBLICCHATTYPE: TDPublicChatType> TDPublicChatType for &PUBLICCHATTYPE {}
impl<PUBLICCHATTYPE: TDPublicChatType> TDPublicChatType for &mut PUBLICCHATTYPE {}

impl<PUBLICFORWARD: TDPublicForward> TDPublicForward for &PUBLICFORWARD {}
impl<PUBLICFORWARD: TDPublicForward> TDPublicForward for &mut PUBLICFORWARD {}

impl<PUSHMESSAGECONTENT: TDPushMessageContent> TDPushMessageContent for &PUSHMESSAGECONTENT {}
impl<PUSHMESSAGECONTENT: TDPushMessageContent> TDPushMessageContent for &mut PUSHMESSAGECONTENT {}

impl<REACTIONNOTIFICATIONSOURCE: TDReactionNotificationSource> TDReactionNotificationSource
  for &REACTIONNOTIFICATIONSOURCE
{
}
impl<REACTIONNOTIFICATIONSOURCE: TDReactionNotificationSource> TDReactionNotificationSource
  for &mut REACTIONNOTIFICATIONSOURCE
{
}

impl<REACTIONTYPE: TDReactionType> TDReactionType for &REACTIONTYPE {}
impl<REACTIONTYPE: TDReactionType> TDReactionType for &mut REACTIONTYPE {}

impl<REACTIONUNAVAILABILITYREASON: TDReactionUnavailabilityReason> TDReactionUnavailabilityReason
  for &REACTIONUNAVAILABILITYREASON
{
}
impl<REACTIONUNAVAILABILITYREASON: TDReactionUnavailabilityReason> TDReactionUnavailabilityReason
  for &mut REACTIONUNAVAILABILITYREASON
{
}

impl<REPLYMARKUP: TDReplyMarkup> TDReplyMarkup for &REPLYMARKUP {}
impl<REPLYMARKUP: TDReplyMarkup> TDReplyMarkup for &mut REPLYMARKUP {}

impl<REPORTCHATRESULT: TDReportChatResult> TDReportChatResult for &REPORTCHATRESULT {}
impl<REPORTCHATRESULT: TDReportChatResult> TDReportChatResult for &mut REPORTCHATRESULT {}

impl<REPORTREASON: TDReportReason> TDReportReason for &REPORTREASON {}
impl<REPORTREASON: TDReportReason> TDReportReason for &mut REPORTREASON {}

impl<REPORTSPONSOREDRESULT: TDReportSponsoredResult> TDReportSponsoredResult
  for &REPORTSPONSOREDRESULT
{
}
impl<REPORTSPONSOREDRESULT: TDReportSponsoredResult> TDReportSponsoredResult
  for &mut REPORTSPONSOREDRESULT
{
}

impl<REPORTSTORYRESULT: TDReportStoryResult> TDReportStoryResult for &REPORTSTORYRESULT {}
impl<REPORTSTORYRESULT: TDReportStoryResult> TDReportStoryResult for &mut REPORTSTORYRESULT {}

impl<RESENDCODEREASON: TDResendCodeReason> TDResendCodeReason for &RESENDCODEREASON {}
impl<RESENDCODEREASON: TDResendCodeReason> TDResendCodeReason for &mut RESENDCODEREASON {}

impl<RESETPASSWORDRESULT: TDResetPasswordResult> TDResetPasswordResult for &RESETPASSWORDRESULT {}
impl<RESETPASSWORDRESULT: TDResetPasswordResult> TDResetPasswordResult
  for &mut RESETPASSWORDRESULT
{
}

impl<REVENUEWITHDRAWALSTATE: TDRevenueWithdrawalState> TDRevenueWithdrawalState
  for &REVENUEWITHDRAWALSTATE
{
}
impl<REVENUEWITHDRAWALSTATE: TDRevenueWithdrawalState> TDRevenueWithdrawalState
  for &mut REVENUEWITHDRAWALSTATE
{
}

impl<RICHMESSAGESOURCE: TDRichMessageSource> TDRichMessageSource for &RICHMESSAGESOURCE {}
impl<RICHMESSAGESOURCE: TDRichMessageSource> TDRichMessageSource for &mut RICHMESSAGESOURCE {}

impl<RICHTEXT: TDRichText> TDRichText for &RICHTEXT {}
impl<RICHTEXT: TDRichText> TDRichText for &mut RICHTEXT {}

impl<SAVEDMESSAGESTOPICTYPE: TDSavedMessagesTopicType> TDSavedMessagesTopicType
  for &SAVEDMESSAGESTOPICTYPE
{
}
impl<SAVEDMESSAGESTOPICTYPE: TDSavedMessagesTopicType> TDSavedMessagesTopicType
  for &mut SAVEDMESSAGESTOPICTYPE
{
}

impl<SEARCHCHATTYPEFILTER: TDSearchChatTypeFilter> TDSearchChatTypeFilter
  for &SEARCHCHATTYPEFILTER
{
}
impl<SEARCHCHATTYPEFILTER: TDSearchChatTypeFilter> TDSearchChatTypeFilter
  for &mut SEARCHCHATTYPEFILTER
{
}

impl<SEARCHMESSAGESCHATTYPEFILTER: TDSearchMessagesChatTypeFilter> TDSearchMessagesChatTypeFilter
  for &SEARCHMESSAGESCHATTYPEFILTER
{
}
impl<SEARCHMESSAGESCHATTYPEFILTER: TDSearchMessagesChatTypeFilter> TDSearchMessagesChatTypeFilter
  for &mut SEARCHMESSAGESCHATTYPEFILTER
{
}

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

impl<SENTGIFT: TDSentGift> TDSentGift for &SENTGIFT {}
impl<SENTGIFT: TDSentGift> TDSentGift for &mut SENTGIFT {}

impl<SESSIONDEVICETYPE: TDSessionDeviceType> TDSessionDeviceType for &SESSIONDEVICETYPE {}
impl<SESSIONDEVICETYPE: TDSessionDeviceType> TDSessionDeviceType for &mut SESSIONDEVICETYPE {}

impl<SESSIONTYPE: TDSessionType> TDSessionType for &SESSIONTYPE {}
impl<SESSIONTYPE: TDSessionType> TDSessionType for &mut SESSIONTYPE {}

impl<SETTINGSSECTION: TDSettingsSection> TDSettingsSection for &SETTINGSSECTION {}
impl<SETTINGSSECTION: TDSettingsSection> TDSettingsSection for &mut SETTINGSSECTION {}

impl<SPEECHRECOGNITIONRESULT: TDSpeechRecognitionResult> TDSpeechRecognitionResult
  for &SPEECHRECOGNITIONRESULT
{
}
impl<SPEECHRECOGNITIONRESULT: TDSpeechRecognitionResult> TDSpeechRecognitionResult
  for &mut SPEECHRECOGNITIONRESULT
{
}

impl<STARSUBSCRIPTIONTYPE: TDStarSubscriptionType> TDStarSubscriptionType
  for &STARSUBSCRIPTIONTYPE
{
}
impl<STARSUBSCRIPTIONTYPE: TDStarSubscriptionType> TDStarSubscriptionType
  for &mut STARSUBSCRIPTIONTYPE
{
}

impl<STARTRANSACTIONTYPE: TDStarTransactionType> TDStarTransactionType for &STARTRANSACTIONTYPE {}
impl<STARTRANSACTIONTYPE: TDStarTransactionType> TDStarTransactionType
  for &mut STARTRANSACTIONTYPE
{
}

impl<STARTLIVESTORYRESULT: TDStartLiveStoryResult> TDStartLiveStoryResult
  for &STARTLIVESTORYRESULT
{
}
impl<STARTLIVESTORYRESULT: TDStartLiveStoryResult> TDStartLiveStoryResult
  for &mut STARTLIVESTORYRESULT
{
}

impl<STATISTICALGRAPH: TDStatisticalGraph> TDStatisticalGraph for &STATISTICALGRAPH {}
impl<STATISTICALGRAPH: TDStatisticalGraph> TDStatisticalGraph for &mut STATISTICALGRAPH {}

impl<STICKERFORMAT: TDStickerFormat> TDStickerFormat for &STICKERFORMAT {}
impl<STICKERFORMAT: TDStickerFormat> TDStickerFormat for &mut STICKERFORMAT {}

impl<STICKERFULLTYPE: TDStickerFullType> TDStickerFullType for &STICKERFULLTYPE {}
impl<STICKERFULLTYPE: TDStickerFullType> TDStickerFullType for &mut STICKERFULLTYPE {}

impl<STICKERTYPE: TDStickerType> TDStickerType for &STICKERTYPE {}
impl<STICKERTYPE: TDStickerType> TDStickerType for &mut STICKERTYPE {}

impl<STOREPAYMENTPURPOSE: TDStorePaymentPurpose> TDStorePaymentPurpose for &STOREPAYMENTPURPOSE {}
impl<STOREPAYMENTPURPOSE: TDStorePaymentPurpose> TDStorePaymentPurpose
  for &mut STOREPAYMENTPURPOSE
{
}

impl<STORETRANSACTION: TDStoreTransaction> TDStoreTransaction for &STORETRANSACTION {}
impl<STORETRANSACTION: TDStoreTransaction> TDStoreTransaction for &mut STORETRANSACTION {}

impl<STORYAREATYPE: TDStoryAreaType> TDStoryAreaType for &STORYAREATYPE {}
impl<STORYAREATYPE: TDStoryAreaType> TDStoryAreaType for &mut STORYAREATYPE {}

impl<STORYCONTENT: TDStoryContent> TDStoryContent for &STORYCONTENT {}
impl<STORYCONTENT: TDStoryContent> TDStoryContent for &mut STORYCONTENT {}

impl<STORYCONTENTTYPE: TDStoryContentType> TDStoryContentType for &STORYCONTENTTYPE {}
impl<STORYCONTENTTYPE: TDStoryContentType> TDStoryContentType for &mut STORYCONTENTTYPE {}

impl<STORYINTERACTIONTYPE: TDStoryInteractionType> TDStoryInteractionType
  for &STORYINTERACTIONTYPE
{
}
impl<STORYINTERACTIONTYPE: TDStoryInteractionType> TDStoryInteractionType
  for &mut STORYINTERACTIONTYPE
{
}

impl<STORYLIST: TDStoryList> TDStoryList for &STORYLIST {}
impl<STORYLIST: TDStoryList> TDStoryList for &mut STORYLIST {}

impl<STORYORIGIN: TDStoryOrigin> TDStoryOrigin for &STORYORIGIN {}
impl<STORYORIGIN: TDStoryOrigin> TDStoryOrigin for &mut STORYORIGIN {}

impl<STORYPRIVACYSETTINGS: TDStoryPrivacySettings> TDStoryPrivacySettings
  for &STORYPRIVACYSETTINGS
{
}
impl<STORYPRIVACYSETTINGS: TDStoryPrivacySettings> TDStoryPrivacySettings
  for &mut STORYPRIVACYSETTINGS
{
}

impl<SUGGESTEDACTION: TDSuggestedAction> TDSuggestedAction for &SUGGESTEDACTION {}
impl<SUGGESTEDACTION: TDSuggestedAction> TDSuggestedAction for &mut SUGGESTEDACTION {}

impl<SUGGESTEDPOSTPRICE: TDSuggestedPostPrice> TDSuggestedPostPrice for &SUGGESTEDPOSTPRICE {}
impl<SUGGESTEDPOSTPRICE: TDSuggestedPostPrice> TDSuggestedPostPrice for &mut SUGGESTEDPOSTPRICE {}

impl<SUGGESTEDPOSTREFUNDREASON: TDSuggestedPostRefundReason> TDSuggestedPostRefundReason
  for &SUGGESTEDPOSTREFUNDREASON
{
}
impl<SUGGESTEDPOSTREFUNDREASON: TDSuggestedPostRefundReason> TDSuggestedPostRefundReason
  for &mut SUGGESTEDPOSTREFUNDREASON
{
}

impl<SUGGESTEDPOSTSTATE: TDSuggestedPostState> TDSuggestedPostState for &SUGGESTEDPOSTSTATE {}
impl<SUGGESTEDPOSTSTATE: TDSuggestedPostState> TDSuggestedPostState for &mut SUGGESTEDPOSTSTATE {}

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

impl<TARGETCHAT: TDTargetChat> TDTargetChat for &TARGETCHAT {}
impl<TARGETCHAT: TDTargetChat> TDTargetChat for &mut TARGETCHAT {}

impl<TELEGRAMPAYMENTPURPOSE: TDTelegramPaymentPurpose> TDTelegramPaymentPurpose
  for &TELEGRAMPAYMENTPURPOSE
{
}
impl<TELEGRAMPAYMENTPURPOSE: TDTelegramPaymentPurpose> TDTelegramPaymentPurpose
  for &mut TELEGRAMPAYMENTPURPOSE
{
}

impl<TEXTENTITYTYPE: TDTextEntityType> TDTextEntityType for &TEXTENTITYTYPE {}
impl<TEXTENTITYTYPE: TDTextEntityType> TDTextEntityType for &mut TEXTENTITYTYPE {}

impl<TEXTPARSEMODE: TDTextParseMode> TDTextParseMode for &TEXTPARSEMODE {}
impl<TEXTPARSEMODE: TDTextParseMode> TDTextParseMode for &mut TEXTPARSEMODE {}

impl<THUMBNAILFORMAT: TDThumbnailFormat> TDThumbnailFormat for &THUMBNAILFORMAT {}
impl<THUMBNAILFORMAT: TDThumbnailFormat> TDThumbnailFormat for &mut THUMBNAILFORMAT {}

impl<TONCENTERAPIREQUESTTYPE: TDTonCenterApiRequestType> TDTonCenterApiRequestType
  for &TONCENTERAPIREQUESTTYPE
{
}
impl<TONCENTERAPIREQUESTTYPE: TDTonCenterApiRequestType> TDTonCenterApiRequestType
  for &mut TONCENTERAPIREQUESTTYPE
{
}

impl<TONCONNECTCONNECTITEM: TDTonConnectConnectItem> TDTonConnectConnectItem
  for &TONCONNECTCONNECTITEM
{
}
impl<TONCONNECTCONNECTITEM: TDTonConnectConnectItem> TDTonConnectConnectItem
  for &mut TONCONNECTCONNECTITEM
{
}

impl<TONCONNECTMANIFEST: TDTonConnectManifest> TDTonConnectManifest for &TONCONNECTMANIFEST {}
impl<TONCONNECTMANIFEST: TDTonConnectManifest> TDTonConnectManifest for &mut TONCONNECTMANIFEST {}

impl<TONCONNECTREQUESTSTATE: TDTonConnectRequestState> TDTonConnectRequestState
  for &TONCONNECTREQUESTSTATE
{
}
impl<TONCONNECTREQUESTSTATE: TDTonConnectRequestState> TDTonConnectRequestState
  for &mut TONCONNECTREQUESTSTATE
{
}

impl<TONCONNECTSESSIONSTATE: TDTonConnectSessionState> TDTonConnectSessionState
  for &TONCONNECTSESSIONSTATE
{
}
impl<TONCONNECTSESSIONSTATE: TDTonConnectSessionState> TDTonConnectSessionState
  for &mut TONCONNECTSESSIONSTATE
{
}

impl<TONTRANSACTIONTYPE: TDTonTransactionType> TDTonTransactionType for &TONTRANSACTIONTYPE {}
impl<TONTRANSACTIONTYPE: TDTonTransactionType> TDTonTransactionType for &mut TONTRANSACTIONTYPE {}

impl<TONWALLETTRANSACTIONSTATE: TDTonWalletTransactionState> TDTonWalletTransactionState
  for &TONWALLETTRANSACTIONSTATE
{
}
impl<TONWALLETTRANSACTIONSTATE: TDTonWalletTransactionState> TDTonWalletTransactionState
  for &mut TONWALLETTRANSACTIONSTATE
{
}

impl<TONWALLETTRANSACTIONTYPE: TDTonWalletTransactionType> TDTonWalletTransactionType
  for &TONWALLETTRANSACTIONTYPE
{
}
impl<TONWALLETTRANSACTIONTYPE: TDTonWalletTransactionType> TDTonWalletTransactionType
  for &mut TONWALLETTRANSACTIONTYPE
{
}

impl<TONWALLETTRANSFERRECEIVER: TDTonWalletTransferReceiver> TDTonWalletTransferReceiver
  for &TONWALLETTRANSFERRECEIVER
{
}
impl<TONWALLETTRANSFERRECEIVER: TDTonWalletTransferReceiver> TDTonWalletTransferReceiver
  for &mut TONWALLETTRANSFERRECEIVER
{
}

impl<TOPCHATCATEGORY: TDTopChatCategory> TDTopChatCategory for &TOPCHATCATEGORY {}
impl<TOPCHATCATEGORY: TDTopChatCategory> TDTopChatCategory for &mut TOPCHATCATEGORY {}

impl<TRANSACTIONDIRECTION: TDTransactionDirection> TDTransactionDirection
  for &TRANSACTIONDIRECTION
{
}
impl<TRANSACTIONDIRECTION: TDTransactionDirection> TDTransactionDirection
  for &mut TRANSACTIONDIRECTION
{
}

impl<UPDATE: TDUpdate> TDUpdate for &UPDATE {}
impl<UPDATE: TDUpdate> TDUpdate for &mut UPDATE {}

impl<UPGRADEDGIFTATTRIBUTEID: TDUpgradedGiftAttributeId> TDUpgradedGiftAttributeId
  for &UPGRADEDGIFTATTRIBUTEID
{
}
impl<UPGRADEDGIFTATTRIBUTEID: TDUpgradedGiftAttributeId> TDUpgradedGiftAttributeId
  for &mut UPGRADEDGIFTATTRIBUTEID
{
}

impl<UPGRADEDGIFTATTRIBUTERARITY: TDUpgradedGiftAttributeRarity> TDUpgradedGiftAttributeRarity
  for &UPGRADEDGIFTATTRIBUTERARITY
{
}
impl<UPGRADEDGIFTATTRIBUTERARITY: TDUpgradedGiftAttributeRarity> TDUpgradedGiftAttributeRarity
  for &mut UPGRADEDGIFTATTRIBUTERARITY
{
}

impl<UPGRADEDGIFTORIGIN: TDUpgradedGiftOrigin> TDUpgradedGiftOrigin for &UPGRADEDGIFTORIGIN {}
impl<UPGRADEDGIFTORIGIN: TDUpgradedGiftOrigin> TDUpgradedGiftOrigin for &mut UPGRADEDGIFTORIGIN {}

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

impl<WEBAPPOPENMODE: TDWebAppOpenMode> TDWebAppOpenMode for &WEBAPPOPENMODE {}
impl<WEBAPPOPENMODE: TDWebAppOpenMode> TDWebAppOpenMode for &mut WEBAPPOPENMODE {}

impl<WEBBROWSERTYPE: TDWebBrowserType> TDWebBrowserType for &WEBBROWSERTYPE {}
impl<WEBBROWSERTYPE: TDWebBrowserType> TDWebBrowserType for &mut WEBBROWSERTYPE {}

impl<WEBPROXYCARRIERERROR: TDWebProxyCarrierError> TDWebProxyCarrierError
  for &WEBPROXYCARRIERERROR
{
}
impl<WEBPROXYCARRIERERROR: TDWebProxyCarrierError> TDWebProxyCarrierError
  for &mut WEBPROXYCARRIERERROR
{
}

impl<WEBPROXYCARRIERMESSAGE: TDWebProxyCarrierMessage> TDWebProxyCarrierMessage
  for &WEBPROXYCARRIERMESSAGE
{
}
impl<WEBPROXYCARRIERMESSAGE: TDWebProxyCarrierMessage> TDWebProxyCarrierMessage
  for &mut WEBPROXYCARRIERMESSAGE
{
}

#[derive(Debug, Clone)]
pub enum TdType {
  TestUseUpdate(TestUseUpdate),
  UpdateAccentColors(UpdateAccentColors),
  UpdateActiveEmojiReactions(UpdateActiveEmojiReactions),
  UpdateActiveGiftAuctions(UpdateActiveGiftAuctions),
  UpdateActiveLiveLocationMessages(UpdateActiveLiveLocationMessages),
  UpdateActiveNotifications(UpdateActiveNotifications),
  UpdateAgeVerificationParameters(UpdateAgeVerificationParameters),
  UpdateAllowedBotUsernamePremiumSuffixes(UpdateAllowedBotUsernamePremiumSuffixes),
  UpdateAnimatedEmojiMessageClicked(UpdateAnimatedEmojiMessageClicked),
  UpdateAnimationSearchParameters(UpdateAnimationSearchParameters),
  UpdateApplicationRecaptchaVerificationRequired(UpdateApplicationRecaptchaVerificationRequired),
  UpdateApplicationVerificationRequired(UpdateApplicationVerificationRequired),
  UpdateAttachmentMenuBots(UpdateAttachmentMenuBots),
  UpdateAuthorizationState(UpdateAuthorizationState),
  UpdateAutosaveSettings(UpdateAutosaveSettings),
  UpdateAvailableMessageEffects(UpdateAvailableMessageEffects),
  UpdateBasicGroup(UpdateBasicGroup),
  UpdateBasicGroupFullInfo(UpdateBasicGroupFullInfo),
  UpdateBusinessConnection(UpdateBusinessConnection),
  UpdateBusinessMessageEdited(UpdateBusinessMessageEdited),
  UpdateBusinessMessagesDeleted(UpdateBusinessMessagesDeleted),
  UpdateCall(UpdateCall),
  UpdateChatAccentColors(UpdateChatAccentColors),
  UpdateChatAction(UpdateChatAction),
  UpdateChatActionBar(UpdateChatActionBar),
  UpdateChatActiveStories(UpdateChatActiveStories),
  UpdateChatAddedToList(UpdateChatAddedToList),
  UpdateChatAvailableReactions(UpdateChatAvailableReactions),
  UpdateChatBackground(UpdateChatBackground),
  UpdateChatBlockList(UpdateChatBlockList),
  UpdateChatBoost(UpdateChatBoost),
  UpdateChatBusinessBotManageBar(UpdateChatBusinessBotManageBar),
  UpdateChatDefaultDisableNotification(UpdateChatDefaultDisableNotification),
  UpdateChatDraftMessage(UpdateChatDraftMessage),
  UpdateChatEmojiStatus(UpdateChatEmojiStatus),
  UpdateChatFolders(UpdateChatFolders),
  UpdateChatHasProtectedContent(UpdateChatHasProtectedContent),
  UpdateChatHasScheduledMessages(UpdateChatHasScheduledMessages),
  UpdateChatHasWelcomeMessages(UpdateChatHasWelcomeMessages),
  UpdateChatIsMarkedAsUnread(UpdateChatIsMarkedAsUnread),
  UpdateChatIsTranslatable(UpdateChatIsTranslatable),
  UpdateChatJoinResult(UpdateChatJoinResult),
  UpdateChatLastMessage(UpdateChatLastMessage),
  UpdateChatMember(UpdateChatMember),
  UpdateChatMessageAutoDeleteTime(UpdateChatMessageAutoDeleteTime),
  UpdateChatMessageSender(UpdateChatMessageSender),
  UpdateChatNotificationSettings(UpdateChatNotificationSettings),
  UpdateChatOnlineMemberCount(UpdateChatOnlineMemberCount),
  UpdateChatPendingJoinRequests(UpdateChatPendingJoinRequests),
  UpdateChatPermissions(UpdateChatPermissions),
  UpdateChatPhoto(UpdateChatPhoto),
  UpdateChatPosition(UpdateChatPosition),
  UpdateChatReadInbox(UpdateChatReadInbox),
  UpdateChatReadOutbox(UpdateChatReadOutbox),
  UpdateChatRemovedFromList(UpdateChatRemovedFromList),
  UpdateChatReplyMarkup(UpdateChatReplyMarkup),
  UpdateChatRevenueAmount(UpdateChatRevenueAmount),
  UpdateChatTheme(UpdateChatTheme),
  UpdateChatTitle(UpdateChatTitle),
  UpdateChatUnreadMentionCount(UpdateChatUnreadMentionCount),
  UpdateChatUnreadPollVoteCount(UpdateChatUnreadPollVoteCount),
  UpdateChatUnreadReactionCount(UpdateChatUnreadReactionCount),
  UpdateChatVideoChat(UpdateChatVideoChat),
  UpdateChatViewAsTopics(UpdateChatViewAsTopics),
  UpdateChatWelcomeMessages(UpdateChatWelcomeMessages),
  UpdateCommunity(UpdateCommunity),
  UpdateCommunityFullInfo(UpdateCommunityFullInfo),
  UpdateConnectionState(UpdateConnectionState),
  UpdateContactCloseBirthdays(UpdateContactCloseBirthdays),
  UpdateDefaultBackground(UpdateDefaultBackground),
  UpdateDefaultPaidReactionType(UpdateDefaultPaidReactionType),
  UpdateDefaultReactionType(UpdateDefaultReactionType),
  UpdateDeleteMessages(UpdateDeleteMessages),
  UpdateDiceEmojis(UpdateDiceEmojis),
  UpdateDirectMessagesChatTopic(UpdateDirectMessagesChatTopic),
  UpdateEmojiChatThemes(UpdateEmojiChatThemes),
  UpdateFavoriteStickers(UpdateFavoriteStickers),
  UpdateFile(UpdateFile),
  UpdateFileAddedToDownloads(UpdateFileAddedToDownloads),
  UpdateFileDownload(UpdateFileDownload),
  UpdateFileDownloads(UpdateFileDownloads),
  UpdateFileGenerationStart(UpdateFileGenerationStart),
  UpdateFileGenerationStop(UpdateFileGenerationStop),
  UpdateFileRemovedFromDownloads(UpdateFileRemovedFromDownloads),
  UpdateForumTopic(UpdateForumTopic),
  UpdateForumTopicInfo(UpdateForumTopicInfo),
  UpdateFreezeState(UpdateFreezeState),
  UpdateGiftAuctionState(UpdateGiftAuctionState),
  UpdateGramRevenueStatus(UpdateGramRevenueStatus),
  UpdateGroupCall(UpdateGroupCall),
  UpdateGroupCallMessageLevels(UpdateGroupCallMessageLevels),
  UpdateGroupCallMessageSendFailed(UpdateGroupCallMessageSendFailed),
  UpdateGroupCallMessagesDeleted(UpdateGroupCallMessagesDeleted),
  UpdateGroupCallParticipant(UpdateGroupCallParticipant),
  UpdateGroupCallParticipants(UpdateGroupCallParticipants),
  UpdateGroupCallVerificationState(UpdateGroupCallVerificationState),
  UpdateHavePendingNotifications(UpdateHavePendingNotifications),
  UpdateInstalledStickerSets(UpdateInstalledStickerSets),
  UpdateLanguagePackStrings(UpdateLanguagePackStrings),
  UpdateLiveStoryTopDonors(UpdateLiveStoryTopDonors),
  UpdateManagedBot(UpdateManagedBot),
  UpdateMessageContainsUnreadPollVotes(UpdateMessageContainsUnreadPollVotes),
  UpdateMessageContent(UpdateMessageContent),
  UpdateMessageContentOpened(UpdateMessageContentOpened),
  UpdateMessageEdited(UpdateMessageEdited),
  UpdateMessageEphemeralContent(UpdateMessageEphemeralContent),
  UpdateMessageFactCheck(UpdateMessageFactCheck),
  UpdateMessageInteractionInfo(UpdateMessageInteractionInfo),
  UpdateMessageIsPinned(UpdateMessageIsPinned),
  UpdateMessageLiveLocationViewed(UpdateMessageLiveLocationViewed),
  UpdateMessageMentionRead(UpdateMessageMentionRead),
  UpdateMessageReaction(UpdateMessageReaction),
  UpdateMessageReactions(UpdateMessageReactions),
  UpdateMessageSendAcknowledged(UpdateMessageSendAcknowledged),
  UpdateMessageSendFailed(UpdateMessageSendFailed),
  UpdateMessageSendSucceeded(UpdateMessageSendSucceeded),
  UpdateMessageSuggestedPostInfo(UpdateMessageSuggestedPostInfo),
  UpdateMessageUnreadReactions(UpdateMessageUnreadReactions),
  UpdateNewBusinessCallbackQuery(UpdateNewBusinessCallbackQuery),
  UpdateNewBusinessMessage(UpdateNewBusinessMessage),
  UpdateNewCallSignalingData(UpdateNewCallSignalingData),
  UpdateNewCallbackQuery(UpdateNewCallbackQuery),
  UpdateNewChat(UpdateNewChat),
  UpdateNewChatJoinRequest(UpdateNewChatJoinRequest),
  UpdateNewChosenInlineResult(UpdateNewChosenInlineResult),
  UpdateNewCustomEvent(UpdateNewCustomEvent),
  UpdateNewCustomQuery(UpdateNewCustomQuery),
  UpdateNewGroupCallMessage(UpdateNewGroupCallMessage),
  UpdateNewGroupCallPaidReaction(UpdateNewGroupCallPaidReaction),
  UpdateNewGuestQuery(UpdateNewGuestQuery),
  UpdateNewInlineCallbackQuery(UpdateNewInlineCallbackQuery),
  UpdateNewInlineQuery(UpdateNewInlineQuery),
  UpdateNewMessage(UpdateNewMessage),
  UpdateNewOauthRequest(UpdateNewOauthRequest),
  UpdateNewPreCheckoutQuery(UpdateNewPreCheckoutQuery),
  UpdateNewShippingQuery(UpdateNewShippingQuery),
  UpdateNotification(UpdateNotification),
  UpdateNotificationGroup(UpdateNotificationGroup),
  UpdateOption(UpdateOption),
  UpdateOwnedGramCount(UpdateOwnedGramCount),
  UpdateOwnedStarCount(UpdateOwnedStarCount),
  UpdatePaidMediaPurchased(UpdatePaidMediaPurchased),
  UpdatePendingMessage(UpdatePendingMessage),
  UpdatePoll(UpdatePoll),
  UpdatePollAnswer(UpdatePollAnswer),
  UpdateProfileAccentColors(UpdateProfileAccentColors),
  UpdateQuickReplyShortcut(UpdateQuickReplyShortcut),
  UpdateQuickReplyShortcutDeleted(UpdateQuickReplyShortcutDeleted),
  UpdateQuickReplyShortcutMessages(UpdateQuickReplyShortcutMessages),
  UpdateQuickReplyShortcuts(UpdateQuickReplyShortcuts),
  UpdateReactionNotificationSettings(UpdateReactionNotificationSettings),
  UpdateRecentStickers(UpdateRecentStickers),
  UpdateSavedAnimations(UpdateSavedAnimations),
  UpdateSavedMessagesTags(UpdateSavedMessagesTags),
  UpdateSavedMessagesTopic(UpdateSavedMessagesTopic),
  UpdateSavedMessagesTopicCount(UpdateSavedMessagesTopicCount),
  UpdateSavedNotificationSounds(UpdateSavedNotificationSounds),
  UpdateScopeNotificationSettings(UpdateScopeNotificationSettings),
  UpdateSecretChat(UpdateSecretChat),
  UpdateServiceNotification(UpdateServiceNotification),
  UpdateSpeechRecognitionTrial(UpdateSpeechRecognitionTrial),
  UpdateSpeedLimitNotification(UpdateSpeedLimitNotification),
  UpdateStakeDiceState(UpdateStakeDiceState),
  UpdateStarRevenueStatus(UpdateStarRevenueStatus),
  UpdateStickerSet(UpdateStickerSet),
  UpdateStopMessageDraft(UpdateStopMessageDraft),
  UpdateStory(UpdateStory),
  UpdateStoryDeleted(UpdateStoryDeleted),
  UpdateStoryListChatCount(UpdateStoryListChatCount),
  UpdateStoryPostFailed(UpdateStoryPostFailed),
  UpdateStoryPostSucceeded(UpdateStoryPostSucceeded),
  UpdateStoryStealthMode(UpdateStoryStealthMode),
  UpdateSuggestedActions(UpdateSuggestedActions),
  UpdateSupergroup(UpdateSupergroup),
  UpdateSupergroupFullInfo(UpdateSupergroupFullInfo),
  UpdateTermsOfService(UpdateTermsOfService),
  UpdateTextCompositionStyles(UpdateTextCompositionStyles),
  UpdateTonWalletGaslessTransfersInfo(UpdateTonWalletGaslessTransfersInfo),
  UpdateTonWalletState(UpdateTonWalletState),
  UpdateTonWalletTonConnectSession(UpdateTonWalletTonConnectSession),
  UpdateTonWalletTonConnectSessionDisconnectRequired(
    UpdateTonWalletTonConnectSessionDisconnectRequired,
  ),
  UpdateTopicMessageCount(UpdateTopicMessageCount),
  UpdateTrendingStickerSets(UpdateTrendingStickerSets),
  UpdateTrustedMiniAppBots(UpdateTrustedMiniAppBots),
  UpdateUnconfirmedSession(UpdateUnconfirmedSession),
  UpdateUnreadChatCount(UpdateUnreadChatCount),
  UpdateUnreadMessageCount(UpdateUnreadMessageCount),
  UpdateUser(UpdateUser),
  UpdateUserFullInfo(UpdateUserFullInfo),
  UpdateUserPrivacySettingRules(UpdateUserPrivacySettingRules),
  UpdateUserStatus(UpdateUserStatus),
  UpdateUserSubscription(UpdateUserSubscription),
  UpdateVideoPublished(UpdateVideoPublished),
  UpdateWebAppMessageSent(UpdateWebAppMessageSent),
  UpdateWebBrowserSettings(UpdateWebBrowserSettings),
  UpdateWebProxyCarrierClosed(UpdateWebProxyCarrierClosed),
  UpdateWebProxyCarrierData(UpdateWebProxyCarrierData),
  UpdateWebProxyCarrierOpened(UpdateWebProxyCarrierOpened),

  AuthorizationState(AuthorizationState),
  CanPostStoryResult(CanPostStoryResult),
  CanSendGiftResult(CanSendGiftResult),
  CanSendMessageToUserResult(CanSendMessageToUserResult),
  CanTransferOwnershipResult(CanTransferOwnershipResult),
  ChatJoinResult(ChatJoinResult),
  ChatStatistics(ChatStatistics),
  CheckChatUsernameResult(CheckChatUsernameResult),
  CheckStickerSetNameResult(CheckStickerSetNameResult),
  CraftGiftResult(CraftGiftResult),
  GiftResaleResult(GiftResaleResult),
  GiveawayInfo(GiveawayInfo),
  InternalLinkType(InternalLinkType),
  InviteGroupCallParticipantResult(InviteGroupCallParticipantResult),
  JsonValue(JsonValue),
  LanguagePackStringValue(LanguagePackStringValue),
  LogStream(LogStream),
  LoginUrlInfo(LoginUrlInfo),
  MessageFileType(MessageFileType),
  MessageReadDate(MessageReadDate),
  MessageSender(MessageSender),
  OptionValue(OptionValue),
  PassportElement(PassportElement),
  ReportChatResult(ReportChatResult),
  ReportSponsoredResult(ReportSponsoredResult),
  ReportStoryResult(ReportStoryResult),
  ResetPasswordResult(ResetPasswordResult),
  StartLiveStoryResult(StartLiveStoryResult),
  StatisticalGraph(StatisticalGraph),
  Update(Update),
  WebBrowserType(WebBrowserType),
  AccountTtl(AccountTtl),
  AddedProxies(AddedProxies),
  AddedProxy(AddedProxy),
  AddedReactions(AddedReactions),
  AnimatedEmoji(AnimatedEmoji),
  Animation(Animation),
  Animations(Animations),
  ArchiveChatListSettings(ArchiveChatListSettings),
  AttachmentMenuBot(AttachmentMenuBot),
  Audios(Audios),
  AuthenticationCodeInfo(AuthenticationCodeInfo),
  AutoDownloadSettingsPresets(AutoDownloadSettingsPresets),
  AutosaveSettings(AutosaveSettings),
  AvailableGifts(AvailableGifts),
  AvailableReactions(AvailableReactions),
  Background(Background),
  Backgrounds(Backgrounds),
  BankCardInfo(BankCardInfo),
  BasicGroup(BasicGroup),
  BasicGroupFullInfo(BasicGroupFullInfo),
  BotAccessSettings(BotAccessSettings),
  BotCommands(BotCommands),
  BotMediaPreview(BotMediaPreview),
  BotMediaPreviewInfo(BotMediaPreviewInfo),
  BotMediaPreviews(BotMediaPreviews),
  BotMenuButton(BotMenuButton),
  BusinessChatLink(BusinessChatLink),
  BusinessChatLinkInfo(BusinessChatLinkInfo),
  BusinessChatLinks(BusinessChatLinks),
  BusinessConnectedBotInfo(BusinessConnectedBotInfo),
  BusinessConnection(BusinessConnection),
  BusinessFeatures(BusinessFeatures),
  BusinessMessage(BusinessMessage),
  BusinessMessages(BusinessMessages),
  CallId(CallId),
  CallbackQueryAnswer(CallbackQueryAnswer),
  Chat(Chat),
  ChatActiveStories(ChatActiveStories),
  ChatAdministrators(ChatAdministrators),
  ChatBoostFeatures(ChatBoostFeatures),
  ChatBoostLevelFeatures(ChatBoostLevelFeatures),
  ChatBoostLink(ChatBoostLink),
  ChatBoostLinkInfo(ChatBoostLinkInfo),
  ChatBoostSlots(ChatBoostSlots),
  ChatBoostStatus(ChatBoostStatus),
  ChatEvents(ChatEvents),
  ChatFolder(ChatFolder),
  ChatFolderIcon(ChatFolderIcon),
  ChatFolderInfo(ChatFolderInfo),
  ChatFolderInviteLink(ChatFolderInviteLink),
  ChatFolderInviteLinkInfo(ChatFolderInviteLinkInfo),
  ChatFolderInviteLinks(ChatFolderInviteLinks),
  ChatInviteLink(ChatInviteLink),
  ChatInviteLinkCounts(ChatInviteLinkCounts),
  ChatInviteLinkInfo(ChatInviteLinkInfo),
  ChatInviteLinkMembers(ChatInviteLinkMembers),
  ChatInviteLinks(ChatInviteLinks),
  ChatJoinRequests(ChatJoinRequests),
  ChatLists(ChatLists),
  ChatMember(ChatMember),
  ChatMembers(ChatMembers),
  ChatMessageSenders(ChatMessageSenders),
  ChatPhotos(ChatPhotos),
  ChatRevenueStatistics(ChatRevenueStatistics),
  ChatRevenueTransactions(ChatRevenueTransactions),
  Chats(Chats),
  CollectibleItemInfo(CollectibleItemInfo),
  CommunityFullInfo(CommunityFullInfo),
  CommunityId(CommunityId),
  ConnectedAffiliateProgram(ConnectedAffiliateProgram),
  ConnectedAffiliatePrograms(ConnectedAffiliatePrograms),
  ConnectedWebsites(ConnectedWebsites),
  Count(Count),
  Countries(Countries),
  CountryInfo(CountryInfo),
  CreatedBasicGroupChat(CreatedBasicGroupChat),
  Currencies(Currencies),
  CurrencyExchangeRates(CurrencyExchangeRates),
  CurrentWeather(CurrentWeather),
  CustomRequestResult(CustomRequestResult),
  Data(Data),
  DatabaseStatistics(DatabaseStatistics),
  DeepLinkInfo(DeepLinkInfo),
  DirectMessagesChatTopic(DirectMessagesChatTopic),
  EmailAddressAuthenticationCodeInfo(EmailAddressAuthenticationCodeInfo),
  EmojiCategories(EmojiCategories),
  EmojiKeywords(EmojiKeywords),
  EmojiReaction(EmojiReaction),
  EmojiStatusCustomEmojis(EmojiStatusCustomEmojis),
  EmojiStatuses(EmojiStatuses),
  Emojis(Emojis),
  Error(Error),
  FailedToAddMembers(FailedToAddMembers),
  File(File),
  FileDownloadedPrefixSize(FileDownloadedPrefixSize),
  FixedText(FixedText),
  FormattedText(FormattedText),
  ForumTopic(ForumTopic),
  ForumTopicInfo(ForumTopicInfo),
  ForumTopics(ForumTopics),
  FoundAffiliatePrograms(FoundAffiliatePrograms),
  FoundChatBoosts(FoundChatBoosts),
  FoundChatMessages(FoundChatMessages),
  FoundFileDownloads(FoundFileDownloads),
  FoundMessages(FoundMessages),
  FoundPosition(FoundPosition),
  FoundPositions(FoundPositions),
  FoundPublicPosts(FoundPublicPosts),
  FoundStories(FoundStories),
  FoundUsers(FoundUsers),
  FoundWebApp(FoundWebApp),
  GameHighScores(GameHighScores),
  GiftAuctionAcquiredGifts(GiftAuctionAcquiredGifts),
  GiftAuctionState(GiftAuctionState),
  GiftChatThemes(GiftChatThemes),
  GiftCollection(GiftCollection),
  GiftCollections(GiftCollections),
  GiftUpgradePreview(GiftUpgradePreview),
  GiftUpgradeVariants(GiftUpgradeVariants),
  GiftsForCrafting(GiftsForCrafting),
  GiftsForResale(GiftsForResale),
  GramRevenueStatistics(GramRevenueStatistics),
  GroupCall(GroupCall),
  GroupCallId(GroupCallId),
  GroupCallInfo(GroupCallInfo),
  GroupCallParticipant(GroupCallParticipant),
  GroupCallParticipants(GroupCallParticipants),
  GroupCallStreams(GroupCallStreams),
  Hashtags(Hashtags),
  HttpUrl(HttpUrl),
  ImportedContacts(ImportedContacts),
  InlineMessageId(InlineMessageId),
  InlineQueryResults(InlineQueryResults),
  KeyboardButton(KeyboardButton),
  LanguagePackInfo(LanguagePackInfo),
  LanguagePackStrings(LanguagePackStrings),
  LinkPreview(LinkPreview),
  LiveStoryDonors(LiveStoryDonors),
  LocalizationTargetInfo(LocalizationTargetInfo),
  LogTags(LogTags),
  LogVerbosityLevel(LogVerbosityLevel),
  MainWebApp(MainWebApp),
  Message(Message),
  MessageAutoDeleteTime(MessageAutoDeleteTime),
  MessageCalendar(MessageCalendar),
  MessageEffect(MessageEffect),
  MessageLink(MessageLink),
  MessageLinkInfo(MessageLinkInfo),
  MessagePositions(MessagePositions),
  MessageProperties(MessageProperties),
  MessageSenders(MessageSenders),
  MessageStatistics(MessageStatistics),
  MessageThreadInfo(MessageThreadInfo),
  MessageViewers(MessageViewers),
  Messages(Messages),
  NetworkStatistics(NetworkStatistics),
  NewChatPrivacySettings(NewChatPrivacySettings),
  NotificationSound(NotificationSound),
  NotificationSounds(NotificationSounds),
  OauthLinkInfo(OauthLinkInfo),
  Ok(Ok),
  OnRampPaymentAvailability(OnRampPaymentAvailability),
  OnRampPaymentLimits(OnRampPaymentLimits),
  OnRampPaymentQuote(OnRampPaymentQuote),
  OnRampPaymentSession(OnRampPaymentSession),
  OnRampProviders(OnRampProviders),
  OrderInfo(OrderInfo),
  Outline(Outline),
  Passkey(Passkey),
  Passkeys(Passkeys),
  PassportAuthorizationForm(PassportAuthorizationForm),
  PassportElements(PassportElements),
  PassportElementsWithErrors(PassportElementsWithErrors),
  PasswordState(PasswordState),
  PaymentForm(PaymentForm),
  PaymentReceipt(PaymentReceipt),
  PaymentResult(PaymentResult),
  PhoneNumberInfo(PhoneNumberInfo),
  PollOptionProperties(PollOptionProperties),
  PollVoteStatistics(PollVoteStatistics),
  PollVoters(PollVoters),
  PremiumFeatures(PremiumFeatures),
  PremiumGiftCodeInfo(PremiumGiftCodeInfo),
  PremiumGiftPaymentOptions(PremiumGiftPaymentOptions),
  PremiumGiveawayPaymentOptions(PremiumGiveawayPaymentOptions),
  PremiumLimit(PremiumLimit),
  PremiumState(PremiumState),
  PreparedInlineMessage(PreparedInlineMessage),
  PreparedInlineMessageId(PreparedInlineMessageId),
  ProfileShareThemeBackdrops(ProfileShareThemeBackdrops),
  PublicForwards(PublicForwards),
  PublicPostSearchLimits(PublicPostSearchLimits),
  PushReceiverId(PushReceiverId),
  QuickReplyMessage(QuickReplyMessage),
  QuickReplyMessages(QuickReplyMessages),
  ReadDatePrivacySettings(ReadDatePrivacySettings),
  ReceivedGift(ReceivedGift),
  ReceivedGifts(ReceivedGifts),
  RecommendedChatFolders(RecommendedChatFolders),
  RecoveryEmailAddress(RecoveryEmailAddress),
  RichMessage(RichMessage),
  RtmpUrl(RtmpUrl),
  SavedMessagesTags(SavedMessagesTags),
  ScopeNotificationSettings(ScopeNotificationSettings),
  Seconds(Seconds),
  SecretChat(SecretChat),
  Session(Session),
  Sessions(Sessions),
  SponsoredChats(SponsoredChats),
  SponsoredMessages(SponsoredMessages),
  StakeDiceState(StakeDiceState),
  StarAmount(StarAmount),
  StarCount(StarCount),
  StarGiveawayPaymentOptions(StarGiveawayPaymentOptions),
  StarPaymentOptions(StarPaymentOptions),
  StarRevenueStatistics(StarRevenueStatistics),
  StarSubscriptions(StarSubscriptions),
  StarTransactions(StarTransactions),
  Sticker(Sticker),
  StickerSet(StickerSet),
  StickerSets(StickerSets),
  Stickers(Stickers),
  StorageStatistics(StorageStatistics),
  StorageStatisticsFast(StorageStatisticsFast),
  Stories(Stories),
  Story(Story),
  StoryAlbum(StoryAlbum),
  StoryAlbums(StoryAlbums),
  StoryInteractions(StoryInteractions),
  StoryStatistics(StoryStatistics),
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
  TextCompositionStyle(TextCompositionStyle),
  TextCompositionStyleExample(TextCompositionStyleExample),
  TextEntities(TextEntities),
  TimeZones(TimeZones),
  TonCenterStreamingApiUrl(TonCenterStreamingApiUrl),
  TonConnectChallenge(TonConnectChallenge),
  TonConnectRequests(TonConnectRequests),
  TonConnectSession(TonConnectSession),
  TonConnectSessionEventId(TonConnectSessionEventId),
  TonConnectSessions(TonConnectSessions),
  TonNfts(TonNfts),
  TonTransactions(TonTransactions),
  TonWalletCard(TonWalletCard),
  TonWalletOwnershipProofChallenge(TonWalletOwnershipProofChallenge),
  TonWalletSecretPhrase(TonWalletSecretPhrase),
  TonWalletTransaction(TonWalletTransaction),
  TonWalletTransactions(TonWalletTransactions),
  TonWalletTransferResult(TonWalletTransferResult),
  TrendingStickerSets(TrendingStickerSets),
  Updates(Updates),
  UpgradeGiftResult(UpgradeGiftResult),
  UpgradedGift(UpgradedGift),
  UpgradedGiftCollectionNames(UpgradedGiftCollectionNames),
  UpgradedGiftDepositInfo(UpgradedGiftDepositInfo),
  UpgradedGiftValueInfo(UpgradedGiftValueInfo),
  User(User),
  UserFullInfo(UserFullInfo),
  UserLink(UserLink),
  UserPrivacySettingRules(UserPrivacySettingRules),
  UserSupportInfo(UserSupportInfo),
  UserTonWalletAddress(UserTonWalletAddress),
  UserTonWalletAddresses(UserTonWalletAddresses),
  Users(Users),
  ValidatedOrderInfo(ValidatedOrderInfo),
  VideoMessageAdvertisements(VideoMessageAdvertisements),
  WalletBotBalance(WalletBotBalance),
  WebAppInfo(WebAppInfo),
  WebAppUrl(WebAppUrl),
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
     (updateAccentColors, UpdateAccentColors);
     (updateActiveEmojiReactions, UpdateActiveEmojiReactions);
     (updateActiveGiftAuctions, UpdateActiveGiftAuctions);
     (updateActiveLiveLocationMessages, UpdateActiveLiveLocationMessages);
     (updateActiveNotifications, UpdateActiveNotifications);
     (updateAgeVerificationParameters, UpdateAgeVerificationParameters);
     (updateAllowedBotUsernamePremiumSuffixes, UpdateAllowedBotUsernamePremiumSuffixes);
     (updateAnimatedEmojiMessageClicked, UpdateAnimatedEmojiMessageClicked);
     (updateAnimationSearchParameters, UpdateAnimationSearchParameters);
     (updateApplicationRecaptchaVerificationRequired, UpdateApplicationRecaptchaVerificationRequired);
     (updateApplicationVerificationRequired, UpdateApplicationVerificationRequired);
     (updateAttachmentMenuBots, UpdateAttachmentMenuBots);
     (updateAuthorizationState, UpdateAuthorizationState);
     (updateAutosaveSettings, UpdateAutosaveSettings);
     (updateAvailableMessageEffects, UpdateAvailableMessageEffects);
     (updateBasicGroup, UpdateBasicGroup);
     (updateBasicGroupFullInfo, UpdateBasicGroupFullInfo);
     (updateBusinessConnection, UpdateBusinessConnection);
     (updateBusinessMessageEdited, UpdateBusinessMessageEdited);
     (updateBusinessMessagesDeleted, UpdateBusinessMessagesDeleted);
     (updateCall, UpdateCall);
     (updateChatAccentColors, UpdateChatAccentColors);
     (updateChatAction, UpdateChatAction);
     (updateChatActionBar, UpdateChatActionBar);
     (updateChatActiveStories, UpdateChatActiveStories);
     (updateChatAddedToList, UpdateChatAddedToList);
     (updateChatAvailableReactions, UpdateChatAvailableReactions);
     (updateChatBackground, UpdateChatBackground);
     (updateChatBlockList, UpdateChatBlockList);
     (updateChatBoost, UpdateChatBoost);
     (updateChatBusinessBotManageBar, UpdateChatBusinessBotManageBar);
     (updateChatDefaultDisableNotification, UpdateChatDefaultDisableNotification);
     (updateChatDraftMessage, UpdateChatDraftMessage);
     (updateChatEmojiStatus, UpdateChatEmojiStatus);
     (updateChatFolders, UpdateChatFolders);
     (updateChatHasProtectedContent, UpdateChatHasProtectedContent);
     (updateChatHasScheduledMessages, UpdateChatHasScheduledMessages);
     (updateChatHasWelcomeMessages, UpdateChatHasWelcomeMessages);
     (updateChatIsMarkedAsUnread, UpdateChatIsMarkedAsUnread);
     (updateChatIsTranslatable, UpdateChatIsTranslatable);
     (updateChatJoinResult, UpdateChatJoinResult);
     (updateChatLastMessage, UpdateChatLastMessage);
     (updateChatMember, UpdateChatMember);
     (updateChatMessageAutoDeleteTime, UpdateChatMessageAutoDeleteTime);
     (updateChatMessageSender, UpdateChatMessageSender);
     (updateChatNotificationSettings, UpdateChatNotificationSettings);
     (updateChatOnlineMemberCount, UpdateChatOnlineMemberCount);
     (updateChatPendingJoinRequests, UpdateChatPendingJoinRequests);
     (updateChatPermissions, UpdateChatPermissions);
     (updateChatPhoto, UpdateChatPhoto);
     (updateChatPosition, UpdateChatPosition);
     (updateChatReadInbox, UpdateChatReadInbox);
     (updateChatReadOutbox, UpdateChatReadOutbox);
     (updateChatRemovedFromList, UpdateChatRemovedFromList);
     (updateChatReplyMarkup, UpdateChatReplyMarkup);
     (updateChatRevenueAmount, UpdateChatRevenueAmount);
     (updateChatTheme, UpdateChatTheme);
     (updateChatTitle, UpdateChatTitle);
     (updateChatUnreadMentionCount, UpdateChatUnreadMentionCount);
     (updateChatUnreadPollVoteCount, UpdateChatUnreadPollVoteCount);
     (updateChatUnreadReactionCount, UpdateChatUnreadReactionCount);
     (updateChatVideoChat, UpdateChatVideoChat);
     (updateChatViewAsTopics, UpdateChatViewAsTopics);
     (updateChatWelcomeMessages, UpdateChatWelcomeMessages);
     (updateCommunity, UpdateCommunity);
     (updateCommunityFullInfo, UpdateCommunityFullInfo);
     (updateConnectionState, UpdateConnectionState);
     (updateContactCloseBirthdays, UpdateContactCloseBirthdays);
     (updateDefaultBackground, UpdateDefaultBackground);
     (updateDefaultPaidReactionType, UpdateDefaultPaidReactionType);
     (updateDefaultReactionType, UpdateDefaultReactionType);
     (updateDeleteMessages, UpdateDeleteMessages);
     (updateDiceEmojis, UpdateDiceEmojis);
     (updateDirectMessagesChatTopic, UpdateDirectMessagesChatTopic);
     (updateEmojiChatThemes, UpdateEmojiChatThemes);
     (updateFavoriteStickers, UpdateFavoriteStickers);
     (updateFile, UpdateFile);
     (updateFileAddedToDownloads, UpdateFileAddedToDownloads);
     (updateFileDownload, UpdateFileDownload);
     (updateFileDownloads, UpdateFileDownloads);
     (updateFileGenerationStart, UpdateFileGenerationStart);
     (updateFileGenerationStop, UpdateFileGenerationStop);
     (updateFileRemovedFromDownloads, UpdateFileRemovedFromDownloads);
     (updateForumTopic, UpdateForumTopic);
     (updateForumTopicInfo, UpdateForumTopicInfo);
     (updateFreezeState, UpdateFreezeState);
     (updateGiftAuctionState, UpdateGiftAuctionState);
     (updateGramRevenueStatus, UpdateGramRevenueStatus);
     (updateGroupCall, UpdateGroupCall);
     (updateGroupCallMessageLevels, UpdateGroupCallMessageLevels);
     (updateGroupCallMessageSendFailed, UpdateGroupCallMessageSendFailed);
     (updateGroupCallMessagesDeleted, UpdateGroupCallMessagesDeleted);
     (updateGroupCallParticipant, UpdateGroupCallParticipant);
     (updateGroupCallParticipants, UpdateGroupCallParticipants);
     (updateGroupCallVerificationState, UpdateGroupCallVerificationState);
     (updateHavePendingNotifications, UpdateHavePendingNotifications);
     (updateInstalledStickerSets, UpdateInstalledStickerSets);
     (updateLanguagePackStrings, UpdateLanguagePackStrings);
     (updateLiveStoryTopDonors, UpdateLiveStoryTopDonors);
     (updateManagedBot, UpdateManagedBot);
     (updateMessageContainsUnreadPollVotes, UpdateMessageContainsUnreadPollVotes);
     (updateMessageContent, UpdateMessageContent);
     (updateMessageContentOpened, UpdateMessageContentOpened);
     (updateMessageEdited, UpdateMessageEdited);
     (updateMessageEphemeralContent, UpdateMessageEphemeralContent);
     (updateMessageFactCheck, UpdateMessageFactCheck);
     (updateMessageInteractionInfo, UpdateMessageInteractionInfo);
     (updateMessageIsPinned, UpdateMessageIsPinned);
     (updateMessageLiveLocationViewed, UpdateMessageLiveLocationViewed);
     (updateMessageMentionRead, UpdateMessageMentionRead);
     (updateMessageReaction, UpdateMessageReaction);
     (updateMessageReactions, UpdateMessageReactions);
     (updateMessageSendAcknowledged, UpdateMessageSendAcknowledged);
     (updateMessageSendFailed, UpdateMessageSendFailed);
     (updateMessageSendSucceeded, UpdateMessageSendSucceeded);
     (updateMessageSuggestedPostInfo, UpdateMessageSuggestedPostInfo);
     (updateMessageUnreadReactions, UpdateMessageUnreadReactions);
     (updateNewBusinessCallbackQuery, UpdateNewBusinessCallbackQuery);
     (updateNewBusinessMessage, UpdateNewBusinessMessage);
     (updateNewCallSignalingData, UpdateNewCallSignalingData);
     (updateNewCallbackQuery, UpdateNewCallbackQuery);
     (updateNewChat, UpdateNewChat);
     (updateNewChatJoinRequest, UpdateNewChatJoinRequest);
     (updateNewChosenInlineResult, UpdateNewChosenInlineResult);
     (updateNewCustomEvent, UpdateNewCustomEvent);
     (updateNewCustomQuery, UpdateNewCustomQuery);
     (updateNewGroupCallMessage, UpdateNewGroupCallMessage);
     (updateNewGroupCallPaidReaction, UpdateNewGroupCallPaidReaction);
     (updateNewGuestQuery, UpdateNewGuestQuery);
     (updateNewInlineCallbackQuery, UpdateNewInlineCallbackQuery);
     (updateNewInlineQuery, UpdateNewInlineQuery);
     (updateNewMessage, UpdateNewMessage);
     (updateNewOauthRequest, UpdateNewOauthRequest);
     (updateNewPreCheckoutQuery, UpdateNewPreCheckoutQuery);
     (updateNewShippingQuery, UpdateNewShippingQuery);
     (updateNotification, UpdateNotification);
     (updateNotificationGroup, UpdateNotificationGroup);
     (updateOption, UpdateOption);
     (updateOwnedGramCount, UpdateOwnedGramCount);
     (updateOwnedStarCount, UpdateOwnedStarCount);
     (updatePaidMediaPurchased, UpdatePaidMediaPurchased);
     (updatePendingMessage, UpdatePendingMessage);
     (updatePoll, UpdatePoll);
     (updatePollAnswer, UpdatePollAnswer);
     (updateProfileAccentColors, UpdateProfileAccentColors);
     (updateQuickReplyShortcut, UpdateQuickReplyShortcut);
     (updateQuickReplyShortcutDeleted, UpdateQuickReplyShortcutDeleted);
     (updateQuickReplyShortcutMessages, UpdateQuickReplyShortcutMessages);
     (updateQuickReplyShortcuts, UpdateQuickReplyShortcuts);
     (updateReactionNotificationSettings, UpdateReactionNotificationSettings);
     (updateRecentStickers, UpdateRecentStickers);
     (updateSavedAnimations, UpdateSavedAnimations);
     (updateSavedMessagesTags, UpdateSavedMessagesTags);
     (updateSavedMessagesTopic, UpdateSavedMessagesTopic);
     (updateSavedMessagesTopicCount, UpdateSavedMessagesTopicCount);
     (updateSavedNotificationSounds, UpdateSavedNotificationSounds);
     (updateScopeNotificationSettings, UpdateScopeNotificationSettings);
     (updateSecretChat, UpdateSecretChat);
     (updateServiceNotification, UpdateServiceNotification);
     (updateSpeechRecognitionTrial, UpdateSpeechRecognitionTrial);
     (updateSpeedLimitNotification, UpdateSpeedLimitNotification);
     (updateStakeDiceState, UpdateStakeDiceState);
     (updateStarRevenueStatus, UpdateStarRevenueStatus);
     (updateStickerSet, UpdateStickerSet);
     (updateStopMessageDraft, UpdateStopMessageDraft);
     (updateStory, UpdateStory);
     (updateStoryDeleted, UpdateStoryDeleted);
     (updateStoryListChatCount, UpdateStoryListChatCount);
     (updateStoryPostFailed, UpdateStoryPostFailed);
     (updateStoryPostSucceeded, UpdateStoryPostSucceeded);
     (updateStoryStealthMode, UpdateStoryStealthMode);
     (updateSuggestedActions, UpdateSuggestedActions);
     (updateSupergroup, UpdateSupergroup);
     (updateSupergroupFullInfo, UpdateSupergroupFullInfo);
     (updateTermsOfService, UpdateTermsOfService);
     (updateTextCompositionStyles, UpdateTextCompositionStyles);
     (updateTonWalletGaslessTransfersInfo, UpdateTonWalletGaslessTransfersInfo);
     (updateTonWalletState, UpdateTonWalletState);
     (updateTonWalletTonConnectSession, UpdateTonWalletTonConnectSession);
     (updateTonWalletTonConnectSessionDisconnectRequired, UpdateTonWalletTonConnectSessionDisconnectRequired);
     (updateTopicMessageCount, UpdateTopicMessageCount);
     (updateTrendingStickerSets, UpdateTrendingStickerSets);
     (updateTrustedMiniAppBots, UpdateTrustedMiniAppBots);
     (updateUnconfirmedSession, UpdateUnconfirmedSession);
     (updateUnreadChatCount, UpdateUnreadChatCount);
     (updateUnreadMessageCount, UpdateUnreadMessageCount);
     (updateUser, UpdateUser);
     (updateUserFullInfo, UpdateUserFullInfo);
     (updateUserPrivacySettingRules, UpdateUserPrivacySettingRules);
     (updateUserStatus, UpdateUserStatus);
     (updateUserSubscription, UpdateUserSubscription);
     (updateVideoPublished, UpdateVideoPublished);
     (updateWebAppMessageSent, UpdateWebAppMessageSent);
     (updateWebBrowserSettings, UpdateWebBrowserSettings);
     (updateWebProxyCarrierClosed, UpdateWebProxyCarrierClosed);
     (updateWebProxyCarrierData, UpdateWebProxyCarrierData);
     (updateWebProxyCarrierOpened, UpdateWebProxyCarrierOpened);

     (AuthorizationState, AuthorizationState);
     (CanPostStoryResult, CanPostStoryResult);
     (CanSendGiftResult, CanSendGiftResult);
     (CanSendMessageToUserResult, CanSendMessageToUserResult);
     (CanTransferOwnershipResult, CanTransferOwnershipResult);
     (ChatJoinResult, ChatJoinResult);
     (ChatStatistics, ChatStatistics);
     (CheckChatUsernameResult, CheckChatUsernameResult);
     (CheckStickerSetNameResult, CheckStickerSetNameResult);
     (CraftGiftResult, CraftGiftResult);
     (GiftResaleResult, GiftResaleResult);
     (GiveawayInfo, GiveawayInfo);
     (InternalLinkType, InternalLinkType);
     (InviteGroupCallParticipantResult, InviteGroupCallParticipantResult);
     (JsonValue, JsonValue);
     (LanguagePackStringValue, LanguagePackStringValue);
     (LogStream, LogStream);
     (LoginUrlInfo, LoginUrlInfo);
     (MessageFileType, MessageFileType);
     (MessageReadDate, MessageReadDate);
     (MessageSender, MessageSender);
     (OptionValue, OptionValue);
     (PassportElement, PassportElement);
     (ReportChatResult, ReportChatResult);
     (ReportSponsoredResult, ReportSponsoredResult);
     (ReportStoryResult, ReportStoryResult);
     (ResetPasswordResult, ResetPasswordResult);
     (StartLiveStoryResult, StartLiveStoryResult);
     (StatisticalGraph, StatisticalGraph);
     (Update, Update);
     (WebBrowserType, WebBrowserType);
     (accountTtl, AccountTtl);
     (addedProxies, AddedProxies);
     (addedProxy, AddedProxy);
     (addedReactions, AddedReactions);
     (animatedEmoji, AnimatedEmoji);
     (animation, Animation);
     (animations, Animations);
     (archiveChatListSettings, ArchiveChatListSettings);
     (attachmentMenuBot, AttachmentMenuBot);
     (audios, Audios);
     (authenticationCodeInfo, AuthenticationCodeInfo);
     (autoDownloadSettingsPresets, AutoDownloadSettingsPresets);
     (autosaveSettings, AutosaveSettings);
     (availableGifts, AvailableGifts);
     (availableReactions, AvailableReactions);
     (background, Background);
     (backgrounds, Backgrounds);
     (bankCardInfo, BankCardInfo);
     (basicGroup, BasicGroup);
     (basicGroupFullInfo, BasicGroupFullInfo);
     (botAccessSettings, BotAccessSettings);
     (botCommands, BotCommands);
     (botMediaPreview, BotMediaPreview);
     (botMediaPreviewInfo, BotMediaPreviewInfo);
     (botMediaPreviews, BotMediaPreviews);
     (botMenuButton, BotMenuButton);
     (businessChatLink, BusinessChatLink);
     (businessChatLinkInfo, BusinessChatLinkInfo);
     (businessChatLinks, BusinessChatLinks);
     (businessConnectedBotInfo, BusinessConnectedBotInfo);
     (businessConnection, BusinessConnection);
     (businessFeatures, BusinessFeatures);
     (businessMessage, BusinessMessage);
     (businessMessages, BusinessMessages);
     (callId, CallId);
     (callbackQueryAnswer, CallbackQueryAnswer);
     (chat, Chat);
     (chatActiveStories, ChatActiveStories);
     (chatAdministrators, ChatAdministrators);
     (chatBoostFeatures, ChatBoostFeatures);
     (chatBoostLevelFeatures, ChatBoostLevelFeatures);
     (chatBoostLink, ChatBoostLink);
     (chatBoostLinkInfo, ChatBoostLinkInfo);
     (chatBoostSlots, ChatBoostSlots);
     (chatBoostStatus, ChatBoostStatus);
     (chatEvents, ChatEvents);
     (chatFolder, ChatFolder);
     (chatFolderIcon, ChatFolderIcon);
     (chatFolderInfo, ChatFolderInfo);
     (chatFolderInviteLink, ChatFolderInviteLink);
     (chatFolderInviteLinkInfo, ChatFolderInviteLinkInfo);
     (chatFolderInviteLinks, ChatFolderInviteLinks);
     (chatInviteLink, ChatInviteLink);
     (chatInviteLinkCounts, ChatInviteLinkCounts);
     (chatInviteLinkInfo, ChatInviteLinkInfo);
     (chatInviteLinkMembers, ChatInviteLinkMembers);
     (chatInviteLinks, ChatInviteLinks);
     (chatJoinRequests, ChatJoinRequests);
     (chatLists, ChatLists);
     (chatMember, ChatMember);
     (chatMembers, ChatMembers);
     (chatMessageSenders, ChatMessageSenders);
     (chatPhotos, ChatPhotos);
     (chatRevenueStatistics, ChatRevenueStatistics);
     (chatRevenueTransactions, ChatRevenueTransactions);
     (chats, Chats);
     (collectibleItemInfo, CollectibleItemInfo);
     (communityFullInfo, CommunityFullInfo);
     (communityId, CommunityId);
     (connectedAffiliateProgram, ConnectedAffiliateProgram);
     (connectedAffiliatePrograms, ConnectedAffiliatePrograms);
     (connectedWebsites, ConnectedWebsites);
     (count, Count);
     (countries, Countries);
     (countryInfo, CountryInfo);
     (createdBasicGroupChat, CreatedBasicGroupChat);
     (currencies, Currencies);
     (currencyExchangeRates, CurrencyExchangeRates);
     (currentWeather, CurrentWeather);
     (customRequestResult, CustomRequestResult);
     (data, Data);
     (databaseStatistics, DatabaseStatistics);
     (deepLinkInfo, DeepLinkInfo);
     (directMessagesChatTopic, DirectMessagesChatTopic);
     (emailAddressAuthenticationCodeInfo, EmailAddressAuthenticationCodeInfo);
     (emojiCategories, EmojiCategories);
     (emojiKeywords, EmojiKeywords);
     (emojiReaction, EmojiReaction);
     (emojiStatusCustomEmojis, EmojiStatusCustomEmojis);
     (emojiStatuses, EmojiStatuses);
     (emojis, Emojis);
     (error, Error);
     (failedToAddMembers, FailedToAddMembers);
     (file, File);
     (fileDownloadedPrefixSize, FileDownloadedPrefixSize);
     (fixedText, FixedText);
     (formattedText, FormattedText);
     (forumTopic, ForumTopic);
     (forumTopicInfo, ForumTopicInfo);
     (forumTopics, ForumTopics);
     (foundAffiliatePrograms, FoundAffiliatePrograms);
     (foundChatBoosts, FoundChatBoosts);
     (foundChatMessages, FoundChatMessages);
     (foundFileDownloads, FoundFileDownloads);
     (foundMessages, FoundMessages);
     (foundPosition, FoundPosition);
     (foundPositions, FoundPositions);
     (foundPublicPosts, FoundPublicPosts);
     (foundStories, FoundStories);
     (foundUsers, FoundUsers);
     (foundWebApp, FoundWebApp);
     (gameHighScores, GameHighScores);
     (giftAuctionAcquiredGifts, GiftAuctionAcquiredGifts);
     (giftAuctionState, GiftAuctionState);
     (giftChatThemes, GiftChatThemes);
     (giftCollection, GiftCollection);
     (giftCollections, GiftCollections);
     (giftUpgradePreview, GiftUpgradePreview);
     (giftUpgradeVariants, GiftUpgradeVariants);
     (giftsForCrafting, GiftsForCrafting);
     (giftsForResale, GiftsForResale);
     (gramRevenueStatistics, GramRevenueStatistics);
     (groupCall, GroupCall);
     (groupCallId, GroupCallId);
     (groupCallInfo, GroupCallInfo);
     (groupCallParticipant, GroupCallParticipant);
     (groupCallParticipants, GroupCallParticipants);
     (groupCallStreams, GroupCallStreams);
     (hashtags, Hashtags);
     (httpUrl, HttpUrl);
     (importedContacts, ImportedContacts);
     (inlineMessageId, InlineMessageId);
     (inlineQueryResults, InlineQueryResults);
     (keyboardButton, KeyboardButton);
     (languagePackInfo, LanguagePackInfo);
     (languagePackStrings, LanguagePackStrings);
     (linkPreview, LinkPreview);
     (liveStoryDonors, LiveStoryDonors);
     (localizationTargetInfo, LocalizationTargetInfo);
     (logTags, LogTags);
     (logVerbosityLevel, LogVerbosityLevel);
     (mainWebApp, MainWebApp);
     (message, Message);
     (messageAutoDeleteTime, MessageAutoDeleteTime);
     (messageCalendar, MessageCalendar);
     (messageEffect, MessageEffect);
     (messageLink, MessageLink);
     (messageLinkInfo, MessageLinkInfo);
     (messagePositions, MessagePositions);
     (messageProperties, MessageProperties);
     (messageSenders, MessageSenders);
     (messageStatistics, MessageStatistics);
     (messageThreadInfo, MessageThreadInfo);
     (messageViewers, MessageViewers);
     (messages, Messages);
     (networkStatistics, NetworkStatistics);
     (newChatPrivacySettings, NewChatPrivacySettings);
     (notificationSound, NotificationSound);
     (notificationSounds, NotificationSounds);
     (oauthLinkInfo, OauthLinkInfo);
     (ok, Ok);
     (onRampPaymentAvailability, OnRampPaymentAvailability);
     (onRampPaymentLimits, OnRampPaymentLimits);
     (onRampPaymentQuote, OnRampPaymentQuote);
     (onRampPaymentSession, OnRampPaymentSession);
     (onRampProviders, OnRampProviders);
     (orderInfo, OrderInfo);
     (outline, Outline);
     (passkey, Passkey);
     (passkeys, Passkeys);
     (passportAuthorizationForm, PassportAuthorizationForm);
     (passportElements, PassportElements);
     (passportElementsWithErrors, PassportElementsWithErrors);
     (passwordState, PasswordState);
     (paymentForm, PaymentForm);
     (paymentReceipt, PaymentReceipt);
     (paymentResult, PaymentResult);
     (phoneNumberInfo, PhoneNumberInfo);
     (pollOptionProperties, PollOptionProperties);
     (pollVoteStatistics, PollVoteStatistics);
     (pollVoters, PollVoters);
     (premiumFeatures, PremiumFeatures);
     (premiumGiftCodeInfo, PremiumGiftCodeInfo);
     (premiumGiftPaymentOptions, PremiumGiftPaymentOptions);
     (premiumGiveawayPaymentOptions, PremiumGiveawayPaymentOptions);
     (premiumLimit, PremiumLimit);
     (premiumState, PremiumState);
     (preparedInlineMessage, PreparedInlineMessage);
     (preparedInlineMessageId, PreparedInlineMessageId);
     (profileShareThemeBackdrops, ProfileShareThemeBackdrops);
     (publicForwards, PublicForwards);
     (publicPostSearchLimits, PublicPostSearchLimits);
     (pushReceiverId, PushReceiverId);
     (quickReplyMessage, QuickReplyMessage);
     (quickReplyMessages, QuickReplyMessages);
     (readDatePrivacySettings, ReadDatePrivacySettings);
     (receivedGift, ReceivedGift);
     (receivedGifts, ReceivedGifts);
     (recommendedChatFolders, RecommendedChatFolders);
     (recoveryEmailAddress, RecoveryEmailAddress);
     (richMessage, RichMessage);
     (rtmpUrl, RtmpUrl);
     (savedMessagesTags, SavedMessagesTags);
     (scopeNotificationSettings, ScopeNotificationSettings);
     (seconds, Seconds);
     (secretChat, SecretChat);
     (session, Session);
     (sessions, Sessions);
     (sponsoredChats, SponsoredChats);
     (sponsoredMessages, SponsoredMessages);
     (stakeDiceState, StakeDiceState);
     (starAmount, StarAmount);
     (starCount, StarCount);
     (starGiveawayPaymentOptions, StarGiveawayPaymentOptions);
     (starPaymentOptions, StarPaymentOptions);
     (starRevenueStatistics, StarRevenueStatistics);
     (starSubscriptions, StarSubscriptions);
     (starTransactions, StarTransactions);
     (sticker, Sticker);
     (stickerSet, StickerSet);
     (stickerSets, StickerSets);
     (stickers, Stickers);
     (storageStatistics, StorageStatistics);
     (storageStatisticsFast, StorageStatisticsFast);
     (stories, Stories);
     (story, Story);
     (storyAlbum, StoryAlbum);
     (storyAlbums, StoryAlbums);
     (storyInteractions, StoryInteractions);
     (storyStatistics, StoryStatistics);
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
     (textCompositionStyle, TextCompositionStyle);
     (textCompositionStyleExample, TextCompositionStyleExample);
     (textEntities, TextEntities);
     (timeZones, TimeZones);
     (tonCenterStreamingApiUrl, TonCenterStreamingApiUrl);
     (tonConnectChallenge, TonConnectChallenge);
     (tonConnectRequests, TonConnectRequests);
     (tonConnectSession, TonConnectSession);
     (tonConnectSessionEventId, TonConnectSessionEventId);
     (tonConnectSessions, TonConnectSessions);
     (tonNfts, TonNfts);
     (tonTransactions, TonTransactions);
     (tonWalletCard, TonWalletCard);
     (tonWalletOwnershipProofChallenge, TonWalletOwnershipProofChallenge);
     (tonWalletSecretPhrase, TonWalletSecretPhrase);
     (tonWalletTransaction, TonWalletTransaction);
     (tonWalletTransactions, TonWalletTransactions);
     (tonWalletTransferResult, TonWalletTransferResult);
     (trendingStickerSets, TrendingStickerSets);
     (updates, Updates);
     (upgradeGiftResult, UpgradeGiftResult);
     (upgradedGift, UpgradedGift);
     (upgradedGiftCollectionNames, UpgradedGiftCollectionNames);
     (upgradedGiftDepositInfo, UpgradedGiftDepositInfo);
     (upgradedGiftValueInfo, UpgradedGiftValueInfo);
     (user, User);
     (userFullInfo, UserFullInfo);
     (userLink, UserLink);
     (userPrivacySettingRules, UserPrivacySettingRules);
     (userSupportInfo, UserSupportInfo);
     (userTonWalletAddress, UserTonWalletAddress);
     (userTonWalletAddresses, UserTonWalletAddresses);
     (users, Users);
     (validatedOrderInfo, ValidatedOrderInfo);
     (videoMessageAdvertisements, VideoMessageAdvertisements);
     (walletBotBalance, WalletBotBalance);
     (webAppInfo, WebAppInfo);
     (webAppUrl, WebAppUrl);
     (webPageInstantView, WebPageInstantView);

    )(deserializer)
  }
}

impl RObject for TdType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      Self::TestUseUpdate(value) => value.td_name(),
      Self::UpdateAccentColors(value) => value.td_name(),
      Self::UpdateActiveEmojiReactions(value) => value.td_name(),
      Self::UpdateActiveGiftAuctions(value) => value.td_name(),
      Self::UpdateActiveLiveLocationMessages(value) => value.td_name(),
      Self::UpdateActiveNotifications(value) => value.td_name(),
      Self::UpdateAgeVerificationParameters(value) => value.td_name(),
      Self::UpdateAllowedBotUsernamePremiumSuffixes(value) => value.td_name(),
      Self::UpdateAnimatedEmojiMessageClicked(value) => value.td_name(),
      Self::UpdateAnimationSearchParameters(value) => value.td_name(),
      Self::UpdateApplicationRecaptchaVerificationRequired(value) => value.td_name(),
      Self::UpdateApplicationVerificationRequired(value) => value.td_name(),
      Self::UpdateAttachmentMenuBots(value) => value.td_name(),
      Self::UpdateAuthorizationState(value) => value.td_name(),
      Self::UpdateAutosaveSettings(value) => value.td_name(),
      Self::UpdateAvailableMessageEffects(value) => value.td_name(),
      Self::UpdateBasicGroup(value) => value.td_name(),
      Self::UpdateBasicGroupFullInfo(value) => value.td_name(),
      Self::UpdateBusinessConnection(value) => value.td_name(),
      Self::UpdateBusinessMessageEdited(value) => value.td_name(),
      Self::UpdateBusinessMessagesDeleted(value) => value.td_name(),
      Self::UpdateCall(value) => value.td_name(),
      Self::UpdateChatAccentColors(value) => value.td_name(),
      Self::UpdateChatAction(value) => value.td_name(),
      Self::UpdateChatActionBar(value) => value.td_name(),
      Self::UpdateChatActiveStories(value) => value.td_name(),
      Self::UpdateChatAddedToList(value) => value.td_name(),
      Self::UpdateChatAvailableReactions(value) => value.td_name(),
      Self::UpdateChatBackground(value) => value.td_name(),
      Self::UpdateChatBlockList(value) => value.td_name(),
      Self::UpdateChatBoost(value) => value.td_name(),
      Self::UpdateChatBusinessBotManageBar(value) => value.td_name(),
      Self::UpdateChatDefaultDisableNotification(value) => value.td_name(),
      Self::UpdateChatDraftMessage(value) => value.td_name(),
      Self::UpdateChatEmojiStatus(value) => value.td_name(),
      Self::UpdateChatFolders(value) => value.td_name(),
      Self::UpdateChatHasProtectedContent(value) => value.td_name(),
      Self::UpdateChatHasScheduledMessages(value) => value.td_name(),
      Self::UpdateChatHasWelcomeMessages(value) => value.td_name(),
      Self::UpdateChatIsMarkedAsUnread(value) => value.td_name(),
      Self::UpdateChatIsTranslatable(value) => value.td_name(),
      Self::UpdateChatJoinResult(value) => value.td_name(),
      Self::UpdateChatLastMessage(value) => value.td_name(),
      Self::UpdateChatMember(value) => value.td_name(),
      Self::UpdateChatMessageAutoDeleteTime(value) => value.td_name(),
      Self::UpdateChatMessageSender(value) => value.td_name(),
      Self::UpdateChatNotificationSettings(value) => value.td_name(),
      Self::UpdateChatOnlineMemberCount(value) => value.td_name(),
      Self::UpdateChatPendingJoinRequests(value) => value.td_name(),
      Self::UpdateChatPermissions(value) => value.td_name(),
      Self::UpdateChatPhoto(value) => value.td_name(),
      Self::UpdateChatPosition(value) => value.td_name(),
      Self::UpdateChatReadInbox(value) => value.td_name(),
      Self::UpdateChatReadOutbox(value) => value.td_name(),
      Self::UpdateChatRemovedFromList(value) => value.td_name(),
      Self::UpdateChatReplyMarkup(value) => value.td_name(),
      Self::UpdateChatRevenueAmount(value) => value.td_name(),
      Self::UpdateChatTheme(value) => value.td_name(),
      Self::UpdateChatTitle(value) => value.td_name(),
      Self::UpdateChatUnreadMentionCount(value) => value.td_name(),
      Self::UpdateChatUnreadPollVoteCount(value) => value.td_name(),
      Self::UpdateChatUnreadReactionCount(value) => value.td_name(),
      Self::UpdateChatVideoChat(value) => value.td_name(),
      Self::UpdateChatViewAsTopics(value) => value.td_name(),
      Self::UpdateChatWelcomeMessages(value) => value.td_name(),
      Self::UpdateCommunity(value) => value.td_name(),
      Self::UpdateCommunityFullInfo(value) => value.td_name(),
      Self::UpdateConnectionState(value) => value.td_name(),
      Self::UpdateContactCloseBirthdays(value) => value.td_name(),
      Self::UpdateDefaultBackground(value) => value.td_name(),
      Self::UpdateDefaultPaidReactionType(value) => value.td_name(),
      Self::UpdateDefaultReactionType(value) => value.td_name(),
      Self::UpdateDeleteMessages(value) => value.td_name(),
      Self::UpdateDiceEmojis(value) => value.td_name(),
      Self::UpdateDirectMessagesChatTopic(value) => value.td_name(),
      Self::UpdateEmojiChatThemes(value) => value.td_name(),
      Self::UpdateFavoriteStickers(value) => value.td_name(),
      Self::UpdateFile(value) => value.td_name(),
      Self::UpdateFileAddedToDownloads(value) => value.td_name(),
      Self::UpdateFileDownload(value) => value.td_name(),
      Self::UpdateFileDownloads(value) => value.td_name(),
      Self::UpdateFileGenerationStart(value) => value.td_name(),
      Self::UpdateFileGenerationStop(value) => value.td_name(),
      Self::UpdateFileRemovedFromDownloads(value) => value.td_name(),
      Self::UpdateForumTopic(value) => value.td_name(),
      Self::UpdateForumTopicInfo(value) => value.td_name(),
      Self::UpdateFreezeState(value) => value.td_name(),
      Self::UpdateGiftAuctionState(value) => value.td_name(),
      Self::UpdateGramRevenueStatus(value) => value.td_name(),
      Self::UpdateGroupCall(value) => value.td_name(),
      Self::UpdateGroupCallMessageLevels(value) => value.td_name(),
      Self::UpdateGroupCallMessageSendFailed(value) => value.td_name(),
      Self::UpdateGroupCallMessagesDeleted(value) => value.td_name(),
      Self::UpdateGroupCallParticipant(value) => value.td_name(),
      Self::UpdateGroupCallParticipants(value) => value.td_name(),
      Self::UpdateGroupCallVerificationState(value) => value.td_name(),
      Self::UpdateHavePendingNotifications(value) => value.td_name(),
      Self::UpdateInstalledStickerSets(value) => value.td_name(),
      Self::UpdateLanguagePackStrings(value) => value.td_name(),
      Self::UpdateLiveStoryTopDonors(value) => value.td_name(),
      Self::UpdateManagedBot(value) => value.td_name(),
      Self::UpdateMessageContainsUnreadPollVotes(value) => value.td_name(),
      Self::UpdateMessageContent(value) => value.td_name(),
      Self::UpdateMessageContentOpened(value) => value.td_name(),
      Self::UpdateMessageEdited(value) => value.td_name(),
      Self::UpdateMessageEphemeralContent(value) => value.td_name(),
      Self::UpdateMessageFactCheck(value) => value.td_name(),
      Self::UpdateMessageInteractionInfo(value) => value.td_name(),
      Self::UpdateMessageIsPinned(value) => value.td_name(),
      Self::UpdateMessageLiveLocationViewed(value) => value.td_name(),
      Self::UpdateMessageMentionRead(value) => value.td_name(),
      Self::UpdateMessageReaction(value) => value.td_name(),
      Self::UpdateMessageReactions(value) => value.td_name(),
      Self::UpdateMessageSendAcknowledged(value) => value.td_name(),
      Self::UpdateMessageSendFailed(value) => value.td_name(),
      Self::UpdateMessageSendSucceeded(value) => value.td_name(),
      Self::UpdateMessageSuggestedPostInfo(value) => value.td_name(),
      Self::UpdateMessageUnreadReactions(value) => value.td_name(),
      Self::UpdateNewBusinessCallbackQuery(value) => value.td_name(),
      Self::UpdateNewBusinessMessage(value) => value.td_name(),
      Self::UpdateNewCallSignalingData(value) => value.td_name(),
      Self::UpdateNewCallbackQuery(value) => value.td_name(),
      Self::UpdateNewChat(value) => value.td_name(),
      Self::UpdateNewChatJoinRequest(value) => value.td_name(),
      Self::UpdateNewChosenInlineResult(value) => value.td_name(),
      Self::UpdateNewCustomEvent(value) => value.td_name(),
      Self::UpdateNewCustomQuery(value) => value.td_name(),
      Self::UpdateNewGroupCallMessage(value) => value.td_name(),
      Self::UpdateNewGroupCallPaidReaction(value) => value.td_name(),
      Self::UpdateNewGuestQuery(value) => value.td_name(),
      Self::UpdateNewInlineCallbackQuery(value) => value.td_name(),
      Self::UpdateNewInlineQuery(value) => value.td_name(),
      Self::UpdateNewMessage(value) => value.td_name(),
      Self::UpdateNewOauthRequest(value) => value.td_name(),
      Self::UpdateNewPreCheckoutQuery(value) => value.td_name(),
      Self::UpdateNewShippingQuery(value) => value.td_name(),
      Self::UpdateNotification(value) => value.td_name(),
      Self::UpdateNotificationGroup(value) => value.td_name(),
      Self::UpdateOption(value) => value.td_name(),
      Self::UpdateOwnedGramCount(value) => value.td_name(),
      Self::UpdateOwnedStarCount(value) => value.td_name(),
      Self::UpdatePaidMediaPurchased(value) => value.td_name(),
      Self::UpdatePendingMessage(value) => value.td_name(),
      Self::UpdatePoll(value) => value.td_name(),
      Self::UpdatePollAnswer(value) => value.td_name(),
      Self::UpdateProfileAccentColors(value) => value.td_name(),
      Self::UpdateQuickReplyShortcut(value) => value.td_name(),
      Self::UpdateQuickReplyShortcutDeleted(value) => value.td_name(),
      Self::UpdateQuickReplyShortcutMessages(value) => value.td_name(),
      Self::UpdateQuickReplyShortcuts(value) => value.td_name(),
      Self::UpdateReactionNotificationSettings(value) => value.td_name(),
      Self::UpdateRecentStickers(value) => value.td_name(),
      Self::UpdateSavedAnimations(value) => value.td_name(),
      Self::UpdateSavedMessagesTags(value) => value.td_name(),
      Self::UpdateSavedMessagesTopic(value) => value.td_name(),
      Self::UpdateSavedMessagesTopicCount(value) => value.td_name(),
      Self::UpdateSavedNotificationSounds(value) => value.td_name(),
      Self::UpdateScopeNotificationSettings(value) => value.td_name(),
      Self::UpdateSecretChat(value) => value.td_name(),
      Self::UpdateServiceNotification(value) => value.td_name(),
      Self::UpdateSpeechRecognitionTrial(value) => value.td_name(),
      Self::UpdateSpeedLimitNotification(value) => value.td_name(),
      Self::UpdateStakeDiceState(value) => value.td_name(),
      Self::UpdateStarRevenueStatus(value) => value.td_name(),
      Self::UpdateStickerSet(value) => value.td_name(),
      Self::UpdateStopMessageDraft(value) => value.td_name(),
      Self::UpdateStory(value) => value.td_name(),
      Self::UpdateStoryDeleted(value) => value.td_name(),
      Self::UpdateStoryListChatCount(value) => value.td_name(),
      Self::UpdateStoryPostFailed(value) => value.td_name(),
      Self::UpdateStoryPostSucceeded(value) => value.td_name(),
      Self::UpdateStoryStealthMode(value) => value.td_name(),
      Self::UpdateSuggestedActions(value) => value.td_name(),
      Self::UpdateSupergroup(value) => value.td_name(),
      Self::UpdateSupergroupFullInfo(value) => value.td_name(),
      Self::UpdateTermsOfService(value) => value.td_name(),
      Self::UpdateTextCompositionStyles(value) => value.td_name(),
      Self::UpdateTonWalletGaslessTransfersInfo(value) => value.td_name(),
      Self::UpdateTonWalletState(value) => value.td_name(),
      Self::UpdateTonWalletTonConnectSession(value) => value.td_name(),
      Self::UpdateTonWalletTonConnectSessionDisconnectRequired(value) => value.td_name(),
      Self::UpdateTopicMessageCount(value) => value.td_name(),
      Self::UpdateTrendingStickerSets(value) => value.td_name(),
      Self::UpdateTrustedMiniAppBots(value) => value.td_name(),
      Self::UpdateUnconfirmedSession(value) => value.td_name(),
      Self::UpdateUnreadChatCount(value) => value.td_name(),
      Self::UpdateUnreadMessageCount(value) => value.td_name(),
      Self::UpdateUser(value) => value.td_name(),
      Self::UpdateUserFullInfo(value) => value.td_name(),
      Self::UpdateUserPrivacySettingRules(value) => value.td_name(),
      Self::UpdateUserStatus(value) => value.td_name(),
      Self::UpdateUserSubscription(value) => value.td_name(),
      Self::UpdateVideoPublished(value) => value.td_name(),
      Self::UpdateWebAppMessageSent(value) => value.td_name(),
      Self::UpdateWebBrowserSettings(value) => value.td_name(),
      Self::UpdateWebProxyCarrierClosed(value) => value.td_name(),
      Self::UpdateWebProxyCarrierData(value) => value.td_name(),
      Self::UpdateWebProxyCarrierOpened(value) => value.td_name(),

      Self::AuthorizationState(value) => value.td_name(),
      Self::CanPostStoryResult(value) => value.td_name(),
      Self::CanSendGiftResult(value) => value.td_name(),
      Self::CanSendMessageToUserResult(value) => value.td_name(),
      Self::CanTransferOwnershipResult(value) => value.td_name(),
      Self::ChatJoinResult(value) => value.td_name(),
      Self::ChatStatistics(value) => value.td_name(),
      Self::CheckChatUsernameResult(value) => value.td_name(),
      Self::CheckStickerSetNameResult(value) => value.td_name(),
      Self::CraftGiftResult(value) => value.td_name(),
      Self::GiftResaleResult(value) => value.td_name(),
      Self::GiveawayInfo(value) => value.td_name(),
      Self::InternalLinkType(value) => value.td_name(),
      Self::InviteGroupCallParticipantResult(value) => value.td_name(),
      Self::JsonValue(value) => value.td_name(),
      Self::LanguagePackStringValue(value) => value.td_name(),
      Self::LogStream(value) => value.td_name(),
      Self::LoginUrlInfo(value) => value.td_name(),
      Self::MessageFileType(value) => value.td_name(),
      Self::MessageReadDate(value) => value.td_name(),
      Self::MessageSender(value) => value.td_name(),
      Self::OptionValue(value) => value.td_name(),
      Self::PassportElement(value) => value.td_name(),
      Self::ReportChatResult(value) => value.td_name(),
      Self::ReportSponsoredResult(value) => value.td_name(),
      Self::ReportStoryResult(value) => value.td_name(),
      Self::ResetPasswordResult(value) => value.td_name(),
      Self::StartLiveStoryResult(value) => value.td_name(),
      Self::StatisticalGraph(value) => value.td_name(),
      Self::Update(value) => value.td_name(),
      Self::WebBrowserType(value) => value.td_name(),
      Self::AccountTtl(value) => value.td_name(),
      Self::AddedProxies(value) => value.td_name(),
      Self::AddedProxy(value) => value.td_name(),
      Self::AddedReactions(value) => value.td_name(),
      Self::AnimatedEmoji(value) => value.td_name(),
      Self::Animation(value) => value.td_name(),
      Self::Animations(value) => value.td_name(),
      Self::ArchiveChatListSettings(value) => value.td_name(),
      Self::AttachmentMenuBot(value) => value.td_name(),
      Self::Audios(value) => value.td_name(),
      Self::AuthenticationCodeInfo(value) => value.td_name(),
      Self::AutoDownloadSettingsPresets(value) => value.td_name(),
      Self::AutosaveSettings(value) => value.td_name(),
      Self::AvailableGifts(value) => value.td_name(),
      Self::AvailableReactions(value) => value.td_name(),
      Self::Background(value) => value.td_name(),
      Self::Backgrounds(value) => value.td_name(),
      Self::BankCardInfo(value) => value.td_name(),
      Self::BasicGroup(value) => value.td_name(),
      Self::BasicGroupFullInfo(value) => value.td_name(),
      Self::BotAccessSettings(value) => value.td_name(),
      Self::BotCommands(value) => value.td_name(),
      Self::BotMediaPreview(value) => value.td_name(),
      Self::BotMediaPreviewInfo(value) => value.td_name(),
      Self::BotMediaPreviews(value) => value.td_name(),
      Self::BotMenuButton(value) => value.td_name(),
      Self::BusinessChatLink(value) => value.td_name(),
      Self::BusinessChatLinkInfo(value) => value.td_name(),
      Self::BusinessChatLinks(value) => value.td_name(),
      Self::BusinessConnectedBotInfo(value) => value.td_name(),
      Self::BusinessConnection(value) => value.td_name(),
      Self::BusinessFeatures(value) => value.td_name(),
      Self::BusinessMessage(value) => value.td_name(),
      Self::BusinessMessages(value) => value.td_name(),
      Self::CallId(value) => value.td_name(),
      Self::CallbackQueryAnswer(value) => value.td_name(),
      Self::Chat(value) => value.td_name(),
      Self::ChatActiveStories(value) => value.td_name(),
      Self::ChatAdministrators(value) => value.td_name(),
      Self::ChatBoostFeatures(value) => value.td_name(),
      Self::ChatBoostLevelFeatures(value) => value.td_name(),
      Self::ChatBoostLink(value) => value.td_name(),
      Self::ChatBoostLinkInfo(value) => value.td_name(),
      Self::ChatBoostSlots(value) => value.td_name(),
      Self::ChatBoostStatus(value) => value.td_name(),
      Self::ChatEvents(value) => value.td_name(),
      Self::ChatFolder(value) => value.td_name(),
      Self::ChatFolderIcon(value) => value.td_name(),
      Self::ChatFolderInfo(value) => value.td_name(),
      Self::ChatFolderInviteLink(value) => value.td_name(),
      Self::ChatFolderInviteLinkInfo(value) => value.td_name(),
      Self::ChatFolderInviteLinks(value) => value.td_name(),
      Self::ChatInviteLink(value) => value.td_name(),
      Self::ChatInviteLinkCounts(value) => value.td_name(),
      Self::ChatInviteLinkInfo(value) => value.td_name(),
      Self::ChatInviteLinkMembers(value) => value.td_name(),
      Self::ChatInviteLinks(value) => value.td_name(),
      Self::ChatJoinRequests(value) => value.td_name(),
      Self::ChatLists(value) => value.td_name(),
      Self::ChatMember(value) => value.td_name(),
      Self::ChatMembers(value) => value.td_name(),
      Self::ChatMessageSenders(value) => value.td_name(),
      Self::ChatPhotos(value) => value.td_name(),
      Self::ChatRevenueStatistics(value) => value.td_name(),
      Self::ChatRevenueTransactions(value) => value.td_name(),
      Self::Chats(value) => value.td_name(),
      Self::CollectibleItemInfo(value) => value.td_name(),
      Self::CommunityFullInfo(value) => value.td_name(),
      Self::CommunityId(value) => value.td_name(),
      Self::ConnectedAffiliateProgram(value) => value.td_name(),
      Self::ConnectedAffiliatePrograms(value) => value.td_name(),
      Self::ConnectedWebsites(value) => value.td_name(),
      Self::Count(value) => value.td_name(),
      Self::Countries(value) => value.td_name(),
      Self::CountryInfo(value) => value.td_name(),
      Self::CreatedBasicGroupChat(value) => value.td_name(),
      Self::Currencies(value) => value.td_name(),
      Self::CurrencyExchangeRates(value) => value.td_name(),
      Self::CurrentWeather(value) => value.td_name(),
      Self::CustomRequestResult(value) => value.td_name(),
      Self::Data(value) => value.td_name(),
      Self::DatabaseStatistics(value) => value.td_name(),
      Self::DeepLinkInfo(value) => value.td_name(),
      Self::DirectMessagesChatTopic(value) => value.td_name(),
      Self::EmailAddressAuthenticationCodeInfo(value) => value.td_name(),
      Self::EmojiCategories(value) => value.td_name(),
      Self::EmojiKeywords(value) => value.td_name(),
      Self::EmojiReaction(value) => value.td_name(),
      Self::EmojiStatusCustomEmojis(value) => value.td_name(),
      Self::EmojiStatuses(value) => value.td_name(),
      Self::Emojis(value) => value.td_name(),
      Self::Error(value) => value.td_name(),
      Self::FailedToAddMembers(value) => value.td_name(),
      Self::File(value) => value.td_name(),
      Self::FileDownloadedPrefixSize(value) => value.td_name(),
      Self::FixedText(value) => value.td_name(),
      Self::FormattedText(value) => value.td_name(),
      Self::ForumTopic(value) => value.td_name(),
      Self::ForumTopicInfo(value) => value.td_name(),
      Self::ForumTopics(value) => value.td_name(),
      Self::FoundAffiliatePrograms(value) => value.td_name(),
      Self::FoundChatBoosts(value) => value.td_name(),
      Self::FoundChatMessages(value) => value.td_name(),
      Self::FoundFileDownloads(value) => value.td_name(),
      Self::FoundMessages(value) => value.td_name(),
      Self::FoundPosition(value) => value.td_name(),
      Self::FoundPositions(value) => value.td_name(),
      Self::FoundPublicPosts(value) => value.td_name(),
      Self::FoundStories(value) => value.td_name(),
      Self::FoundUsers(value) => value.td_name(),
      Self::FoundWebApp(value) => value.td_name(),
      Self::GameHighScores(value) => value.td_name(),
      Self::GiftAuctionAcquiredGifts(value) => value.td_name(),
      Self::GiftAuctionState(value) => value.td_name(),
      Self::GiftChatThemes(value) => value.td_name(),
      Self::GiftCollection(value) => value.td_name(),
      Self::GiftCollections(value) => value.td_name(),
      Self::GiftUpgradePreview(value) => value.td_name(),
      Self::GiftUpgradeVariants(value) => value.td_name(),
      Self::GiftsForCrafting(value) => value.td_name(),
      Self::GiftsForResale(value) => value.td_name(),
      Self::GramRevenueStatistics(value) => value.td_name(),
      Self::GroupCall(value) => value.td_name(),
      Self::GroupCallId(value) => value.td_name(),
      Self::GroupCallInfo(value) => value.td_name(),
      Self::GroupCallParticipant(value) => value.td_name(),
      Self::GroupCallParticipants(value) => value.td_name(),
      Self::GroupCallStreams(value) => value.td_name(),
      Self::Hashtags(value) => value.td_name(),
      Self::HttpUrl(value) => value.td_name(),
      Self::ImportedContacts(value) => value.td_name(),
      Self::InlineMessageId(value) => value.td_name(),
      Self::InlineQueryResults(value) => value.td_name(),
      Self::KeyboardButton(value) => value.td_name(),
      Self::LanguagePackInfo(value) => value.td_name(),
      Self::LanguagePackStrings(value) => value.td_name(),
      Self::LinkPreview(value) => value.td_name(),
      Self::LiveStoryDonors(value) => value.td_name(),
      Self::LocalizationTargetInfo(value) => value.td_name(),
      Self::LogTags(value) => value.td_name(),
      Self::LogVerbosityLevel(value) => value.td_name(),
      Self::MainWebApp(value) => value.td_name(),
      Self::Message(value) => value.td_name(),
      Self::MessageAutoDeleteTime(value) => value.td_name(),
      Self::MessageCalendar(value) => value.td_name(),
      Self::MessageEffect(value) => value.td_name(),
      Self::MessageLink(value) => value.td_name(),
      Self::MessageLinkInfo(value) => value.td_name(),
      Self::MessagePositions(value) => value.td_name(),
      Self::MessageProperties(value) => value.td_name(),
      Self::MessageSenders(value) => value.td_name(),
      Self::MessageStatistics(value) => value.td_name(),
      Self::MessageThreadInfo(value) => value.td_name(),
      Self::MessageViewers(value) => value.td_name(),
      Self::Messages(value) => value.td_name(),
      Self::NetworkStatistics(value) => value.td_name(),
      Self::NewChatPrivacySettings(value) => value.td_name(),
      Self::NotificationSound(value) => value.td_name(),
      Self::NotificationSounds(value) => value.td_name(),
      Self::OauthLinkInfo(value) => value.td_name(),
      Self::Ok(value) => value.td_name(),
      Self::OnRampPaymentAvailability(value) => value.td_name(),
      Self::OnRampPaymentLimits(value) => value.td_name(),
      Self::OnRampPaymentQuote(value) => value.td_name(),
      Self::OnRampPaymentSession(value) => value.td_name(),
      Self::OnRampProviders(value) => value.td_name(),
      Self::OrderInfo(value) => value.td_name(),
      Self::Outline(value) => value.td_name(),
      Self::Passkey(value) => value.td_name(),
      Self::Passkeys(value) => value.td_name(),
      Self::PassportAuthorizationForm(value) => value.td_name(),
      Self::PassportElements(value) => value.td_name(),
      Self::PassportElementsWithErrors(value) => value.td_name(),
      Self::PasswordState(value) => value.td_name(),
      Self::PaymentForm(value) => value.td_name(),
      Self::PaymentReceipt(value) => value.td_name(),
      Self::PaymentResult(value) => value.td_name(),
      Self::PhoneNumberInfo(value) => value.td_name(),
      Self::PollOptionProperties(value) => value.td_name(),
      Self::PollVoteStatistics(value) => value.td_name(),
      Self::PollVoters(value) => value.td_name(),
      Self::PremiumFeatures(value) => value.td_name(),
      Self::PremiumGiftCodeInfo(value) => value.td_name(),
      Self::PremiumGiftPaymentOptions(value) => value.td_name(),
      Self::PremiumGiveawayPaymentOptions(value) => value.td_name(),
      Self::PremiumLimit(value) => value.td_name(),
      Self::PremiumState(value) => value.td_name(),
      Self::PreparedInlineMessage(value) => value.td_name(),
      Self::PreparedInlineMessageId(value) => value.td_name(),
      Self::ProfileShareThemeBackdrops(value) => value.td_name(),
      Self::PublicForwards(value) => value.td_name(),
      Self::PublicPostSearchLimits(value) => value.td_name(),
      Self::PushReceiverId(value) => value.td_name(),
      Self::QuickReplyMessage(value) => value.td_name(),
      Self::QuickReplyMessages(value) => value.td_name(),
      Self::ReadDatePrivacySettings(value) => value.td_name(),
      Self::ReceivedGift(value) => value.td_name(),
      Self::ReceivedGifts(value) => value.td_name(),
      Self::RecommendedChatFolders(value) => value.td_name(),
      Self::RecoveryEmailAddress(value) => value.td_name(),
      Self::RichMessage(value) => value.td_name(),
      Self::RtmpUrl(value) => value.td_name(),
      Self::SavedMessagesTags(value) => value.td_name(),
      Self::ScopeNotificationSettings(value) => value.td_name(),
      Self::Seconds(value) => value.td_name(),
      Self::SecretChat(value) => value.td_name(),
      Self::Session(value) => value.td_name(),
      Self::Sessions(value) => value.td_name(),
      Self::SponsoredChats(value) => value.td_name(),
      Self::SponsoredMessages(value) => value.td_name(),
      Self::StakeDiceState(value) => value.td_name(),
      Self::StarAmount(value) => value.td_name(),
      Self::StarCount(value) => value.td_name(),
      Self::StarGiveawayPaymentOptions(value) => value.td_name(),
      Self::StarPaymentOptions(value) => value.td_name(),
      Self::StarRevenueStatistics(value) => value.td_name(),
      Self::StarSubscriptions(value) => value.td_name(),
      Self::StarTransactions(value) => value.td_name(),
      Self::Sticker(value) => value.td_name(),
      Self::StickerSet(value) => value.td_name(),
      Self::StickerSets(value) => value.td_name(),
      Self::Stickers(value) => value.td_name(),
      Self::StorageStatistics(value) => value.td_name(),
      Self::StorageStatisticsFast(value) => value.td_name(),
      Self::Stories(value) => value.td_name(),
      Self::Story(value) => value.td_name(),
      Self::StoryAlbum(value) => value.td_name(),
      Self::StoryAlbums(value) => value.td_name(),
      Self::StoryInteractions(value) => value.td_name(),
      Self::StoryStatistics(value) => value.td_name(),
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
      Self::TextCompositionStyle(value) => value.td_name(),
      Self::TextCompositionStyleExample(value) => value.td_name(),
      Self::TextEntities(value) => value.td_name(),
      Self::TimeZones(value) => value.td_name(),
      Self::TonCenterStreamingApiUrl(value) => value.td_name(),
      Self::TonConnectChallenge(value) => value.td_name(),
      Self::TonConnectRequests(value) => value.td_name(),
      Self::TonConnectSession(value) => value.td_name(),
      Self::TonConnectSessionEventId(value) => value.td_name(),
      Self::TonConnectSessions(value) => value.td_name(),
      Self::TonNfts(value) => value.td_name(),
      Self::TonTransactions(value) => value.td_name(),
      Self::TonWalletCard(value) => value.td_name(),
      Self::TonWalletOwnershipProofChallenge(value) => value.td_name(),
      Self::TonWalletSecretPhrase(value) => value.td_name(),
      Self::TonWalletTransaction(value) => value.td_name(),
      Self::TonWalletTransactions(value) => value.td_name(),
      Self::TonWalletTransferResult(value) => value.td_name(),
      Self::TrendingStickerSets(value) => value.td_name(),
      Self::Updates(value) => value.td_name(),
      Self::UpgradeGiftResult(value) => value.td_name(),
      Self::UpgradedGift(value) => value.td_name(),
      Self::UpgradedGiftCollectionNames(value) => value.td_name(),
      Self::UpgradedGiftDepositInfo(value) => value.td_name(),
      Self::UpgradedGiftValueInfo(value) => value.td_name(),
      Self::User(value) => value.td_name(),
      Self::UserFullInfo(value) => value.td_name(),
      Self::UserLink(value) => value.td_name(),
      Self::UserPrivacySettingRules(value) => value.td_name(),
      Self::UserSupportInfo(value) => value.td_name(),
      Self::UserTonWalletAddress(value) => value.td_name(),
      Self::UserTonWalletAddresses(value) => value.td_name(),
      Self::Users(value) => value.td_name(),
      Self::ValidatedOrderInfo(value) => value.td_name(),
      Self::VideoMessageAdvertisements(value) => value.td_name(),
      Self::WalletBotBalance(value) => value.td_name(),
      Self::WebAppInfo(value) => value.td_name(),
      Self::WebAppUrl(value) => value.td_name(),
      Self::WebPageInstantView(value) => value.td_name(),
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      Self::TestUseUpdate(value) => value.extra(),
      Self::UpdateAccentColors(value) => value.extra(),
      Self::UpdateActiveEmojiReactions(value) => value.extra(),
      Self::UpdateActiveGiftAuctions(value) => value.extra(),
      Self::UpdateActiveLiveLocationMessages(value) => value.extra(),
      Self::UpdateActiveNotifications(value) => value.extra(),
      Self::UpdateAgeVerificationParameters(value) => value.extra(),
      Self::UpdateAllowedBotUsernamePremiumSuffixes(value) => value.extra(),
      Self::UpdateAnimatedEmojiMessageClicked(value) => value.extra(),
      Self::UpdateAnimationSearchParameters(value) => value.extra(),
      Self::UpdateApplicationRecaptchaVerificationRequired(value) => value.extra(),
      Self::UpdateApplicationVerificationRequired(value) => value.extra(),
      Self::UpdateAttachmentMenuBots(value) => value.extra(),
      Self::UpdateAuthorizationState(value) => value.extra(),
      Self::UpdateAutosaveSettings(value) => value.extra(),
      Self::UpdateAvailableMessageEffects(value) => value.extra(),
      Self::UpdateBasicGroup(value) => value.extra(),
      Self::UpdateBasicGroupFullInfo(value) => value.extra(),
      Self::UpdateBusinessConnection(value) => value.extra(),
      Self::UpdateBusinessMessageEdited(value) => value.extra(),
      Self::UpdateBusinessMessagesDeleted(value) => value.extra(),
      Self::UpdateCall(value) => value.extra(),
      Self::UpdateChatAccentColors(value) => value.extra(),
      Self::UpdateChatAction(value) => value.extra(),
      Self::UpdateChatActionBar(value) => value.extra(),
      Self::UpdateChatActiveStories(value) => value.extra(),
      Self::UpdateChatAddedToList(value) => value.extra(),
      Self::UpdateChatAvailableReactions(value) => value.extra(),
      Self::UpdateChatBackground(value) => value.extra(),
      Self::UpdateChatBlockList(value) => value.extra(),
      Self::UpdateChatBoost(value) => value.extra(),
      Self::UpdateChatBusinessBotManageBar(value) => value.extra(),
      Self::UpdateChatDefaultDisableNotification(value) => value.extra(),
      Self::UpdateChatDraftMessage(value) => value.extra(),
      Self::UpdateChatEmojiStatus(value) => value.extra(),
      Self::UpdateChatFolders(value) => value.extra(),
      Self::UpdateChatHasProtectedContent(value) => value.extra(),
      Self::UpdateChatHasScheduledMessages(value) => value.extra(),
      Self::UpdateChatHasWelcomeMessages(value) => value.extra(),
      Self::UpdateChatIsMarkedAsUnread(value) => value.extra(),
      Self::UpdateChatIsTranslatable(value) => value.extra(),
      Self::UpdateChatJoinResult(value) => value.extra(),
      Self::UpdateChatLastMessage(value) => value.extra(),
      Self::UpdateChatMember(value) => value.extra(),
      Self::UpdateChatMessageAutoDeleteTime(value) => value.extra(),
      Self::UpdateChatMessageSender(value) => value.extra(),
      Self::UpdateChatNotificationSettings(value) => value.extra(),
      Self::UpdateChatOnlineMemberCount(value) => value.extra(),
      Self::UpdateChatPendingJoinRequests(value) => value.extra(),
      Self::UpdateChatPermissions(value) => value.extra(),
      Self::UpdateChatPhoto(value) => value.extra(),
      Self::UpdateChatPosition(value) => value.extra(),
      Self::UpdateChatReadInbox(value) => value.extra(),
      Self::UpdateChatReadOutbox(value) => value.extra(),
      Self::UpdateChatRemovedFromList(value) => value.extra(),
      Self::UpdateChatReplyMarkup(value) => value.extra(),
      Self::UpdateChatRevenueAmount(value) => value.extra(),
      Self::UpdateChatTheme(value) => value.extra(),
      Self::UpdateChatTitle(value) => value.extra(),
      Self::UpdateChatUnreadMentionCount(value) => value.extra(),
      Self::UpdateChatUnreadPollVoteCount(value) => value.extra(),
      Self::UpdateChatUnreadReactionCount(value) => value.extra(),
      Self::UpdateChatVideoChat(value) => value.extra(),
      Self::UpdateChatViewAsTopics(value) => value.extra(),
      Self::UpdateChatWelcomeMessages(value) => value.extra(),
      Self::UpdateCommunity(value) => value.extra(),
      Self::UpdateCommunityFullInfo(value) => value.extra(),
      Self::UpdateConnectionState(value) => value.extra(),
      Self::UpdateContactCloseBirthdays(value) => value.extra(),
      Self::UpdateDefaultBackground(value) => value.extra(),
      Self::UpdateDefaultPaidReactionType(value) => value.extra(),
      Self::UpdateDefaultReactionType(value) => value.extra(),
      Self::UpdateDeleteMessages(value) => value.extra(),
      Self::UpdateDiceEmojis(value) => value.extra(),
      Self::UpdateDirectMessagesChatTopic(value) => value.extra(),
      Self::UpdateEmojiChatThemes(value) => value.extra(),
      Self::UpdateFavoriteStickers(value) => value.extra(),
      Self::UpdateFile(value) => value.extra(),
      Self::UpdateFileAddedToDownloads(value) => value.extra(),
      Self::UpdateFileDownload(value) => value.extra(),
      Self::UpdateFileDownloads(value) => value.extra(),
      Self::UpdateFileGenerationStart(value) => value.extra(),
      Self::UpdateFileGenerationStop(value) => value.extra(),
      Self::UpdateFileRemovedFromDownloads(value) => value.extra(),
      Self::UpdateForumTopic(value) => value.extra(),
      Self::UpdateForumTopicInfo(value) => value.extra(),
      Self::UpdateFreezeState(value) => value.extra(),
      Self::UpdateGiftAuctionState(value) => value.extra(),
      Self::UpdateGramRevenueStatus(value) => value.extra(),
      Self::UpdateGroupCall(value) => value.extra(),
      Self::UpdateGroupCallMessageLevels(value) => value.extra(),
      Self::UpdateGroupCallMessageSendFailed(value) => value.extra(),
      Self::UpdateGroupCallMessagesDeleted(value) => value.extra(),
      Self::UpdateGroupCallParticipant(value) => value.extra(),
      Self::UpdateGroupCallParticipants(value) => value.extra(),
      Self::UpdateGroupCallVerificationState(value) => value.extra(),
      Self::UpdateHavePendingNotifications(value) => value.extra(),
      Self::UpdateInstalledStickerSets(value) => value.extra(),
      Self::UpdateLanguagePackStrings(value) => value.extra(),
      Self::UpdateLiveStoryTopDonors(value) => value.extra(),
      Self::UpdateManagedBot(value) => value.extra(),
      Self::UpdateMessageContainsUnreadPollVotes(value) => value.extra(),
      Self::UpdateMessageContent(value) => value.extra(),
      Self::UpdateMessageContentOpened(value) => value.extra(),
      Self::UpdateMessageEdited(value) => value.extra(),
      Self::UpdateMessageEphemeralContent(value) => value.extra(),
      Self::UpdateMessageFactCheck(value) => value.extra(),
      Self::UpdateMessageInteractionInfo(value) => value.extra(),
      Self::UpdateMessageIsPinned(value) => value.extra(),
      Self::UpdateMessageLiveLocationViewed(value) => value.extra(),
      Self::UpdateMessageMentionRead(value) => value.extra(),
      Self::UpdateMessageReaction(value) => value.extra(),
      Self::UpdateMessageReactions(value) => value.extra(),
      Self::UpdateMessageSendAcknowledged(value) => value.extra(),
      Self::UpdateMessageSendFailed(value) => value.extra(),
      Self::UpdateMessageSendSucceeded(value) => value.extra(),
      Self::UpdateMessageSuggestedPostInfo(value) => value.extra(),
      Self::UpdateMessageUnreadReactions(value) => value.extra(),
      Self::UpdateNewBusinessCallbackQuery(value) => value.extra(),
      Self::UpdateNewBusinessMessage(value) => value.extra(),
      Self::UpdateNewCallSignalingData(value) => value.extra(),
      Self::UpdateNewCallbackQuery(value) => value.extra(),
      Self::UpdateNewChat(value) => value.extra(),
      Self::UpdateNewChatJoinRequest(value) => value.extra(),
      Self::UpdateNewChosenInlineResult(value) => value.extra(),
      Self::UpdateNewCustomEvent(value) => value.extra(),
      Self::UpdateNewCustomQuery(value) => value.extra(),
      Self::UpdateNewGroupCallMessage(value) => value.extra(),
      Self::UpdateNewGroupCallPaidReaction(value) => value.extra(),
      Self::UpdateNewGuestQuery(value) => value.extra(),
      Self::UpdateNewInlineCallbackQuery(value) => value.extra(),
      Self::UpdateNewInlineQuery(value) => value.extra(),
      Self::UpdateNewMessage(value) => value.extra(),
      Self::UpdateNewOauthRequest(value) => value.extra(),
      Self::UpdateNewPreCheckoutQuery(value) => value.extra(),
      Self::UpdateNewShippingQuery(value) => value.extra(),
      Self::UpdateNotification(value) => value.extra(),
      Self::UpdateNotificationGroup(value) => value.extra(),
      Self::UpdateOption(value) => value.extra(),
      Self::UpdateOwnedGramCount(value) => value.extra(),
      Self::UpdateOwnedStarCount(value) => value.extra(),
      Self::UpdatePaidMediaPurchased(value) => value.extra(),
      Self::UpdatePendingMessage(value) => value.extra(),
      Self::UpdatePoll(value) => value.extra(),
      Self::UpdatePollAnswer(value) => value.extra(),
      Self::UpdateProfileAccentColors(value) => value.extra(),
      Self::UpdateQuickReplyShortcut(value) => value.extra(),
      Self::UpdateQuickReplyShortcutDeleted(value) => value.extra(),
      Self::UpdateQuickReplyShortcutMessages(value) => value.extra(),
      Self::UpdateQuickReplyShortcuts(value) => value.extra(),
      Self::UpdateReactionNotificationSettings(value) => value.extra(),
      Self::UpdateRecentStickers(value) => value.extra(),
      Self::UpdateSavedAnimations(value) => value.extra(),
      Self::UpdateSavedMessagesTags(value) => value.extra(),
      Self::UpdateSavedMessagesTopic(value) => value.extra(),
      Self::UpdateSavedMessagesTopicCount(value) => value.extra(),
      Self::UpdateSavedNotificationSounds(value) => value.extra(),
      Self::UpdateScopeNotificationSettings(value) => value.extra(),
      Self::UpdateSecretChat(value) => value.extra(),
      Self::UpdateServiceNotification(value) => value.extra(),
      Self::UpdateSpeechRecognitionTrial(value) => value.extra(),
      Self::UpdateSpeedLimitNotification(value) => value.extra(),
      Self::UpdateStakeDiceState(value) => value.extra(),
      Self::UpdateStarRevenueStatus(value) => value.extra(),
      Self::UpdateStickerSet(value) => value.extra(),
      Self::UpdateStopMessageDraft(value) => value.extra(),
      Self::UpdateStory(value) => value.extra(),
      Self::UpdateStoryDeleted(value) => value.extra(),
      Self::UpdateStoryListChatCount(value) => value.extra(),
      Self::UpdateStoryPostFailed(value) => value.extra(),
      Self::UpdateStoryPostSucceeded(value) => value.extra(),
      Self::UpdateStoryStealthMode(value) => value.extra(),
      Self::UpdateSuggestedActions(value) => value.extra(),
      Self::UpdateSupergroup(value) => value.extra(),
      Self::UpdateSupergroupFullInfo(value) => value.extra(),
      Self::UpdateTermsOfService(value) => value.extra(),
      Self::UpdateTextCompositionStyles(value) => value.extra(),
      Self::UpdateTonWalletGaslessTransfersInfo(value) => value.extra(),
      Self::UpdateTonWalletState(value) => value.extra(),
      Self::UpdateTonWalletTonConnectSession(value) => value.extra(),
      Self::UpdateTonWalletTonConnectSessionDisconnectRequired(value) => value.extra(),
      Self::UpdateTopicMessageCount(value) => value.extra(),
      Self::UpdateTrendingStickerSets(value) => value.extra(),
      Self::UpdateTrustedMiniAppBots(value) => value.extra(),
      Self::UpdateUnconfirmedSession(value) => value.extra(),
      Self::UpdateUnreadChatCount(value) => value.extra(),
      Self::UpdateUnreadMessageCount(value) => value.extra(),
      Self::UpdateUser(value) => value.extra(),
      Self::UpdateUserFullInfo(value) => value.extra(),
      Self::UpdateUserPrivacySettingRules(value) => value.extra(),
      Self::UpdateUserStatus(value) => value.extra(),
      Self::UpdateUserSubscription(value) => value.extra(),
      Self::UpdateVideoPublished(value) => value.extra(),
      Self::UpdateWebAppMessageSent(value) => value.extra(),
      Self::UpdateWebBrowserSettings(value) => value.extra(),
      Self::UpdateWebProxyCarrierClosed(value) => value.extra(),
      Self::UpdateWebProxyCarrierData(value) => value.extra(),
      Self::UpdateWebProxyCarrierOpened(value) => value.extra(),

      Self::AuthorizationState(value) => value.extra(),
      Self::CanPostStoryResult(value) => value.extra(),
      Self::CanSendGiftResult(value) => value.extra(),
      Self::CanSendMessageToUserResult(value) => value.extra(),
      Self::CanTransferOwnershipResult(value) => value.extra(),
      Self::ChatJoinResult(value) => value.extra(),
      Self::ChatStatistics(value) => value.extra(),
      Self::CheckChatUsernameResult(value) => value.extra(),
      Self::CheckStickerSetNameResult(value) => value.extra(),
      Self::CraftGiftResult(value) => value.extra(),
      Self::GiftResaleResult(value) => value.extra(),
      Self::GiveawayInfo(value) => value.extra(),
      Self::InternalLinkType(value) => value.extra(),
      Self::InviteGroupCallParticipantResult(value) => value.extra(),
      Self::JsonValue(value) => value.extra(),
      Self::LanguagePackStringValue(value) => value.extra(),
      Self::LogStream(value) => value.extra(),
      Self::LoginUrlInfo(value) => value.extra(),
      Self::MessageFileType(value) => value.extra(),
      Self::MessageReadDate(value) => value.extra(),
      Self::MessageSender(value) => value.extra(),
      Self::OptionValue(value) => value.extra(),
      Self::PassportElement(value) => value.extra(),
      Self::ReportChatResult(value) => value.extra(),
      Self::ReportSponsoredResult(value) => value.extra(),
      Self::ReportStoryResult(value) => value.extra(),
      Self::ResetPasswordResult(value) => value.extra(),
      Self::StartLiveStoryResult(value) => value.extra(),
      Self::StatisticalGraph(value) => value.extra(),
      Self::Update(value) => value.extra(),
      Self::WebBrowserType(value) => value.extra(),
      Self::AccountTtl(value) => value.extra(),
      Self::AddedProxies(value) => value.extra(),
      Self::AddedProxy(value) => value.extra(),
      Self::AddedReactions(value) => value.extra(),
      Self::AnimatedEmoji(value) => value.extra(),
      Self::Animation(value) => value.extra(),
      Self::Animations(value) => value.extra(),
      Self::ArchiveChatListSettings(value) => value.extra(),
      Self::AttachmentMenuBot(value) => value.extra(),
      Self::Audios(value) => value.extra(),
      Self::AuthenticationCodeInfo(value) => value.extra(),
      Self::AutoDownloadSettingsPresets(value) => value.extra(),
      Self::AutosaveSettings(value) => value.extra(),
      Self::AvailableGifts(value) => value.extra(),
      Self::AvailableReactions(value) => value.extra(),
      Self::Background(value) => value.extra(),
      Self::Backgrounds(value) => value.extra(),
      Self::BankCardInfo(value) => value.extra(),
      Self::BasicGroup(value) => value.extra(),
      Self::BasicGroupFullInfo(value) => value.extra(),
      Self::BotAccessSettings(value) => value.extra(),
      Self::BotCommands(value) => value.extra(),
      Self::BotMediaPreview(value) => value.extra(),
      Self::BotMediaPreviewInfo(value) => value.extra(),
      Self::BotMediaPreviews(value) => value.extra(),
      Self::BotMenuButton(value) => value.extra(),
      Self::BusinessChatLink(value) => value.extra(),
      Self::BusinessChatLinkInfo(value) => value.extra(),
      Self::BusinessChatLinks(value) => value.extra(),
      Self::BusinessConnectedBotInfo(value) => value.extra(),
      Self::BusinessConnection(value) => value.extra(),
      Self::BusinessFeatures(value) => value.extra(),
      Self::BusinessMessage(value) => value.extra(),
      Self::BusinessMessages(value) => value.extra(),
      Self::CallId(value) => value.extra(),
      Self::CallbackQueryAnswer(value) => value.extra(),
      Self::Chat(value) => value.extra(),
      Self::ChatActiveStories(value) => value.extra(),
      Self::ChatAdministrators(value) => value.extra(),
      Self::ChatBoostFeatures(value) => value.extra(),
      Self::ChatBoostLevelFeatures(value) => value.extra(),
      Self::ChatBoostLink(value) => value.extra(),
      Self::ChatBoostLinkInfo(value) => value.extra(),
      Self::ChatBoostSlots(value) => value.extra(),
      Self::ChatBoostStatus(value) => value.extra(),
      Self::ChatEvents(value) => value.extra(),
      Self::ChatFolder(value) => value.extra(),
      Self::ChatFolderIcon(value) => value.extra(),
      Self::ChatFolderInfo(value) => value.extra(),
      Self::ChatFolderInviteLink(value) => value.extra(),
      Self::ChatFolderInviteLinkInfo(value) => value.extra(),
      Self::ChatFolderInviteLinks(value) => value.extra(),
      Self::ChatInviteLink(value) => value.extra(),
      Self::ChatInviteLinkCounts(value) => value.extra(),
      Self::ChatInviteLinkInfo(value) => value.extra(),
      Self::ChatInviteLinkMembers(value) => value.extra(),
      Self::ChatInviteLinks(value) => value.extra(),
      Self::ChatJoinRequests(value) => value.extra(),
      Self::ChatLists(value) => value.extra(),
      Self::ChatMember(value) => value.extra(),
      Self::ChatMembers(value) => value.extra(),
      Self::ChatMessageSenders(value) => value.extra(),
      Self::ChatPhotos(value) => value.extra(),
      Self::ChatRevenueStatistics(value) => value.extra(),
      Self::ChatRevenueTransactions(value) => value.extra(),
      Self::Chats(value) => value.extra(),
      Self::CollectibleItemInfo(value) => value.extra(),
      Self::CommunityFullInfo(value) => value.extra(),
      Self::CommunityId(value) => value.extra(),
      Self::ConnectedAffiliateProgram(value) => value.extra(),
      Self::ConnectedAffiliatePrograms(value) => value.extra(),
      Self::ConnectedWebsites(value) => value.extra(),
      Self::Count(value) => value.extra(),
      Self::Countries(value) => value.extra(),
      Self::CountryInfo(value) => value.extra(),
      Self::CreatedBasicGroupChat(value) => value.extra(),
      Self::Currencies(value) => value.extra(),
      Self::CurrencyExchangeRates(value) => value.extra(),
      Self::CurrentWeather(value) => value.extra(),
      Self::CustomRequestResult(value) => value.extra(),
      Self::Data(value) => value.extra(),
      Self::DatabaseStatistics(value) => value.extra(),
      Self::DeepLinkInfo(value) => value.extra(),
      Self::DirectMessagesChatTopic(value) => value.extra(),
      Self::EmailAddressAuthenticationCodeInfo(value) => value.extra(),
      Self::EmojiCategories(value) => value.extra(),
      Self::EmojiKeywords(value) => value.extra(),
      Self::EmojiReaction(value) => value.extra(),
      Self::EmojiStatusCustomEmojis(value) => value.extra(),
      Self::EmojiStatuses(value) => value.extra(),
      Self::Emojis(value) => value.extra(),
      Self::Error(value) => value.extra(),
      Self::FailedToAddMembers(value) => value.extra(),
      Self::File(value) => value.extra(),
      Self::FileDownloadedPrefixSize(value) => value.extra(),
      Self::FixedText(value) => value.extra(),
      Self::FormattedText(value) => value.extra(),
      Self::ForumTopic(value) => value.extra(),
      Self::ForumTopicInfo(value) => value.extra(),
      Self::ForumTopics(value) => value.extra(),
      Self::FoundAffiliatePrograms(value) => value.extra(),
      Self::FoundChatBoosts(value) => value.extra(),
      Self::FoundChatMessages(value) => value.extra(),
      Self::FoundFileDownloads(value) => value.extra(),
      Self::FoundMessages(value) => value.extra(),
      Self::FoundPosition(value) => value.extra(),
      Self::FoundPositions(value) => value.extra(),
      Self::FoundPublicPosts(value) => value.extra(),
      Self::FoundStories(value) => value.extra(),
      Self::FoundUsers(value) => value.extra(),
      Self::FoundWebApp(value) => value.extra(),
      Self::GameHighScores(value) => value.extra(),
      Self::GiftAuctionAcquiredGifts(value) => value.extra(),
      Self::GiftAuctionState(value) => value.extra(),
      Self::GiftChatThemes(value) => value.extra(),
      Self::GiftCollection(value) => value.extra(),
      Self::GiftCollections(value) => value.extra(),
      Self::GiftUpgradePreview(value) => value.extra(),
      Self::GiftUpgradeVariants(value) => value.extra(),
      Self::GiftsForCrafting(value) => value.extra(),
      Self::GiftsForResale(value) => value.extra(),
      Self::GramRevenueStatistics(value) => value.extra(),
      Self::GroupCall(value) => value.extra(),
      Self::GroupCallId(value) => value.extra(),
      Self::GroupCallInfo(value) => value.extra(),
      Self::GroupCallParticipant(value) => value.extra(),
      Self::GroupCallParticipants(value) => value.extra(),
      Self::GroupCallStreams(value) => value.extra(),
      Self::Hashtags(value) => value.extra(),
      Self::HttpUrl(value) => value.extra(),
      Self::ImportedContacts(value) => value.extra(),
      Self::InlineMessageId(value) => value.extra(),
      Self::InlineQueryResults(value) => value.extra(),
      Self::KeyboardButton(value) => value.extra(),
      Self::LanguagePackInfo(value) => value.extra(),
      Self::LanguagePackStrings(value) => value.extra(),
      Self::LinkPreview(value) => value.extra(),
      Self::LiveStoryDonors(value) => value.extra(),
      Self::LocalizationTargetInfo(value) => value.extra(),
      Self::LogTags(value) => value.extra(),
      Self::LogVerbosityLevel(value) => value.extra(),
      Self::MainWebApp(value) => value.extra(),
      Self::Message(value) => value.extra(),
      Self::MessageAutoDeleteTime(value) => value.extra(),
      Self::MessageCalendar(value) => value.extra(),
      Self::MessageEffect(value) => value.extra(),
      Self::MessageLink(value) => value.extra(),
      Self::MessageLinkInfo(value) => value.extra(),
      Self::MessagePositions(value) => value.extra(),
      Self::MessageProperties(value) => value.extra(),
      Self::MessageSenders(value) => value.extra(),
      Self::MessageStatistics(value) => value.extra(),
      Self::MessageThreadInfo(value) => value.extra(),
      Self::MessageViewers(value) => value.extra(),
      Self::Messages(value) => value.extra(),
      Self::NetworkStatistics(value) => value.extra(),
      Self::NewChatPrivacySettings(value) => value.extra(),
      Self::NotificationSound(value) => value.extra(),
      Self::NotificationSounds(value) => value.extra(),
      Self::OauthLinkInfo(value) => value.extra(),
      Self::Ok(value) => value.extra(),
      Self::OnRampPaymentAvailability(value) => value.extra(),
      Self::OnRampPaymentLimits(value) => value.extra(),
      Self::OnRampPaymentQuote(value) => value.extra(),
      Self::OnRampPaymentSession(value) => value.extra(),
      Self::OnRampProviders(value) => value.extra(),
      Self::OrderInfo(value) => value.extra(),
      Self::Outline(value) => value.extra(),
      Self::Passkey(value) => value.extra(),
      Self::Passkeys(value) => value.extra(),
      Self::PassportAuthorizationForm(value) => value.extra(),
      Self::PassportElements(value) => value.extra(),
      Self::PassportElementsWithErrors(value) => value.extra(),
      Self::PasswordState(value) => value.extra(),
      Self::PaymentForm(value) => value.extra(),
      Self::PaymentReceipt(value) => value.extra(),
      Self::PaymentResult(value) => value.extra(),
      Self::PhoneNumberInfo(value) => value.extra(),
      Self::PollOptionProperties(value) => value.extra(),
      Self::PollVoteStatistics(value) => value.extra(),
      Self::PollVoters(value) => value.extra(),
      Self::PremiumFeatures(value) => value.extra(),
      Self::PremiumGiftCodeInfo(value) => value.extra(),
      Self::PremiumGiftPaymentOptions(value) => value.extra(),
      Self::PremiumGiveawayPaymentOptions(value) => value.extra(),
      Self::PremiumLimit(value) => value.extra(),
      Self::PremiumState(value) => value.extra(),
      Self::PreparedInlineMessage(value) => value.extra(),
      Self::PreparedInlineMessageId(value) => value.extra(),
      Self::ProfileShareThemeBackdrops(value) => value.extra(),
      Self::PublicForwards(value) => value.extra(),
      Self::PublicPostSearchLimits(value) => value.extra(),
      Self::PushReceiverId(value) => value.extra(),
      Self::QuickReplyMessage(value) => value.extra(),
      Self::QuickReplyMessages(value) => value.extra(),
      Self::ReadDatePrivacySettings(value) => value.extra(),
      Self::ReceivedGift(value) => value.extra(),
      Self::ReceivedGifts(value) => value.extra(),
      Self::RecommendedChatFolders(value) => value.extra(),
      Self::RecoveryEmailAddress(value) => value.extra(),
      Self::RichMessage(value) => value.extra(),
      Self::RtmpUrl(value) => value.extra(),
      Self::SavedMessagesTags(value) => value.extra(),
      Self::ScopeNotificationSettings(value) => value.extra(),
      Self::Seconds(value) => value.extra(),
      Self::SecretChat(value) => value.extra(),
      Self::Session(value) => value.extra(),
      Self::Sessions(value) => value.extra(),
      Self::SponsoredChats(value) => value.extra(),
      Self::SponsoredMessages(value) => value.extra(),
      Self::StakeDiceState(value) => value.extra(),
      Self::StarAmount(value) => value.extra(),
      Self::StarCount(value) => value.extra(),
      Self::StarGiveawayPaymentOptions(value) => value.extra(),
      Self::StarPaymentOptions(value) => value.extra(),
      Self::StarRevenueStatistics(value) => value.extra(),
      Self::StarSubscriptions(value) => value.extra(),
      Self::StarTransactions(value) => value.extra(),
      Self::Sticker(value) => value.extra(),
      Self::StickerSet(value) => value.extra(),
      Self::StickerSets(value) => value.extra(),
      Self::Stickers(value) => value.extra(),
      Self::StorageStatistics(value) => value.extra(),
      Self::StorageStatisticsFast(value) => value.extra(),
      Self::Stories(value) => value.extra(),
      Self::Story(value) => value.extra(),
      Self::StoryAlbum(value) => value.extra(),
      Self::StoryAlbums(value) => value.extra(),
      Self::StoryInteractions(value) => value.extra(),
      Self::StoryStatistics(value) => value.extra(),
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
      Self::TextCompositionStyle(value) => value.extra(),
      Self::TextCompositionStyleExample(value) => value.extra(),
      Self::TextEntities(value) => value.extra(),
      Self::TimeZones(value) => value.extra(),
      Self::TonCenterStreamingApiUrl(value) => value.extra(),
      Self::TonConnectChallenge(value) => value.extra(),
      Self::TonConnectRequests(value) => value.extra(),
      Self::TonConnectSession(value) => value.extra(),
      Self::TonConnectSessionEventId(value) => value.extra(),
      Self::TonConnectSessions(value) => value.extra(),
      Self::TonNfts(value) => value.extra(),
      Self::TonTransactions(value) => value.extra(),
      Self::TonWalletCard(value) => value.extra(),
      Self::TonWalletOwnershipProofChallenge(value) => value.extra(),
      Self::TonWalletSecretPhrase(value) => value.extra(),
      Self::TonWalletTransaction(value) => value.extra(),
      Self::TonWalletTransactions(value) => value.extra(),
      Self::TonWalletTransferResult(value) => value.extra(),
      Self::TrendingStickerSets(value) => value.extra(),
      Self::Updates(value) => value.extra(),
      Self::UpgradeGiftResult(value) => value.extra(),
      Self::UpgradedGift(value) => value.extra(),
      Self::UpgradedGiftCollectionNames(value) => value.extra(),
      Self::UpgradedGiftDepositInfo(value) => value.extra(),
      Self::UpgradedGiftValueInfo(value) => value.extra(),
      Self::User(value) => value.extra(),
      Self::UserFullInfo(value) => value.extra(),
      Self::UserLink(value) => value.extra(),
      Self::UserPrivacySettingRules(value) => value.extra(),
      Self::UserSupportInfo(value) => value.extra(),
      Self::UserTonWalletAddress(value) => value.extra(),
      Self::UserTonWalletAddresses(value) => value.extra(),
      Self::Users(value) => value.extra(),
      Self::ValidatedOrderInfo(value) => value.extra(),
      Self::VideoMessageAdvertisements(value) => value.extra(),
      Self::WalletBotBalance(value) => value.extra(),
      Self::WebAppInfo(value) => value.extra(),
      Self::WebAppUrl(value) => value.extra(),
      Self::WebPageInstantView(value) => value.extra(),
    }
  }
  /// Return td type to json string
  fn to_json(&self) -> RTDResult<String> {
    match self {
      Self::TestUseUpdate(value) => value.to_json(),
      Self::UpdateAccentColors(value) => value.to_json(),
      Self::UpdateActiveEmojiReactions(value) => value.to_json(),
      Self::UpdateActiveGiftAuctions(value) => value.to_json(),
      Self::UpdateActiveLiveLocationMessages(value) => value.to_json(),
      Self::UpdateActiveNotifications(value) => value.to_json(),
      Self::UpdateAgeVerificationParameters(value) => value.to_json(),
      Self::UpdateAllowedBotUsernamePremiumSuffixes(value) => value.to_json(),
      Self::UpdateAnimatedEmojiMessageClicked(value) => value.to_json(),
      Self::UpdateAnimationSearchParameters(value) => value.to_json(),
      Self::UpdateApplicationRecaptchaVerificationRequired(value) => value.to_json(),
      Self::UpdateApplicationVerificationRequired(value) => value.to_json(),
      Self::UpdateAttachmentMenuBots(value) => value.to_json(),
      Self::UpdateAuthorizationState(value) => value.to_json(),
      Self::UpdateAutosaveSettings(value) => value.to_json(),
      Self::UpdateAvailableMessageEffects(value) => value.to_json(),
      Self::UpdateBasicGroup(value) => value.to_json(),
      Self::UpdateBasicGroupFullInfo(value) => value.to_json(),
      Self::UpdateBusinessConnection(value) => value.to_json(),
      Self::UpdateBusinessMessageEdited(value) => value.to_json(),
      Self::UpdateBusinessMessagesDeleted(value) => value.to_json(),
      Self::UpdateCall(value) => value.to_json(),
      Self::UpdateChatAccentColors(value) => value.to_json(),
      Self::UpdateChatAction(value) => value.to_json(),
      Self::UpdateChatActionBar(value) => value.to_json(),
      Self::UpdateChatActiveStories(value) => value.to_json(),
      Self::UpdateChatAddedToList(value) => value.to_json(),
      Self::UpdateChatAvailableReactions(value) => value.to_json(),
      Self::UpdateChatBackground(value) => value.to_json(),
      Self::UpdateChatBlockList(value) => value.to_json(),
      Self::UpdateChatBoost(value) => value.to_json(),
      Self::UpdateChatBusinessBotManageBar(value) => value.to_json(),
      Self::UpdateChatDefaultDisableNotification(value) => value.to_json(),
      Self::UpdateChatDraftMessage(value) => value.to_json(),
      Self::UpdateChatEmojiStatus(value) => value.to_json(),
      Self::UpdateChatFolders(value) => value.to_json(),
      Self::UpdateChatHasProtectedContent(value) => value.to_json(),
      Self::UpdateChatHasScheduledMessages(value) => value.to_json(),
      Self::UpdateChatHasWelcomeMessages(value) => value.to_json(),
      Self::UpdateChatIsMarkedAsUnread(value) => value.to_json(),
      Self::UpdateChatIsTranslatable(value) => value.to_json(),
      Self::UpdateChatJoinResult(value) => value.to_json(),
      Self::UpdateChatLastMessage(value) => value.to_json(),
      Self::UpdateChatMember(value) => value.to_json(),
      Self::UpdateChatMessageAutoDeleteTime(value) => value.to_json(),
      Self::UpdateChatMessageSender(value) => value.to_json(),
      Self::UpdateChatNotificationSettings(value) => value.to_json(),
      Self::UpdateChatOnlineMemberCount(value) => value.to_json(),
      Self::UpdateChatPendingJoinRequests(value) => value.to_json(),
      Self::UpdateChatPermissions(value) => value.to_json(),
      Self::UpdateChatPhoto(value) => value.to_json(),
      Self::UpdateChatPosition(value) => value.to_json(),
      Self::UpdateChatReadInbox(value) => value.to_json(),
      Self::UpdateChatReadOutbox(value) => value.to_json(),
      Self::UpdateChatRemovedFromList(value) => value.to_json(),
      Self::UpdateChatReplyMarkup(value) => value.to_json(),
      Self::UpdateChatRevenueAmount(value) => value.to_json(),
      Self::UpdateChatTheme(value) => value.to_json(),
      Self::UpdateChatTitle(value) => value.to_json(),
      Self::UpdateChatUnreadMentionCount(value) => value.to_json(),
      Self::UpdateChatUnreadPollVoteCount(value) => value.to_json(),
      Self::UpdateChatUnreadReactionCount(value) => value.to_json(),
      Self::UpdateChatVideoChat(value) => value.to_json(),
      Self::UpdateChatViewAsTopics(value) => value.to_json(),
      Self::UpdateChatWelcomeMessages(value) => value.to_json(),
      Self::UpdateCommunity(value) => value.to_json(),
      Self::UpdateCommunityFullInfo(value) => value.to_json(),
      Self::UpdateConnectionState(value) => value.to_json(),
      Self::UpdateContactCloseBirthdays(value) => value.to_json(),
      Self::UpdateDefaultBackground(value) => value.to_json(),
      Self::UpdateDefaultPaidReactionType(value) => value.to_json(),
      Self::UpdateDefaultReactionType(value) => value.to_json(),
      Self::UpdateDeleteMessages(value) => value.to_json(),
      Self::UpdateDiceEmojis(value) => value.to_json(),
      Self::UpdateDirectMessagesChatTopic(value) => value.to_json(),
      Self::UpdateEmojiChatThemes(value) => value.to_json(),
      Self::UpdateFavoriteStickers(value) => value.to_json(),
      Self::UpdateFile(value) => value.to_json(),
      Self::UpdateFileAddedToDownloads(value) => value.to_json(),
      Self::UpdateFileDownload(value) => value.to_json(),
      Self::UpdateFileDownloads(value) => value.to_json(),
      Self::UpdateFileGenerationStart(value) => value.to_json(),
      Self::UpdateFileGenerationStop(value) => value.to_json(),
      Self::UpdateFileRemovedFromDownloads(value) => value.to_json(),
      Self::UpdateForumTopic(value) => value.to_json(),
      Self::UpdateForumTopicInfo(value) => value.to_json(),
      Self::UpdateFreezeState(value) => value.to_json(),
      Self::UpdateGiftAuctionState(value) => value.to_json(),
      Self::UpdateGramRevenueStatus(value) => value.to_json(),
      Self::UpdateGroupCall(value) => value.to_json(),
      Self::UpdateGroupCallMessageLevels(value) => value.to_json(),
      Self::UpdateGroupCallMessageSendFailed(value) => value.to_json(),
      Self::UpdateGroupCallMessagesDeleted(value) => value.to_json(),
      Self::UpdateGroupCallParticipant(value) => value.to_json(),
      Self::UpdateGroupCallParticipants(value) => value.to_json(),
      Self::UpdateGroupCallVerificationState(value) => value.to_json(),
      Self::UpdateHavePendingNotifications(value) => value.to_json(),
      Self::UpdateInstalledStickerSets(value) => value.to_json(),
      Self::UpdateLanguagePackStrings(value) => value.to_json(),
      Self::UpdateLiveStoryTopDonors(value) => value.to_json(),
      Self::UpdateManagedBot(value) => value.to_json(),
      Self::UpdateMessageContainsUnreadPollVotes(value) => value.to_json(),
      Self::UpdateMessageContent(value) => value.to_json(),
      Self::UpdateMessageContentOpened(value) => value.to_json(),
      Self::UpdateMessageEdited(value) => value.to_json(),
      Self::UpdateMessageEphemeralContent(value) => value.to_json(),
      Self::UpdateMessageFactCheck(value) => value.to_json(),
      Self::UpdateMessageInteractionInfo(value) => value.to_json(),
      Self::UpdateMessageIsPinned(value) => value.to_json(),
      Self::UpdateMessageLiveLocationViewed(value) => value.to_json(),
      Self::UpdateMessageMentionRead(value) => value.to_json(),
      Self::UpdateMessageReaction(value) => value.to_json(),
      Self::UpdateMessageReactions(value) => value.to_json(),
      Self::UpdateMessageSendAcknowledged(value) => value.to_json(),
      Self::UpdateMessageSendFailed(value) => value.to_json(),
      Self::UpdateMessageSendSucceeded(value) => value.to_json(),
      Self::UpdateMessageSuggestedPostInfo(value) => value.to_json(),
      Self::UpdateMessageUnreadReactions(value) => value.to_json(),
      Self::UpdateNewBusinessCallbackQuery(value) => value.to_json(),
      Self::UpdateNewBusinessMessage(value) => value.to_json(),
      Self::UpdateNewCallSignalingData(value) => value.to_json(),
      Self::UpdateNewCallbackQuery(value) => value.to_json(),
      Self::UpdateNewChat(value) => value.to_json(),
      Self::UpdateNewChatJoinRequest(value) => value.to_json(),
      Self::UpdateNewChosenInlineResult(value) => value.to_json(),
      Self::UpdateNewCustomEvent(value) => value.to_json(),
      Self::UpdateNewCustomQuery(value) => value.to_json(),
      Self::UpdateNewGroupCallMessage(value) => value.to_json(),
      Self::UpdateNewGroupCallPaidReaction(value) => value.to_json(),
      Self::UpdateNewGuestQuery(value) => value.to_json(),
      Self::UpdateNewInlineCallbackQuery(value) => value.to_json(),
      Self::UpdateNewInlineQuery(value) => value.to_json(),
      Self::UpdateNewMessage(value) => value.to_json(),
      Self::UpdateNewOauthRequest(value) => value.to_json(),
      Self::UpdateNewPreCheckoutQuery(value) => value.to_json(),
      Self::UpdateNewShippingQuery(value) => value.to_json(),
      Self::UpdateNotification(value) => value.to_json(),
      Self::UpdateNotificationGroup(value) => value.to_json(),
      Self::UpdateOption(value) => value.to_json(),
      Self::UpdateOwnedGramCount(value) => value.to_json(),
      Self::UpdateOwnedStarCount(value) => value.to_json(),
      Self::UpdatePaidMediaPurchased(value) => value.to_json(),
      Self::UpdatePendingMessage(value) => value.to_json(),
      Self::UpdatePoll(value) => value.to_json(),
      Self::UpdatePollAnswer(value) => value.to_json(),
      Self::UpdateProfileAccentColors(value) => value.to_json(),
      Self::UpdateQuickReplyShortcut(value) => value.to_json(),
      Self::UpdateQuickReplyShortcutDeleted(value) => value.to_json(),
      Self::UpdateQuickReplyShortcutMessages(value) => value.to_json(),
      Self::UpdateQuickReplyShortcuts(value) => value.to_json(),
      Self::UpdateReactionNotificationSettings(value) => value.to_json(),
      Self::UpdateRecentStickers(value) => value.to_json(),
      Self::UpdateSavedAnimations(value) => value.to_json(),
      Self::UpdateSavedMessagesTags(value) => value.to_json(),
      Self::UpdateSavedMessagesTopic(value) => value.to_json(),
      Self::UpdateSavedMessagesTopicCount(value) => value.to_json(),
      Self::UpdateSavedNotificationSounds(value) => value.to_json(),
      Self::UpdateScopeNotificationSettings(value) => value.to_json(),
      Self::UpdateSecretChat(value) => value.to_json(),
      Self::UpdateServiceNotification(value) => value.to_json(),
      Self::UpdateSpeechRecognitionTrial(value) => value.to_json(),
      Self::UpdateSpeedLimitNotification(value) => value.to_json(),
      Self::UpdateStakeDiceState(value) => value.to_json(),
      Self::UpdateStarRevenueStatus(value) => value.to_json(),
      Self::UpdateStickerSet(value) => value.to_json(),
      Self::UpdateStopMessageDraft(value) => value.to_json(),
      Self::UpdateStory(value) => value.to_json(),
      Self::UpdateStoryDeleted(value) => value.to_json(),
      Self::UpdateStoryListChatCount(value) => value.to_json(),
      Self::UpdateStoryPostFailed(value) => value.to_json(),
      Self::UpdateStoryPostSucceeded(value) => value.to_json(),
      Self::UpdateStoryStealthMode(value) => value.to_json(),
      Self::UpdateSuggestedActions(value) => value.to_json(),
      Self::UpdateSupergroup(value) => value.to_json(),
      Self::UpdateSupergroupFullInfo(value) => value.to_json(),
      Self::UpdateTermsOfService(value) => value.to_json(),
      Self::UpdateTextCompositionStyles(value) => value.to_json(),
      Self::UpdateTonWalletGaslessTransfersInfo(value) => value.to_json(),
      Self::UpdateTonWalletState(value) => value.to_json(),
      Self::UpdateTonWalletTonConnectSession(value) => value.to_json(),
      Self::UpdateTonWalletTonConnectSessionDisconnectRequired(value) => value.to_json(),
      Self::UpdateTopicMessageCount(value) => value.to_json(),
      Self::UpdateTrendingStickerSets(value) => value.to_json(),
      Self::UpdateTrustedMiniAppBots(value) => value.to_json(),
      Self::UpdateUnconfirmedSession(value) => value.to_json(),
      Self::UpdateUnreadChatCount(value) => value.to_json(),
      Self::UpdateUnreadMessageCount(value) => value.to_json(),
      Self::UpdateUser(value) => value.to_json(),
      Self::UpdateUserFullInfo(value) => value.to_json(),
      Self::UpdateUserPrivacySettingRules(value) => value.to_json(),
      Self::UpdateUserStatus(value) => value.to_json(),
      Self::UpdateUserSubscription(value) => value.to_json(),
      Self::UpdateVideoPublished(value) => value.to_json(),
      Self::UpdateWebAppMessageSent(value) => value.to_json(),
      Self::UpdateWebBrowserSettings(value) => value.to_json(),
      Self::UpdateWebProxyCarrierClosed(value) => value.to_json(),
      Self::UpdateWebProxyCarrierData(value) => value.to_json(),
      Self::UpdateWebProxyCarrierOpened(value) => value.to_json(),

      Self::AuthorizationState(value) => value.to_json(),
      Self::CanPostStoryResult(value) => value.to_json(),
      Self::CanSendGiftResult(value) => value.to_json(),
      Self::CanSendMessageToUserResult(value) => value.to_json(),
      Self::CanTransferOwnershipResult(value) => value.to_json(),
      Self::ChatJoinResult(value) => value.to_json(),
      Self::ChatStatistics(value) => value.to_json(),
      Self::CheckChatUsernameResult(value) => value.to_json(),
      Self::CheckStickerSetNameResult(value) => value.to_json(),
      Self::CraftGiftResult(value) => value.to_json(),
      Self::GiftResaleResult(value) => value.to_json(),
      Self::GiveawayInfo(value) => value.to_json(),
      Self::InternalLinkType(value) => value.to_json(),
      Self::InviteGroupCallParticipantResult(value) => value.to_json(),
      Self::JsonValue(value) => value.to_json(),
      Self::LanguagePackStringValue(value) => value.to_json(),
      Self::LogStream(value) => value.to_json(),
      Self::LoginUrlInfo(value) => value.to_json(),
      Self::MessageFileType(value) => value.to_json(),
      Self::MessageReadDate(value) => value.to_json(),
      Self::MessageSender(value) => value.to_json(),
      Self::OptionValue(value) => value.to_json(),
      Self::PassportElement(value) => value.to_json(),
      Self::ReportChatResult(value) => value.to_json(),
      Self::ReportSponsoredResult(value) => value.to_json(),
      Self::ReportStoryResult(value) => value.to_json(),
      Self::ResetPasswordResult(value) => value.to_json(),
      Self::StartLiveStoryResult(value) => value.to_json(),
      Self::StatisticalGraph(value) => value.to_json(),
      Self::Update(value) => value.to_json(),
      Self::WebBrowserType(value) => value.to_json(),
      Self::AccountTtl(value) => value.to_json(),
      Self::AddedProxies(value) => value.to_json(),
      Self::AddedProxy(value) => value.to_json(),
      Self::AddedReactions(value) => value.to_json(),
      Self::AnimatedEmoji(value) => value.to_json(),
      Self::Animation(value) => value.to_json(),
      Self::Animations(value) => value.to_json(),
      Self::ArchiveChatListSettings(value) => value.to_json(),
      Self::AttachmentMenuBot(value) => value.to_json(),
      Self::Audios(value) => value.to_json(),
      Self::AuthenticationCodeInfo(value) => value.to_json(),
      Self::AutoDownloadSettingsPresets(value) => value.to_json(),
      Self::AutosaveSettings(value) => value.to_json(),
      Self::AvailableGifts(value) => value.to_json(),
      Self::AvailableReactions(value) => value.to_json(),
      Self::Background(value) => value.to_json(),
      Self::Backgrounds(value) => value.to_json(),
      Self::BankCardInfo(value) => value.to_json(),
      Self::BasicGroup(value) => value.to_json(),
      Self::BasicGroupFullInfo(value) => value.to_json(),
      Self::BotAccessSettings(value) => value.to_json(),
      Self::BotCommands(value) => value.to_json(),
      Self::BotMediaPreview(value) => value.to_json(),
      Self::BotMediaPreviewInfo(value) => value.to_json(),
      Self::BotMediaPreviews(value) => value.to_json(),
      Self::BotMenuButton(value) => value.to_json(),
      Self::BusinessChatLink(value) => value.to_json(),
      Self::BusinessChatLinkInfo(value) => value.to_json(),
      Self::BusinessChatLinks(value) => value.to_json(),
      Self::BusinessConnectedBotInfo(value) => value.to_json(),
      Self::BusinessConnection(value) => value.to_json(),
      Self::BusinessFeatures(value) => value.to_json(),
      Self::BusinessMessage(value) => value.to_json(),
      Self::BusinessMessages(value) => value.to_json(),
      Self::CallId(value) => value.to_json(),
      Self::CallbackQueryAnswer(value) => value.to_json(),
      Self::Chat(value) => value.to_json(),
      Self::ChatActiveStories(value) => value.to_json(),
      Self::ChatAdministrators(value) => value.to_json(),
      Self::ChatBoostFeatures(value) => value.to_json(),
      Self::ChatBoostLevelFeatures(value) => value.to_json(),
      Self::ChatBoostLink(value) => value.to_json(),
      Self::ChatBoostLinkInfo(value) => value.to_json(),
      Self::ChatBoostSlots(value) => value.to_json(),
      Self::ChatBoostStatus(value) => value.to_json(),
      Self::ChatEvents(value) => value.to_json(),
      Self::ChatFolder(value) => value.to_json(),
      Self::ChatFolderIcon(value) => value.to_json(),
      Self::ChatFolderInfo(value) => value.to_json(),
      Self::ChatFolderInviteLink(value) => value.to_json(),
      Self::ChatFolderInviteLinkInfo(value) => value.to_json(),
      Self::ChatFolderInviteLinks(value) => value.to_json(),
      Self::ChatInviteLink(value) => value.to_json(),
      Self::ChatInviteLinkCounts(value) => value.to_json(),
      Self::ChatInviteLinkInfo(value) => value.to_json(),
      Self::ChatInviteLinkMembers(value) => value.to_json(),
      Self::ChatInviteLinks(value) => value.to_json(),
      Self::ChatJoinRequests(value) => value.to_json(),
      Self::ChatLists(value) => value.to_json(),
      Self::ChatMember(value) => value.to_json(),
      Self::ChatMembers(value) => value.to_json(),
      Self::ChatMessageSenders(value) => value.to_json(),
      Self::ChatPhotos(value) => value.to_json(),
      Self::ChatRevenueStatistics(value) => value.to_json(),
      Self::ChatRevenueTransactions(value) => value.to_json(),
      Self::Chats(value) => value.to_json(),
      Self::CollectibleItemInfo(value) => value.to_json(),
      Self::CommunityFullInfo(value) => value.to_json(),
      Self::CommunityId(value) => value.to_json(),
      Self::ConnectedAffiliateProgram(value) => value.to_json(),
      Self::ConnectedAffiliatePrograms(value) => value.to_json(),
      Self::ConnectedWebsites(value) => value.to_json(),
      Self::Count(value) => value.to_json(),
      Self::Countries(value) => value.to_json(),
      Self::CountryInfo(value) => value.to_json(),
      Self::CreatedBasicGroupChat(value) => value.to_json(),
      Self::Currencies(value) => value.to_json(),
      Self::CurrencyExchangeRates(value) => value.to_json(),
      Self::CurrentWeather(value) => value.to_json(),
      Self::CustomRequestResult(value) => value.to_json(),
      Self::Data(value) => value.to_json(),
      Self::DatabaseStatistics(value) => value.to_json(),
      Self::DeepLinkInfo(value) => value.to_json(),
      Self::DirectMessagesChatTopic(value) => value.to_json(),
      Self::EmailAddressAuthenticationCodeInfo(value) => value.to_json(),
      Self::EmojiCategories(value) => value.to_json(),
      Self::EmojiKeywords(value) => value.to_json(),
      Self::EmojiReaction(value) => value.to_json(),
      Self::EmojiStatusCustomEmojis(value) => value.to_json(),
      Self::EmojiStatuses(value) => value.to_json(),
      Self::Emojis(value) => value.to_json(),
      Self::Error(value) => value.to_json(),
      Self::FailedToAddMembers(value) => value.to_json(),
      Self::File(value) => value.to_json(),
      Self::FileDownloadedPrefixSize(value) => value.to_json(),
      Self::FixedText(value) => value.to_json(),
      Self::FormattedText(value) => value.to_json(),
      Self::ForumTopic(value) => value.to_json(),
      Self::ForumTopicInfo(value) => value.to_json(),
      Self::ForumTopics(value) => value.to_json(),
      Self::FoundAffiliatePrograms(value) => value.to_json(),
      Self::FoundChatBoosts(value) => value.to_json(),
      Self::FoundChatMessages(value) => value.to_json(),
      Self::FoundFileDownloads(value) => value.to_json(),
      Self::FoundMessages(value) => value.to_json(),
      Self::FoundPosition(value) => value.to_json(),
      Self::FoundPositions(value) => value.to_json(),
      Self::FoundPublicPosts(value) => value.to_json(),
      Self::FoundStories(value) => value.to_json(),
      Self::FoundUsers(value) => value.to_json(),
      Self::FoundWebApp(value) => value.to_json(),
      Self::GameHighScores(value) => value.to_json(),
      Self::GiftAuctionAcquiredGifts(value) => value.to_json(),
      Self::GiftAuctionState(value) => value.to_json(),
      Self::GiftChatThemes(value) => value.to_json(),
      Self::GiftCollection(value) => value.to_json(),
      Self::GiftCollections(value) => value.to_json(),
      Self::GiftUpgradePreview(value) => value.to_json(),
      Self::GiftUpgradeVariants(value) => value.to_json(),
      Self::GiftsForCrafting(value) => value.to_json(),
      Self::GiftsForResale(value) => value.to_json(),
      Self::GramRevenueStatistics(value) => value.to_json(),
      Self::GroupCall(value) => value.to_json(),
      Self::GroupCallId(value) => value.to_json(),
      Self::GroupCallInfo(value) => value.to_json(),
      Self::GroupCallParticipant(value) => value.to_json(),
      Self::GroupCallParticipants(value) => value.to_json(),
      Self::GroupCallStreams(value) => value.to_json(),
      Self::Hashtags(value) => value.to_json(),
      Self::HttpUrl(value) => value.to_json(),
      Self::ImportedContacts(value) => value.to_json(),
      Self::InlineMessageId(value) => value.to_json(),
      Self::InlineQueryResults(value) => value.to_json(),
      Self::KeyboardButton(value) => value.to_json(),
      Self::LanguagePackInfo(value) => value.to_json(),
      Self::LanguagePackStrings(value) => value.to_json(),
      Self::LinkPreview(value) => value.to_json(),
      Self::LiveStoryDonors(value) => value.to_json(),
      Self::LocalizationTargetInfo(value) => value.to_json(),
      Self::LogTags(value) => value.to_json(),
      Self::LogVerbosityLevel(value) => value.to_json(),
      Self::MainWebApp(value) => value.to_json(),
      Self::Message(value) => value.to_json(),
      Self::MessageAutoDeleteTime(value) => value.to_json(),
      Self::MessageCalendar(value) => value.to_json(),
      Self::MessageEffect(value) => value.to_json(),
      Self::MessageLink(value) => value.to_json(),
      Self::MessageLinkInfo(value) => value.to_json(),
      Self::MessagePositions(value) => value.to_json(),
      Self::MessageProperties(value) => value.to_json(),
      Self::MessageSenders(value) => value.to_json(),
      Self::MessageStatistics(value) => value.to_json(),
      Self::MessageThreadInfo(value) => value.to_json(),
      Self::MessageViewers(value) => value.to_json(),
      Self::Messages(value) => value.to_json(),
      Self::NetworkStatistics(value) => value.to_json(),
      Self::NewChatPrivacySettings(value) => value.to_json(),
      Self::NotificationSound(value) => value.to_json(),
      Self::NotificationSounds(value) => value.to_json(),
      Self::OauthLinkInfo(value) => value.to_json(),
      Self::Ok(value) => value.to_json(),
      Self::OnRampPaymentAvailability(value) => value.to_json(),
      Self::OnRampPaymentLimits(value) => value.to_json(),
      Self::OnRampPaymentQuote(value) => value.to_json(),
      Self::OnRampPaymentSession(value) => value.to_json(),
      Self::OnRampProviders(value) => value.to_json(),
      Self::OrderInfo(value) => value.to_json(),
      Self::Outline(value) => value.to_json(),
      Self::Passkey(value) => value.to_json(),
      Self::Passkeys(value) => value.to_json(),
      Self::PassportAuthorizationForm(value) => value.to_json(),
      Self::PassportElements(value) => value.to_json(),
      Self::PassportElementsWithErrors(value) => value.to_json(),
      Self::PasswordState(value) => value.to_json(),
      Self::PaymentForm(value) => value.to_json(),
      Self::PaymentReceipt(value) => value.to_json(),
      Self::PaymentResult(value) => value.to_json(),
      Self::PhoneNumberInfo(value) => value.to_json(),
      Self::PollOptionProperties(value) => value.to_json(),
      Self::PollVoteStatistics(value) => value.to_json(),
      Self::PollVoters(value) => value.to_json(),
      Self::PremiumFeatures(value) => value.to_json(),
      Self::PremiumGiftCodeInfo(value) => value.to_json(),
      Self::PremiumGiftPaymentOptions(value) => value.to_json(),
      Self::PremiumGiveawayPaymentOptions(value) => value.to_json(),
      Self::PremiumLimit(value) => value.to_json(),
      Self::PremiumState(value) => value.to_json(),
      Self::PreparedInlineMessage(value) => value.to_json(),
      Self::PreparedInlineMessageId(value) => value.to_json(),
      Self::ProfileShareThemeBackdrops(value) => value.to_json(),
      Self::PublicForwards(value) => value.to_json(),
      Self::PublicPostSearchLimits(value) => value.to_json(),
      Self::PushReceiverId(value) => value.to_json(),
      Self::QuickReplyMessage(value) => value.to_json(),
      Self::QuickReplyMessages(value) => value.to_json(),
      Self::ReadDatePrivacySettings(value) => value.to_json(),
      Self::ReceivedGift(value) => value.to_json(),
      Self::ReceivedGifts(value) => value.to_json(),
      Self::RecommendedChatFolders(value) => value.to_json(),
      Self::RecoveryEmailAddress(value) => value.to_json(),
      Self::RichMessage(value) => value.to_json(),
      Self::RtmpUrl(value) => value.to_json(),
      Self::SavedMessagesTags(value) => value.to_json(),
      Self::ScopeNotificationSettings(value) => value.to_json(),
      Self::Seconds(value) => value.to_json(),
      Self::SecretChat(value) => value.to_json(),
      Self::Session(value) => value.to_json(),
      Self::Sessions(value) => value.to_json(),
      Self::SponsoredChats(value) => value.to_json(),
      Self::SponsoredMessages(value) => value.to_json(),
      Self::StakeDiceState(value) => value.to_json(),
      Self::StarAmount(value) => value.to_json(),
      Self::StarCount(value) => value.to_json(),
      Self::StarGiveawayPaymentOptions(value) => value.to_json(),
      Self::StarPaymentOptions(value) => value.to_json(),
      Self::StarRevenueStatistics(value) => value.to_json(),
      Self::StarSubscriptions(value) => value.to_json(),
      Self::StarTransactions(value) => value.to_json(),
      Self::Sticker(value) => value.to_json(),
      Self::StickerSet(value) => value.to_json(),
      Self::StickerSets(value) => value.to_json(),
      Self::Stickers(value) => value.to_json(),
      Self::StorageStatistics(value) => value.to_json(),
      Self::StorageStatisticsFast(value) => value.to_json(),
      Self::Stories(value) => value.to_json(),
      Self::Story(value) => value.to_json(),
      Self::StoryAlbum(value) => value.to_json(),
      Self::StoryAlbums(value) => value.to_json(),
      Self::StoryInteractions(value) => value.to_json(),
      Self::StoryStatistics(value) => value.to_json(),
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
      Self::TextCompositionStyle(value) => value.to_json(),
      Self::TextCompositionStyleExample(value) => value.to_json(),
      Self::TextEntities(value) => value.to_json(),
      Self::TimeZones(value) => value.to_json(),
      Self::TonCenterStreamingApiUrl(value) => value.to_json(),
      Self::TonConnectChallenge(value) => value.to_json(),
      Self::TonConnectRequests(value) => value.to_json(),
      Self::TonConnectSession(value) => value.to_json(),
      Self::TonConnectSessionEventId(value) => value.to_json(),
      Self::TonConnectSessions(value) => value.to_json(),
      Self::TonNfts(value) => value.to_json(),
      Self::TonTransactions(value) => value.to_json(),
      Self::TonWalletCard(value) => value.to_json(),
      Self::TonWalletOwnershipProofChallenge(value) => value.to_json(),
      Self::TonWalletSecretPhrase(value) => value.to_json(),
      Self::TonWalletTransaction(value) => value.to_json(),
      Self::TonWalletTransactions(value) => value.to_json(),
      Self::TonWalletTransferResult(value) => value.to_json(),
      Self::TrendingStickerSets(value) => value.to_json(),
      Self::Updates(value) => value.to_json(),
      Self::UpgradeGiftResult(value) => value.to_json(),
      Self::UpgradedGift(value) => value.to_json(),
      Self::UpgradedGiftCollectionNames(value) => value.to_json(),
      Self::UpgradedGiftDepositInfo(value) => value.to_json(),
      Self::UpgradedGiftValueInfo(value) => value.to_json(),
      Self::User(value) => value.to_json(),
      Self::UserFullInfo(value) => value.to_json(),
      Self::UserLink(value) => value.to_json(),
      Self::UserPrivacySettingRules(value) => value.to_json(),
      Self::UserSupportInfo(value) => value.to_json(),
      Self::UserTonWalletAddress(value) => value.to_json(),
      Self::UserTonWalletAddresses(value) => value.to_json(),
      Self::Users(value) => value.to_json(),
      Self::ValidatedOrderInfo(value) => value.to_json(),
      Self::VideoMessageAdvertisements(value) => value.to_json(),
      Self::WalletBotBalance(value) => value.to_json(),
      Self::WebAppInfo(value) => value.to_json(),
      Self::WebAppUrl(value) => value.to_json(),
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

    // TdType's debug deserialize frame is larger than the default test-thread stack.
    let handle = std::thread::Builder::new()
      .stack_size(32 * 1024 * 1024)
      .spawn(|| match from_json::<TdType>(AUTH_JSON) {
        Ok(TdType::UpdateAuthorizationState(_)) => {}
        Ok(other) => panic!("from_json failed: {:?}", other),
        Err(err) => panic!("{}", err),
      })
      .expect("spawn");
    handle.join().expect("join");
  }
}
