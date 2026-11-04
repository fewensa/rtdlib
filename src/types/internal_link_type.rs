use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes an internal https://t.me or tg: link, which must be processed by the application in a special way
pub trait TDInternalLinkType: Debug + RObject {}

/// Describes an internal https://t.me or tg: link, which must be processed by the application in a special way
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InternalLinkType {
  #[doc(hidden)]
  _Default(()),
  /// Returns information about the type of internal link. Returns a 404 error if the link is not internal. Can be called before authorization
  GetInternalLinkType(GetInternalLinkType),
  /// The link is a link to an attachment menu bot to be opened in the specified or a chosen chat. Process given target_chat to open the chat. Then, call searchPublicChat with the given bot username, check that the user is a bot and can be added to attachment menu. Then, use getAttachmentMenuBot to receive information about the bot. If the bot isn't added to attachment menu, then show a disclaimer about Mini Apps being third-party applications, ask the user to accept their Terms of service and confirm adding the bot to side and attachment menu. If the user accepts the terms and confirms adding, then use toggleBotIsAddedToAttachmentMenu to add the bot. If the attachment menu bot can't be used in the opened chat, show an error to the user. If the bot is added to attachment menu and can be used in the chat, then use openWebApp with the given URL
  AttachmentMenuBot(InternalLinkTypeAttachmentMenuBot),
  /// The link contains an authentication code. Call checkAuthenticationCode with the code if the current authorization state is authorizationStateWaitCode
  AuthenticationCode(InternalLinkTypeAuthenticationCode),
  /// The link is a link to a background. Call searchBackground with the given background name to process the link. If background is found and the user wants to apply it, then call setDefaultBackground
  Background(InternalLinkTypeBackground),
  /// The link is a link to a Telegram bot, which is expected to be added to a channel chat as an administrator. Call searchPublicChat with the given bot username and check that the user is a bot, ask the current user to select a channel chat to add the bot to as an administrator. Then, call getChatMember to receive the current bot rights in the chat and if the bot already is an administrator, check that the current user can edit its administrator rights and combine received rights with the requested administrator rights. Then, show confirmation box to the user, and call setChatMemberStatus with the chosen chat and confirmed rights
  BotAddToChannel(InternalLinkTypeBotAddToChannel),
  /// The link is a link to a chat with a Telegram bot. Call searchPublicChat with the given bot username, check that the user is a bot, show START button in the chat with the bot, and then call sendBotStartMessage with the given start parameter after the button is pressed
  BotStart(InternalLinkTypeBotStart),
  /// The link is a link to a Telegram bot, which is expected to be added to a group chat. Call searchPublicChat with the given bot username, check that the user is a bot and can be added to groups, ask the current user to select a basic group or a supergroup chat to add the bot to, taking into account that bots can be added to a public supergroup only by administrators of the supergroup. If administrator rights are provided by the link, call getChatMember to receive the current bot rights in the chat and if the bot already is an administrator, check that the current user can edit its administrator rights, combine received rights with the requested administrator rights, show confirmation box to the user, and call setChatMemberStatus with the chosen chat and confirmed administrator rights. Before call to setChatMemberStatus it may be required to upgrade the chosen basic group chat to a supergroup chat. Then, if start_parameter isn't empty, call sendBotStartMessage with the given start parameter and the chosen chat; otherwise, just send /start message with bot's username added to the chat
  BotStartInGroup(InternalLinkTypeBotStartInGroup),
  /// The link is a link to a business chat. Use getBusinessChatLinkInfo with the provided link name to get information about the link, then open received private chat and replace chat draft with the provided text
  BusinessChat(InternalLinkTypeBusinessChat),
  /// The link is a link to the Call tab or page
  CallsPage(InternalLinkTypeCallsPage),
  /// The link is an affiliate program link. Call searchChatAffiliateProgram with the given username and referrer to process the link
  ChatAffiliateProgram(InternalLinkTypeChatAffiliateProgram),
  /// The link is a link to boost a Telegram chat. Call getChatBoostLinkInfo with the given URL to process the link. If the chat is found, then call getChatBoostStatus and getAvailableChatBoostSlots to get the current boost status and check whether the chat can be boosted. If the user wants to boost the chat and the chat can be boosted, then call boostChat
  ChatBoost(InternalLinkTypeChatBoost),
  /// The link is an invite link to a chat folder. Call checkChatFolderInviteLink with the given invite link to process the link. If the link is valid and the user wants to join the chat folder, then call addChatFolderByInviteLink
  ChatFolderInvite(InternalLinkTypeChatFolderInvite),
  /// The link is a chat invite link. Call checkChatInviteLink with the given invite link to process the link. If the link is valid and the user wants to join the chat, then call joinChatByInviteLink
  ChatInvite(InternalLinkTypeChatInvite),
  /// The link is a link that allows to select some chats
  ChatSelection(InternalLinkTypeChatSelection),
  /// The link is a link to the Contacts tab or page
  ContactsPage(InternalLinkTypeContactsPage),
  /// The link is a link to a channel direct messages chat by username of the channel. Call searchPublicChat with the given chat username to process the link. If the chat is found and is channel, open the direct messages chat of the channel
  DirectMessagesChat(InternalLinkTypeDirectMessagesChat),
  /// The link is a link to a game. Call searchPublicChat with the given bot username, check that the user is a bot, ask the current user to select a chat to send the game, and then call sendMessage with inputMessageGame
  Game(InternalLinkTypeGame),
  /// The link is a link to a gift auction. Call getGiftAuctionState with the given auction identifier to process the link
  GiftAuction(InternalLinkTypeGiftAuction),
  /// The link is a link to a gift collection. Call searchPublicChat with the given username, then call getReceivedGifts with the received gift owner identifier and the given collection identifier, then show the collection if received
  GiftCollection(InternalLinkTypeGiftCollection),
  /// The link is a link to a group call that isn't bound to a chat. Use getGroupCallParticipants to get the list of group call participants and show them on the join group call screen. Call joinGroupCall with the given invite_link to join the call
  GroupCall(InternalLinkTypeGroupCall),
  /// The link must be opened in an Instant View. Call getWebPageInstantView with the given URL to process the link. If Instant View is found, then show it, otherwise, open the fallback URL in an external browser
  InstantView(InternalLinkTypeInstantView),
  /// The link is a link to an invoice. Call getPaymentForm with the given invoice name to process the link
  Invoice(InternalLinkTypeInvoice),
  /// The link is a link to a language pack. Call getLanguagePackInfo with the given language pack identifier to process the link. If the language pack is found and the user wants to apply it, then call setOption for the option "language_pack_id"
  LanguagePack(InternalLinkTypeLanguagePack),
  /// The link is a link to a live story. Call searchPublicChat with the given chat username, then getChatActiveStories to get active stories in the chat, then find a live story among active stories of the chat, and then joinLiveStory to join the live story
  LiveStory(InternalLinkTypeLiveStory),
  /// The link is a link to the main Web App of a bot. Call searchPublicChat with the given bot username, check that the user is a bot and has the main Web App. If the bot can be added to attachment menu, then use getAttachmentMenuBot to receive information about the bot, then if the bot isn't added to side menu, show a disclaimer about Mini Apps being third-party applications, ask the user to accept their Terms of service and confirm adding the bot to side and attachment menu, then if the user accepts the terms and confirms adding, use toggleBotIsAddedToAttachmentMenu to add the bot. Then, use getMainWebApp with the given start parameter and mode and open the returned URL as a Web App
  MainWebApp(InternalLinkTypeMainWebApp),
  /// The link is a link to a Telegram message or a forum topic. Call getMessageLinkInfo with the given URL to process the link, and then open received forum topic or chat and show the message there
  Message(InternalLinkTypeMessage),
  /// The link contains a message draft text. A share screen needs to be shown to the user, then the chosen chat must be opened and the text is added to the input field
  MessageDraft(InternalLinkTypeMessageDraft),
  /// The link is a link to the My Profile application page
  MyProfilePage(InternalLinkTypeMyProfilePage),
  /// The link is a link to the screen for creating a new channel chat
  NewChannelChat(InternalLinkTypeNewChannelChat),
  /// The link is a link to the screen for creating a new group chat
  NewGroupChat(InternalLinkTypeNewGroupChat),
  /// The link is a link to the screen for creating a new private chat with a contact
  NewPrivateChat(InternalLinkTypeNewPrivateChat),
  /// The link is a link to open the story posting interface
  NewStory(InternalLinkTypeNewStory),
  /// The link is an OAuth link. Call getOauthLinkInfo with the given URL to process the link if the link was received from outside of the application; otherwise, ignore it. After getOauthLinkInfo, show the user confirmation dialog and process it with checkOauthRequestMatchCode, acceptOauthRequest, or declineOauthRequest
  Oauth(InternalLinkTypeOauth),
  /// The link contains a request of Telegram passport data. Call getPassportAuthorizationForm with the given parameters to process the link if the link was received from outside of the application; otherwise, ignore it
  PassportDataRequest(InternalLinkTypePassportDataRequest),
  /// The link can be used to confirm ownership of a phone number to prevent account deletion. Call sendPhoneNumberCode with the given phone number and with phoneNumberCodeTypeConfirmOwnership with the given hash to process the link. If succeeded, call checkPhoneNumberCode to check entered by the user code, or resendPhoneNumberCode to resend it
  PhoneNumberConfirmation(InternalLinkTypePhoneNumberConfirmation),
  /// The link is a link to the Premium features screen of the application from which the user can subscribe to Telegram Premium. Call getPremiumFeatures with the given referrer to process the link
  PremiumFeaturesPage(InternalLinkTypePremiumFeaturesPage),
  /// The link is a link with a Telegram Premium gift code. Call checkPremiumGiftCode with the given code to process the link. If the code is valid and the user wants to apply it, then call applyPremiumGiftCode
  PremiumGiftCode(InternalLinkTypePremiumGiftCode),
  /// The link is a link to the screen for gifting Telegram Premium subscriptions to friends via inputInvoiceTelegram with telegramPaymentPurposePremiumGift payments or in-store purchases
  PremiumGiftPurchase(InternalLinkTypePremiumGiftPurchase),
  /// The link is a link to a proxy. Call addProxy with the given parameters to process the link and add the proxy
  Proxy(InternalLinkTypeProxy),
  /// The link is a link to a chat by its username. Call searchPublicChat with the given chat username to process the link. If the chat is found, open its profile information screen or the chat itself. If draft text isn't empty and the chat is a private chat with a regular user, then put the draft text in the input field
  PublicChat(InternalLinkTypePublicChat),
  /// The link can be used to login the current user on another device, but it must be scanned from QR-code using in-app camera. An alert similar to "This code can be used to allow someone to log in to your Telegram account. To confirm Telegram login, please go to Settings > Devices > Scan QR and scan the code" needs to be shown
  QrCodeAuthentication(InternalLinkTypeQrCodeAuthentication),
  /// The link is a link to a dialog for creating of a managed bot. Call searchPublicChat with the given manager bot username. If the chat is found, the chat is a chat with a bot and the bot has can_manage_bots == true, then show bot creation confirmation dialog with the given suggested_bot_username and suggested_bot_name. If user agrees, call createBot with via_link == true to create the bot
  RequestManagedBot(InternalLinkTypeRequestManagedBot),
  /// The link forces restore of App Store purchases when opened. For official iOS application only
  RestorePurchases(InternalLinkTypeRestorePurchases),
  /// The link is a link to the Saved Messages chat. Call createPrivateChat with getOption("my_id") and open the chat
  SavedMessages(InternalLinkTypeSavedMessages),
  /// The link is a link to the global chat and messages search field
  Search(InternalLinkTypeSearch),
  /// The link is a link to application settings
  Settings(InternalLinkTypeSettings),
  /// The link is a link to the Telegram Star purchase section of the application
  StarPurchase(InternalLinkTypeStarPurchase),
  /// The link is a link to a sticker set. Call searchStickerSet with the given sticker set name to process the link and show the sticker set. If the sticker set is found and the user wants to add it, then call changeStickerSet
  StickerSet(InternalLinkTypeStickerSet),
  /// The link is a link to a story. Call searchPublicChat with the given poster username, then call getStory with the received chat identifier and the given story identifier, then show the story if received
  Story(InternalLinkTypeStory),
  /// The link is a link to an album of stories. Call searchPublicChat with the given username, then call getStoryAlbumStories with the received chat identifier and the given story album identifier, then show the story album if received
  StoryAlbum(InternalLinkTypeStoryAlbum),
  /// The link is a link to a text composition style. Call searchTextCompositionStyle with the given style name to get information about the style. If the style is found and the user wants to add it, then call addTextCompositionStyle
  TextCompositionStyle(InternalLinkTypeTextCompositionStyle),
  /// The link is a link to a cloud theme. TDLib has no theme support yet
  Theme(InternalLinkTypeTheme),
  /// The link is a universal TON Connect link
  TonConnect(InternalLinkTypeTonConnect),
  /// The link is a link for sending Grams from the user's TON wallet
  TonWalletTransfer(InternalLinkTypeTonWalletTransfer),
  /// The link is an unknown tg: link. Call getDeepLinkInfo to process the link
  UnknownDeepLink(InternalLinkTypeUnknownDeepLink),
  /// The link is a link to an upgraded gift. Call getUpgradedGift with the given name to process the link
  UpgradedGift(InternalLinkTypeUpgradedGift),
  /// The link is a link to a user by its phone number. Call searchUserByPhoneNumber with the given phone number to process the link. If the user is found, then call createPrivateChat and open user's profile information screen or the chat itself. If draft text isn't empty, then put the draft text in the input field
  UserPhoneNumber(InternalLinkTypeUserPhoneNumber),
  /// The link is a link to a user by a temporary token. Call searchUserByToken with the given token to process the link. If the user is found, then call createPrivateChat and open the chat
  UserToken(InternalLinkTypeUserToken),
  /// The link is a link to a video chat. Call searchPublicChat with the given chat username, and then joinVideoChat with the given invite hash to process the link
  VideoChat(InternalLinkTypeVideoChat),
  /// The link is a link to a Web App. Call searchPublicChat with the given bot username, check that the user is a bot. If the bot is restricted for the current user, then show an error message. Otherwise, call searchWebApp with the received bot and the given web_app_short_name. Process received foundWebApp by showing a confirmation dialog if needed. If the bot can be added to attachment or side menu, but isn't added yet, then show a disclaimer about Mini Apps being third-party applications instead of the dialog and ask the user to accept their Terms of service. If the user accepts the terms and confirms adding, then use toggleBotIsAddedToAttachmentMenu to add the bot. Then, call getWebAppLinkUrl and open the returned URL as a Web App
  WebApp(InternalLinkTypeWebApp),
}

impl Default for InternalLinkType {
  fn default() -> Self {
    InternalLinkType::_Default(())
  }
}

impl<'de> Deserialize<'de> for InternalLinkType {
  fn deserialize<D>(deserializer: D) -> Result<InternalLinkType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InternalLinkType,
      (getInternalLinkType, GetInternalLinkType);
      (internalLinkTypeAttachmentMenuBot, AttachmentMenuBot);
      (internalLinkTypeAuthenticationCode, AuthenticationCode);
      (internalLinkTypeBackground, Background);
      (internalLinkTypeBotAddToChannel, BotAddToChannel);
      (internalLinkTypeBotStart, BotStart);
      (internalLinkTypeBotStartInGroup, BotStartInGroup);
      (internalLinkTypeBusinessChat, BusinessChat);
      (internalLinkTypeCallsPage, CallsPage);
      (internalLinkTypeChatAffiliateProgram, ChatAffiliateProgram);
      (internalLinkTypeChatBoost, ChatBoost);
      (internalLinkTypeChatFolderInvite, ChatFolderInvite);
      (internalLinkTypeChatInvite, ChatInvite);
      (internalLinkTypeChatSelection, ChatSelection);
      (internalLinkTypeContactsPage, ContactsPage);
      (internalLinkTypeDirectMessagesChat, DirectMessagesChat);
      (internalLinkTypeGame, Game);
      (internalLinkTypeGiftAuction, GiftAuction);
      (internalLinkTypeGiftCollection, GiftCollection);
      (internalLinkTypeGroupCall, GroupCall);
      (internalLinkTypeInstantView, InstantView);
      (internalLinkTypeInvoice, Invoice);
      (internalLinkTypeLanguagePack, LanguagePack);
      (internalLinkTypeLiveStory, LiveStory);
      (internalLinkTypeMainWebApp, MainWebApp);
      (internalLinkTypeMessage, Message);
      (internalLinkTypeMessageDraft, MessageDraft);
      (internalLinkTypeMyProfilePage, MyProfilePage);
      (internalLinkTypeNewChannelChat, NewChannelChat);
      (internalLinkTypeNewGroupChat, NewGroupChat);
      (internalLinkTypeNewPrivateChat, NewPrivateChat);
      (internalLinkTypeNewStory, NewStory);
      (internalLinkTypeOauth, Oauth);
      (internalLinkTypePassportDataRequest, PassportDataRequest);
      (internalLinkTypePhoneNumberConfirmation, PhoneNumberConfirmation);
      (internalLinkTypePremiumFeaturesPage, PremiumFeaturesPage);
      (internalLinkTypePremiumGiftCode, PremiumGiftCode);
      (internalLinkTypePremiumGiftPurchase, PremiumGiftPurchase);
      (internalLinkTypeProxy, Proxy);
      (internalLinkTypePublicChat, PublicChat);
      (internalLinkTypeQrCodeAuthentication, QrCodeAuthentication);
      (internalLinkTypeRequestManagedBot, RequestManagedBot);
      (internalLinkTypeRestorePurchases, RestorePurchases);
      (internalLinkTypeSavedMessages, SavedMessages);
      (internalLinkTypeSearch, Search);
      (internalLinkTypeSettings, Settings);
      (internalLinkTypeStarPurchase, StarPurchase);
      (internalLinkTypeStickerSet, StickerSet);
      (internalLinkTypeStory, Story);
      (internalLinkTypeStoryAlbum, StoryAlbum);
      (internalLinkTypeTextCompositionStyle, TextCompositionStyle);
      (internalLinkTypeTheme, Theme);
      (internalLinkTypeTonConnect, TonConnect);
      (internalLinkTypeTonWalletTransfer, TonWalletTransfer);
      (internalLinkTypeUnknownDeepLink, UnknownDeepLink);
      (internalLinkTypeUpgradedGift, UpgradedGift);
      (internalLinkTypeUserPhoneNumber, UserPhoneNumber);
      (internalLinkTypeUserToken, UserToken);
      (internalLinkTypeVideoChat, VideoChat);
      (internalLinkTypeWebApp, WebApp);

    )(deserializer)
  }
}

impl RObject for InternalLinkType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      InternalLinkType::GetInternalLinkType(t) => t.td_name(),
      InternalLinkType::AttachmentMenuBot(t) => t.td_name(),
      InternalLinkType::AuthenticationCode(t) => t.td_name(),
      InternalLinkType::Background(t) => t.td_name(),
      InternalLinkType::BotAddToChannel(t) => t.td_name(),
      InternalLinkType::BotStart(t) => t.td_name(),
      InternalLinkType::BotStartInGroup(t) => t.td_name(),
      InternalLinkType::BusinessChat(t) => t.td_name(),
      InternalLinkType::CallsPage(t) => t.td_name(),
      InternalLinkType::ChatAffiliateProgram(t) => t.td_name(),
      InternalLinkType::ChatBoost(t) => t.td_name(),
      InternalLinkType::ChatFolderInvite(t) => t.td_name(),
      InternalLinkType::ChatInvite(t) => t.td_name(),
      InternalLinkType::ChatSelection(t) => t.td_name(),
      InternalLinkType::ContactsPage(t) => t.td_name(),
      InternalLinkType::DirectMessagesChat(t) => t.td_name(),
      InternalLinkType::Game(t) => t.td_name(),
      InternalLinkType::GiftAuction(t) => t.td_name(),
      InternalLinkType::GiftCollection(t) => t.td_name(),
      InternalLinkType::GroupCall(t) => t.td_name(),
      InternalLinkType::InstantView(t) => t.td_name(),
      InternalLinkType::Invoice(t) => t.td_name(),
      InternalLinkType::LanguagePack(t) => t.td_name(),
      InternalLinkType::LiveStory(t) => t.td_name(),
      InternalLinkType::MainWebApp(t) => t.td_name(),
      InternalLinkType::Message(t) => t.td_name(),
      InternalLinkType::MessageDraft(t) => t.td_name(),
      InternalLinkType::MyProfilePage(t) => t.td_name(),
      InternalLinkType::NewChannelChat(t) => t.td_name(),
      InternalLinkType::NewGroupChat(t) => t.td_name(),
      InternalLinkType::NewPrivateChat(t) => t.td_name(),
      InternalLinkType::NewStory(t) => t.td_name(),
      InternalLinkType::Oauth(t) => t.td_name(),
      InternalLinkType::PassportDataRequest(t) => t.td_name(),
      InternalLinkType::PhoneNumberConfirmation(t) => t.td_name(),
      InternalLinkType::PremiumFeaturesPage(t) => t.td_name(),
      InternalLinkType::PremiumGiftCode(t) => t.td_name(),
      InternalLinkType::PremiumGiftPurchase(t) => t.td_name(),
      InternalLinkType::Proxy(t) => t.td_name(),
      InternalLinkType::PublicChat(t) => t.td_name(),
      InternalLinkType::QrCodeAuthentication(t) => t.td_name(),
      InternalLinkType::RequestManagedBot(t) => t.td_name(),
      InternalLinkType::RestorePurchases(t) => t.td_name(),
      InternalLinkType::SavedMessages(t) => t.td_name(),
      InternalLinkType::Search(t) => t.td_name(),
      InternalLinkType::Settings(t) => t.td_name(),
      InternalLinkType::StarPurchase(t) => t.td_name(),
      InternalLinkType::StickerSet(t) => t.td_name(),
      InternalLinkType::Story(t) => t.td_name(),
      InternalLinkType::StoryAlbum(t) => t.td_name(),
      InternalLinkType::TextCompositionStyle(t) => t.td_name(),
      InternalLinkType::Theme(t) => t.td_name(),
      InternalLinkType::TonConnect(t) => t.td_name(),
      InternalLinkType::TonWalletTransfer(t) => t.td_name(),
      InternalLinkType::UnknownDeepLink(t) => t.td_name(),
      InternalLinkType::UpgradedGift(t) => t.td_name(),
      InternalLinkType::UserPhoneNumber(t) => t.td_name(),
      InternalLinkType::UserToken(t) => t.td_name(),
      InternalLinkType::VideoChat(t) => t.td_name(),
      InternalLinkType::WebApp(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      InternalLinkType::GetInternalLinkType(t) => t.extra(),
      InternalLinkType::AttachmentMenuBot(t) => t.extra(),
      InternalLinkType::AuthenticationCode(t) => t.extra(),
      InternalLinkType::Background(t) => t.extra(),
      InternalLinkType::BotAddToChannel(t) => t.extra(),
      InternalLinkType::BotStart(t) => t.extra(),
      InternalLinkType::BotStartInGroup(t) => t.extra(),
      InternalLinkType::BusinessChat(t) => t.extra(),
      InternalLinkType::CallsPage(t) => t.extra(),
      InternalLinkType::ChatAffiliateProgram(t) => t.extra(),
      InternalLinkType::ChatBoost(t) => t.extra(),
      InternalLinkType::ChatFolderInvite(t) => t.extra(),
      InternalLinkType::ChatInvite(t) => t.extra(),
      InternalLinkType::ChatSelection(t) => t.extra(),
      InternalLinkType::ContactsPage(t) => t.extra(),
      InternalLinkType::DirectMessagesChat(t) => t.extra(),
      InternalLinkType::Game(t) => t.extra(),
      InternalLinkType::GiftAuction(t) => t.extra(),
      InternalLinkType::GiftCollection(t) => t.extra(),
      InternalLinkType::GroupCall(t) => t.extra(),
      InternalLinkType::InstantView(t) => t.extra(),
      InternalLinkType::Invoice(t) => t.extra(),
      InternalLinkType::LanguagePack(t) => t.extra(),
      InternalLinkType::LiveStory(t) => t.extra(),
      InternalLinkType::MainWebApp(t) => t.extra(),
      InternalLinkType::Message(t) => t.extra(),
      InternalLinkType::MessageDraft(t) => t.extra(),
      InternalLinkType::MyProfilePage(t) => t.extra(),
      InternalLinkType::NewChannelChat(t) => t.extra(),
      InternalLinkType::NewGroupChat(t) => t.extra(),
      InternalLinkType::NewPrivateChat(t) => t.extra(),
      InternalLinkType::NewStory(t) => t.extra(),
      InternalLinkType::Oauth(t) => t.extra(),
      InternalLinkType::PassportDataRequest(t) => t.extra(),
      InternalLinkType::PhoneNumberConfirmation(t) => t.extra(),
      InternalLinkType::PremiumFeaturesPage(t) => t.extra(),
      InternalLinkType::PremiumGiftCode(t) => t.extra(),
      InternalLinkType::PremiumGiftPurchase(t) => t.extra(),
      InternalLinkType::Proxy(t) => t.extra(),
      InternalLinkType::PublicChat(t) => t.extra(),
      InternalLinkType::QrCodeAuthentication(t) => t.extra(),
      InternalLinkType::RequestManagedBot(t) => t.extra(),
      InternalLinkType::RestorePurchases(t) => t.extra(),
      InternalLinkType::SavedMessages(t) => t.extra(),
      InternalLinkType::Search(t) => t.extra(),
      InternalLinkType::Settings(t) => t.extra(),
      InternalLinkType::StarPurchase(t) => t.extra(),
      InternalLinkType::StickerSet(t) => t.extra(),
      InternalLinkType::Story(t) => t.extra(),
      InternalLinkType::StoryAlbum(t) => t.extra(),
      InternalLinkType::TextCompositionStyle(t) => t.extra(),
      InternalLinkType::Theme(t) => t.extra(),
      InternalLinkType::TonConnect(t) => t.extra(),
      InternalLinkType::TonWalletTransfer(t) => t.extra(),
      InternalLinkType::UnknownDeepLink(t) => t.extra(),
      InternalLinkType::UpgradedGift(t) => t.extra(),
      InternalLinkType::UserPhoneNumber(t) => t.extra(),
      InternalLinkType::UserToken(t) => t.extra(),
      InternalLinkType::VideoChat(t) => t.extra(),
      InternalLinkType::WebApp(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InternalLinkType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, InternalLinkType::_Default(_))
  }

  pub fn is_get_internal_link_type(&self) -> bool {
    matches!(self, InternalLinkType::GetInternalLinkType(_))
  }
  pub fn is_attachment_menu_bot(&self) -> bool {
    matches!(self, InternalLinkType::AttachmentMenuBot(_))
  }
  pub fn is_authentication_code(&self) -> bool {
    matches!(self, InternalLinkType::AuthenticationCode(_))
  }
  pub fn is_background(&self) -> bool {
    matches!(self, InternalLinkType::Background(_))
  }
  pub fn is_bot_add_to_channel(&self) -> bool {
    matches!(self, InternalLinkType::BotAddToChannel(_))
  }
  pub fn is_bot_start(&self) -> bool {
    matches!(self, InternalLinkType::BotStart(_))
  }
  pub fn is_bot_start_in_group(&self) -> bool {
    matches!(self, InternalLinkType::BotStartInGroup(_))
  }
  pub fn is_business_chat(&self) -> bool {
    matches!(self, InternalLinkType::BusinessChat(_))
  }
  pub fn is_calls_page(&self) -> bool {
    matches!(self, InternalLinkType::CallsPage(_))
  }
  pub fn is_chat_affiliate_program(&self) -> bool {
    matches!(self, InternalLinkType::ChatAffiliateProgram(_))
  }
  pub fn is_chat_boost(&self) -> bool {
    matches!(self, InternalLinkType::ChatBoost(_))
  }
  pub fn is_chat_folder_invite(&self) -> bool {
    matches!(self, InternalLinkType::ChatFolderInvite(_))
  }
  pub fn is_chat_invite(&self) -> bool {
    matches!(self, InternalLinkType::ChatInvite(_))
  }
  pub fn is_chat_selection(&self) -> bool {
    matches!(self, InternalLinkType::ChatSelection(_))
  }
  pub fn is_contacts_page(&self) -> bool {
    matches!(self, InternalLinkType::ContactsPage(_))
  }
  pub fn is_direct_messages_chat(&self) -> bool {
    matches!(self, InternalLinkType::DirectMessagesChat(_))
  }
  pub fn is_game(&self) -> bool {
    matches!(self, InternalLinkType::Game(_))
  }
  pub fn is_gift_auction(&self) -> bool {
    matches!(self, InternalLinkType::GiftAuction(_))
  }
  pub fn is_gift_collection(&self) -> bool {
    matches!(self, InternalLinkType::GiftCollection(_))
  }
  pub fn is_group_call(&self) -> bool {
    matches!(self, InternalLinkType::GroupCall(_))
  }
  pub fn is_instant_view(&self) -> bool {
    matches!(self, InternalLinkType::InstantView(_))
  }
  pub fn is_invoice(&self) -> bool {
    matches!(self, InternalLinkType::Invoice(_))
  }
  pub fn is_language_pack(&self) -> bool {
    matches!(self, InternalLinkType::LanguagePack(_))
  }
  pub fn is_live_story(&self) -> bool {
    matches!(self, InternalLinkType::LiveStory(_))
  }
  pub fn is_main_web_app(&self) -> bool {
    matches!(self, InternalLinkType::MainWebApp(_))
  }
  pub fn is_message(&self) -> bool {
    matches!(self, InternalLinkType::Message(_))
  }
  pub fn is_message_draft(&self) -> bool {
    matches!(self, InternalLinkType::MessageDraft(_))
  }
  pub fn is_my_profile_page(&self) -> bool {
    matches!(self, InternalLinkType::MyProfilePage(_))
  }
  pub fn is_new_channel_chat(&self) -> bool {
    matches!(self, InternalLinkType::NewChannelChat(_))
  }
  pub fn is_new_group_chat(&self) -> bool {
    matches!(self, InternalLinkType::NewGroupChat(_))
  }
  pub fn is_new_private_chat(&self) -> bool {
    matches!(self, InternalLinkType::NewPrivateChat(_))
  }
  pub fn is_new_story(&self) -> bool {
    matches!(self, InternalLinkType::NewStory(_))
  }
  pub fn is_oauth(&self) -> bool {
    matches!(self, InternalLinkType::Oauth(_))
  }
  pub fn is_passport_data_request(&self) -> bool {
    matches!(self, InternalLinkType::PassportDataRequest(_))
  }
  pub fn is_phone_number_confirmation(&self) -> bool {
    matches!(self, InternalLinkType::PhoneNumberConfirmation(_))
  }
  pub fn is_premium_features_page(&self) -> bool {
    matches!(self, InternalLinkType::PremiumFeaturesPage(_))
  }
  pub fn is_premium_gift_code(&self) -> bool {
    matches!(self, InternalLinkType::PremiumGiftCode(_))
  }
  pub fn is_premium_gift_purchase(&self) -> bool {
    matches!(self, InternalLinkType::PremiumGiftPurchase(_))
  }
  pub fn is_proxy(&self) -> bool {
    matches!(self, InternalLinkType::Proxy(_))
  }
  pub fn is_public_chat(&self) -> bool {
    matches!(self, InternalLinkType::PublicChat(_))
  }
  pub fn is_qr_code_authentication(&self) -> bool {
    matches!(self, InternalLinkType::QrCodeAuthentication(_))
  }
  pub fn is_request_managed_bot(&self) -> bool {
    matches!(self, InternalLinkType::RequestManagedBot(_))
  }
  pub fn is_restore_purchases(&self) -> bool {
    matches!(self, InternalLinkType::RestorePurchases(_))
  }
  pub fn is_saved_messages(&self) -> bool {
    matches!(self, InternalLinkType::SavedMessages(_))
  }
  pub fn is_search(&self) -> bool {
    matches!(self, InternalLinkType::Search(_))
  }
  pub fn is_settings(&self) -> bool {
    matches!(self, InternalLinkType::Settings(_))
  }
  pub fn is_star_purchase(&self) -> bool {
    matches!(self, InternalLinkType::StarPurchase(_))
  }
  pub fn is_sticker_set(&self) -> bool {
    matches!(self, InternalLinkType::StickerSet(_))
  }
  pub fn is_story(&self) -> bool {
    matches!(self, InternalLinkType::Story(_))
  }
  pub fn is_story_album(&self) -> bool {
    matches!(self, InternalLinkType::StoryAlbum(_))
  }
  pub fn is_text_composition_style(&self) -> bool {
    matches!(self, InternalLinkType::TextCompositionStyle(_))
  }
  pub fn is_theme(&self) -> bool {
    matches!(self, InternalLinkType::Theme(_))
  }
  pub fn is_ton_connect(&self) -> bool {
    matches!(self, InternalLinkType::TonConnect(_))
  }
  pub fn is_ton_wallet_transfer(&self) -> bool {
    matches!(self, InternalLinkType::TonWalletTransfer(_))
  }
  pub fn is_unknown_deep_link(&self) -> bool {
    matches!(self, InternalLinkType::UnknownDeepLink(_))
  }
  pub fn is_upgraded_gift(&self) -> bool {
    matches!(self, InternalLinkType::UpgradedGift(_))
  }
  pub fn is_user_phone_number(&self) -> bool {
    matches!(self, InternalLinkType::UserPhoneNumber(_))
  }
  pub fn is_user_token(&self) -> bool {
    matches!(self, InternalLinkType::UserToken(_))
  }
  pub fn is_video_chat(&self) -> bool {
    matches!(self, InternalLinkType::VideoChat(_))
  }
  pub fn is_web_app(&self) -> bool {
    matches!(self, InternalLinkType::WebApp(_))
  }

  pub fn on_get_internal_link_type<F: FnOnce(&GetInternalLinkType)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::GetInternalLinkType(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_attachment_menu_bot<F: FnOnce(&InternalLinkTypeAttachmentMenuBot)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InternalLinkType::AttachmentMenuBot(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_authentication_code<F: FnOnce(&InternalLinkTypeAuthenticationCode)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InternalLinkType::AuthenticationCode(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_background<F: FnOnce(&InternalLinkTypeBackground)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::Background(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_bot_add_to_channel<F: FnOnce(&InternalLinkTypeBotAddToChannel)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InternalLinkType::BotAddToChannel(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_bot_start<F: FnOnce(&InternalLinkTypeBotStart)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::BotStart(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_bot_start_in_group<F: FnOnce(&InternalLinkTypeBotStartInGroup)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InternalLinkType::BotStartInGroup(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_business_chat<F: FnOnce(&InternalLinkTypeBusinessChat)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::BusinessChat(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_calls_page<F: FnOnce(&InternalLinkTypeCallsPage)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::CallsPage(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_affiliate_program<F: FnOnce(&InternalLinkTypeChatAffiliateProgram)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InternalLinkType::ChatAffiliateProgram(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_boost<F: FnOnce(&InternalLinkTypeChatBoost)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::ChatBoost(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_folder_invite<F: FnOnce(&InternalLinkTypeChatFolderInvite)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InternalLinkType::ChatFolderInvite(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_invite<F: FnOnce(&InternalLinkTypeChatInvite)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::ChatInvite(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_selection<F: FnOnce(&InternalLinkTypeChatSelection)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::ChatSelection(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_contacts_page<F: FnOnce(&InternalLinkTypeContactsPage)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::ContactsPage(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_direct_messages_chat<F: FnOnce(&InternalLinkTypeDirectMessagesChat)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InternalLinkType::DirectMessagesChat(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_game<F: FnOnce(&InternalLinkTypeGame)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::Game(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_gift_auction<F: FnOnce(&InternalLinkTypeGiftAuction)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::GiftAuction(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_gift_collection<F: FnOnce(&InternalLinkTypeGiftCollection)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::GiftCollection(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_group_call<F: FnOnce(&InternalLinkTypeGroupCall)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::GroupCall(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_instant_view<F: FnOnce(&InternalLinkTypeInstantView)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::InstantView(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_invoice<F: FnOnce(&InternalLinkTypeInvoice)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::Invoice(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_language_pack<F: FnOnce(&InternalLinkTypeLanguagePack)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::LanguagePack(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_live_story<F: FnOnce(&InternalLinkTypeLiveStory)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::LiveStory(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_main_web_app<F: FnOnce(&InternalLinkTypeMainWebApp)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::MainWebApp(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message<F: FnOnce(&InternalLinkTypeMessage)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::Message(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message_draft<F: FnOnce(&InternalLinkTypeMessageDraft)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::MessageDraft(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_my_profile_page<F: FnOnce(&InternalLinkTypeMyProfilePage)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::MyProfilePage(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_new_channel_chat<F: FnOnce(&InternalLinkTypeNewChannelChat)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::NewChannelChat(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_new_group_chat<F: FnOnce(&InternalLinkTypeNewGroupChat)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::NewGroupChat(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_new_private_chat<F: FnOnce(&InternalLinkTypeNewPrivateChat)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::NewPrivateChat(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_new_story<F: FnOnce(&InternalLinkTypeNewStory)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::NewStory(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_oauth<F: FnOnce(&InternalLinkTypeOauth)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::Oauth(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_passport_data_request<F: FnOnce(&InternalLinkTypePassportDataRequest)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InternalLinkType::PassportDataRequest(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_phone_number_confirmation<F: FnOnce(&InternalLinkTypePhoneNumberConfirmation)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InternalLinkType::PhoneNumberConfirmation(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_premium_features_page<F: FnOnce(&InternalLinkTypePremiumFeaturesPage)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InternalLinkType::PremiumFeaturesPage(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_premium_gift_code<F: FnOnce(&InternalLinkTypePremiumGiftCode)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::PremiumGiftCode(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_premium_gift_purchase<F: FnOnce(&InternalLinkTypePremiumGiftPurchase)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InternalLinkType::PremiumGiftPurchase(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_proxy<F: FnOnce(&InternalLinkTypeProxy)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::Proxy(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_public_chat<F: FnOnce(&InternalLinkTypePublicChat)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::PublicChat(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_qr_code_authentication<F: FnOnce(&InternalLinkTypeQrCodeAuthentication)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InternalLinkType::QrCodeAuthentication(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_request_managed_bot<F: FnOnce(&InternalLinkTypeRequestManagedBot)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InternalLinkType::RequestManagedBot(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_restore_purchases<F: FnOnce(&InternalLinkTypeRestorePurchases)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InternalLinkType::RestorePurchases(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_saved_messages<F: FnOnce(&InternalLinkTypeSavedMessages)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::SavedMessages(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_search<F: FnOnce(&InternalLinkTypeSearch)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::Search(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_settings<F: FnOnce(&InternalLinkTypeSettings)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::Settings(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_star_purchase<F: FnOnce(&InternalLinkTypeStarPurchase)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::StarPurchase(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_sticker_set<F: FnOnce(&InternalLinkTypeStickerSet)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::StickerSet(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_story<F: FnOnce(&InternalLinkTypeStory)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::Story(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_story_album<F: FnOnce(&InternalLinkTypeStoryAlbum)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::StoryAlbum(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_text_composition_style<F: FnOnce(&InternalLinkTypeTextCompositionStyle)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InternalLinkType::TextCompositionStyle(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_theme<F: FnOnce(&InternalLinkTypeTheme)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::Theme(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ton_connect<F: FnOnce(&InternalLinkTypeTonConnect)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::TonConnect(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ton_wallet_transfer<F: FnOnce(&InternalLinkTypeTonWalletTransfer)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InternalLinkType::TonWalletTransfer(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_unknown_deep_link<F: FnOnce(&InternalLinkTypeUnknownDeepLink)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::UnknownDeepLink(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_upgraded_gift<F: FnOnce(&InternalLinkTypeUpgradedGift)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::UpgradedGift(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_user_phone_number<F: FnOnce(&InternalLinkTypeUserPhoneNumber)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::UserPhoneNumber(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_user_token<F: FnOnce(&InternalLinkTypeUserToken)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::UserToken(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_video_chat<F: FnOnce(&InternalLinkTypeVideoChat)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::VideoChat(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_web_app<F: FnOnce(&InternalLinkTypeWebApp)>(&self, fnc: F) -> &Self {
    if let InternalLinkType::WebApp(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_get_internal_link_type(&self) -> Option<&GetInternalLinkType> {
    if let InternalLinkType::GetInternalLinkType(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_attachment_menu_bot(&self) -> Option<&InternalLinkTypeAttachmentMenuBot> {
    if let InternalLinkType::AttachmentMenuBot(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_authentication_code(&self) -> Option<&InternalLinkTypeAuthenticationCode> {
    if let InternalLinkType::AuthenticationCode(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_background(&self) -> Option<&InternalLinkTypeBackground> {
    if let InternalLinkType::Background(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_bot_add_to_channel(&self) -> Option<&InternalLinkTypeBotAddToChannel> {
    if let InternalLinkType::BotAddToChannel(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_bot_start(&self) -> Option<&InternalLinkTypeBotStart> {
    if let InternalLinkType::BotStart(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_bot_start_in_group(&self) -> Option<&InternalLinkTypeBotStartInGroup> {
    if let InternalLinkType::BotStartInGroup(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_business_chat(&self) -> Option<&InternalLinkTypeBusinessChat> {
    if let InternalLinkType::BusinessChat(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_calls_page(&self) -> Option<&InternalLinkTypeCallsPage> {
    if let InternalLinkType::CallsPage(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_affiliate_program(&self) -> Option<&InternalLinkTypeChatAffiliateProgram> {
    if let InternalLinkType::ChatAffiliateProgram(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_boost(&self) -> Option<&InternalLinkTypeChatBoost> {
    if let InternalLinkType::ChatBoost(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_folder_invite(&self) -> Option<&InternalLinkTypeChatFolderInvite> {
    if let InternalLinkType::ChatFolderInvite(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_invite(&self) -> Option<&InternalLinkTypeChatInvite> {
    if let InternalLinkType::ChatInvite(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_selection(&self) -> Option<&InternalLinkTypeChatSelection> {
    if let InternalLinkType::ChatSelection(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_contacts_page(&self) -> Option<&InternalLinkTypeContactsPage> {
    if let InternalLinkType::ContactsPage(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_direct_messages_chat(&self) -> Option<&InternalLinkTypeDirectMessagesChat> {
    if let InternalLinkType::DirectMessagesChat(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_game(&self) -> Option<&InternalLinkTypeGame> {
    if let InternalLinkType::Game(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_gift_auction(&self) -> Option<&InternalLinkTypeGiftAuction> {
    if let InternalLinkType::GiftAuction(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_gift_collection(&self) -> Option<&InternalLinkTypeGiftCollection> {
    if let InternalLinkType::GiftCollection(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_group_call(&self) -> Option<&InternalLinkTypeGroupCall> {
    if let InternalLinkType::GroupCall(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_instant_view(&self) -> Option<&InternalLinkTypeInstantView> {
    if let InternalLinkType::InstantView(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_invoice(&self) -> Option<&InternalLinkTypeInvoice> {
    if let InternalLinkType::Invoice(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_language_pack(&self) -> Option<&InternalLinkTypeLanguagePack> {
    if let InternalLinkType::LanguagePack(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_live_story(&self) -> Option<&InternalLinkTypeLiveStory> {
    if let InternalLinkType::LiveStory(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_main_web_app(&self) -> Option<&InternalLinkTypeMainWebApp> {
    if let InternalLinkType::MainWebApp(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message(&self) -> Option<&InternalLinkTypeMessage> {
    if let InternalLinkType::Message(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message_draft(&self) -> Option<&InternalLinkTypeMessageDraft> {
    if let InternalLinkType::MessageDraft(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_my_profile_page(&self) -> Option<&InternalLinkTypeMyProfilePage> {
    if let InternalLinkType::MyProfilePage(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_new_channel_chat(&self) -> Option<&InternalLinkTypeNewChannelChat> {
    if let InternalLinkType::NewChannelChat(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_new_group_chat(&self) -> Option<&InternalLinkTypeNewGroupChat> {
    if let InternalLinkType::NewGroupChat(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_new_private_chat(&self) -> Option<&InternalLinkTypeNewPrivateChat> {
    if let InternalLinkType::NewPrivateChat(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_new_story(&self) -> Option<&InternalLinkTypeNewStory> {
    if let InternalLinkType::NewStory(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_oauth(&self) -> Option<&InternalLinkTypeOauth> {
    if let InternalLinkType::Oauth(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_passport_data_request(&self) -> Option<&InternalLinkTypePassportDataRequest> {
    if let InternalLinkType::PassportDataRequest(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_phone_number_confirmation(&self) -> Option<&InternalLinkTypePhoneNumberConfirmation> {
    if let InternalLinkType::PhoneNumberConfirmation(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_premium_features_page(&self) -> Option<&InternalLinkTypePremiumFeaturesPage> {
    if let InternalLinkType::PremiumFeaturesPage(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_premium_gift_code(&self) -> Option<&InternalLinkTypePremiumGiftCode> {
    if let InternalLinkType::PremiumGiftCode(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_premium_gift_purchase(&self) -> Option<&InternalLinkTypePremiumGiftPurchase> {
    if let InternalLinkType::PremiumGiftPurchase(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_proxy(&self) -> Option<&InternalLinkTypeProxy> {
    if let InternalLinkType::Proxy(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_public_chat(&self) -> Option<&InternalLinkTypePublicChat> {
    if let InternalLinkType::PublicChat(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_qr_code_authentication(&self) -> Option<&InternalLinkTypeQrCodeAuthentication> {
    if let InternalLinkType::QrCodeAuthentication(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_request_managed_bot(&self) -> Option<&InternalLinkTypeRequestManagedBot> {
    if let InternalLinkType::RequestManagedBot(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_restore_purchases(&self) -> Option<&InternalLinkTypeRestorePurchases> {
    if let InternalLinkType::RestorePurchases(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_saved_messages(&self) -> Option<&InternalLinkTypeSavedMessages> {
    if let InternalLinkType::SavedMessages(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_search(&self) -> Option<&InternalLinkTypeSearch> {
    if let InternalLinkType::Search(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_settings(&self) -> Option<&InternalLinkTypeSettings> {
    if let InternalLinkType::Settings(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_star_purchase(&self) -> Option<&InternalLinkTypeStarPurchase> {
    if let InternalLinkType::StarPurchase(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_sticker_set(&self) -> Option<&InternalLinkTypeStickerSet> {
    if let InternalLinkType::StickerSet(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_story(&self) -> Option<&InternalLinkTypeStory> {
    if let InternalLinkType::Story(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_story_album(&self) -> Option<&InternalLinkTypeStoryAlbum> {
    if let InternalLinkType::StoryAlbum(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_text_composition_style(&self) -> Option<&InternalLinkTypeTextCompositionStyle> {
    if let InternalLinkType::TextCompositionStyle(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_theme(&self) -> Option<&InternalLinkTypeTheme> {
    if let InternalLinkType::Theme(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ton_connect(&self) -> Option<&InternalLinkTypeTonConnect> {
    if let InternalLinkType::TonConnect(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ton_wallet_transfer(&self) -> Option<&InternalLinkTypeTonWalletTransfer> {
    if let InternalLinkType::TonWalletTransfer(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_unknown_deep_link(&self) -> Option<&InternalLinkTypeUnknownDeepLink> {
    if let InternalLinkType::UnknownDeepLink(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_upgraded_gift(&self) -> Option<&InternalLinkTypeUpgradedGift> {
    if let InternalLinkType::UpgradedGift(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_user_phone_number(&self) -> Option<&InternalLinkTypeUserPhoneNumber> {
    if let InternalLinkType::UserPhoneNumber(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_user_token(&self) -> Option<&InternalLinkTypeUserToken> {
    if let InternalLinkType::UserToken(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_video_chat(&self) -> Option<&InternalLinkTypeVideoChat> {
    if let InternalLinkType::VideoChat(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_web_app(&self) -> Option<&InternalLinkTypeWebApp> {
    if let InternalLinkType::WebApp(t) = self {
      return Some(t);
    }
    None
  }

  pub fn get_internal_link_type<T: AsRef<GetInternalLinkType>>(t: T) -> Self {
    InternalLinkType::GetInternalLinkType(t.as_ref().clone())
  }

  pub fn attachment_menu_bot<T: AsRef<InternalLinkTypeAttachmentMenuBot>>(t: T) -> Self {
    InternalLinkType::AttachmentMenuBot(t.as_ref().clone())
  }

  pub fn authentication_code<T: AsRef<InternalLinkTypeAuthenticationCode>>(t: T) -> Self {
    InternalLinkType::AuthenticationCode(t.as_ref().clone())
  }

  pub fn background<T: AsRef<InternalLinkTypeBackground>>(t: T) -> Self {
    InternalLinkType::Background(t.as_ref().clone())
  }

  pub fn bot_add_to_channel<T: AsRef<InternalLinkTypeBotAddToChannel>>(t: T) -> Self {
    InternalLinkType::BotAddToChannel(t.as_ref().clone())
  }

  pub fn bot_start<T: AsRef<InternalLinkTypeBotStart>>(t: T) -> Self {
    InternalLinkType::BotStart(t.as_ref().clone())
  }

  pub fn bot_start_in_group<T: AsRef<InternalLinkTypeBotStartInGroup>>(t: T) -> Self {
    InternalLinkType::BotStartInGroup(t.as_ref().clone())
  }

  pub fn business_chat<T: AsRef<InternalLinkTypeBusinessChat>>(t: T) -> Self {
    InternalLinkType::BusinessChat(t.as_ref().clone())
  }

  pub fn calls_page<T: AsRef<InternalLinkTypeCallsPage>>(t: T) -> Self {
    InternalLinkType::CallsPage(t.as_ref().clone())
  }

  pub fn chat_affiliate_program<T: AsRef<InternalLinkTypeChatAffiliateProgram>>(t: T) -> Self {
    InternalLinkType::ChatAffiliateProgram(t.as_ref().clone())
  }

  pub fn chat_boost<T: AsRef<InternalLinkTypeChatBoost>>(t: T) -> Self {
    InternalLinkType::ChatBoost(t.as_ref().clone())
  }

  pub fn chat_folder_invite<T: AsRef<InternalLinkTypeChatFolderInvite>>(t: T) -> Self {
    InternalLinkType::ChatFolderInvite(t.as_ref().clone())
  }

  pub fn chat_invite<T: AsRef<InternalLinkTypeChatInvite>>(t: T) -> Self {
    InternalLinkType::ChatInvite(t.as_ref().clone())
  }

  pub fn chat_selection<T: AsRef<InternalLinkTypeChatSelection>>(t: T) -> Self {
    InternalLinkType::ChatSelection(t.as_ref().clone())
  }

  pub fn contacts_page<T: AsRef<InternalLinkTypeContactsPage>>(t: T) -> Self {
    InternalLinkType::ContactsPage(t.as_ref().clone())
  }

  pub fn direct_messages_chat<T: AsRef<InternalLinkTypeDirectMessagesChat>>(t: T) -> Self {
    InternalLinkType::DirectMessagesChat(t.as_ref().clone())
  }

  pub fn game<T: AsRef<InternalLinkTypeGame>>(t: T) -> Self {
    InternalLinkType::Game(t.as_ref().clone())
  }

  pub fn gift_auction<T: AsRef<InternalLinkTypeGiftAuction>>(t: T) -> Self {
    InternalLinkType::GiftAuction(t.as_ref().clone())
  }

  pub fn gift_collection<T: AsRef<InternalLinkTypeGiftCollection>>(t: T) -> Self {
    InternalLinkType::GiftCollection(t.as_ref().clone())
  }

  pub fn group_call<T: AsRef<InternalLinkTypeGroupCall>>(t: T) -> Self {
    InternalLinkType::GroupCall(t.as_ref().clone())
  }

  pub fn instant_view<T: AsRef<InternalLinkTypeInstantView>>(t: T) -> Self {
    InternalLinkType::InstantView(t.as_ref().clone())
  }

  pub fn invoice<T: AsRef<InternalLinkTypeInvoice>>(t: T) -> Self {
    InternalLinkType::Invoice(t.as_ref().clone())
  }

  pub fn language_pack<T: AsRef<InternalLinkTypeLanguagePack>>(t: T) -> Self {
    InternalLinkType::LanguagePack(t.as_ref().clone())
  }

  pub fn live_story<T: AsRef<InternalLinkTypeLiveStory>>(t: T) -> Self {
    InternalLinkType::LiveStory(t.as_ref().clone())
  }

  pub fn main_web_app<T: AsRef<InternalLinkTypeMainWebApp>>(t: T) -> Self {
    InternalLinkType::MainWebApp(t.as_ref().clone())
  }

  pub fn message<T: AsRef<InternalLinkTypeMessage>>(t: T) -> Self {
    InternalLinkType::Message(t.as_ref().clone())
  }

  pub fn message_draft<T: AsRef<InternalLinkTypeMessageDraft>>(t: T) -> Self {
    InternalLinkType::MessageDraft(t.as_ref().clone())
  }

  pub fn my_profile_page<T: AsRef<InternalLinkTypeMyProfilePage>>(t: T) -> Self {
    InternalLinkType::MyProfilePage(t.as_ref().clone())
  }

  pub fn new_channel_chat<T: AsRef<InternalLinkTypeNewChannelChat>>(t: T) -> Self {
    InternalLinkType::NewChannelChat(t.as_ref().clone())
  }

  pub fn new_group_chat<T: AsRef<InternalLinkTypeNewGroupChat>>(t: T) -> Self {
    InternalLinkType::NewGroupChat(t.as_ref().clone())
  }

  pub fn new_private_chat<T: AsRef<InternalLinkTypeNewPrivateChat>>(t: T) -> Self {
    InternalLinkType::NewPrivateChat(t.as_ref().clone())
  }

  pub fn new_story<T: AsRef<InternalLinkTypeNewStory>>(t: T) -> Self {
    InternalLinkType::NewStory(t.as_ref().clone())
  }

  pub fn oauth<T: AsRef<InternalLinkTypeOauth>>(t: T) -> Self {
    InternalLinkType::Oauth(t.as_ref().clone())
  }

  pub fn passport_data_request<T: AsRef<InternalLinkTypePassportDataRequest>>(t: T) -> Self {
    InternalLinkType::PassportDataRequest(t.as_ref().clone())
  }

  pub fn phone_number_confirmation<T: AsRef<InternalLinkTypePhoneNumberConfirmation>>(
    t: T,
  ) -> Self {
    InternalLinkType::PhoneNumberConfirmation(t.as_ref().clone())
  }

  pub fn premium_features_page<T: AsRef<InternalLinkTypePremiumFeaturesPage>>(t: T) -> Self {
    InternalLinkType::PremiumFeaturesPage(t.as_ref().clone())
  }

  pub fn premium_gift_code<T: AsRef<InternalLinkTypePremiumGiftCode>>(t: T) -> Self {
    InternalLinkType::PremiumGiftCode(t.as_ref().clone())
  }

  pub fn premium_gift_purchase<T: AsRef<InternalLinkTypePremiumGiftPurchase>>(t: T) -> Self {
    InternalLinkType::PremiumGiftPurchase(t.as_ref().clone())
  }

  pub fn proxy<T: AsRef<InternalLinkTypeProxy>>(t: T) -> Self {
    InternalLinkType::Proxy(t.as_ref().clone())
  }

  pub fn public_chat<T: AsRef<InternalLinkTypePublicChat>>(t: T) -> Self {
    InternalLinkType::PublicChat(t.as_ref().clone())
  }

  pub fn qr_code_authentication<T: AsRef<InternalLinkTypeQrCodeAuthentication>>(t: T) -> Self {
    InternalLinkType::QrCodeAuthentication(t.as_ref().clone())
  }

  pub fn request_managed_bot<T: AsRef<InternalLinkTypeRequestManagedBot>>(t: T) -> Self {
    InternalLinkType::RequestManagedBot(t.as_ref().clone())
  }

  pub fn restore_purchases<T: AsRef<InternalLinkTypeRestorePurchases>>(t: T) -> Self {
    InternalLinkType::RestorePurchases(t.as_ref().clone())
  }

  pub fn saved_messages<T: AsRef<InternalLinkTypeSavedMessages>>(t: T) -> Self {
    InternalLinkType::SavedMessages(t.as_ref().clone())
  }

  pub fn search<T: AsRef<InternalLinkTypeSearch>>(t: T) -> Self {
    InternalLinkType::Search(t.as_ref().clone())
  }

  pub fn settings<T: AsRef<InternalLinkTypeSettings>>(t: T) -> Self {
    InternalLinkType::Settings(t.as_ref().clone())
  }

  pub fn star_purchase<T: AsRef<InternalLinkTypeStarPurchase>>(t: T) -> Self {
    InternalLinkType::StarPurchase(t.as_ref().clone())
  }

  pub fn sticker_set<T: AsRef<InternalLinkTypeStickerSet>>(t: T) -> Self {
    InternalLinkType::StickerSet(t.as_ref().clone())
  }

  pub fn story<T: AsRef<InternalLinkTypeStory>>(t: T) -> Self {
    InternalLinkType::Story(t.as_ref().clone())
  }

  pub fn story_album<T: AsRef<InternalLinkTypeStoryAlbum>>(t: T) -> Self {
    InternalLinkType::StoryAlbum(t.as_ref().clone())
  }

  pub fn text_composition_style<T: AsRef<InternalLinkTypeTextCompositionStyle>>(t: T) -> Self {
    InternalLinkType::TextCompositionStyle(t.as_ref().clone())
  }

  pub fn theme<T: AsRef<InternalLinkTypeTheme>>(t: T) -> Self {
    InternalLinkType::Theme(t.as_ref().clone())
  }

  pub fn ton_connect<T: AsRef<InternalLinkTypeTonConnect>>(t: T) -> Self {
    InternalLinkType::TonConnect(t.as_ref().clone())
  }

  pub fn ton_wallet_transfer<T: AsRef<InternalLinkTypeTonWalletTransfer>>(t: T) -> Self {
    InternalLinkType::TonWalletTransfer(t.as_ref().clone())
  }

  pub fn unknown_deep_link<T: AsRef<InternalLinkTypeUnknownDeepLink>>(t: T) -> Self {
    InternalLinkType::UnknownDeepLink(t.as_ref().clone())
  }

  pub fn upgraded_gift<T: AsRef<InternalLinkTypeUpgradedGift>>(t: T) -> Self {
    InternalLinkType::UpgradedGift(t.as_ref().clone())
  }

  pub fn user_phone_number<T: AsRef<InternalLinkTypeUserPhoneNumber>>(t: T) -> Self {
    InternalLinkType::UserPhoneNumber(t.as_ref().clone())
  }

  pub fn user_token<T: AsRef<InternalLinkTypeUserToken>>(t: T) -> Self {
    InternalLinkType::UserToken(t.as_ref().clone())
  }

  pub fn video_chat<T: AsRef<InternalLinkTypeVideoChat>>(t: T) -> Self {
    InternalLinkType::VideoChat(t.as_ref().clone())
  }

  pub fn web_app<T: AsRef<InternalLinkTypeWebApp>>(t: T) -> Self {
    InternalLinkType::WebApp(t.as_ref().clone())
  }
}

impl AsRef<InternalLinkType> for InternalLinkType {
  fn as_ref(&self) -> &InternalLinkType {
    self
  }
}

/// The link is a link to an attachment menu bot to be opened in the specified or a chosen chat. Process given target_chat to open the chat. Then, call searchPublicChat with the given bot username, check that the user is a bot and can be added to attachment menu. Then, use getAttachmentMenuBot to receive information about the bot. If the bot isn't added to attachment menu, then show a disclaimer about Mini Apps being third-party applications, ask the user to accept their Terms of service and confirm adding the bot to side and attachment menu. If the user accepts the terms and confirms adding, then use toggleBotIsAddedToAttachmentMenu to add the bot. If the attachment menu bot can't be used in the opened chat, show an error to the user. If the bot is added to attachment menu and can be used in the chat, then use openWebApp with the given URL
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeAttachmentMenuBot {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Target chat to be opened
  target_chat: TargetChat,
  /// Username of the bot
  bot_username: String,
  /// URL to be passed to openWebApp
  url: String,
}

impl RObject for InternalLinkTypeAttachmentMenuBot {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeAttachmentMenuBot"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeAttachmentMenuBot {}

impl InternalLinkTypeAttachmentMenuBot {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeAttachmentMenuBotBuilder {
    let inner = InternalLinkTypeAttachmentMenuBot {
      td_name: "internalLinkTypeAttachmentMenuBot".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeAttachmentMenuBotBuilder { inner }
  }

  pub fn target_chat(&self) -> &TargetChat {
    &self.target_chat
  }

  pub fn bot_username(&self) -> &String {
    &self.bot_username
  }

  pub fn url(&self) -> &String {
    &self.url
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeAttachmentMenuBotBuilder {
  inner: InternalLinkTypeAttachmentMenuBot,
}

impl RTDInternalLinkTypeAttachmentMenuBotBuilder {
  pub fn build(&self) -> InternalLinkTypeAttachmentMenuBot {
    self.inner.clone()
  }

  pub fn target_chat<T: AsRef<TargetChat>>(&mut self, target_chat: T) -> &mut Self {
    self.inner.target_chat = target_chat.as_ref().clone();
    self
  }

  pub fn bot_username<T: AsRef<str>>(&mut self, bot_username: T) -> &mut Self {
    self.inner.bot_username = bot_username.as_ref().to_string();
    self
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeAttachmentMenuBot> for InternalLinkTypeAttachmentMenuBot {
  fn as_ref(&self) -> &InternalLinkTypeAttachmentMenuBot {
    self
  }
}

impl AsRef<InternalLinkTypeAttachmentMenuBot> for RTDInternalLinkTypeAttachmentMenuBotBuilder {
  fn as_ref(&self) -> &InternalLinkTypeAttachmentMenuBot {
    &self.inner
  }
}

/// The link contains an authentication code. Call checkAuthenticationCode with the code if the current authorization state is authorizationStateWaitCode
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeAuthenticationCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The authentication code
  code: String,
}

impl RObject for InternalLinkTypeAuthenticationCode {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeAuthenticationCode"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeAuthenticationCode {}

impl InternalLinkTypeAuthenticationCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeAuthenticationCodeBuilder {
    let inner = InternalLinkTypeAuthenticationCode {
      td_name: "internalLinkTypeAuthenticationCode".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeAuthenticationCodeBuilder { inner }
  }

  pub fn code(&self) -> &String {
    &self.code
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeAuthenticationCodeBuilder {
  inner: InternalLinkTypeAuthenticationCode,
}

impl RTDInternalLinkTypeAuthenticationCodeBuilder {
  pub fn build(&self) -> InternalLinkTypeAuthenticationCode {
    self.inner.clone()
  }

  pub fn code<T: AsRef<str>>(&mut self, code: T) -> &mut Self {
    self.inner.code = code.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeAuthenticationCode> for InternalLinkTypeAuthenticationCode {
  fn as_ref(&self) -> &InternalLinkTypeAuthenticationCode {
    self
  }
}

impl AsRef<InternalLinkTypeAuthenticationCode> for RTDInternalLinkTypeAuthenticationCodeBuilder {
  fn as_ref(&self) -> &InternalLinkTypeAuthenticationCode {
    &self.inner
  }
}

/// The link is a link to a background. Call searchBackground with the given background name to process the link. If background is found and the user wants to apply it, then call setDefaultBackground
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeBackground {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the background
  background_name: String,
}

impl RObject for InternalLinkTypeBackground {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeBackground"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeBackground {}

impl InternalLinkTypeBackground {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeBackgroundBuilder {
    let inner = InternalLinkTypeBackground {
      td_name: "internalLinkTypeBackground".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeBackgroundBuilder { inner }
  }

  pub fn background_name(&self) -> &String {
    &self.background_name
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeBackgroundBuilder {
  inner: InternalLinkTypeBackground,
}

impl RTDInternalLinkTypeBackgroundBuilder {
  pub fn build(&self) -> InternalLinkTypeBackground {
    self.inner.clone()
  }

  pub fn background_name<T: AsRef<str>>(&mut self, background_name: T) -> &mut Self {
    self.inner.background_name = background_name.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeBackground> for InternalLinkTypeBackground {
  fn as_ref(&self) -> &InternalLinkTypeBackground {
    self
  }
}

impl AsRef<InternalLinkTypeBackground> for RTDInternalLinkTypeBackgroundBuilder {
  fn as_ref(&self) -> &InternalLinkTypeBackground {
    &self.inner
  }
}

/// The link is a link to a Telegram bot, which is expected to be added to a channel chat as an administrator. Call searchPublicChat with the given bot username and check that the user is a bot, ask the current user to select a channel chat to add the bot to as an administrator. Then, call getChatMember to receive the current bot rights in the chat and if the bot already is an administrator, check that the current user can edit its administrator rights and combine received rights with the requested administrator rights. Then, show confirmation box to the user, and call setChatMemberStatus with the chosen chat and confirmed rights
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeBotAddToChannel {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Username of the bot
  bot_username: String,
  /// Expected administrator rights for the bot
  administrator_rights: ChatAdministratorRights,
}

impl RObject for InternalLinkTypeBotAddToChannel {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeBotAddToChannel"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeBotAddToChannel {}

impl InternalLinkTypeBotAddToChannel {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeBotAddToChannelBuilder {
    let inner = InternalLinkTypeBotAddToChannel {
      td_name: "internalLinkTypeBotAddToChannel".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeBotAddToChannelBuilder { inner }
  }

  pub fn bot_username(&self) -> &String {
    &self.bot_username
  }

  pub fn administrator_rights(&self) -> &ChatAdministratorRights {
    &self.administrator_rights
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeBotAddToChannelBuilder {
  inner: InternalLinkTypeBotAddToChannel,
}

impl RTDInternalLinkTypeBotAddToChannelBuilder {
  pub fn build(&self) -> InternalLinkTypeBotAddToChannel {
    self.inner.clone()
  }

  pub fn bot_username<T: AsRef<str>>(&mut self, bot_username: T) -> &mut Self {
    self.inner.bot_username = bot_username.as_ref().to_string();
    self
  }

  pub fn administrator_rights<T: AsRef<ChatAdministratorRights>>(
    &mut self,
    administrator_rights: T,
  ) -> &mut Self {
    self.inner.administrator_rights = administrator_rights.as_ref().clone();
    self
  }
}

impl AsRef<InternalLinkTypeBotAddToChannel> for InternalLinkTypeBotAddToChannel {
  fn as_ref(&self) -> &InternalLinkTypeBotAddToChannel {
    self
  }
}

impl AsRef<InternalLinkTypeBotAddToChannel> for RTDInternalLinkTypeBotAddToChannelBuilder {
  fn as_ref(&self) -> &InternalLinkTypeBotAddToChannel {
    &self.inner
  }
}

/// The link is a link to a chat with a Telegram bot. Call searchPublicChat with the given bot username, check that the user is a bot, show START button in the chat with the bot, and then call sendBotStartMessage with the given start parameter after the button is pressed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeBotStart {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Username of the bot
  bot_username: String,
  /// The parameter to be passed to sendBotStartMessage
  start_parameter: String,
  /// True, if sendBotStartMessage must be called automatically without showing the START button
  autostart: bool,
}

impl RObject for InternalLinkTypeBotStart {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeBotStart"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeBotStart {}

impl InternalLinkTypeBotStart {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeBotStartBuilder {
    let inner = InternalLinkTypeBotStart {
      td_name: "internalLinkTypeBotStart".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeBotStartBuilder { inner }
  }

  pub fn bot_username(&self) -> &String {
    &self.bot_username
  }

  pub fn start_parameter(&self) -> &String {
    &self.start_parameter
  }

  pub fn autostart(&self) -> bool {
    self.autostart
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeBotStartBuilder {
  inner: InternalLinkTypeBotStart,
}

impl RTDInternalLinkTypeBotStartBuilder {
  pub fn build(&self) -> InternalLinkTypeBotStart {
    self.inner.clone()
  }

  pub fn bot_username<T: AsRef<str>>(&mut self, bot_username: T) -> &mut Self {
    self.inner.bot_username = bot_username.as_ref().to_string();
    self
  }

  pub fn start_parameter<T: AsRef<str>>(&mut self, start_parameter: T) -> &mut Self {
    self.inner.start_parameter = start_parameter.as_ref().to_string();
    self
  }

  pub fn autostart(&mut self, autostart: bool) -> &mut Self {
    self.inner.autostart = autostart;
    self
  }
}

impl AsRef<InternalLinkTypeBotStart> for InternalLinkTypeBotStart {
  fn as_ref(&self) -> &InternalLinkTypeBotStart {
    self
  }
}

impl AsRef<InternalLinkTypeBotStart> for RTDInternalLinkTypeBotStartBuilder {
  fn as_ref(&self) -> &InternalLinkTypeBotStart {
    &self.inner
  }
}

/// The link is a link to a Telegram bot, which is expected to be added to a group chat. Call searchPublicChat with the given bot username, check that the user is a bot and can be added to groups, ask the current user to select a basic group or a supergroup chat to add the bot to, taking into account that bots can be added to a public supergroup only by administrators of the supergroup. If administrator rights are provided by the link, call getChatMember to receive the current bot rights in the chat and if the bot already is an administrator, check that the current user can edit its administrator rights, combine received rights with the requested administrator rights, show confirmation box to the user, and call setChatMemberStatus with the chosen chat and confirmed administrator rights. Before call to setChatMemberStatus it may be required to upgrade the chosen basic group chat to a supergroup chat. Then, if start_parameter isn't empty, call sendBotStartMessage with the given start parameter and the chosen chat; otherwise, just send /start message with bot's username added to the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeBotStartInGroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Username of the bot
  bot_username: String,
  /// The parameter to be passed to sendBotStartMessage
  start_parameter: String,
  /// Expected administrator rights for the bot; may be null
  administrator_rights: Option<ChatAdministratorRights>,
}

impl RObject for InternalLinkTypeBotStartInGroup {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeBotStartInGroup"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeBotStartInGroup {}

impl InternalLinkTypeBotStartInGroup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeBotStartInGroupBuilder {
    let inner = InternalLinkTypeBotStartInGroup {
      td_name: "internalLinkTypeBotStartInGroup".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeBotStartInGroupBuilder { inner }
  }

  pub fn bot_username(&self) -> &String {
    &self.bot_username
  }

  pub fn start_parameter(&self) -> &String {
    &self.start_parameter
  }

  pub fn administrator_rights(&self) -> &Option<ChatAdministratorRights> {
    &self.administrator_rights
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeBotStartInGroupBuilder {
  inner: InternalLinkTypeBotStartInGroup,
}

impl RTDInternalLinkTypeBotStartInGroupBuilder {
  pub fn build(&self) -> InternalLinkTypeBotStartInGroup {
    self.inner.clone()
  }

  pub fn bot_username<T: AsRef<str>>(&mut self, bot_username: T) -> &mut Self {
    self.inner.bot_username = bot_username.as_ref().to_string();
    self
  }

  pub fn start_parameter<T: AsRef<str>>(&mut self, start_parameter: T) -> &mut Self {
    self.inner.start_parameter = start_parameter.as_ref().to_string();
    self
  }

  pub fn administrator_rights<T: AsRef<ChatAdministratorRights>>(
    &mut self,
    administrator_rights: T,
  ) -> &mut Self {
    self.inner.administrator_rights = Some(administrator_rights.as_ref().clone());
    self
  }
}

impl AsRef<InternalLinkTypeBotStartInGroup> for InternalLinkTypeBotStartInGroup {
  fn as_ref(&self) -> &InternalLinkTypeBotStartInGroup {
    self
  }
}

impl AsRef<InternalLinkTypeBotStartInGroup> for RTDInternalLinkTypeBotStartInGroupBuilder {
  fn as_ref(&self) -> &InternalLinkTypeBotStartInGroup {
    &self.inner
  }
}

/// The link is a link to a business chat. Use getBusinessChatLinkInfo with the provided link name to get information about the link, then open received private chat and replace chat draft with the provided text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeBusinessChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the link
  link_name: String,
}

impl RObject for InternalLinkTypeBusinessChat {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeBusinessChat"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeBusinessChat {}

impl InternalLinkTypeBusinessChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeBusinessChatBuilder {
    let inner = InternalLinkTypeBusinessChat {
      td_name: "internalLinkTypeBusinessChat".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeBusinessChatBuilder { inner }
  }

  pub fn link_name(&self) -> &String {
    &self.link_name
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeBusinessChatBuilder {
  inner: InternalLinkTypeBusinessChat,
}

impl RTDInternalLinkTypeBusinessChatBuilder {
  pub fn build(&self) -> InternalLinkTypeBusinessChat {
    self.inner.clone()
  }

  pub fn link_name<T: AsRef<str>>(&mut self, link_name: T) -> &mut Self {
    self.inner.link_name = link_name.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeBusinessChat> for InternalLinkTypeBusinessChat {
  fn as_ref(&self) -> &InternalLinkTypeBusinessChat {
    self
  }
}

impl AsRef<InternalLinkTypeBusinessChat> for RTDInternalLinkTypeBusinessChatBuilder {
  fn as_ref(&self) -> &InternalLinkTypeBusinessChat {
    &self.inner
  }
}

/// The link is a link to the Call tab or page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeCallsPage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Section of the page; may be one of "", "all", "missed", "edit", "show-tab", "start-call"
  section: String,
}

impl RObject for InternalLinkTypeCallsPage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeCallsPage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeCallsPage {}

impl InternalLinkTypeCallsPage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeCallsPageBuilder {
    let inner = InternalLinkTypeCallsPage {
      td_name: "internalLinkTypeCallsPage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeCallsPageBuilder { inner }
  }

  pub fn section(&self) -> &String {
    &self.section
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeCallsPageBuilder {
  inner: InternalLinkTypeCallsPage,
}

impl RTDInternalLinkTypeCallsPageBuilder {
  pub fn build(&self) -> InternalLinkTypeCallsPage {
    self.inner.clone()
  }

  pub fn section<T: AsRef<str>>(&mut self, section: T) -> &mut Self {
    self.inner.section = section.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeCallsPage> for InternalLinkTypeCallsPage {
  fn as_ref(&self) -> &InternalLinkTypeCallsPage {
    self
  }
}

impl AsRef<InternalLinkTypeCallsPage> for RTDInternalLinkTypeCallsPageBuilder {
  fn as_ref(&self) -> &InternalLinkTypeCallsPage {
    &self.inner
  }
}

/// The link is an affiliate program link. Call searchChatAffiliateProgram with the given username and referrer to process the link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeChatAffiliateProgram {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Username to be passed to searchChatAffiliateProgram
  username: String,
  /// Referrer to be passed to searchChatAffiliateProgram
  referrer: String,
}

impl RObject for InternalLinkTypeChatAffiliateProgram {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeChatAffiliateProgram"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeChatAffiliateProgram {}

impl InternalLinkTypeChatAffiliateProgram {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeChatAffiliateProgramBuilder {
    let inner = InternalLinkTypeChatAffiliateProgram {
      td_name: "internalLinkTypeChatAffiliateProgram".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeChatAffiliateProgramBuilder { inner }
  }

  pub fn username(&self) -> &String {
    &self.username
  }

  pub fn referrer(&self) -> &String {
    &self.referrer
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeChatAffiliateProgramBuilder {
  inner: InternalLinkTypeChatAffiliateProgram,
}

impl RTDInternalLinkTypeChatAffiliateProgramBuilder {
  pub fn build(&self) -> InternalLinkTypeChatAffiliateProgram {
    self.inner.clone()
  }

  pub fn username<T: AsRef<str>>(&mut self, username: T) -> &mut Self {
    self.inner.username = username.as_ref().to_string();
    self
  }

  pub fn referrer<T: AsRef<str>>(&mut self, referrer: T) -> &mut Self {
    self.inner.referrer = referrer.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeChatAffiliateProgram> for InternalLinkTypeChatAffiliateProgram {
  fn as_ref(&self) -> &InternalLinkTypeChatAffiliateProgram {
    self
  }
}

impl AsRef<InternalLinkTypeChatAffiliateProgram>
  for RTDInternalLinkTypeChatAffiliateProgramBuilder
{
  fn as_ref(&self) -> &InternalLinkTypeChatAffiliateProgram {
    &self.inner
  }
}

/// The link is a link to boost a Telegram chat. Call getChatBoostLinkInfo with the given URL to process the link. If the chat is found, then call getChatBoostStatus and getAvailableChatBoostSlots to get the current boost status and check whether the chat can be boosted. If the user wants to boost the chat and the chat can be boosted, then call boostChat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeChatBoost {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// URL to be passed to getChatBoostLinkInfo
  url: String,
}

impl RObject for InternalLinkTypeChatBoost {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeChatBoost"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeChatBoost {}

impl InternalLinkTypeChatBoost {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeChatBoostBuilder {
    let inner = InternalLinkTypeChatBoost {
      td_name: "internalLinkTypeChatBoost".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeChatBoostBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeChatBoostBuilder {
  inner: InternalLinkTypeChatBoost,
}

impl RTDInternalLinkTypeChatBoostBuilder {
  pub fn build(&self) -> InternalLinkTypeChatBoost {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeChatBoost> for InternalLinkTypeChatBoost {
  fn as_ref(&self) -> &InternalLinkTypeChatBoost {
    self
  }
}

impl AsRef<InternalLinkTypeChatBoost> for RTDInternalLinkTypeChatBoostBuilder {
  fn as_ref(&self) -> &InternalLinkTypeChatBoost {
    &self.inner
  }
}

/// The link is an invite link to a chat folder. Call checkChatFolderInviteLink with the given invite link to process the link. If the link is valid and the user wants to join the chat folder, then call addChatFolderByInviteLink
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeChatFolderInvite {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Internal representation of the invite link
  invite_link: String,
}

impl RObject for InternalLinkTypeChatFolderInvite {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeChatFolderInvite"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeChatFolderInvite {}

impl InternalLinkTypeChatFolderInvite {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeChatFolderInviteBuilder {
    let inner = InternalLinkTypeChatFolderInvite {
      td_name: "internalLinkTypeChatFolderInvite".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeChatFolderInviteBuilder { inner }
  }

  pub fn invite_link(&self) -> &String {
    &self.invite_link
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeChatFolderInviteBuilder {
  inner: InternalLinkTypeChatFolderInvite,
}

impl RTDInternalLinkTypeChatFolderInviteBuilder {
  pub fn build(&self) -> InternalLinkTypeChatFolderInvite {
    self.inner.clone()
  }

  pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
    self.inner.invite_link = invite_link.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeChatFolderInvite> for InternalLinkTypeChatFolderInvite {
  fn as_ref(&self) -> &InternalLinkTypeChatFolderInvite {
    self
  }
}

impl AsRef<InternalLinkTypeChatFolderInvite> for RTDInternalLinkTypeChatFolderInviteBuilder {
  fn as_ref(&self) -> &InternalLinkTypeChatFolderInvite {
    &self.inner
  }
}

/// The link is a chat invite link. Call checkChatInviteLink with the given invite link to process the link. If the link is valid and the user wants to join the chat, then call joinChatByInviteLink
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeChatInvite {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Internal representation of the invite link
  invite_link: String,
}

impl RObject for InternalLinkTypeChatInvite {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeChatInvite"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeChatInvite {}

impl InternalLinkTypeChatInvite {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeChatInviteBuilder {
    let inner = InternalLinkTypeChatInvite {
      td_name: "internalLinkTypeChatInvite".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeChatInviteBuilder { inner }
  }

  pub fn invite_link(&self) -> &String {
    &self.invite_link
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeChatInviteBuilder {
  inner: InternalLinkTypeChatInvite,
}

impl RTDInternalLinkTypeChatInviteBuilder {
  pub fn build(&self) -> InternalLinkTypeChatInvite {
    self.inner.clone()
  }

  pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
    self.inner.invite_link = invite_link.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeChatInvite> for InternalLinkTypeChatInvite {
  fn as_ref(&self) -> &InternalLinkTypeChatInvite {
    self
  }
}

impl AsRef<InternalLinkTypeChatInvite> for RTDInternalLinkTypeChatInviteBuilder {
  fn as_ref(&self) -> &InternalLinkTypeChatInvite {
    &self.inner
  }
}

/// The link is a link that allows to select some chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeChatSelection {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for InternalLinkTypeChatSelection {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeChatSelection"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeChatSelection {}

impl InternalLinkTypeChatSelection {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeChatSelectionBuilder {
    let inner = InternalLinkTypeChatSelection {
      td_name: "internalLinkTypeChatSelection".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDInternalLinkTypeChatSelectionBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeChatSelectionBuilder {
  inner: InternalLinkTypeChatSelection,
}

impl RTDInternalLinkTypeChatSelectionBuilder {
  pub fn build(&self) -> InternalLinkTypeChatSelection {
    self.inner.clone()
  }
}

impl AsRef<InternalLinkTypeChatSelection> for InternalLinkTypeChatSelection {
  fn as_ref(&self) -> &InternalLinkTypeChatSelection {
    self
  }
}

impl AsRef<InternalLinkTypeChatSelection> for RTDInternalLinkTypeChatSelectionBuilder {
  fn as_ref(&self) -> &InternalLinkTypeChatSelection {
    &self.inner
  }
}

/// The link is a link to the Contacts tab or page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeContactsPage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Section of the page; may be one of "", "search", "sort", "new", "invite", "manage"
  section: String,
}

impl RObject for InternalLinkTypeContactsPage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeContactsPage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeContactsPage {}

impl InternalLinkTypeContactsPage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeContactsPageBuilder {
    let inner = InternalLinkTypeContactsPage {
      td_name: "internalLinkTypeContactsPage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeContactsPageBuilder { inner }
  }

  pub fn section(&self) -> &String {
    &self.section
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeContactsPageBuilder {
  inner: InternalLinkTypeContactsPage,
}

impl RTDInternalLinkTypeContactsPageBuilder {
  pub fn build(&self) -> InternalLinkTypeContactsPage {
    self.inner.clone()
  }

  pub fn section<T: AsRef<str>>(&mut self, section: T) -> &mut Self {
    self.inner.section = section.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeContactsPage> for InternalLinkTypeContactsPage {
  fn as_ref(&self) -> &InternalLinkTypeContactsPage {
    self
  }
}

impl AsRef<InternalLinkTypeContactsPage> for RTDInternalLinkTypeContactsPageBuilder {
  fn as_ref(&self) -> &InternalLinkTypeContactsPage {
    &self.inner
  }
}

/// The link is a link to a channel direct messages chat by username of the channel. Call searchPublicChat with the given chat username to process the link. If the chat is found and is channel, open the direct messages chat of the channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeDirectMessagesChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Username of the channel
  channel_username: String,
}

impl RObject for InternalLinkTypeDirectMessagesChat {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeDirectMessagesChat"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeDirectMessagesChat {}

impl InternalLinkTypeDirectMessagesChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeDirectMessagesChatBuilder {
    let inner = InternalLinkTypeDirectMessagesChat {
      td_name: "internalLinkTypeDirectMessagesChat".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeDirectMessagesChatBuilder { inner }
  }

  pub fn channel_username(&self) -> &String {
    &self.channel_username
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeDirectMessagesChatBuilder {
  inner: InternalLinkTypeDirectMessagesChat,
}

impl RTDInternalLinkTypeDirectMessagesChatBuilder {
  pub fn build(&self) -> InternalLinkTypeDirectMessagesChat {
    self.inner.clone()
  }

  pub fn channel_username<T: AsRef<str>>(&mut self, channel_username: T) -> &mut Self {
    self.inner.channel_username = channel_username.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeDirectMessagesChat> for InternalLinkTypeDirectMessagesChat {
  fn as_ref(&self) -> &InternalLinkTypeDirectMessagesChat {
    self
  }
}

impl AsRef<InternalLinkTypeDirectMessagesChat> for RTDInternalLinkTypeDirectMessagesChatBuilder {
  fn as_ref(&self) -> &InternalLinkTypeDirectMessagesChat {
    &self.inner
  }
}

/// The link is a link to a game. Call searchPublicChat with the given bot username, check that the user is a bot, ask the current user to select a chat to send the game, and then call sendMessage with inputMessageGame
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeGame {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Username of the bot that owns the game
  bot_username: String,
  /// Short name of the game
  game_short_name: String,
}

impl RObject for InternalLinkTypeGame {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeGame"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeGame {}

impl InternalLinkTypeGame {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeGameBuilder {
    let inner = InternalLinkTypeGame {
      td_name: "internalLinkTypeGame".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeGameBuilder { inner }
  }

  pub fn bot_username(&self) -> &String {
    &self.bot_username
  }

  pub fn game_short_name(&self) -> &String {
    &self.game_short_name
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeGameBuilder {
  inner: InternalLinkTypeGame,
}

impl RTDInternalLinkTypeGameBuilder {
  pub fn build(&self) -> InternalLinkTypeGame {
    self.inner.clone()
  }

  pub fn bot_username<T: AsRef<str>>(&mut self, bot_username: T) -> &mut Self {
    self.inner.bot_username = bot_username.as_ref().to_string();
    self
  }

  pub fn game_short_name<T: AsRef<str>>(&mut self, game_short_name: T) -> &mut Self {
    self.inner.game_short_name = game_short_name.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeGame> for InternalLinkTypeGame {
  fn as_ref(&self) -> &InternalLinkTypeGame {
    self
  }
}

impl AsRef<InternalLinkTypeGame> for RTDInternalLinkTypeGameBuilder {
  fn as_ref(&self) -> &InternalLinkTypeGame {
    &self.inner
  }
}

/// The link is a link to a gift auction. Call getGiftAuctionState with the given auction identifier to process the link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeGiftAuction {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the auction
  auction_id: String,
}

impl RObject for InternalLinkTypeGiftAuction {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeGiftAuction"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeGiftAuction {}

impl InternalLinkTypeGiftAuction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeGiftAuctionBuilder {
    let inner = InternalLinkTypeGiftAuction {
      td_name: "internalLinkTypeGiftAuction".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeGiftAuctionBuilder { inner }
  }

  pub fn auction_id(&self) -> &String {
    &self.auction_id
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeGiftAuctionBuilder {
  inner: InternalLinkTypeGiftAuction,
}

impl RTDInternalLinkTypeGiftAuctionBuilder {
  pub fn build(&self) -> InternalLinkTypeGiftAuction {
    self.inner.clone()
  }

  pub fn auction_id<T: AsRef<str>>(&mut self, auction_id: T) -> &mut Self {
    self.inner.auction_id = auction_id.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeGiftAuction> for InternalLinkTypeGiftAuction {
  fn as_ref(&self) -> &InternalLinkTypeGiftAuction {
    self
  }
}

impl AsRef<InternalLinkTypeGiftAuction> for RTDInternalLinkTypeGiftAuctionBuilder {
  fn as_ref(&self) -> &InternalLinkTypeGiftAuction {
    &self.inner
  }
}

/// The link is a link to a gift collection. Call searchPublicChat with the given username, then call getReceivedGifts with the received gift owner identifier and the given collection identifier, then show the collection if received
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeGiftCollection {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Username of the owner of the gift collection
  gift_owner_username: String,
  /// Gift collection identifier
  collection_id: i64,
}

impl RObject for InternalLinkTypeGiftCollection {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeGiftCollection"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeGiftCollection {}

impl InternalLinkTypeGiftCollection {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeGiftCollectionBuilder {
    let inner = InternalLinkTypeGiftCollection {
      td_name: "internalLinkTypeGiftCollection".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeGiftCollectionBuilder { inner }
  }

  pub fn gift_owner_username(&self) -> &String {
    &self.gift_owner_username
  }

  pub fn collection_id(&self) -> i64 {
    self.collection_id
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeGiftCollectionBuilder {
  inner: InternalLinkTypeGiftCollection,
}

impl RTDInternalLinkTypeGiftCollectionBuilder {
  pub fn build(&self) -> InternalLinkTypeGiftCollection {
    self.inner.clone()
  }

  pub fn gift_owner_username<T: AsRef<str>>(&mut self, gift_owner_username: T) -> &mut Self {
    self.inner.gift_owner_username = gift_owner_username.as_ref().to_string();
    self
  }

  pub fn collection_id(&mut self, collection_id: i64) -> &mut Self {
    self.inner.collection_id = collection_id;
    self
  }
}

impl AsRef<InternalLinkTypeGiftCollection> for InternalLinkTypeGiftCollection {
  fn as_ref(&self) -> &InternalLinkTypeGiftCollection {
    self
  }
}

impl AsRef<InternalLinkTypeGiftCollection> for RTDInternalLinkTypeGiftCollectionBuilder {
  fn as_ref(&self) -> &InternalLinkTypeGiftCollection {
    &self.inner
  }
}

/// The link is a link to a group call that isn't bound to a chat. Use getGroupCallParticipants to get the list of group call participants and show them on the join group call screen. Call joinGroupCall with the given invite_link to join the call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeGroupCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Internal representation of the invite link
  invite_link: String,
}

impl RObject for InternalLinkTypeGroupCall {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeGroupCall"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeGroupCall {}

impl InternalLinkTypeGroupCall {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeGroupCallBuilder {
    let inner = InternalLinkTypeGroupCall {
      td_name: "internalLinkTypeGroupCall".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeGroupCallBuilder { inner }
  }

  pub fn invite_link(&self) -> &String {
    &self.invite_link
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeGroupCallBuilder {
  inner: InternalLinkTypeGroupCall,
}

impl RTDInternalLinkTypeGroupCallBuilder {
  pub fn build(&self) -> InternalLinkTypeGroupCall {
    self.inner.clone()
  }

  pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
    self.inner.invite_link = invite_link.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeGroupCall> for InternalLinkTypeGroupCall {
  fn as_ref(&self) -> &InternalLinkTypeGroupCall {
    self
  }
}

impl AsRef<InternalLinkTypeGroupCall> for RTDInternalLinkTypeGroupCallBuilder {
  fn as_ref(&self) -> &InternalLinkTypeGroupCall {
    &self.inner
  }
}

/// The link must be opened in an Instant View. Call getWebPageInstantView with the given URL to process the link. If Instant View is found, then show it, otherwise, open the fallback URL in an external browser
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeInstantView {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// URL to be passed to getWebPageInstantView
  url: String,
  /// An URL to open if getWebPageInstantView fails
  fallback_url: String,
}

impl RObject for InternalLinkTypeInstantView {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeInstantView"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeInstantView {}

impl InternalLinkTypeInstantView {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeInstantViewBuilder {
    let inner = InternalLinkTypeInstantView {
      td_name: "internalLinkTypeInstantView".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeInstantViewBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }

  pub fn fallback_url(&self) -> &String {
    &self.fallback_url
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeInstantViewBuilder {
  inner: InternalLinkTypeInstantView,
}

impl RTDInternalLinkTypeInstantViewBuilder {
  pub fn build(&self) -> InternalLinkTypeInstantView {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

  pub fn fallback_url<T: AsRef<str>>(&mut self, fallback_url: T) -> &mut Self {
    self.inner.fallback_url = fallback_url.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeInstantView> for InternalLinkTypeInstantView {
  fn as_ref(&self) -> &InternalLinkTypeInstantView {
    self
  }
}

impl AsRef<InternalLinkTypeInstantView> for RTDInternalLinkTypeInstantViewBuilder {
  fn as_ref(&self) -> &InternalLinkTypeInstantView {
    &self.inner
  }
}

/// The link is a link to an invoice. Call getPaymentForm with the given invoice name to process the link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeInvoice {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the invoice
  invoice_name: String,
}

impl RObject for InternalLinkTypeInvoice {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeInvoice"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeInvoice {}

impl InternalLinkTypeInvoice {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeInvoiceBuilder {
    let inner = InternalLinkTypeInvoice {
      td_name: "internalLinkTypeInvoice".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeInvoiceBuilder { inner }
  }

  pub fn invoice_name(&self) -> &String {
    &self.invoice_name
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeInvoiceBuilder {
  inner: InternalLinkTypeInvoice,
}

impl RTDInternalLinkTypeInvoiceBuilder {
  pub fn build(&self) -> InternalLinkTypeInvoice {
    self.inner.clone()
  }

  pub fn invoice_name<T: AsRef<str>>(&mut self, invoice_name: T) -> &mut Self {
    self.inner.invoice_name = invoice_name.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeInvoice> for InternalLinkTypeInvoice {
  fn as_ref(&self) -> &InternalLinkTypeInvoice {
    self
  }
}

impl AsRef<InternalLinkTypeInvoice> for RTDInternalLinkTypeInvoiceBuilder {
  fn as_ref(&self) -> &InternalLinkTypeInvoice {
    &self.inner
  }
}

/// The link is a link to a language pack. Call getLanguagePackInfo with the given language pack identifier to process the link. If the language pack is found and the user wants to apply it, then call setOption for the option "language_pack_id"
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeLanguagePack {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Language pack identifier
  language_pack_id: String,
}

impl RObject for InternalLinkTypeLanguagePack {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeLanguagePack"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeLanguagePack {}

impl InternalLinkTypeLanguagePack {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeLanguagePackBuilder {
    let inner = InternalLinkTypeLanguagePack {
      td_name: "internalLinkTypeLanguagePack".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeLanguagePackBuilder { inner }
  }

  pub fn language_pack_id(&self) -> &String {
    &self.language_pack_id
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeLanguagePackBuilder {
  inner: InternalLinkTypeLanguagePack,
}

impl RTDInternalLinkTypeLanguagePackBuilder {
  pub fn build(&self) -> InternalLinkTypeLanguagePack {
    self.inner.clone()
  }

  pub fn language_pack_id<T: AsRef<str>>(&mut self, language_pack_id: T) -> &mut Self {
    self.inner.language_pack_id = language_pack_id.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeLanguagePack> for InternalLinkTypeLanguagePack {
  fn as_ref(&self) -> &InternalLinkTypeLanguagePack {
    self
  }
}

impl AsRef<InternalLinkTypeLanguagePack> for RTDInternalLinkTypeLanguagePackBuilder {
  fn as_ref(&self) -> &InternalLinkTypeLanguagePack {
    &self.inner
  }
}

/// The link is a link to a live story. Call searchPublicChat with the given chat username, then getChatActiveStories to get active stories in the chat, then find a live story among active stories of the chat, and then joinLiveStory to join the live story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeLiveStory {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Username of the poster of the story
  story_poster_username: String,
}

impl RObject for InternalLinkTypeLiveStory {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeLiveStory"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeLiveStory {}

impl InternalLinkTypeLiveStory {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeLiveStoryBuilder {
    let inner = InternalLinkTypeLiveStory {
      td_name: "internalLinkTypeLiveStory".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeLiveStoryBuilder { inner }
  }

  pub fn story_poster_username(&self) -> &String {
    &self.story_poster_username
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeLiveStoryBuilder {
  inner: InternalLinkTypeLiveStory,
}

impl RTDInternalLinkTypeLiveStoryBuilder {
  pub fn build(&self) -> InternalLinkTypeLiveStory {
    self.inner.clone()
  }

  pub fn story_poster_username<T: AsRef<str>>(&mut self, story_poster_username: T) -> &mut Self {
    self.inner.story_poster_username = story_poster_username.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeLiveStory> for InternalLinkTypeLiveStory {
  fn as_ref(&self) -> &InternalLinkTypeLiveStory {
    self
  }
}

impl AsRef<InternalLinkTypeLiveStory> for RTDInternalLinkTypeLiveStoryBuilder {
  fn as_ref(&self) -> &InternalLinkTypeLiveStory {
    &self.inner
  }
}

/// The link is a link to the main Web App of a bot. Call searchPublicChat with the given bot username, check that the user is a bot and has the main Web App. If the bot can be added to attachment menu, then use getAttachmentMenuBot to receive information about the bot, then if the bot isn't added to side menu, show a disclaimer about Mini Apps being third-party applications, ask the user to accept their Terms of service and confirm adding the bot to side and attachment menu, then if the user accepts the terms and confirms adding, use toggleBotIsAddedToAttachmentMenu to add the bot. Then, use getMainWebApp with the given start parameter and mode and open the returned URL as a Web App
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeMainWebApp {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Username of the bot
  bot_username: String,
  /// Start parameter to be passed to getMainWebApp
  start_parameter: String,
  /// The mode to be passed to getMainWebApp
  mode: WebAppOpenMode,
}

impl RObject for InternalLinkTypeMainWebApp {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeMainWebApp"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeMainWebApp {}

impl InternalLinkTypeMainWebApp {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeMainWebAppBuilder {
    let inner = InternalLinkTypeMainWebApp {
      td_name: "internalLinkTypeMainWebApp".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeMainWebAppBuilder { inner }
  }

  pub fn bot_username(&self) -> &String {
    &self.bot_username
  }

  pub fn start_parameter(&self) -> &String {
    &self.start_parameter
  }

  pub fn mode(&self) -> &WebAppOpenMode {
    &self.mode
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeMainWebAppBuilder {
  inner: InternalLinkTypeMainWebApp,
}

impl RTDInternalLinkTypeMainWebAppBuilder {
  pub fn build(&self) -> InternalLinkTypeMainWebApp {
    self.inner.clone()
  }

  pub fn bot_username<T: AsRef<str>>(&mut self, bot_username: T) -> &mut Self {
    self.inner.bot_username = bot_username.as_ref().to_string();
    self
  }

  pub fn start_parameter<T: AsRef<str>>(&mut self, start_parameter: T) -> &mut Self {
    self.inner.start_parameter = start_parameter.as_ref().to_string();
    self
  }

  pub fn mode<T: AsRef<WebAppOpenMode>>(&mut self, mode: T) -> &mut Self {
    self.inner.mode = mode.as_ref().clone();
    self
  }
}

impl AsRef<InternalLinkTypeMainWebApp> for InternalLinkTypeMainWebApp {
  fn as_ref(&self) -> &InternalLinkTypeMainWebApp {
    self
  }
}

impl AsRef<InternalLinkTypeMainWebApp> for RTDInternalLinkTypeMainWebAppBuilder {
  fn as_ref(&self) -> &InternalLinkTypeMainWebApp {
    &self.inner
  }
}

/// The link is a link to a Telegram message or a forum topic. Call getMessageLinkInfo with the given URL to process the link, and then open received forum topic or chat and show the message there
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// URL to be passed to getMessageLinkInfo
  url: String,
}

impl RObject for InternalLinkTypeMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeMessage {}

impl InternalLinkTypeMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeMessageBuilder {
    let inner = InternalLinkTypeMessage {
      td_name: "internalLinkTypeMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeMessageBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeMessageBuilder {
  inner: InternalLinkTypeMessage,
}

impl RTDInternalLinkTypeMessageBuilder {
  pub fn build(&self) -> InternalLinkTypeMessage {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeMessage> for InternalLinkTypeMessage {
  fn as_ref(&self) -> &InternalLinkTypeMessage {
    self
  }
}

impl AsRef<InternalLinkTypeMessage> for RTDInternalLinkTypeMessageBuilder {
  fn as_ref(&self) -> &InternalLinkTypeMessage {
    &self.inner
  }
}

/// The link contains a message draft text. A share screen needs to be shown to the user, then the chosen chat must be opened and the text is added to the input field
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeMessageDraft {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Message draft text
  text: FormattedText,
  /// True, if the first line of the text contains a link. If true, the input field needs to be focused and the text after the link must be selected
  contains_link: bool,
}

impl RObject for InternalLinkTypeMessageDraft {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeMessageDraft"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeMessageDraft {}

impl InternalLinkTypeMessageDraft {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeMessageDraftBuilder {
    let inner = InternalLinkTypeMessageDraft {
      td_name: "internalLinkTypeMessageDraft".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeMessageDraftBuilder { inner }
  }

  pub fn text(&self) -> &FormattedText {
    &self.text
  }

  pub fn contains_link(&self) -> bool {
    self.contains_link
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeMessageDraftBuilder {
  inner: InternalLinkTypeMessageDraft,
}

impl RTDInternalLinkTypeMessageDraftBuilder {
  pub fn build(&self) -> InternalLinkTypeMessageDraft {
    self.inner.clone()
  }

  pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn contains_link(&mut self, contains_link: bool) -> &mut Self {
    self.inner.contains_link = contains_link;
    self
  }
}

impl AsRef<InternalLinkTypeMessageDraft> for InternalLinkTypeMessageDraft {
  fn as_ref(&self) -> &InternalLinkTypeMessageDraft {
    self
  }
}

impl AsRef<InternalLinkTypeMessageDraft> for RTDInternalLinkTypeMessageDraftBuilder {
  fn as_ref(&self) -> &InternalLinkTypeMessageDraft {
    &self.inner
  }
}

/// The link is a link to the My Profile application page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeMyProfilePage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Section of the page; may be one of "", "posts", "posts/all-stories", "posts/add-album", "gifts", "archived-posts"
  section: String,
}

impl RObject for InternalLinkTypeMyProfilePage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeMyProfilePage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeMyProfilePage {}

impl InternalLinkTypeMyProfilePage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeMyProfilePageBuilder {
    let inner = InternalLinkTypeMyProfilePage {
      td_name: "internalLinkTypeMyProfilePage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeMyProfilePageBuilder { inner }
  }

  pub fn section(&self) -> &String {
    &self.section
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeMyProfilePageBuilder {
  inner: InternalLinkTypeMyProfilePage,
}

impl RTDInternalLinkTypeMyProfilePageBuilder {
  pub fn build(&self) -> InternalLinkTypeMyProfilePage {
    self.inner.clone()
  }

  pub fn section<T: AsRef<str>>(&mut self, section: T) -> &mut Self {
    self.inner.section = section.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeMyProfilePage> for InternalLinkTypeMyProfilePage {
  fn as_ref(&self) -> &InternalLinkTypeMyProfilePage {
    self
  }
}

impl AsRef<InternalLinkTypeMyProfilePage> for RTDInternalLinkTypeMyProfilePageBuilder {
  fn as_ref(&self) -> &InternalLinkTypeMyProfilePage {
    &self.inner
  }
}

/// The link is a link to the screen for creating a new channel chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeNewChannelChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for InternalLinkTypeNewChannelChat {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeNewChannelChat"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeNewChannelChat {}

impl InternalLinkTypeNewChannelChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeNewChannelChatBuilder {
    let inner = InternalLinkTypeNewChannelChat {
      td_name: "internalLinkTypeNewChannelChat".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDInternalLinkTypeNewChannelChatBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeNewChannelChatBuilder {
  inner: InternalLinkTypeNewChannelChat,
}

impl RTDInternalLinkTypeNewChannelChatBuilder {
  pub fn build(&self) -> InternalLinkTypeNewChannelChat {
    self.inner.clone()
  }
}

impl AsRef<InternalLinkTypeNewChannelChat> for InternalLinkTypeNewChannelChat {
  fn as_ref(&self) -> &InternalLinkTypeNewChannelChat {
    self
  }
}

impl AsRef<InternalLinkTypeNewChannelChat> for RTDInternalLinkTypeNewChannelChatBuilder {
  fn as_ref(&self) -> &InternalLinkTypeNewChannelChat {
    &self.inner
  }
}

/// The link is a link to the screen for creating a new group chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeNewGroupChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for InternalLinkTypeNewGroupChat {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeNewGroupChat"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeNewGroupChat {}

impl InternalLinkTypeNewGroupChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeNewGroupChatBuilder {
    let inner = InternalLinkTypeNewGroupChat {
      td_name: "internalLinkTypeNewGroupChat".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDInternalLinkTypeNewGroupChatBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeNewGroupChatBuilder {
  inner: InternalLinkTypeNewGroupChat,
}

impl RTDInternalLinkTypeNewGroupChatBuilder {
  pub fn build(&self) -> InternalLinkTypeNewGroupChat {
    self.inner.clone()
  }
}

impl AsRef<InternalLinkTypeNewGroupChat> for InternalLinkTypeNewGroupChat {
  fn as_ref(&self) -> &InternalLinkTypeNewGroupChat {
    self
  }
}

impl AsRef<InternalLinkTypeNewGroupChat> for RTDInternalLinkTypeNewGroupChatBuilder {
  fn as_ref(&self) -> &InternalLinkTypeNewGroupChat {
    &self.inner
  }
}

/// The link is a link to the screen for creating a new private chat with a contact
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeNewPrivateChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for InternalLinkTypeNewPrivateChat {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeNewPrivateChat"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeNewPrivateChat {}

impl InternalLinkTypeNewPrivateChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeNewPrivateChatBuilder {
    let inner = InternalLinkTypeNewPrivateChat {
      td_name: "internalLinkTypeNewPrivateChat".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDInternalLinkTypeNewPrivateChatBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeNewPrivateChatBuilder {
  inner: InternalLinkTypeNewPrivateChat,
}

impl RTDInternalLinkTypeNewPrivateChatBuilder {
  pub fn build(&self) -> InternalLinkTypeNewPrivateChat {
    self.inner.clone()
  }
}

impl AsRef<InternalLinkTypeNewPrivateChat> for InternalLinkTypeNewPrivateChat {
  fn as_ref(&self) -> &InternalLinkTypeNewPrivateChat {
    self
  }
}

impl AsRef<InternalLinkTypeNewPrivateChat> for RTDInternalLinkTypeNewPrivateChatBuilder {
  fn as_ref(&self) -> &InternalLinkTypeNewPrivateChat {
    &self.inner
  }
}

/// The link is a link to open the story posting interface
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeNewStory {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The type of the content of the story to post; may be null if unspecified
  content_type: Option<StoryContentType>,
}

impl RObject for InternalLinkTypeNewStory {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeNewStory"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeNewStory {}

impl InternalLinkTypeNewStory {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeNewStoryBuilder {
    let inner = InternalLinkTypeNewStory {
      td_name: "internalLinkTypeNewStory".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeNewStoryBuilder { inner }
  }

  pub fn content_type(&self) -> &Option<StoryContentType> {
    &self.content_type
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeNewStoryBuilder {
  inner: InternalLinkTypeNewStory,
}

impl RTDInternalLinkTypeNewStoryBuilder {
  pub fn build(&self) -> InternalLinkTypeNewStory {
    self.inner.clone()
  }

  pub fn content_type<T: AsRef<StoryContentType>>(&mut self, content_type: T) -> &mut Self {
    self.inner.content_type = Some(content_type.as_ref().clone());
    self
  }
}

impl AsRef<InternalLinkTypeNewStory> for InternalLinkTypeNewStory {
  fn as_ref(&self) -> &InternalLinkTypeNewStory {
    self
  }
}

impl AsRef<InternalLinkTypeNewStory> for RTDInternalLinkTypeNewStoryBuilder {
  fn as_ref(&self) -> &InternalLinkTypeNewStory {
    &self.inner
  }
}

/// The link is an OAuth link. Call getOauthLinkInfo with the given URL to process the link if the link was received from outside of the application; otherwise, ignore it. After getOauthLinkInfo, show the user confirmation dialog and process it with checkOauthRequestMatchCode, acceptOauthRequest, or declineOauthRequest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeOauth {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// URL to be passed to getOauthLinkInfo
  url: String,
}

impl RObject for InternalLinkTypeOauth {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeOauth"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeOauth {}

impl InternalLinkTypeOauth {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeOauthBuilder {
    let inner = InternalLinkTypeOauth {
      td_name: "internalLinkTypeOauth".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeOauthBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeOauthBuilder {
  inner: InternalLinkTypeOauth,
}

impl RTDInternalLinkTypeOauthBuilder {
  pub fn build(&self) -> InternalLinkTypeOauth {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeOauth> for InternalLinkTypeOauth {
  fn as_ref(&self) -> &InternalLinkTypeOauth {
    self
  }
}

impl AsRef<InternalLinkTypeOauth> for RTDInternalLinkTypeOauthBuilder {
  fn as_ref(&self) -> &InternalLinkTypeOauth {
    &self.inner
  }
}

/// The link contains a request of Telegram passport data. Call getPassportAuthorizationForm with the given parameters to process the link if the link was received from outside of the application; otherwise, ignore it
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypePassportDataRequest {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier of the service's bot; the corresponding user may be unknown yet
  bot_user_id: i64,
  /// Telegram Passport element types requested by the service
  scope: String,
  /// Service's public key
  public_key: String,
  /// Unique request identifier provided by the service
  nonce: String,
  /// An HTTP URL to open once the request is finished, canceled, or failed with the parameters tg_passport=success, tg_passport=cancel, or tg_passport=error&error=... respectively. If empty, then onActivityResult method must be used to return response on Android, or the link tgbot{bot_user_id}://passport/success or tgbot{bot_user_id}://passport/cancel must be opened otherwise
  callback_url: String,
}

impl RObject for InternalLinkTypePassportDataRequest {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypePassportDataRequest"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypePassportDataRequest {}

impl InternalLinkTypePassportDataRequest {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypePassportDataRequestBuilder {
    let inner = InternalLinkTypePassportDataRequest {
      td_name: "internalLinkTypePassportDataRequest".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypePassportDataRequestBuilder { inner }
  }

  pub fn bot_user_id(&self) -> i64 {
    self.bot_user_id
  }

  pub fn scope(&self) -> &String {
    &self.scope
  }

  pub fn public_key(&self) -> &String {
    &self.public_key
  }

  pub fn nonce(&self) -> &String {
    &self.nonce
  }

  pub fn callback_url(&self) -> &String {
    &self.callback_url
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypePassportDataRequestBuilder {
  inner: InternalLinkTypePassportDataRequest,
}

impl RTDInternalLinkTypePassportDataRequestBuilder {
  pub fn build(&self) -> InternalLinkTypePassportDataRequest {
    self.inner.clone()
  }

  pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
    self.inner.bot_user_id = bot_user_id;
    self
  }

  pub fn scope<T: AsRef<str>>(&mut self, scope: T) -> &mut Self {
    self.inner.scope = scope.as_ref().to_string();
    self
  }

  pub fn public_key<T: AsRef<str>>(&mut self, public_key: T) -> &mut Self {
    self.inner.public_key = public_key.as_ref().to_string();
    self
  }

  pub fn nonce<T: AsRef<str>>(&mut self, nonce: T) -> &mut Self {
    self.inner.nonce = nonce.as_ref().to_string();
    self
  }

  pub fn callback_url<T: AsRef<str>>(&mut self, callback_url: T) -> &mut Self {
    self.inner.callback_url = callback_url.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypePassportDataRequest> for InternalLinkTypePassportDataRequest {
  fn as_ref(&self) -> &InternalLinkTypePassportDataRequest {
    self
  }
}

impl AsRef<InternalLinkTypePassportDataRequest> for RTDInternalLinkTypePassportDataRequestBuilder {
  fn as_ref(&self) -> &InternalLinkTypePassportDataRequest {
    &self.inner
  }
}

/// The link can be used to confirm ownership of a phone number to prevent account deletion. Call sendPhoneNumberCode with the given phone number and with phoneNumberCodeTypeConfirmOwnership with the given hash to process the link. If succeeded, call checkPhoneNumberCode to check entered by the user code, or resendPhoneNumberCode to resend it
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypePhoneNumberConfirmation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Hash value from the link
  hash: String,
  /// Phone number value from the link
  phone_number: String,
}

impl RObject for InternalLinkTypePhoneNumberConfirmation {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypePhoneNumberConfirmation"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypePhoneNumberConfirmation {}

impl InternalLinkTypePhoneNumberConfirmation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypePhoneNumberConfirmationBuilder {
    let inner = InternalLinkTypePhoneNumberConfirmation {
      td_name: "internalLinkTypePhoneNumberConfirmation".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypePhoneNumberConfirmationBuilder { inner }
  }

  pub fn hash(&self) -> &String {
    &self.hash
  }

  pub fn phone_number(&self) -> &String {
    &self.phone_number
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypePhoneNumberConfirmationBuilder {
  inner: InternalLinkTypePhoneNumberConfirmation,
}

impl RTDInternalLinkTypePhoneNumberConfirmationBuilder {
  pub fn build(&self) -> InternalLinkTypePhoneNumberConfirmation {
    self.inner.clone()
  }

  pub fn hash<T: AsRef<str>>(&mut self, hash: T) -> &mut Self {
    self.inner.hash = hash.as_ref().to_string();
    self
  }

  pub fn phone_number<T: AsRef<str>>(&mut self, phone_number: T) -> &mut Self {
    self.inner.phone_number = phone_number.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypePhoneNumberConfirmation> for InternalLinkTypePhoneNumberConfirmation {
  fn as_ref(&self) -> &InternalLinkTypePhoneNumberConfirmation {
    self
  }
}

impl AsRef<InternalLinkTypePhoneNumberConfirmation>
  for RTDInternalLinkTypePhoneNumberConfirmationBuilder
{
  fn as_ref(&self) -> &InternalLinkTypePhoneNumberConfirmation {
    &self.inner
  }
}

/// The link is a link to the Premium features screen of the application from which the user can subscribe to Telegram Premium. Call getPremiumFeatures with the given referrer to process the link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypePremiumFeaturesPage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Referrer specified in the link
  referrer: String,
}

impl RObject for InternalLinkTypePremiumFeaturesPage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypePremiumFeaturesPage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypePremiumFeaturesPage {}

impl InternalLinkTypePremiumFeaturesPage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypePremiumFeaturesPageBuilder {
    let inner = InternalLinkTypePremiumFeaturesPage {
      td_name: "internalLinkTypePremiumFeaturesPage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypePremiumFeaturesPageBuilder { inner }
  }

  pub fn referrer(&self) -> &String {
    &self.referrer
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypePremiumFeaturesPageBuilder {
  inner: InternalLinkTypePremiumFeaturesPage,
}

impl RTDInternalLinkTypePremiumFeaturesPageBuilder {
  pub fn build(&self) -> InternalLinkTypePremiumFeaturesPage {
    self.inner.clone()
  }

  pub fn referrer<T: AsRef<str>>(&mut self, referrer: T) -> &mut Self {
    self.inner.referrer = referrer.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypePremiumFeaturesPage> for InternalLinkTypePremiumFeaturesPage {
  fn as_ref(&self) -> &InternalLinkTypePremiumFeaturesPage {
    self
  }
}

impl AsRef<InternalLinkTypePremiumFeaturesPage> for RTDInternalLinkTypePremiumFeaturesPageBuilder {
  fn as_ref(&self) -> &InternalLinkTypePremiumFeaturesPage {
    &self.inner
  }
}

/// The link is a link with a Telegram Premium gift code. Call checkPremiumGiftCode with the given code to process the link. If the code is valid and the user wants to apply it, then call applyPremiumGiftCode
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypePremiumGiftCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The Telegram Premium gift code
  code: String,
}

impl RObject for InternalLinkTypePremiumGiftCode {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypePremiumGiftCode"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypePremiumGiftCode {}

impl InternalLinkTypePremiumGiftCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypePremiumGiftCodeBuilder {
    let inner = InternalLinkTypePremiumGiftCode {
      td_name: "internalLinkTypePremiumGiftCode".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypePremiumGiftCodeBuilder { inner }
  }

  pub fn code(&self) -> &String {
    &self.code
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypePremiumGiftCodeBuilder {
  inner: InternalLinkTypePremiumGiftCode,
}

impl RTDInternalLinkTypePremiumGiftCodeBuilder {
  pub fn build(&self) -> InternalLinkTypePremiumGiftCode {
    self.inner.clone()
  }

  pub fn code<T: AsRef<str>>(&mut self, code: T) -> &mut Self {
    self.inner.code = code.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypePremiumGiftCode> for InternalLinkTypePremiumGiftCode {
  fn as_ref(&self) -> &InternalLinkTypePremiumGiftCode {
    self
  }
}

impl AsRef<InternalLinkTypePremiumGiftCode> for RTDInternalLinkTypePremiumGiftCodeBuilder {
  fn as_ref(&self) -> &InternalLinkTypePremiumGiftCode {
    &self.inner
  }
}

/// The link is a link to the screen for gifting Telegram Premium subscriptions to friends via inputInvoiceTelegram with telegramPaymentPurposePremiumGift payments or in-store purchases
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypePremiumGiftPurchase {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Referrer specified in the link
  referrer: String,
}

impl RObject for InternalLinkTypePremiumGiftPurchase {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypePremiumGiftPurchase"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypePremiumGiftPurchase {}

impl InternalLinkTypePremiumGiftPurchase {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypePremiumGiftPurchaseBuilder {
    let inner = InternalLinkTypePremiumGiftPurchase {
      td_name: "internalLinkTypePremiumGiftPurchase".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypePremiumGiftPurchaseBuilder { inner }
  }

  pub fn referrer(&self) -> &String {
    &self.referrer
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypePremiumGiftPurchaseBuilder {
  inner: InternalLinkTypePremiumGiftPurchase,
}

impl RTDInternalLinkTypePremiumGiftPurchaseBuilder {
  pub fn build(&self) -> InternalLinkTypePremiumGiftPurchase {
    self.inner.clone()
  }

  pub fn referrer<T: AsRef<str>>(&mut self, referrer: T) -> &mut Self {
    self.inner.referrer = referrer.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypePremiumGiftPurchase> for InternalLinkTypePremiumGiftPurchase {
  fn as_ref(&self) -> &InternalLinkTypePremiumGiftPurchase {
    self
  }
}

impl AsRef<InternalLinkTypePremiumGiftPurchase> for RTDInternalLinkTypePremiumGiftPurchaseBuilder {
  fn as_ref(&self) -> &InternalLinkTypePremiumGiftPurchase {
    &self.inner
  }
}

/// The link is a link to a proxy. Call addProxy with the given parameters to process the link and add the proxy
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeProxy {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The proxy; may be null if the proxy is unsupported, in which case an alert can be shown to the user
  proxy: Option<Proxy>,
}

impl RObject for InternalLinkTypeProxy {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeProxy"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeProxy {}

impl InternalLinkTypeProxy {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeProxyBuilder {
    let inner = InternalLinkTypeProxy {
      td_name: "internalLinkTypeProxy".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeProxyBuilder { inner }
  }

  pub fn proxy(&self) -> &Option<Proxy> {
    &self.proxy
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeProxyBuilder {
  inner: InternalLinkTypeProxy,
}

impl RTDInternalLinkTypeProxyBuilder {
  pub fn build(&self) -> InternalLinkTypeProxy {
    self.inner.clone()
  }

  pub fn proxy<T: AsRef<Proxy>>(&mut self, proxy: T) -> &mut Self {
    self.inner.proxy = Some(proxy.as_ref().clone());
    self
  }
}

impl AsRef<InternalLinkTypeProxy> for InternalLinkTypeProxy {
  fn as_ref(&self) -> &InternalLinkTypeProxy {
    self
  }
}

impl AsRef<InternalLinkTypeProxy> for RTDInternalLinkTypeProxyBuilder {
  fn as_ref(&self) -> &InternalLinkTypeProxy {
    &self.inner
  }
}

/// The link is a link to a chat by its username. Call searchPublicChat with the given chat username to process the link. If the chat is found, open its profile information screen or the chat itself. If draft text isn't empty and the chat is a private chat with a regular user, then put the draft text in the input field
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypePublicChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Username of the chat
  chat_username: String,
  /// Draft text for message to send in the chat
  draft_text: String,
  /// True, if chat profile information screen must be opened; otherwise, the chat itself must be opened
  open_profile: bool,
  /// Token, which allows the current user to send unpaid messages; may be empty if none. If the token isn't empty and the chat is a private chat with a regular user, then call applyUnpaidMessagesToken with the user and the token
  unpaid_messages_token: String,
}

impl RObject for InternalLinkTypePublicChat {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypePublicChat"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypePublicChat {}

impl InternalLinkTypePublicChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypePublicChatBuilder {
    let inner = InternalLinkTypePublicChat {
      td_name: "internalLinkTypePublicChat".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypePublicChatBuilder { inner }
  }

  pub fn chat_username(&self) -> &String {
    &self.chat_username
  }

  pub fn draft_text(&self) -> &String {
    &self.draft_text
  }

  pub fn open_profile(&self) -> bool {
    self.open_profile
  }

  pub fn unpaid_messages_token(&self) -> &String {
    &self.unpaid_messages_token
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypePublicChatBuilder {
  inner: InternalLinkTypePublicChat,
}

impl RTDInternalLinkTypePublicChatBuilder {
  pub fn build(&self) -> InternalLinkTypePublicChat {
    self.inner.clone()
  }

  pub fn chat_username<T: AsRef<str>>(&mut self, chat_username: T) -> &mut Self {
    self.inner.chat_username = chat_username.as_ref().to_string();
    self
  }

  pub fn draft_text<T: AsRef<str>>(&mut self, draft_text: T) -> &mut Self {
    self.inner.draft_text = draft_text.as_ref().to_string();
    self
  }

  pub fn open_profile(&mut self, open_profile: bool) -> &mut Self {
    self.inner.open_profile = open_profile;
    self
  }

  pub fn unpaid_messages_token<T: AsRef<str>>(&mut self, unpaid_messages_token: T) -> &mut Self {
    self.inner.unpaid_messages_token = unpaid_messages_token.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypePublicChat> for InternalLinkTypePublicChat {
  fn as_ref(&self) -> &InternalLinkTypePublicChat {
    self
  }
}

impl AsRef<InternalLinkTypePublicChat> for RTDInternalLinkTypePublicChatBuilder {
  fn as_ref(&self) -> &InternalLinkTypePublicChat {
    &self.inner
  }
}

/// The link can be used to login the current user on another device, but it must be scanned from QR-code using in-app camera. An alert similar to "This code can be used to allow someone to log in to your Telegram account. To confirm Telegram login, please go to Settings > Devices > Scan QR and scan the code" needs to be shown
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeQrCodeAuthentication {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for InternalLinkTypeQrCodeAuthentication {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeQrCodeAuthentication"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeQrCodeAuthentication {}

impl InternalLinkTypeQrCodeAuthentication {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeQrCodeAuthenticationBuilder {
    let inner = InternalLinkTypeQrCodeAuthentication {
      td_name: "internalLinkTypeQrCodeAuthentication".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDInternalLinkTypeQrCodeAuthenticationBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeQrCodeAuthenticationBuilder {
  inner: InternalLinkTypeQrCodeAuthentication,
}

impl RTDInternalLinkTypeQrCodeAuthenticationBuilder {
  pub fn build(&self) -> InternalLinkTypeQrCodeAuthentication {
    self.inner.clone()
  }
}

impl AsRef<InternalLinkTypeQrCodeAuthentication> for InternalLinkTypeQrCodeAuthentication {
  fn as_ref(&self) -> &InternalLinkTypeQrCodeAuthentication {
    self
  }
}

impl AsRef<InternalLinkTypeQrCodeAuthentication>
  for RTDInternalLinkTypeQrCodeAuthenticationBuilder
{
  fn as_ref(&self) -> &InternalLinkTypeQrCodeAuthentication {
    &self.inner
  }
}

/// The link is a link to a dialog for creating of a managed bot. Call searchPublicChat with the given manager bot username. If the chat is found, the chat is a chat with a bot and the bot has can_manage_bots == true, then show bot creation confirmation dialog with the given suggested_bot_username and suggested_bot_name. If user agrees, call createBot with via_link == true to create the bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeRequestManagedBot {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Username of the bot which will manage the new bot
  manager_bot_username: String,
  /// Suggested username for the bot; always ends with "bot" or a string from updateAllowedBotUsernamePremiumSuffixes case-insensitive
  suggested_bot_username: String,
  /// Suggested name for the bot; may be empty if not specified
  suggested_bot_name: String,
}

impl RObject for InternalLinkTypeRequestManagedBot {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeRequestManagedBot"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeRequestManagedBot {}

impl InternalLinkTypeRequestManagedBot {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeRequestManagedBotBuilder {
    let inner = InternalLinkTypeRequestManagedBot {
      td_name: "internalLinkTypeRequestManagedBot".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeRequestManagedBotBuilder { inner }
  }

  pub fn manager_bot_username(&self) -> &String {
    &self.manager_bot_username
  }

  pub fn suggested_bot_username(&self) -> &String {
    &self.suggested_bot_username
  }

  pub fn suggested_bot_name(&self) -> &String {
    &self.suggested_bot_name
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeRequestManagedBotBuilder {
  inner: InternalLinkTypeRequestManagedBot,
}

impl RTDInternalLinkTypeRequestManagedBotBuilder {
  pub fn build(&self) -> InternalLinkTypeRequestManagedBot {
    self.inner.clone()
  }

  pub fn manager_bot_username<T: AsRef<str>>(&mut self, manager_bot_username: T) -> &mut Self {
    self.inner.manager_bot_username = manager_bot_username.as_ref().to_string();
    self
  }

  pub fn suggested_bot_username<T: AsRef<str>>(&mut self, suggested_bot_username: T) -> &mut Self {
    self.inner.suggested_bot_username = suggested_bot_username.as_ref().to_string();
    self
  }

  pub fn suggested_bot_name<T: AsRef<str>>(&mut self, suggested_bot_name: T) -> &mut Self {
    self.inner.suggested_bot_name = suggested_bot_name.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeRequestManagedBot> for InternalLinkTypeRequestManagedBot {
  fn as_ref(&self) -> &InternalLinkTypeRequestManagedBot {
    self
  }
}

impl AsRef<InternalLinkTypeRequestManagedBot> for RTDInternalLinkTypeRequestManagedBotBuilder {
  fn as_ref(&self) -> &InternalLinkTypeRequestManagedBot {
    &self.inner
  }
}

/// The link forces restore of App Store purchases when opened. For official iOS application only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeRestorePurchases {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for InternalLinkTypeRestorePurchases {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeRestorePurchases"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeRestorePurchases {}

impl InternalLinkTypeRestorePurchases {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeRestorePurchasesBuilder {
    let inner = InternalLinkTypeRestorePurchases {
      td_name: "internalLinkTypeRestorePurchases".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDInternalLinkTypeRestorePurchasesBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeRestorePurchasesBuilder {
  inner: InternalLinkTypeRestorePurchases,
}

impl RTDInternalLinkTypeRestorePurchasesBuilder {
  pub fn build(&self) -> InternalLinkTypeRestorePurchases {
    self.inner.clone()
  }
}

impl AsRef<InternalLinkTypeRestorePurchases> for InternalLinkTypeRestorePurchases {
  fn as_ref(&self) -> &InternalLinkTypeRestorePurchases {
    self
  }
}

impl AsRef<InternalLinkTypeRestorePurchases> for RTDInternalLinkTypeRestorePurchasesBuilder {
  fn as_ref(&self) -> &InternalLinkTypeRestorePurchases {
    &self.inner
  }
}

/// The link is a link to the Saved Messages chat. Call createPrivateChat with getOption("my_id") and open the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeSavedMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for InternalLinkTypeSavedMessages {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeSavedMessages"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeSavedMessages {}

impl InternalLinkTypeSavedMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeSavedMessagesBuilder {
    let inner = InternalLinkTypeSavedMessages {
      td_name: "internalLinkTypeSavedMessages".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDInternalLinkTypeSavedMessagesBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeSavedMessagesBuilder {
  inner: InternalLinkTypeSavedMessages,
}

impl RTDInternalLinkTypeSavedMessagesBuilder {
  pub fn build(&self) -> InternalLinkTypeSavedMessages {
    self.inner.clone()
  }
}

impl AsRef<InternalLinkTypeSavedMessages> for InternalLinkTypeSavedMessages {
  fn as_ref(&self) -> &InternalLinkTypeSavedMessages {
    self
  }
}

impl AsRef<InternalLinkTypeSavedMessages> for RTDInternalLinkTypeSavedMessagesBuilder {
  fn as_ref(&self) -> &InternalLinkTypeSavedMessages {
    &self.inner
  }
}

/// The link is a link to the global chat and messages search field
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeSearch {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for InternalLinkTypeSearch {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeSearch"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeSearch {}

impl InternalLinkTypeSearch {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeSearchBuilder {
    let inner = InternalLinkTypeSearch {
      td_name: "internalLinkTypeSearch".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDInternalLinkTypeSearchBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeSearchBuilder {
  inner: InternalLinkTypeSearch,
}

impl RTDInternalLinkTypeSearchBuilder {
  pub fn build(&self) -> InternalLinkTypeSearch {
    self.inner.clone()
  }
}

impl AsRef<InternalLinkTypeSearch> for InternalLinkTypeSearch {
  fn as_ref(&self) -> &InternalLinkTypeSearch {
    self
  }
}

impl AsRef<InternalLinkTypeSearch> for RTDInternalLinkTypeSearchBuilder {
  fn as_ref(&self) -> &InternalLinkTypeSearch {
    &self.inner
  }
}

/// The link is a link to application settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Section of the application settings to open; may be null if none
  section: Option<SettingsSection>,
}

impl RObject for InternalLinkTypeSettings {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeSettings"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeSettings {}

impl InternalLinkTypeSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeSettingsBuilder {
    let inner = InternalLinkTypeSettings {
      td_name: "internalLinkTypeSettings".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeSettingsBuilder { inner }
  }

  pub fn section(&self) -> &Option<SettingsSection> {
    &self.section
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeSettingsBuilder {
  inner: InternalLinkTypeSettings,
}

impl RTDInternalLinkTypeSettingsBuilder {
  pub fn build(&self) -> InternalLinkTypeSettings {
    self.inner.clone()
  }

  pub fn section<T: AsRef<SettingsSection>>(&mut self, section: T) -> &mut Self {
    self.inner.section = Some(section.as_ref().clone());
    self
  }
}

impl AsRef<InternalLinkTypeSettings> for InternalLinkTypeSettings {
  fn as_ref(&self) -> &InternalLinkTypeSettings {
    self
  }
}

impl AsRef<InternalLinkTypeSettings> for RTDInternalLinkTypeSettingsBuilder {
  fn as_ref(&self) -> &InternalLinkTypeSettings {
    &self.inner
  }
}

/// The link is a link to the Telegram Star purchase section of the application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeStarPurchase {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The number of Telegram Stars that must be owned by the user
  star_count: i64,
  /// Purpose of Telegram Star purchase. Arbitrary string specified by the server, for example, "subs" if the Telegram Stars are required to extend channel subscriptions
  purpose: String,
}

impl RObject for InternalLinkTypeStarPurchase {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeStarPurchase"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeStarPurchase {}

impl InternalLinkTypeStarPurchase {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeStarPurchaseBuilder {
    let inner = InternalLinkTypeStarPurchase {
      td_name: "internalLinkTypeStarPurchase".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeStarPurchaseBuilder { inner }
  }

  pub fn star_count(&self) -> i64 {
    self.star_count
  }

  pub fn purpose(&self) -> &String {
    &self.purpose
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeStarPurchaseBuilder {
  inner: InternalLinkTypeStarPurchase,
}

impl RTDInternalLinkTypeStarPurchaseBuilder {
  pub fn build(&self) -> InternalLinkTypeStarPurchase {
    self.inner.clone()
  }

  pub fn star_count(&mut self, star_count: i64) -> &mut Self {
    self.inner.star_count = star_count;
    self
  }

  pub fn purpose<T: AsRef<str>>(&mut self, purpose: T) -> &mut Self {
    self.inner.purpose = purpose.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeStarPurchase> for InternalLinkTypeStarPurchase {
  fn as_ref(&self) -> &InternalLinkTypeStarPurchase {
    self
  }
}

impl AsRef<InternalLinkTypeStarPurchase> for RTDInternalLinkTypeStarPurchaseBuilder {
  fn as_ref(&self) -> &InternalLinkTypeStarPurchase {
    &self.inner
  }
}

/// The link is a link to a sticker set. Call searchStickerSet with the given sticker set name to process the link and show the sticker set. If the sticker set is found and the user wants to add it, then call changeStickerSet
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeStickerSet {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the sticker set
  sticker_set_name: String,
  /// True, if the sticker set is expected to contain custom emoji
  expect_custom_emoji: bool,
}

impl RObject for InternalLinkTypeStickerSet {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeStickerSet"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeStickerSet {}

impl InternalLinkTypeStickerSet {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeStickerSetBuilder {
    let inner = InternalLinkTypeStickerSet {
      td_name: "internalLinkTypeStickerSet".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeStickerSetBuilder { inner }
  }

  pub fn sticker_set_name(&self) -> &String {
    &self.sticker_set_name
  }

  pub fn expect_custom_emoji(&self) -> bool {
    self.expect_custom_emoji
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeStickerSetBuilder {
  inner: InternalLinkTypeStickerSet,
}

impl RTDInternalLinkTypeStickerSetBuilder {
  pub fn build(&self) -> InternalLinkTypeStickerSet {
    self.inner.clone()
  }

  pub fn sticker_set_name<T: AsRef<str>>(&mut self, sticker_set_name: T) -> &mut Self {
    self.inner.sticker_set_name = sticker_set_name.as_ref().to_string();
    self
  }

  pub fn expect_custom_emoji(&mut self, expect_custom_emoji: bool) -> &mut Self {
    self.inner.expect_custom_emoji = expect_custom_emoji;
    self
  }
}

impl AsRef<InternalLinkTypeStickerSet> for InternalLinkTypeStickerSet {
  fn as_ref(&self) -> &InternalLinkTypeStickerSet {
    self
  }
}

impl AsRef<InternalLinkTypeStickerSet> for RTDInternalLinkTypeStickerSetBuilder {
  fn as_ref(&self) -> &InternalLinkTypeStickerSet {
    &self.inner
  }
}

/// The link is a link to a story. Call searchPublicChat with the given poster username, then call getStory with the received chat identifier and the given story identifier, then show the story if received
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeStory {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Username of the poster of the story
  story_poster_username: String,
  /// Story identifier
  story_id: i64,
}

impl RObject for InternalLinkTypeStory {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeStory"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeStory {}

impl InternalLinkTypeStory {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeStoryBuilder {
    let inner = InternalLinkTypeStory {
      td_name: "internalLinkTypeStory".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeStoryBuilder { inner }
  }

  pub fn story_poster_username(&self) -> &String {
    &self.story_poster_username
  }

  pub fn story_id(&self) -> i64 {
    self.story_id
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeStoryBuilder {
  inner: InternalLinkTypeStory,
}

impl RTDInternalLinkTypeStoryBuilder {
  pub fn build(&self) -> InternalLinkTypeStory {
    self.inner.clone()
  }

  pub fn story_poster_username<T: AsRef<str>>(&mut self, story_poster_username: T) -> &mut Self {
    self.inner.story_poster_username = story_poster_username.as_ref().to_string();
    self
  }

  pub fn story_id(&mut self, story_id: i64) -> &mut Self {
    self.inner.story_id = story_id;
    self
  }
}

impl AsRef<InternalLinkTypeStory> for InternalLinkTypeStory {
  fn as_ref(&self) -> &InternalLinkTypeStory {
    self
  }
}

impl AsRef<InternalLinkTypeStory> for RTDInternalLinkTypeStoryBuilder {
  fn as_ref(&self) -> &InternalLinkTypeStory {
    &self.inner
  }
}

/// The link is a link to an album of stories. Call searchPublicChat with the given username, then call getStoryAlbumStories with the received chat identifier and the given story album identifier, then show the story album if received
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeStoryAlbum {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Username of the owner of the story album
  story_album_owner_username: String,
  /// Story album identifier
  story_album_id: i64,
}

impl RObject for InternalLinkTypeStoryAlbum {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeStoryAlbum"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeStoryAlbum {}

impl InternalLinkTypeStoryAlbum {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeStoryAlbumBuilder {
    let inner = InternalLinkTypeStoryAlbum {
      td_name: "internalLinkTypeStoryAlbum".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeStoryAlbumBuilder { inner }
  }

  pub fn story_album_owner_username(&self) -> &String {
    &self.story_album_owner_username
  }

  pub fn story_album_id(&self) -> i64 {
    self.story_album_id
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeStoryAlbumBuilder {
  inner: InternalLinkTypeStoryAlbum,
}

impl RTDInternalLinkTypeStoryAlbumBuilder {
  pub fn build(&self) -> InternalLinkTypeStoryAlbum {
    self.inner.clone()
  }

  pub fn story_album_owner_username<T: AsRef<str>>(
    &mut self,
    story_album_owner_username: T,
  ) -> &mut Self {
    self.inner.story_album_owner_username = story_album_owner_username.as_ref().to_string();
    self
  }

  pub fn story_album_id(&mut self, story_album_id: i64) -> &mut Self {
    self.inner.story_album_id = story_album_id;
    self
  }
}

impl AsRef<InternalLinkTypeStoryAlbum> for InternalLinkTypeStoryAlbum {
  fn as_ref(&self) -> &InternalLinkTypeStoryAlbum {
    self
  }
}

impl AsRef<InternalLinkTypeStoryAlbum> for RTDInternalLinkTypeStoryAlbumBuilder {
  fn as_ref(&self) -> &InternalLinkTypeStoryAlbum {
    &self.inner
  }
}

/// The link is a link to a text composition style. Call searchTextCompositionStyle with the given style name to get information about the style. If the style is found and the user wants to add it, then call addTextCompositionStyle
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeTextCompositionStyle {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the style
  style_name: String,
}

impl RObject for InternalLinkTypeTextCompositionStyle {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeTextCompositionStyle"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeTextCompositionStyle {}

impl InternalLinkTypeTextCompositionStyle {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeTextCompositionStyleBuilder {
    let inner = InternalLinkTypeTextCompositionStyle {
      td_name: "internalLinkTypeTextCompositionStyle".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeTextCompositionStyleBuilder { inner }
  }

  pub fn style_name(&self) -> &String {
    &self.style_name
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeTextCompositionStyleBuilder {
  inner: InternalLinkTypeTextCompositionStyle,
}

impl RTDInternalLinkTypeTextCompositionStyleBuilder {
  pub fn build(&self) -> InternalLinkTypeTextCompositionStyle {
    self.inner.clone()
  }

  pub fn style_name<T: AsRef<str>>(&mut self, style_name: T) -> &mut Self {
    self.inner.style_name = style_name.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeTextCompositionStyle> for InternalLinkTypeTextCompositionStyle {
  fn as_ref(&self) -> &InternalLinkTypeTextCompositionStyle {
    self
  }
}

impl AsRef<InternalLinkTypeTextCompositionStyle>
  for RTDInternalLinkTypeTextCompositionStyleBuilder
{
  fn as_ref(&self) -> &InternalLinkTypeTextCompositionStyle {
    &self.inner
  }
}

/// The link is a link to a cloud theme. TDLib has no theme support yet
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeTheme {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the theme
  theme_name: String,
}

impl RObject for InternalLinkTypeTheme {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeTheme"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeTheme {}

impl InternalLinkTypeTheme {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeThemeBuilder {
    let inner = InternalLinkTypeTheme {
      td_name: "internalLinkTypeTheme".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeThemeBuilder { inner }
  }

  pub fn theme_name(&self) -> &String {
    &self.theme_name
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeThemeBuilder {
  inner: InternalLinkTypeTheme,
}

impl RTDInternalLinkTypeThemeBuilder {
  pub fn build(&self) -> InternalLinkTypeTheme {
    self.inner.clone()
  }

  pub fn theme_name<T: AsRef<str>>(&mut self, theme_name: T) -> &mut Self {
    self.inner.theme_name = theme_name.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeTheme> for InternalLinkTypeTheme {
  fn as_ref(&self) -> &InternalLinkTypeTheme {
    self
  }
}

impl AsRef<InternalLinkTypeTheme> for RTDInternalLinkTypeThemeBuilder {
  fn as_ref(&self) -> &InternalLinkTypeTheme {
    &self.inner
  }
}

/// The link is a universal TON Connect link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeTonConnect {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// TON Connect version
  version: i64,
  /// Hex-encoded public X25519 key of the dApp
  dapp_client_id: String,
  /// TON Connect connect request
  connect_request: TonConnectConnectRequest,
  /// Request return strategy; one of "back", "none", or a URL to open
  return_strategy: String,
  /// Embedded RPC request represented as base64url-encoded JSON
  rpc_request: String,
  /// Identifier to trace the connection request
  trace_id: String,
}

impl RObject for InternalLinkTypeTonConnect {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeTonConnect"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeTonConnect {}

impl InternalLinkTypeTonConnect {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeTonConnectBuilder {
    let inner = InternalLinkTypeTonConnect {
      td_name: "internalLinkTypeTonConnect".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeTonConnectBuilder { inner }
  }

  pub fn version(&self) -> i64 {
    self.version
  }

  pub fn dapp_client_id(&self) -> &String {
    &self.dapp_client_id
  }

  pub fn connect_request(&self) -> &TonConnectConnectRequest {
    &self.connect_request
  }

  pub fn return_strategy(&self) -> &String {
    &self.return_strategy
  }

  pub fn rpc_request(&self) -> &String {
    &self.rpc_request
  }

  pub fn trace_id(&self) -> &String {
    &self.trace_id
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeTonConnectBuilder {
  inner: InternalLinkTypeTonConnect,
}

impl RTDInternalLinkTypeTonConnectBuilder {
  pub fn build(&self) -> InternalLinkTypeTonConnect {
    self.inner.clone()
  }

  pub fn version(&mut self, version: i64) -> &mut Self {
    self.inner.version = version;
    self
  }

  pub fn dapp_client_id<T: AsRef<str>>(&mut self, dapp_client_id: T) -> &mut Self {
    self.inner.dapp_client_id = dapp_client_id.as_ref().to_string();
    self
  }

  pub fn connect_request<T: AsRef<TonConnectConnectRequest>>(
    &mut self,
    connect_request: T,
  ) -> &mut Self {
    self.inner.connect_request = connect_request.as_ref().clone();
    self
  }

  pub fn return_strategy<T: AsRef<str>>(&mut self, return_strategy: T) -> &mut Self {
    self.inner.return_strategy = return_strategy.as_ref().to_string();
    self
  }

  pub fn rpc_request<T: AsRef<str>>(&mut self, rpc_request: T) -> &mut Self {
    self.inner.rpc_request = rpc_request.as_ref().to_string();
    self
  }

  pub fn trace_id<T: AsRef<str>>(&mut self, trace_id: T) -> &mut Self {
    self.inner.trace_id = trace_id.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeTonConnect> for InternalLinkTypeTonConnect {
  fn as_ref(&self) -> &InternalLinkTypeTonConnect {
    self
  }
}

impl AsRef<InternalLinkTypeTonConnect> for RTDInternalLinkTypeTonConnectBuilder {
  fn as_ref(&self) -> &InternalLinkTypeTonConnect {
    &self.inner
  }
}

/// The link is a link for sending Grams from the user's TON wallet
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeTonWalletTransfer {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chosen receiver of the transfer; may be null if none
  receiver: Option<TonWalletTransferReceiver>,
  /// The pre-filled amount of TON Grams to send; in the smallest units of the cryptocurrency
  gram_amount: i64,
}

impl RObject for InternalLinkTypeTonWalletTransfer {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeTonWalletTransfer"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeTonWalletTransfer {}

impl InternalLinkTypeTonWalletTransfer {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeTonWalletTransferBuilder {
    let inner = InternalLinkTypeTonWalletTransfer {
      td_name: "internalLinkTypeTonWalletTransfer".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeTonWalletTransferBuilder { inner }
  }

  pub fn receiver(&self) -> &Option<TonWalletTransferReceiver> {
    &self.receiver
  }

  pub fn gram_amount(&self) -> i64 {
    self.gram_amount
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeTonWalletTransferBuilder {
  inner: InternalLinkTypeTonWalletTransfer,
}

impl RTDInternalLinkTypeTonWalletTransferBuilder {
  pub fn build(&self) -> InternalLinkTypeTonWalletTransfer {
    self.inner.clone()
  }

  pub fn receiver<T: AsRef<TonWalletTransferReceiver>>(&mut self, receiver: T) -> &mut Self {
    self.inner.receiver = Some(receiver.as_ref().clone());
    self
  }

  pub fn gram_amount(&mut self, gram_amount: i64) -> &mut Self {
    self.inner.gram_amount = gram_amount;
    self
  }
}

impl AsRef<InternalLinkTypeTonWalletTransfer> for InternalLinkTypeTonWalletTransfer {
  fn as_ref(&self) -> &InternalLinkTypeTonWalletTransfer {
    self
  }
}

impl AsRef<InternalLinkTypeTonWalletTransfer> for RTDInternalLinkTypeTonWalletTransferBuilder {
  fn as_ref(&self) -> &InternalLinkTypeTonWalletTransfer {
    &self.inner
  }
}

/// The link is an unknown tg: link. Call getDeepLinkInfo to process the link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeUnknownDeepLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Link to be passed to getDeepLinkInfo
  link: String,
}

impl RObject for InternalLinkTypeUnknownDeepLink {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeUnknownDeepLink"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeUnknownDeepLink {}

impl InternalLinkTypeUnknownDeepLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeUnknownDeepLinkBuilder {
    let inner = InternalLinkTypeUnknownDeepLink {
      td_name: "internalLinkTypeUnknownDeepLink".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeUnknownDeepLinkBuilder { inner }
  }

  pub fn link(&self) -> &String {
    &self.link
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeUnknownDeepLinkBuilder {
  inner: InternalLinkTypeUnknownDeepLink,
}

impl RTDInternalLinkTypeUnknownDeepLinkBuilder {
  pub fn build(&self) -> InternalLinkTypeUnknownDeepLink {
    self.inner.clone()
  }

  pub fn link<T: AsRef<str>>(&mut self, link: T) -> &mut Self {
    self.inner.link = link.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeUnknownDeepLink> for InternalLinkTypeUnknownDeepLink {
  fn as_ref(&self) -> &InternalLinkTypeUnknownDeepLink {
    self
  }
}

impl AsRef<InternalLinkTypeUnknownDeepLink> for RTDInternalLinkTypeUnknownDeepLinkBuilder {
  fn as_ref(&self) -> &InternalLinkTypeUnknownDeepLink {
    &self.inner
  }
}

/// The link is a link to an upgraded gift. Call getUpgradedGift with the given name to process the link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeUpgradedGift {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the unique gift
  name: String,
}

impl RObject for InternalLinkTypeUpgradedGift {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeUpgradedGift"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeUpgradedGift {}

impl InternalLinkTypeUpgradedGift {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeUpgradedGiftBuilder {
    let inner = InternalLinkTypeUpgradedGift {
      td_name: "internalLinkTypeUpgradedGift".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeUpgradedGiftBuilder { inner }
  }

  pub fn name(&self) -> &String {
    &self.name
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeUpgradedGiftBuilder {
  inner: InternalLinkTypeUpgradedGift,
}

impl RTDInternalLinkTypeUpgradedGiftBuilder {
  pub fn build(&self) -> InternalLinkTypeUpgradedGift {
    self.inner.clone()
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeUpgradedGift> for InternalLinkTypeUpgradedGift {
  fn as_ref(&self) -> &InternalLinkTypeUpgradedGift {
    self
  }
}

impl AsRef<InternalLinkTypeUpgradedGift> for RTDInternalLinkTypeUpgradedGiftBuilder {
  fn as_ref(&self) -> &InternalLinkTypeUpgradedGift {
    &self.inner
  }
}

/// The link is a link to a user by its phone number. Call searchUserByPhoneNumber with the given phone number to process the link. If the user is found, then call createPrivateChat and open user's profile information screen or the chat itself. If draft text isn't empty, then put the draft text in the input field
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeUserPhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Phone number of the user
  phone_number: String,
  /// Draft text for message to send in the chat
  draft_text: String,
  /// True, if user's profile information screen must be opened; otherwise, the chat itself must be opened
  open_profile: bool,
  /// Token, which allows the current user to send unpaid messages to the user; may be empty if none. If the token isn't empty and the user is found, then call applyUnpaidMessagesToken with the user and the token
  unpaid_messages_token: String,
}

impl RObject for InternalLinkTypeUserPhoneNumber {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeUserPhoneNumber"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeUserPhoneNumber {}

impl InternalLinkTypeUserPhoneNumber {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeUserPhoneNumberBuilder {
    let inner = InternalLinkTypeUserPhoneNumber {
      td_name: "internalLinkTypeUserPhoneNumber".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeUserPhoneNumberBuilder { inner }
  }

  pub fn phone_number(&self) -> &String {
    &self.phone_number
  }

  pub fn draft_text(&self) -> &String {
    &self.draft_text
  }

  pub fn open_profile(&self) -> bool {
    self.open_profile
  }

  pub fn unpaid_messages_token(&self) -> &String {
    &self.unpaid_messages_token
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeUserPhoneNumberBuilder {
  inner: InternalLinkTypeUserPhoneNumber,
}

impl RTDInternalLinkTypeUserPhoneNumberBuilder {
  pub fn build(&self) -> InternalLinkTypeUserPhoneNumber {
    self.inner.clone()
  }

  pub fn phone_number<T: AsRef<str>>(&mut self, phone_number: T) -> &mut Self {
    self.inner.phone_number = phone_number.as_ref().to_string();
    self
  }

  pub fn draft_text<T: AsRef<str>>(&mut self, draft_text: T) -> &mut Self {
    self.inner.draft_text = draft_text.as_ref().to_string();
    self
  }

  pub fn open_profile(&mut self, open_profile: bool) -> &mut Self {
    self.inner.open_profile = open_profile;
    self
  }

  pub fn unpaid_messages_token<T: AsRef<str>>(&mut self, unpaid_messages_token: T) -> &mut Self {
    self.inner.unpaid_messages_token = unpaid_messages_token.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeUserPhoneNumber> for InternalLinkTypeUserPhoneNumber {
  fn as_ref(&self) -> &InternalLinkTypeUserPhoneNumber {
    self
  }
}

impl AsRef<InternalLinkTypeUserPhoneNumber> for RTDInternalLinkTypeUserPhoneNumberBuilder {
  fn as_ref(&self) -> &InternalLinkTypeUserPhoneNumber {
    &self.inner
  }
}

/// The link is a link to a user by a temporary token. Call searchUserByToken with the given token to process the link. If the user is found, then call createPrivateChat and open the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeUserToken {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The token
  token: String,
  /// Token, which allows the current user to send unpaid messages to the user; may be empty if none. If the token isn't empty and the user is found, then call applyUnpaidMessagesToken with the user and the token
  unpaid_messages_token: String,
}

impl RObject for InternalLinkTypeUserToken {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeUserToken"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeUserToken {}

impl InternalLinkTypeUserToken {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeUserTokenBuilder {
    let inner = InternalLinkTypeUserToken {
      td_name: "internalLinkTypeUserToken".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeUserTokenBuilder { inner }
  }

  pub fn token(&self) -> &String {
    &self.token
  }

  pub fn unpaid_messages_token(&self) -> &String {
    &self.unpaid_messages_token
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeUserTokenBuilder {
  inner: InternalLinkTypeUserToken,
}

impl RTDInternalLinkTypeUserTokenBuilder {
  pub fn build(&self) -> InternalLinkTypeUserToken {
    self.inner.clone()
  }

  pub fn token<T: AsRef<str>>(&mut self, token: T) -> &mut Self {
    self.inner.token = token.as_ref().to_string();
    self
  }

  pub fn unpaid_messages_token<T: AsRef<str>>(&mut self, unpaid_messages_token: T) -> &mut Self {
    self.inner.unpaid_messages_token = unpaid_messages_token.as_ref().to_string();
    self
  }
}

impl AsRef<InternalLinkTypeUserToken> for InternalLinkTypeUserToken {
  fn as_ref(&self) -> &InternalLinkTypeUserToken {
    self
  }
}

impl AsRef<InternalLinkTypeUserToken> for RTDInternalLinkTypeUserTokenBuilder {
  fn as_ref(&self) -> &InternalLinkTypeUserToken {
    &self.inner
  }
}

/// The link is a link to a video chat. Call searchPublicChat with the given chat username, and then joinVideoChat with the given invite hash to process the link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeVideoChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Username of the chat with the video chat
  chat_username: String,
  /// If non-empty, invite hash to be used to join the video chat without being muted by administrators
  invite_hash: String,
  /// True, if the video chat is expected to be a live stream in a channel or a broadcast group
  is_live_stream: bool,
}

impl RObject for InternalLinkTypeVideoChat {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeVideoChat"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeVideoChat {}

impl InternalLinkTypeVideoChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeVideoChatBuilder {
    let inner = InternalLinkTypeVideoChat {
      td_name: "internalLinkTypeVideoChat".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeVideoChatBuilder { inner }
  }

  pub fn chat_username(&self) -> &String {
    &self.chat_username
  }

  pub fn invite_hash(&self) -> &String {
    &self.invite_hash
  }

  pub fn is_live_stream(&self) -> bool {
    self.is_live_stream
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeVideoChatBuilder {
  inner: InternalLinkTypeVideoChat,
}

impl RTDInternalLinkTypeVideoChatBuilder {
  pub fn build(&self) -> InternalLinkTypeVideoChat {
    self.inner.clone()
  }

  pub fn chat_username<T: AsRef<str>>(&mut self, chat_username: T) -> &mut Self {
    self.inner.chat_username = chat_username.as_ref().to_string();
    self
  }

  pub fn invite_hash<T: AsRef<str>>(&mut self, invite_hash: T) -> &mut Self {
    self.inner.invite_hash = invite_hash.as_ref().to_string();
    self
  }

  pub fn is_live_stream(&mut self, is_live_stream: bool) -> &mut Self {
    self.inner.is_live_stream = is_live_stream;
    self
  }
}

impl AsRef<InternalLinkTypeVideoChat> for InternalLinkTypeVideoChat {
  fn as_ref(&self) -> &InternalLinkTypeVideoChat {
    self
  }
}

impl AsRef<InternalLinkTypeVideoChat> for RTDInternalLinkTypeVideoChatBuilder {
  fn as_ref(&self) -> &InternalLinkTypeVideoChat {
    &self.inner
  }
}

/// The link is a link to a Web App. Call searchPublicChat with the given bot username, check that the user is a bot. If the bot is restricted for the current user, then show an error message. Otherwise, call searchWebApp with the received bot and the given web_app_short_name. Process received foundWebApp by showing a confirmation dialog if needed. If the bot can be added to attachment or side menu, but isn't added yet, then show a disclaimer about Mini Apps being third-party applications instead of the dialog and ask the user to accept their Terms of service. If the user accepts the terms and confirms adding, then use toggleBotIsAddedToAttachmentMenu to add the bot. Then, call getWebAppLinkUrl and open the returned URL as a Web App
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeWebApp {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Username of the bot that owns the Web App
  bot_username: String,
  /// Short name of the Web App
  web_app_short_name: String,
  /// Start parameter to be passed to getWebAppLinkUrl
  start_parameter: String,
  /// The mode in which the Web App must be opened
  mode: WebAppOpenMode,
}

impl RObject for InternalLinkTypeWebApp {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "internalLinkTypeWebApp"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInternalLinkType for InternalLinkTypeWebApp {}

impl InternalLinkTypeWebApp {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInternalLinkTypeWebAppBuilder {
    let inner = InternalLinkTypeWebApp {
      td_name: "internalLinkTypeWebApp".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInternalLinkTypeWebAppBuilder { inner }
  }

  pub fn bot_username(&self) -> &String {
    &self.bot_username
  }

  pub fn web_app_short_name(&self) -> &String {
    &self.web_app_short_name
  }

  pub fn start_parameter(&self) -> &String {
    &self.start_parameter
  }

  pub fn mode(&self) -> &WebAppOpenMode {
    &self.mode
  }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeWebAppBuilder {
  inner: InternalLinkTypeWebApp,
}

impl RTDInternalLinkTypeWebAppBuilder {
  pub fn build(&self) -> InternalLinkTypeWebApp {
    self.inner.clone()
  }

  pub fn bot_username<T: AsRef<str>>(&mut self, bot_username: T) -> &mut Self {
    self.inner.bot_username = bot_username.as_ref().to_string();
    self
  }

  pub fn web_app_short_name<T: AsRef<str>>(&mut self, web_app_short_name: T) -> &mut Self {
    self.inner.web_app_short_name = web_app_short_name.as_ref().to_string();
    self
  }

  pub fn start_parameter<T: AsRef<str>>(&mut self, start_parameter: T) -> &mut Self {
    self.inner.start_parameter = start_parameter.as_ref().to_string();
    self
  }

  pub fn mode<T: AsRef<WebAppOpenMode>>(&mut self, mode: T) -> &mut Self {
    self.inner.mode = mode.as_ref().clone();
    self
  }
}

impl AsRef<InternalLinkTypeWebApp> for InternalLinkTypeWebApp {
  fn as_ref(&self) -> &InternalLinkTypeWebApp {
    self
  }
}

impl AsRef<InternalLinkTypeWebApp> for RTDInternalLinkTypeWebAppBuilder {
  fn as_ref(&self) -> &InternalLinkTypeWebApp {
    &self.inner
  }
}
