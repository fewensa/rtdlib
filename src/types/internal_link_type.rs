
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes an internal https://t.me or tg: link, which must be processed by the app in a special way
pub trait TDInternalLinkType: Debug + RObject {}

/// Describes an internal https://t.me or tg: link, which must be processed by the app in a special way
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InternalLinkType {
  #[doc(hidden)] _Default(()),
  /// Returns information about the type of an internal link. Returns a 404 error if the link is not internal. Can be called before authorization
  GetInternalLinkType(GetInternalLinkType),
  /// The link is a link to the active sessions section of the app. Use getActiveSessions to handle the link
  ActiveSessions(InternalLinkTypeActiveSessions),
  /// The link contains an authentication code. Call checkAuthenticationCode with the code if the current authorization state is authorizationStateWaitCode
  AuthenticationCode(InternalLinkTypeAuthenticationCode),
  /// The link is a link to a background. Call searchBackground with the given background name to process the link
  Background(InternalLinkTypeBackground),
  /// The link is a link to a chat with a Telegram bot. Call searchPublicChat with the given bot username, check that the user is a bot, show START button in the chat with the bot, and then call sendBotStartMessage with the given start parameter after the button is pressed
  BotStart(InternalLinkTypeBotStart),
  /// The link is a link to a Telegram bot, which is supposed to be added to a group chat. Call searchPublicChat with the given bot username, check that the user is a bot and can be added to groups, ask the current user to select a group to add the bot to, and then call sendBotStartMessage with the given start parameter and the chosen group chat. Bots can be added to a public group only by administrators of the group
  BotStartInGroup(InternalLinkTypeBotStartInGroup),
  /// The link is a link to the change phone number section of the app
  ChangePhoneNumber(InternalLinkTypeChangePhoneNumber),
  /// The link is a chat invite link. Call checkChatInviteLink with the given invite link to process the link
  ChatInvite(InternalLinkTypeChatInvite),
  /// The link is a link to the filter settings section of the app
  FilterSettings(InternalLinkTypeFilterSettings),
  /// The link is a link to a game. Call searchPublicChat with the given bot username, check that the user is a bot, ask the current user to select a chat to send the game, and then call sendMessage with inputMessageGame
  Game(InternalLinkTypeGame),
  /// The link is a link to a language pack. Call getLanguagePackInfo with the given language pack identifier to process the link
  LanguagePack(InternalLinkTypeLanguagePack),
  /// The link is a link to a Telegram message. Call getMessageLinkInfo with the given URL to process the link
  Message(InternalLinkTypeMessage),
  /// The link contains a message draft text. A share screen needs to be shown to the user, then the chosen chat must be opened and the text is added to the input field
  MessageDraft(InternalLinkTypeMessageDraft),
  /// The link contains a request of Telegram passport data. Call getPassportAuthorizationForm with the given parameters to process the link if the link was received from outside of the app, otherwise ignore it
  PassportDataRequest(InternalLinkTypePassportDataRequest),
  /// The link can be used to confirm ownership of a phone number to prevent account deletion. Call sendPhoneNumberConfirmationCode with the given hash and phone number to process the link
  PhoneNumberConfirmation(InternalLinkTypePhoneNumberConfirmation),
  /// The link is a link to a proxy. Call addProxy with the given parameters to process the link and add the proxy
  Proxy(InternalLinkTypeProxy),
  /// The link is a link to a chat by its username. Call searchPublicChat with the given chat username to process the link
  PublicChat(InternalLinkTypePublicChat),
  /// The link can be used to login the current user on another device, but it must be scanned from QR-code using in-app camera. An alert similar to "This code can be used to allow someone to log in to your Telegram account. To confirm Telegram login, please go to Settings > Devices > Scan QR and scan the code" needs to be shown
  QrCodeAuthentication(InternalLinkTypeQrCodeAuthentication),
  /// The link is a link to app settings
  Settings(InternalLinkTypeSettings),
  /// The link is a link to a sticker set. Call searchStickerSet with the given sticker set name to process the link and show the sticker set
  StickerSet(InternalLinkTypeStickerSet),
  /// The link is a link to a theme. TDLib has no theme support yet
  Theme(InternalLinkTypeTheme),
  /// The link is a link to the theme settings section of the app
  ThemeSettings(InternalLinkTypeThemeSettings),
  /// The link is an unknown tg: link. Call getDeepLinkInfo to process the link
  UnknownDeepLink(InternalLinkTypeUnknownDeepLink),
  /// The link is a link to an unsupported proxy. An alert can be shown to the user
  UnsupportedProxy(InternalLinkTypeUnsupportedProxy),
  /// The link is a link to a video chat. Call searchPublicChat with the given chat username, and then joinGoupCall with the given invite hash to process the link
  VideoChat(InternalLinkTypeVideoChat),

}

impl Default for InternalLinkType {
  fn default() -> Self { InternalLinkType::_Default(()) }
}

impl<'de> Deserialize<'de> for InternalLinkType {
  fn deserialize<D>(deserializer: D) -> Result<InternalLinkType, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InternalLinkType,
      (getInternalLinkType, GetInternalLinkType);
      (internalLinkTypeActiveSessions, ActiveSessions);
      (internalLinkTypeAuthenticationCode, AuthenticationCode);
      (internalLinkTypeBackground, Background);
      (internalLinkTypeBotStart, BotStart);
      (internalLinkTypeBotStartInGroup, BotStartInGroup);
      (internalLinkTypeChangePhoneNumber, ChangePhoneNumber);
      (internalLinkTypeChatInvite, ChatInvite);
      (internalLinkTypeFilterSettings, FilterSettings);
      (internalLinkTypeGame, Game);
      (internalLinkTypeLanguagePack, LanguagePack);
      (internalLinkTypeMessage, Message);
      (internalLinkTypeMessageDraft, MessageDraft);
      (internalLinkTypePassportDataRequest, PassportDataRequest);
      (internalLinkTypePhoneNumberConfirmation, PhoneNumberConfirmation);
      (internalLinkTypeProxy, Proxy);
      (internalLinkTypePublicChat, PublicChat);
      (internalLinkTypeQrCodeAuthentication, QrCodeAuthentication);
      (internalLinkTypeSettings, Settings);
      (internalLinkTypeStickerSet, StickerSet);
      (internalLinkTypeTheme, Theme);
      (internalLinkTypeThemeSettings, ThemeSettings);
      (internalLinkTypeUnknownDeepLink, UnknownDeepLink);
      (internalLinkTypeUnsupportedProxy, UnsupportedProxy);
      (internalLinkTypeVideoChat, VideoChat);

    )(deserializer)
  }
}

impl RObject for InternalLinkType {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      InternalLinkType::GetInternalLinkType(t) => t.td_name(),
      InternalLinkType::ActiveSessions(t) => t.td_name(),
      InternalLinkType::AuthenticationCode(t) => t.td_name(),
      InternalLinkType::Background(t) => t.td_name(),
      InternalLinkType::BotStart(t) => t.td_name(),
      InternalLinkType::BotStartInGroup(t) => t.td_name(),
      InternalLinkType::ChangePhoneNumber(t) => t.td_name(),
      InternalLinkType::ChatInvite(t) => t.td_name(),
      InternalLinkType::FilterSettings(t) => t.td_name(),
      InternalLinkType::Game(t) => t.td_name(),
      InternalLinkType::LanguagePack(t) => t.td_name(),
      InternalLinkType::Message(t) => t.td_name(),
      InternalLinkType::MessageDraft(t) => t.td_name(),
      InternalLinkType::PassportDataRequest(t) => t.td_name(),
      InternalLinkType::PhoneNumberConfirmation(t) => t.td_name(),
      InternalLinkType::Proxy(t) => t.td_name(),
      InternalLinkType::PublicChat(t) => t.td_name(),
      InternalLinkType::QrCodeAuthentication(t) => t.td_name(),
      InternalLinkType::Settings(t) => t.td_name(),
      InternalLinkType::StickerSet(t) => t.td_name(),
      InternalLinkType::Theme(t) => t.td_name(),
      InternalLinkType::ThemeSettings(t) => t.td_name(),
      InternalLinkType::UnknownDeepLink(t) => t.td_name(),
      InternalLinkType::UnsupportedProxy(t) => t.td_name(),
      InternalLinkType::VideoChat(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      InternalLinkType::GetInternalLinkType(t) => t.extra(),
      InternalLinkType::ActiveSessions(t) => t.extra(),
      InternalLinkType::AuthenticationCode(t) => t.extra(),
      InternalLinkType::Background(t) => t.extra(),
      InternalLinkType::BotStart(t) => t.extra(),
      InternalLinkType::BotStartInGroup(t) => t.extra(),
      InternalLinkType::ChangePhoneNumber(t) => t.extra(),
      InternalLinkType::ChatInvite(t) => t.extra(),
      InternalLinkType::FilterSettings(t) => t.extra(),
      InternalLinkType::Game(t) => t.extra(),
      InternalLinkType::LanguagePack(t) => t.extra(),
      InternalLinkType::Message(t) => t.extra(),
      InternalLinkType::MessageDraft(t) => t.extra(),
      InternalLinkType::PassportDataRequest(t) => t.extra(),
      InternalLinkType::PhoneNumberConfirmation(t) => t.extra(),
      InternalLinkType::Proxy(t) => t.extra(),
      InternalLinkType::PublicChat(t) => t.extra(),
      InternalLinkType::QrCodeAuthentication(t) => t.extra(),
      InternalLinkType::Settings(t) => t.extra(),
      InternalLinkType::StickerSet(t) => t.extra(),
      InternalLinkType::Theme(t) => t.extra(),
      InternalLinkType::ThemeSettings(t) => t.extra(),
      InternalLinkType::UnknownDeepLink(t) => t.extra(),
      InternalLinkType::UnsupportedProxy(t) => t.extra(),
      InternalLinkType::VideoChat(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl InternalLinkType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let InternalLinkType::_Default(_) = self { true } else { false } }

  pub fn is_get_internal_link_type(&self) -> bool { if let InternalLinkType::GetInternalLinkType(_) = self { true } else { false } }
  pub fn is_active_sessions(&self) -> bool { if let InternalLinkType::ActiveSessions(_) = self { true } else { false } }
  pub fn is_authentication_code(&self) -> bool { if let InternalLinkType::AuthenticationCode(_) = self { true } else { false } }
  pub fn is_background(&self) -> bool { if let InternalLinkType::Background(_) = self { true } else { false } }
  pub fn is_bot_start(&self) -> bool { if let InternalLinkType::BotStart(_) = self { true } else { false } }
  pub fn is_bot_start_in_group(&self) -> bool { if let InternalLinkType::BotStartInGroup(_) = self { true } else { false } }
  pub fn is_change_phone_number(&self) -> bool { if let InternalLinkType::ChangePhoneNumber(_) = self { true } else { false } }
  pub fn is_chat_invite(&self) -> bool { if let InternalLinkType::ChatInvite(_) = self { true } else { false } }
  pub fn is_filter_settings(&self) -> bool { if let InternalLinkType::FilterSettings(_) = self { true } else { false } }
  pub fn is_game(&self) -> bool { if let InternalLinkType::Game(_) = self { true } else { false } }
  pub fn is_language_pack(&self) -> bool { if let InternalLinkType::LanguagePack(_) = self { true } else { false } }
  pub fn is_message(&self) -> bool { if let InternalLinkType::Message(_) = self { true } else { false } }
  pub fn is_message_draft(&self) -> bool { if let InternalLinkType::MessageDraft(_) = self { true } else { false } }
  pub fn is_passport_data_request(&self) -> bool { if let InternalLinkType::PassportDataRequest(_) = self { true } else { false } }
  pub fn is_phone_number_confirmation(&self) -> bool { if let InternalLinkType::PhoneNumberConfirmation(_) = self { true } else { false } }
  pub fn is_proxy(&self) -> bool { if let InternalLinkType::Proxy(_) = self { true } else { false } }
  pub fn is_public_chat(&self) -> bool { if let InternalLinkType::PublicChat(_) = self { true } else { false } }
  pub fn is_qr_code_authentication(&self) -> bool { if let InternalLinkType::QrCodeAuthentication(_) = self { true } else { false } }
  pub fn is_settings(&self) -> bool { if let InternalLinkType::Settings(_) = self { true } else { false } }
  pub fn is_sticker_set(&self) -> bool { if let InternalLinkType::StickerSet(_) = self { true } else { false } }
  pub fn is_theme(&self) -> bool { if let InternalLinkType::Theme(_) = self { true } else { false } }
  pub fn is_theme_settings(&self) -> bool { if let InternalLinkType::ThemeSettings(_) = self { true } else { false } }
  pub fn is_unknown_deep_link(&self) -> bool { if let InternalLinkType::UnknownDeepLink(_) = self { true } else { false } }
  pub fn is_unsupported_proxy(&self) -> bool { if let InternalLinkType::UnsupportedProxy(_) = self { true } else { false } }
  pub fn is_video_chat(&self) -> bool { if let InternalLinkType::VideoChat(_) = self { true } else { false } }

  pub fn on_get_internal_link_type<F: FnOnce(&GetInternalLinkType)>(&self, fnc: F) -> &Self { if let InternalLinkType::GetInternalLinkType(t) = self { fnc(t) }; self }
  pub fn on_active_sessions<F: FnOnce(&InternalLinkTypeActiveSessions)>(&self, fnc: F) -> &Self { if let InternalLinkType::ActiveSessions(t) = self { fnc(t) }; self }
  pub fn on_authentication_code<F: FnOnce(&InternalLinkTypeAuthenticationCode)>(&self, fnc: F) -> &Self { if let InternalLinkType::AuthenticationCode(t) = self { fnc(t) }; self }
  pub fn on_background<F: FnOnce(&InternalLinkTypeBackground)>(&self, fnc: F) -> &Self { if let InternalLinkType::Background(t) = self { fnc(t) }; self }
  pub fn on_bot_start<F: FnOnce(&InternalLinkTypeBotStart)>(&self, fnc: F) -> &Self { if let InternalLinkType::BotStart(t) = self { fnc(t) }; self }
  pub fn on_bot_start_in_group<F: FnOnce(&InternalLinkTypeBotStartInGroup)>(&self, fnc: F) -> &Self { if let InternalLinkType::BotStartInGroup(t) = self { fnc(t) }; self }
  pub fn on_change_phone_number<F: FnOnce(&InternalLinkTypeChangePhoneNumber)>(&self, fnc: F) -> &Self { if let InternalLinkType::ChangePhoneNumber(t) = self { fnc(t) }; self }
  pub fn on_chat_invite<F: FnOnce(&InternalLinkTypeChatInvite)>(&self, fnc: F) -> &Self { if let InternalLinkType::ChatInvite(t) = self { fnc(t) }; self }
  pub fn on_filter_settings<F: FnOnce(&InternalLinkTypeFilterSettings)>(&self, fnc: F) -> &Self { if let InternalLinkType::FilterSettings(t) = self { fnc(t) }; self }
  pub fn on_game<F: FnOnce(&InternalLinkTypeGame)>(&self, fnc: F) -> &Self { if let InternalLinkType::Game(t) = self { fnc(t) }; self }
  pub fn on_language_pack<F: FnOnce(&InternalLinkTypeLanguagePack)>(&self, fnc: F) -> &Self { if let InternalLinkType::LanguagePack(t) = self { fnc(t) }; self }
  pub fn on_message<F: FnOnce(&InternalLinkTypeMessage)>(&self, fnc: F) -> &Self { if let InternalLinkType::Message(t) = self { fnc(t) }; self }
  pub fn on_message_draft<F: FnOnce(&InternalLinkTypeMessageDraft)>(&self, fnc: F) -> &Self { if let InternalLinkType::MessageDraft(t) = self { fnc(t) }; self }
  pub fn on_passport_data_request<F: FnOnce(&InternalLinkTypePassportDataRequest)>(&self, fnc: F) -> &Self { if let InternalLinkType::PassportDataRequest(t) = self { fnc(t) }; self }
  pub fn on_phone_number_confirmation<F: FnOnce(&InternalLinkTypePhoneNumberConfirmation)>(&self, fnc: F) -> &Self { if let InternalLinkType::PhoneNumberConfirmation(t) = self { fnc(t) }; self }
  pub fn on_proxy<F: FnOnce(&InternalLinkTypeProxy)>(&self, fnc: F) -> &Self { if let InternalLinkType::Proxy(t) = self { fnc(t) }; self }
  pub fn on_public_chat<F: FnOnce(&InternalLinkTypePublicChat)>(&self, fnc: F) -> &Self { if let InternalLinkType::PublicChat(t) = self { fnc(t) }; self }
  pub fn on_qr_code_authentication<F: FnOnce(&InternalLinkTypeQrCodeAuthentication)>(&self, fnc: F) -> &Self { if let InternalLinkType::QrCodeAuthentication(t) = self { fnc(t) }; self }
  pub fn on_settings<F: FnOnce(&InternalLinkTypeSettings)>(&self, fnc: F) -> &Self { if let InternalLinkType::Settings(t) = self { fnc(t) }; self }
  pub fn on_sticker_set<F: FnOnce(&InternalLinkTypeStickerSet)>(&self, fnc: F) -> &Self { if let InternalLinkType::StickerSet(t) = self { fnc(t) }; self }
  pub fn on_theme<F: FnOnce(&InternalLinkTypeTheme)>(&self, fnc: F) -> &Self { if let InternalLinkType::Theme(t) = self { fnc(t) }; self }
  pub fn on_theme_settings<F: FnOnce(&InternalLinkTypeThemeSettings)>(&self, fnc: F) -> &Self { if let InternalLinkType::ThemeSettings(t) = self { fnc(t) }; self }
  pub fn on_unknown_deep_link<F: FnOnce(&InternalLinkTypeUnknownDeepLink)>(&self, fnc: F) -> &Self { if let InternalLinkType::UnknownDeepLink(t) = self { fnc(t) }; self }
  pub fn on_unsupported_proxy<F: FnOnce(&InternalLinkTypeUnsupportedProxy)>(&self, fnc: F) -> &Self { if let InternalLinkType::UnsupportedProxy(t) = self { fnc(t) }; self }
  pub fn on_video_chat<F: FnOnce(&InternalLinkTypeVideoChat)>(&self, fnc: F) -> &Self { if let InternalLinkType::VideoChat(t) = self { fnc(t) }; self }

  pub fn as_get_internal_link_type(&self) -> Option<&GetInternalLinkType> { if let InternalLinkType::GetInternalLinkType(t) = self { return Some(t) } None }
  pub fn as_active_sessions(&self) -> Option<&InternalLinkTypeActiveSessions> { if let InternalLinkType::ActiveSessions(t) = self { return Some(t) } None }
  pub fn as_authentication_code(&self) -> Option<&InternalLinkTypeAuthenticationCode> { if let InternalLinkType::AuthenticationCode(t) = self { return Some(t) } None }
  pub fn as_background(&self) -> Option<&InternalLinkTypeBackground> { if let InternalLinkType::Background(t) = self { return Some(t) } None }
  pub fn as_bot_start(&self) -> Option<&InternalLinkTypeBotStart> { if let InternalLinkType::BotStart(t) = self { return Some(t) } None }
  pub fn as_bot_start_in_group(&self) -> Option<&InternalLinkTypeBotStartInGroup> { if let InternalLinkType::BotStartInGroup(t) = self { return Some(t) } None }
  pub fn as_change_phone_number(&self) -> Option<&InternalLinkTypeChangePhoneNumber> { if let InternalLinkType::ChangePhoneNumber(t) = self { return Some(t) } None }
  pub fn as_chat_invite(&self) -> Option<&InternalLinkTypeChatInvite> { if let InternalLinkType::ChatInvite(t) = self { return Some(t) } None }
  pub fn as_filter_settings(&self) -> Option<&InternalLinkTypeFilterSettings> { if let InternalLinkType::FilterSettings(t) = self { return Some(t) } None }
  pub fn as_game(&self) -> Option<&InternalLinkTypeGame> { if let InternalLinkType::Game(t) = self { return Some(t) } None }
  pub fn as_language_pack(&self) -> Option<&InternalLinkTypeLanguagePack> { if let InternalLinkType::LanguagePack(t) = self { return Some(t) } None }
  pub fn as_message(&self) -> Option<&InternalLinkTypeMessage> { if let InternalLinkType::Message(t) = self { return Some(t) } None }
  pub fn as_message_draft(&self) -> Option<&InternalLinkTypeMessageDraft> { if let InternalLinkType::MessageDraft(t) = self { return Some(t) } None }
  pub fn as_passport_data_request(&self) -> Option<&InternalLinkTypePassportDataRequest> { if let InternalLinkType::PassportDataRequest(t) = self { return Some(t) } None }
  pub fn as_phone_number_confirmation(&self) -> Option<&InternalLinkTypePhoneNumberConfirmation> { if let InternalLinkType::PhoneNumberConfirmation(t) = self { return Some(t) } None }
  pub fn as_proxy(&self) -> Option<&InternalLinkTypeProxy> { if let InternalLinkType::Proxy(t) = self { return Some(t) } None }
  pub fn as_public_chat(&self) -> Option<&InternalLinkTypePublicChat> { if let InternalLinkType::PublicChat(t) = self { return Some(t) } None }
  pub fn as_qr_code_authentication(&self) -> Option<&InternalLinkTypeQrCodeAuthentication> { if let InternalLinkType::QrCodeAuthentication(t) = self { return Some(t) } None }
  pub fn as_settings(&self) -> Option<&InternalLinkTypeSettings> { if let InternalLinkType::Settings(t) = self { return Some(t) } None }
  pub fn as_sticker_set(&self) -> Option<&InternalLinkTypeStickerSet> { if let InternalLinkType::StickerSet(t) = self { return Some(t) } None }
  pub fn as_theme(&self) -> Option<&InternalLinkTypeTheme> { if let InternalLinkType::Theme(t) = self { return Some(t) } None }
  pub fn as_theme_settings(&self) -> Option<&InternalLinkTypeThemeSettings> { if let InternalLinkType::ThemeSettings(t) = self { return Some(t) } None }
  pub fn as_unknown_deep_link(&self) -> Option<&InternalLinkTypeUnknownDeepLink> { if let InternalLinkType::UnknownDeepLink(t) = self { return Some(t) } None }
  pub fn as_unsupported_proxy(&self) -> Option<&InternalLinkTypeUnsupportedProxy> { if let InternalLinkType::UnsupportedProxy(t) = self { return Some(t) } None }
  pub fn as_video_chat(&self) -> Option<&InternalLinkTypeVideoChat> { if let InternalLinkType::VideoChat(t) = self { return Some(t) } None }



  pub fn get_internal_link_type<T: AsRef<GetInternalLinkType>>(t: T) -> Self { InternalLinkType::GetInternalLinkType(t.as_ref().clone()) }

  pub fn active_sessions<T: AsRef<InternalLinkTypeActiveSessions>>(t: T) -> Self { InternalLinkType::ActiveSessions(t.as_ref().clone()) }

  pub fn authentication_code<T: AsRef<InternalLinkTypeAuthenticationCode>>(t: T) -> Self { InternalLinkType::AuthenticationCode(t.as_ref().clone()) }

  pub fn background<T: AsRef<InternalLinkTypeBackground>>(t: T) -> Self { InternalLinkType::Background(t.as_ref().clone()) }

  pub fn bot_start<T: AsRef<InternalLinkTypeBotStart>>(t: T) -> Self { InternalLinkType::BotStart(t.as_ref().clone()) }

  pub fn bot_start_in_group<T: AsRef<InternalLinkTypeBotStartInGroup>>(t: T) -> Self { InternalLinkType::BotStartInGroup(t.as_ref().clone()) }

  pub fn change_phone_number<T: AsRef<InternalLinkTypeChangePhoneNumber>>(t: T) -> Self { InternalLinkType::ChangePhoneNumber(t.as_ref().clone()) }

  pub fn chat_invite<T: AsRef<InternalLinkTypeChatInvite>>(t: T) -> Self { InternalLinkType::ChatInvite(t.as_ref().clone()) }

  pub fn filter_settings<T: AsRef<InternalLinkTypeFilterSettings>>(t: T) -> Self { InternalLinkType::FilterSettings(t.as_ref().clone()) }

  pub fn game<T: AsRef<InternalLinkTypeGame>>(t: T) -> Self { InternalLinkType::Game(t.as_ref().clone()) }

  pub fn language_pack<T: AsRef<InternalLinkTypeLanguagePack>>(t: T) -> Self { InternalLinkType::LanguagePack(t.as_ref().clone()) }

  pub fn message<T: AsRef<InternalLinkTypeMessage>>(t: T) -> Self { InternalLinkType::Message(t.as_ref().clone()) }

  pub fn message_draft<T: AsRef<InternalLinkTypeMessageDraft>>(t: T) -> Self { InternalLinkType::MessageDraft(t.as_ref().clone()) }

  pub fn passport_data_request<T: AsRef<InternalLinkTypePassportDataRequest>>(t: T) -> Self { InternalLinkType::PassportDataRequest(t.as_ref().clone()) }

  pub fn phone_number_confirmation<T: AsRef<InternalLinkTypePhoneNumberConfirmation>>(t: T) -> Self { InternalLinkType::PhoneNumberConfirmation(t.as_ref().clone()) }

  pub fn proxy<T: AsRef<InternalLinkTypeProxy>>(t: T) -> Self { InternalLinkType::Proxy(t.as_ref().clone()) }

  pub fn public_chat<T: AsRef<InternalLinkTypePublicChat>>(t: T) -> Self { InternalLinkType::PublicChat(t.as_ref().clone()) }

  pub fn qr_code_authentication<T: AsRef<InternalLinkTypeQrCodeAuthentication>>(t: T) -> Self { InternalLinkType::QrCodeAuthentication(t.as_ref().clone()) }

  pub fn settings<T: AsRef<InternalLinkTypeSettings>>(t: T) -> Self { InternalLinkType::Settings(t.as_ref().clone()) }

  pub fn sticker_set<T: AsRef<InternalLinkTypeStickerSet>>(t: T) -> Self { InternalLinkType::StickerSet(t.as_ref().clone()) }

  pub fn theme<T: AsRef<InternalLinkTypeTheme>>(t: T) -> Self { InternalLinkType::Theme(t.as_ref().clone()) }

  pub fn theme_settings<T: AsRef<InternalLinkTypeThemeSettings>>(t: T) -> Self { InternalLinkType::ThemeSettings(t.as_ref().clone()) }

  pub fn unknown_deep_link<T: AsRef<InternalLinkTypeUnknownDeepLink>>(t: T) -> Self { InternalLinkType::UnknownDeepLink(t.as_ref().clone()) }

  pub fn unsupported_proxy<T: AsRef<InternalLinkTypeUnsupportedProxy>>(t: T) -> Self { InternalLinkType::UnsupportedProxy(t.as_ref().clone()) }

  pub fn video_chat<T: AsRef<InternalLinkTypeVideoChat>>(t: T) -> Self { InternalLinkType::VideoChat(t.as_ref().clone()) }

}

impl AsRef<InternalLinkType> for InternalLinkType {
  fn as_ref(&self) -> &InternalLinkType { self }
}







/// The link is a link to the active sessions section of the app. Use getActiveSessions to handle the link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeActiveSessions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for InternalLinkTypeActiveSessions {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "internalLinkTypeActiveSessions" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInternalLinkType for InternalLinkTypeActiveSessions {}



impl InternalLinkTypeActiveSessions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInternalLinkTypeActiveSessionsBuilder {
    let mut inner = InternalLinkTypeActiveSessions::default();
    inner.td_name = "internalLinkTypeActiveSessions".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInternalLinkTypeActiveSessionsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDInternalLinkTypeActiveSessionsBuilder {
  inner: InternalLinkTypeActiveSessions
}

impl RTDInternalLinkTypeActiveSessionsBuilder {
  pub fn build(&self) -> InternalLinkTypeActiveSessions { self.inner.clone() }

}

impl AsRef<InternalLinkTypeActiveSessions> for InternalLinkTypeActiveSessions {
  fn as_ref(&self) -> &InternalLinkTypeActiveSessions { self }
}

impl AsRef<InternalLinkTypeActiveSessions> for RTDInternalLinkTypeActiveSessionsBuilder {
  fn as_ref(&self) -> &InternalLinkTypeActiveSessions { &self.inner }
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "internalLinkTypeAuthenticationCode" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInternalLinkType for InternalLinkTypeAuthenticationCode {}



impl InternalLinkTypeAuthenticationCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInternalLinkTypeAuthenticationCodeBuilder {
    let mut inner = InternalLinkTypeAuthenticationCode::default();
    inner.td_name = "internalLinkTypeAuthenticationCode".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInternalLinkTypeAuthenticationCodeBuilder { inner }
  }

  pub fn code(&self) -> &String { &self.code }

}

#[doc(hidden)]
pub struct RTDInternalLinkTypeAuthenticationCodeBuilder {
  inner: InternalLinkTypeAuthenticationCode
}

impl RTDInternalLinkTypeAuthenticationCodeBuilder {
  pub fn build(&self) -> InternalLinkTypeAuthenticationCode { self.inner.clone() }

   
  pub fn code<T: AsRef<str>>(&mut self, code: T) -> &mut Self {
    self.inner.code = code.as_ref().to_string();
    self
  }

}

impl AsRef<InternalLinkTypeAuthenticationCode> for InternalLinkTypeAuthenticationCode {
  fn as_ref(&self) -> &InternalLinkTypeAuthenticationCode { self }
}

impl AsRef<InternalLinkTypeAuthenticationCode> for RTDInternalLinkTypeAuthenticationCodeBuilder {
  fn as_ref(&self) -> &InternalLinkTypeAuthenticationCode { &self.inner }
}







/// The link is a link to a background. Call searchBackground with the given background name to process the link
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "internalLinkTypeBackground" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInternalLinkType for InternalLinkTypeBackground {}



impl InternalLinkTypeBackground {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInternalLinkTypeBackgroundBuilder {
    let mut inner = InternalLinkTypeBackground::default();
    inner.td_name = "internalLinkTypeBackground".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInternalLinkTypeBackgroundBuilder { inner }
  }

  pub fn background_name(&self) -> &String { &self.background_name }

}

#[doc(hidden)]
pub struct RTDInternalLinkTypeBackgroundBuilder {
  inner: InternalLinkTypeBackground
}

impl RTDInternalLinkTypeBackgroundBuilder {
  pub fn build(&self) -> InternalLinkTypeBackground { self.inner.clone() }

   
  pub fn background_name<T: AsRef<str>>(&mut self, background_name: T) -> &mut Self {
    self.inner.background_name = background_name.as_ref().to_string();
    self
  }

}

impl AsRef<InternalLinkTypeBackground> for InternalLinkTypeBackground {
  fn as_ref(&self) -> &InternalLinkTypeBackground { self }
}

impl AsRef<InternalLinkTypeBackground> for RTDInternalLinkTypeBackgroundBuilder {
  fn as_ref(&self) -> &InternalLinkTypeBackground { &self.inner }
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
  
}

impl RObject for InternalLinkTypeBotStart {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "internalLinkTypeBotStart" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInternalLinkType for InternalLinkTypeBotStart {}



impl InternalLinkTypeBotStart {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInternalLinkTypeBotStartBuilder {
    let mut inner = InternalLinkTypeBotStart::default();
    inner.td_name = "internalLinkTypeBotStart".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInternalLinkTypeBotStartBuilder { inner }
  }

  pub fn bot_username(&self) -> &String { &self.bot_username }

  pub fn start_parameter(&self) -> &String { &self.start_parameter }

}

#[doc(hidden)]
pub struct RTDInternalLinkTypeBotStartBuilder {
  inner: InternalLinkTypeBotStart
}

impl RTDInternalLinkTypeBotStartBuilder {
  pub fn build(&self) -> InternalLinkTypeBotStart { self.inner.clone() }

   
  pub fn bot_username<T: AsRef<str>>(&mut self, bot_username: T) -> &mut Self {
    self.inner.bot_username = bot_username.as_ref().to_string();
    self
  }

   
  pub fn start_parameter<T: AsRef<str>>(&mut self, start_parameter: T) -> &mut Self {
    self.inner.start_parameter = start_parameter.as_ref().to_string();
    self
  }

}

impl AsRef<InternalLinkTypeBotStart> for InternalLinkTypeBotStart {
  fn as_ref(&self) -> &InternalLinkTypeBotStart { self }
}

impl AsRef<InternalLinkTypeBotStart> for RTDInternalLinkTypeBotStartBuilder {
  fn as_ref(&self) -> &InternalLinkTypeBotStart { &self.inner }
}







/// The link is a link to a Telegram bot, which is supposed to be added to a group chat. Call searchPublicChat with the given bot username, check that the user is a bot and can be added to groups, ask the current user to select a group to add the bot to, and then call sendBotStartMessage with the given start parameter and the chosen group chat. Bots can be added to a public group only by administrators of the group
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
  
}

impl RObject for InternalLinkTypeBotStartInGroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "internalLinkTypeBotStartInGroup" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInternalLinkType for InternalLinkTypeBotStartInGroup {}



impl InternalLinkTypeBotStartInGroup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInternalLinkTypeBotStartInGroupBuilder {
    let mut inner = InternalLinkTypeBotStartInGroup::default();
    inner.td_name = "internalLinkTypeBotStartInGroup".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInternalLinkTypeBotStartInGroupBuilder { inner }
  }

  pub fn bot_username(&self) -> &String { &self.bot_username }

  pub fn start_parameter(&self) -> &String { &self.start_parameter }

}

#[doc(hidden)]
pub struct RTDInternalLinkTypeBotStartInGroupBuilder {
  inner: InternalLinkTypeBotStartInGroup
}

impl RTDInternalLinkTypeBotStartInGroupBuilder {
  pub fn build(&self) -> InternalLinkTypeBotStartInGroup { self.inner.clone() }

   
  pub fn bot_username<T: AsRef<str>>(&mut self, bot_username: T) -> &mut Self {
    self.inner.bot_username = bot_username.as_ref().to_string();
    self
  }

   
  pub fn start_parameter<T: AsRef<str>>(&mut self, start_parameter: T) -> &mut Self {
    self.inner.start_parameter = start_parameter.as_ref().to_string();
    self
  }

}

impl AsRef<InternalLinkTypeBotStartInGroup> for InternalLinkTypeBotStartInGroup {
  fn as_ref(&self) -> &InternalLinkTypeBotStartInGroup { self }
}

impl AsRef<InternalLinkTypeBotStartInGroup> for RTDInternalLinkTypeBotStartInGroupBuilder {
  fn as_ref(&self) -> &InternalLinkTypeBotStartInGroup { &self.inner }
}







/// The link is a link to the change phone number section of the app
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeChangePhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for InternalLinkTypeChangePhoneNumber {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "internalLinkTypeChangePhoneNumber" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInternalLinkType for InternalLinkTypeChangePhoneNumber {}



impl InternalLinkTypeChangePhoneNumber {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInternalLinkTypeChangePhoneNumberBuilder {
    let mut inner = InternalLinkTypeChangePhoneNumber::default();
    inner.td_name = "internalLinkTypeChangePhoneNumber".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInternalLinkTypeChangePhoneNumberBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDInternalLinkTypeChangePhoneNumberBuilder {
  inner: InternalLinkTypeChangePhoneNumber
}

impl RTDInternalLinkTypeChangePhoneNumberBuilder {
  pub fn build(&self) -> InternalLinkTypeChangePhoneNumber { self.inner.clone() }

}

impl AsRef<InternalLinkTypeChangePhoneNumber> for InternalLinkTypeChangePhoneNumber {
  fn as_ref(&self) -> &InternalLinkTypeChangePhoneNumber { self }
}

impl AsRef<InternalLinkTypeChangePhoneNumber> for RTDInternalLinkTypeChangePhoneNumberBuilder {
  fn as_ref(&self) -> &InternalLinkTypeChangePhoneNumber { &self.inner }
}







/// The link is a chat invite link. Call checkChatInviteLink with the given invite link to process the link
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "internalLinkTypeChatInvite" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInternalLinkType for InternalLinkTypeChatInvite {}



impl InternalLinkTypeChatInvite {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInternalLinkTypeChatInviteBuilder {
    let mut inner = InternalLinkTypeChatInvite::default();
    inner.td_name = "internalLinkTypeChatInvite".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInternalLinkTypeChatInviteBuilder { inner }
  }

  pub fn invite_link(&self) -> &String { &self.invite_link }

}

#[doc(hidden)]
pub struct RTDInternalLinkTypeChatInviteBuilder {
  inner: InternalLinkTypeChatInvite
}

impl RTDInternalLinkTypeChatInviteBuilder {
  pub fn build(&self) -> InternalLinkTypeChatInvite { self.inner.clone() }

   
  pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
    self.inner.invite_link = invite_link.as_ref().to_string();
    self
  }

}

impl AsRef<InternalLinkTypeChatInvite> for InternalLinkTypeChatInvite {
  fn as_ref(&self) -> &InternalLinkTypeChatInvite { self }
}

impl AsRef<InternalLinkTypeChatInvite> for RTDInternalLinkTypeChatInviteBuilder {
  fn as_ref(&self) -> &InternalLinkTypeChatInvite { &self.inner }
}







/// The link is a link to the filter settings section of the app
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeFilterSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for InternalLinkTypeFilterSettings {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "internalLinkTypeFilterSettings" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInternalLinkType for InternalLinkTypeFilterSettings {}



impl InternalLinkTypeFilterSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInternalLinkTypeFilterSettingsBuilder {
    let mut inner = InternalLinkTypeFilterSettings::default();
    inner.td_name = "internalLinkTypeFilterSettings".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInternalLinkTypeFilterSettingsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDInternalLinkTypeFilterSettingsBuilder {
  inner: InternalLinkTypeFilterSettings
}

impl RTDInternalLinkTypeFilterSettingsBuilder {
  pub fn build(&self) -> InternalLinkTypeFilterSettings { self.inner.clone() }

}

impl AsRef<InternalLinkTypeFilterSettings> for InternalLinkTypeFilterSettings {
  fn as_ref(&self) -> &InternalLinkTypeFilterSettings { self }
}

impl AsRef<InternalLinkTypeFilterSettings> for RTDInternalLinkTypeFilterSettingsBuilder {
  fn as_ref(&self) -> &InternalLinkTypeFilterSettings { &self.inner }
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "internalLinkTypeGame" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInternalLinkType for InternalLinkTypeGame {}



impl InternalLinkTypeGame {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInternalLinkTypeGameBuilder {
    let mut inner = InternalLinkTypeGame::default();
    inner.td_name = "internalLinkTypeGame".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInternalLinkTypeGameBuilder { inner }
  }

  pub fn bot_username(&self) -> &String { &self.bot_username }

  pub fn game_short_name(&self) -> &String { &self.game_short_name }

}

#[doc(hidden)]
pub struct RTDInternalLinkTypeGameBuilder {
  inner: InternalLinkTypeGame
}

impl RTDInternalLinkTypeGameBuilder {
  pub fn build(&self) -> InternalLinkTypeGame { self.inner.clone() }

   
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
  fn as_ref(&self) -> &InternalLinkTypeGame { self }
}

impl AsRef<InternalLinkTypeGame> for RTDInternalLinkTypeGameBuilder {
  fn as_ref(&self) -> &InternalLinkTypeGame { &self.inner }
}







/// The link is a link to a language pack. Call getLanguagePackInfo with the given language pack identifier to process the link
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "internalLinkTypeLanguagePack" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInternalLinkType for InternalLinkTypeLanguagePack {}



impl InternalLinkTypeLanguagePack {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInternalLinkTypeLanguagePackBuilder {
    let mut inner = InternalLinkTypeLanguagePack::default();
    inner.td_name = "internalLinkTypeLanguagePack".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInternalLinkTypeLanguagePackBuilder { inner }
  }

  pub fn language_pack_id(&self) -> &String { &self.language_pack_id }

}

#[doc(hidden)]
pub struct RTDInternalLinkTypeLanguagePackBuilder {
  inner: InternalLinkTypeLanguagePack
}

impl RTDInternalLinkTypeLanguagePackBuilder {
  pub fn build(&self) -> InternalLinkTypeLanguagePack { self.inner.clone() }

   
  pub fn language_pack_id<T: AsRef<str>>(&mut self, language_pack_id: T) -> &mut Self {
    self.inner.language_pack_id = language_pack_id.as_ref().to_string();
    self
  }

}

impl AsRef<InternalLinkTypeLanguagePack> for InternalLinkTypeLanguagePack {
  fn as_ref(&self) -> &InternalLinkTypeLanguagePack { self }
}

impl AsRef<InternalLinkTypeLanguagePack> for RTDInternalLinkTypeLanguagePackBuilder {
  fn as_ref(&self) -> &InternalLinkTypeLanguagePack { &self.inner }
}







/// The link is a link to a Telegram message. Call getMessageLinkInfo with the given URL to process the link
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "internalLinkTypeMessage" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInternalLinkType for InternalLinkTypeMessage {}



impl InternalLinkTypeMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInternalLinkTypeMessageBuilder {
    let mut inner = InternalLinkTypeMessage::default();
    inner.td_name = "internalLinkTypeMessage".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInternalLinkTypeMessageBuilder { inner }
  }

  pub fn url(&self) -> &String { &self.url }

}

#[doc(hidden)]
pub struct RTDInternalLinkTypeMessageBuilder {
  inner: InternalLinkTypeMessage
}

impl RTDInternalLinkTypeMessageBuilder {
  pub fn build(&self) -> InternalLinkTypeMessage { self.inner.clone() }

   
  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

}

impl AsRef<InternalLinkTypeMessage> for InternalLinkTypeMessage {
  fn as_ref(&self) -> &InternalLinkTypeMessage { self }
}

impl AsRef<InternalLinkTypeMessage> for RTDInternalLinkTypeMessageBuilder {
  fn as_ref(&self) -> &InternalLinkTypeMessage { &self.inner }
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "internalLinkTypeMessageDraft" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInternalLinkType for InternalLinkTypeMessageDraft {}



impl InternalLinkTypeMessageDraft {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInternalLinkTypeMessageDraftBuilder {
    let mut inner = InternalLinkTypeMessageDraft::default();
    inner.td_name = "internalLinkTypeMessageDraft".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInternalLinkTypeMessageDraftBuilder { inner }
  }

  pub fn text(&self) -> &FormattedText { &self.text }

  pub fn contains_link(&self) -> bool { self.contains_link }

}

#[doc(hidden)]
pub struct RTDInternalLinkTypeMessageDraftBuilder {
  inner: InternalLinkTypeMessageDraft
}

impl RTDInternalLinkTypeMessageDraftBuilder {
  pub fn build(&self) -> InternalLinkTypeMessageDraft { self.inner.clone() }

   
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
  fn as_ref(&self) -> &InternalLinkTypeMessageDraft { self }
}

impl AsRef<InternalLinkTypeMessageDraft> for RTDInternalLinkTypeMessageDraftBuilder {
  fn as_ref(&self) -> &InternalLinkTypeMessageDraft { &self.inner }
}







/// The link contains a request of Telegram passport data. Call getPassportAuthorizationForm with the given parameters to process the link if the link was received from outside of the app, otherwise ignore it
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypePassportDataRequest {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier of the service's bot
  bot_user_id: i64,
  /// Telegram Passport element types requested by the service
  scope: String,
  /// Service's public key
  public_key: String,
  /// Unique request identifier provided by the service
  nonce: String,
  /// An HTTP URL to open once the request is finished or canceled with the parameter tg_passport=success or tg_passport=cancel respectively. If empty, then the link tgbot{bot_user_id}://passport/success or tgbot{bot_user_id}://passport/cancel needs to be opened instead
  callback_url: String,
  
}

impl RObject for InternalLinkTypePassportDataRequest {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "internalLinkTypePassportDataRequest" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInternalLinkType for InternalLinkTypePassportDataRequest {}



impl InternalLinkTypePassportDataRequest {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInternalLinkTypePassportDataRequestBuilder {
    let mut inner = InternalLinkTypePassportDataRequest::default();
    inner.td_name = "internalLinkTypePassportDataRequest".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInternalLinkTypePassportDataRequestBuilder { inner }
  }

  pub fn bot_user_id(&self) -> i64 { self.bot_user_id }

  pub fn scope(&self) -> &String { &self.scope }

  pub fn public_key(&self) -> &String { &self.public_key }

  pub fn nonce(&self) -> &String { &self.nonce }

  pub fn callback_url(&self) -> &String { &self.callback_url }

}

#[doc(hidden)]
pub struct RTDInternalLinkTypePassportDataRequestBuilder {
  inner: InternalLinkTypePassportDataRequest
}

impl RTDInternalLinkTypePassportDataRequestBuilder {
  pub fn build(&self) -> InternalLinkTypePassportDataRequest { self.inner.clone() }

   
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
  fn as_ref(&self) -> &InternalLinkTypePassportDataRequest { self }
}

impl AsRef<InternalLinkTypePassportDataRequest> for RTDInternalLinkTypePassportDataRequestBuilder {
  fn as_ref(&self) -> &InternalLinkTypePassportDataRequest { &self.inner }
}







/// The link can be used to confirm ownership of a phone number to prevent account deletion. Call sendPhoneNumberConfirmationCode with the given hash and phone number to process the link
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "internalLinkTypePhoneNumberConfirmation" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInternalLinkType for InternalLinkTypePhoneNumberConfirmation {}



impl InternalLinkTypePhoneNumberConfirmation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInternalLinkTypePhoneNumberConfirmationBuilder {
    let mut inner = InternalLinkTypePhoneNumberConfirmation::default();
    inner.td_name = "internalLinkTypePhoneNumberConfirmation".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInternalLinkTypePhoneNumberConfirmationBuilder { inner }
  }

  pub fn hash(&self) -> &String { &self.hash }

  pub fn phone_number(&self) -> &String { &self.phone_number }

}

#[doc(hidden)]
pub struct RTDInternalLinkTypePhoneNumberConfirmationBuilder {
  inner: InternalLinkTypePhoneNumberConfirmation
}

impl RTDInternalLinkTypePhoneNumberConfirmationBuilder {
  pub fn build(&self) -> InternalLinkTypePhoneNumberConfirmation { self.inner.clone() }

   
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
  fn as_ref(&self) -> &InternalLinkTypePhoneNumberConfirmation { self }
}

impl AsRef<InternalLinkTypePhoneNumberConfirmation> for RTDInternalLinkTypePhoneNumberConfirmationBuilder {
  fn as_ref(&self) -> &InternalLinkTypePhoneNumberConfirmation { &self.inner }
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
  /// Proxy server IP address
  server: String,
  /// Proxy server port
  port: i64,
  /// Type of the proxy
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: ProxyType,
  
}

impl RObject for InternalLinkTypeProxy {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "internalLinkTypeProxy" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInternalLinkType for InternalLinkTypeProxy {}



impl InternalLinkTypeProxy {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInternalLinkTypeProxyBuilder {
    let mut inner = InternalLinkTypeProxy::default();
    inner.td_name = "internalLinkTypeProxy".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInternalLinkTypeProxyBuilder { inner }
  }

  pub fn server(&self) -> &String { &self.server }

  pub fn port(&self) -> i64 { self.port }

  pub fn type_(&self) -> &ProxyType { &self.type_ }

}

#[doc(hidden)]
pub struct RTDInternalLinkTypeProxyBuilder {
  inner: InternalLinkTypeProxy
}

impl RTDInternalLinkTypeProxyBuilder {
  pub fn build(&self) -> InternalLinkTypeProxy { self.inner.clone() }

   
  pub fn server<T: AsRef<str>>(&mut self, server: T) -> &mut Self {
    self.inner.server = server.as_ref().to_string();
    self
  }

   
  pub fn port(&mut self, port: i64) -> &mut Self {
    self.inner.port = port;
    self
  }

   
  pub fn type_<T: AsRef<ProxyType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

}

impl AsRef<InternalLinkTypeProxy> for InternalLinkTypeProxy {
  fn as_ref(&self) -> &InternalLinkTypeProxy { self }
}

impl AsRef<InternalLinkTypeProxy> for RTDInternalLinkTypeProxyBuilder {
  fn as_ref(&self) -> &InternalLinkTypeProxy { &self.inner }
}







/// The link is a link to a chat by its username. Call searchPublicChat with the given chat username to process the link
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
  
}

impl RObject for InternalLinkTypePublicChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "internalLinkTypePublicChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInternalLinkType for InternalLinkTypePublicChat {}



impl InternalLinkTypePublicChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInternalLinkTypePublicChatBuilder {
    let mut inner = InternalLinkTypePublicChat::default();
    inner.td_name = "internalLinkTypePublicChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInternalLinkTypePublicChatBuilder { inner }
  }

  pub fn chat_username(&self) -> &String { &self.chat_username }

}

#[doc(hidden)]
pub struct RTDInternalLinkTypePublicChatBuilder {
  inner: InternalLinkTypePublicChat
}

impl RTDInternalLinkTypePublicChatBuilder {
  pub fn build(&self) -> InternalLinkTypePublicChat { self.inner.clone() }

   
  pub fn chat_username<T: AsRef<str>>(&mut self, chat_username: T) -> &mut Self {
    self.inner.chat_username = chat_username.as_ref().to_string();
    self
  }

}

impl AsRef<InternalLinkTypePublicChat> for InternalLinkTypePublicChat {
  fn as_ref(&self) -> &InternalLinkTypePublicChat { self }
}

impl AsRef<InternalLinkTypePublicChat> for RTDInternalLinkTypePublicChatBuilder {
  fn as_ref(&self) -> &InternalLinkTypePublicChat { &self.inner }
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "internalLinkTypeQrCodeAuthentication" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInternalLinkType for InternalLinkTypeQrCodeAuthentication {}



impl InternalLinkTypeQrCodeAuthentication {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInternalLinkTypeQrCodeAuthenticationBuilder {
    let mut inner = InternalLinkTypeQrCodeAuthentication::default();
    inner.td_name = "internalLinkTypeQrCodeAuthentication".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInternalLinkTypeQrCodeAuthenticationBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDInternalLinkTypeQrCodeAuthenticationBuilder {
  inner: InternalLinkTypeQrCodeAuthentication
}

impl RTDInternalLinkTypeQrCodeAuthenticationBuilder {
  pub fn build(&self) -> InternalLinkTypeQrCodeAuthentication { self.inner.clone() }

}

impl AsRef<InternalLinkTypeQrCodeAuthentication> for InternalLinkTypeQrCodeAuthentication {
  fn as_ref(&self) -> &InternalLinkTypeQrCodeAuthentication { self }
}

impl AsRef<InternalLinkTypeQrCodeAuthentication> for RTDInternalLinkTypeQrCodeAuthenticationBuilder {
  fn as_ref(&self) -> &InternalLinkTypeQrCodeAuthentication { &self.inner }
}







/// The link is a link to app settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for InternalLinkTypeSettings {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "internalLinkTypeSettings" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInternalLinkType for InternalLinkTypeSettings {}



impl InternalLinkTypeSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInternalLinkTypeSettingsBuilder {
    let mut inner = InternalLinkTypeSettings::default();
    inner.td_name = "internalLinkTypeSettings".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInternalLinkTypeSettingsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDInternalLinkTypeSettingsBuilder {
  inner: InternalLinkTypeSettings
}

impl RTDInternalLinkTypeSettingsBuilder {
  pub fn build(&self) -> InternalLinkTypeSettings { self.inner.clone() }

}

impl AsRef<InternalLinkTypeSettings> for InternalLinkTypeSettings {
  fn as_ref(&self) -> &InternalLinkTypeSettings { self }
}

impl AsRef<InternalLinkTypeSettings> for RTDInternalLinkTypeSettingsBuilder {
  fn as_ref(&self) -> &InternalLinkTypeSettings { &self.inner }
}







/// The link is a link to a sticker set. Call searchStickerSet with the given sticker set name to process the link and show the sticker set
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
  
}

impl RObject for InternalLinkTypeStickerSet {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "internalLinkTypeStickerSet" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInternalLinkType for InternalLinkTypeStickerSet {}



impl InternalLinkTypeStickerSet {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInternalLinkTypeStickerSetBuilder {
    let mut inner = InternalLinkTypeStickerSet::default();
    inner.td_name = "internalLinkTypeStickerSet".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInternalLinkTypeStickerSetBuilder { inner }
  }

  pub fn sticker_set_name(&self) -> &String { &self.sticker_set_name }

}

#[doc(hidden)]
pub struct RTDInternalLinkTypeStickerSetBuilder {
  inner: InternalLinkTypeStickerSet
}

impl RTDInternalLinkTypeStickerSetBuilder {
  pub fn build(&self) -> InternalLinkTypeStickerSet { self.inner.clone() }

   
  pub fn sticker_set_name<T: AsRef<str>>(&mut self, sticker_set_name: T) -> &mut Self {
    self.inner.sticker_set_name = sticker_set_name.as_ref().to_string();
    self
  }

}

impl AsRef<InternalLinkTypeStickerSet> for InternalLinkTypeStickerSet {
  fn as_ref(&self) -> &InternalLinkTypeStickerSet { self }
}

impl AsRef<InternalLinkTypeStickerSet> for RTDInternalLinkTypeStickerSetBuilder {
  fn as_ref(&self) -> &InternalLinkTypeStickerSet { &self.inner }
}







/// The link is a link to a theme. TDLib has no theme support yet
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "internalLinkTypeTheme" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInternalLinkType for InternalLinkTypeTheme {}



impl InternalLinkTypeTheme {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInternalLinkTypeThemeBuilder {
    let mut inner = InternalLinkTypeTheme::default();
    inner.td_name = "internalLinkTypeTheme".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInternalLinkTypeThemeBuilder { inner }
  }

  pub fn theme_name(&self) -> &String { &self.theme_name }

}

#[doc(hidden)]
pub struct RTDInternalLinkTypeThemeBuilder {
  inner: InternalLinkTypeTheme
}

impl RTDInternalLinkTypeThemeBuilder {
  pub fn build(&self) -> InternalLinkTypeTheme { self.inner.clone() }

   
  pub fn theme_name<T: AsRef<str>>(&mut self, theme_name: T) -> &mut Self {
    self.inner.theme_name = theme_name.as_ref().to_string();
    self
  }

}

impl AsRef<InternalLinkTypeTheme> for InternalLinkTypeTheme {
  fn as_ref(&self) -> &InternalLinkTypeTheme { self }
}

impl AsRef<InternalLinkTypeTheme> for RTDInternalLinkTypeThemeBuilder {
  fn as_ref(&self) -> &InternalLinkTypeTheme { &self.inner }
}







/// The link is a link to the theme settings section of the app
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeThemeSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for InternalLinkTypeThemeSettings {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "internalLinkTypeThemeSettings" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInternalLinkType for InternalLinkTypeThemeSettings {}



impl InternalLinkTypeThemeSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInternalLinkTypeThemeSettingsBuilder {
    let mut inner = InternalLinkTypeThemeSettings::default();
    inner.td_name = "internalLinkTypeThemeSettings".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInternalLinkTypeThemeSettingsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDInternalLinkTypeThemeSettingsBuilder {
  inner: InternalLinkTypeThemeSettings
}

impl RTDInternalLinkTypeThemeSettingsBuilder {
  pub fn build(&self) -> InternalLinkTypeThemeSettings { self.inner.clone() }

}

impl AsRef<InternalLinkTypeThemeSettings> for InternalLinkTypeThemeSettings {
  fn as_ref(&self) -> &InternalLinkTypeThemeSettings { self }
}

impl AsRef<InternalLinkTypeThemeSettings> for RTDInternalLinkTypeThemeSettingsBuilder {
  fn as_ref(&self) -> &InternalLinkTypeThemeSettings { &self.inner }
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "internalLinkTypeUnknownDeepLink" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInternalLinkType for InternalLinkTypeUnknownDeepLink {}



impl InternalLinkTypeUnknownDeepLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInternalLinkTypeUnknownDeepLinkBuilder {
    let mut inner = InternalLinkTypeUnknownDeepLink::default();
    inner.td_name = "internalLinkTypeUnknownDeepLink".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInternalLinkTypeUnknownDeepLinkBuilder { inner }
  }

  pub fn link(&self) -> &String { &self.link }

}

#[doc(hidden)]
pub struct RTDInternalLinkTypeUnknownDeepLinkBuilder {
  inner: InternalLinkTypeUnknownDeepLink
}

impl RTDInternalLinkTypeUnknownDeepLinkBuilder {
  pub fn build(&self) -> InternalLinkTypeUnknownDeepLink { self.inner.clone() }

   
  pub fn link<T: AsRef<str>>(&mut self, link: T) -> &mut Self {
    self.inner.link = link.as_ref().to_string();
    self
  }

}

impl AsRef<InternalLinkTypeUnknownDeepLink> for InternalLinkTypeUnknownDeepLink {
  fn as_ref(&self) -> &InternalLinkTypeUnknownDeepLink { self }
}

impl AsRef<InternalLinkTypeUnknownDeepLink> for RTDInternalLinkTypeUnknownDeepLinkBuilder {
  fn as_ref(&self) -> &InternalLinkTypeUnknownDeepLink { &self.inner }
}







/// The link is a link to an unsupported proxy. An alert can be shown to the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeUnsupportedProxy {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for InternalLinkTypeUnsupportedProxy {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "internalLinkTypeUnsupportedProxy" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInternalLinkType for InternalLinkTypeUnsupportedProxy {}



impl InternalLinkTypeUnsupportedProxy {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInternalLinkTypeUnsupportedProxyBuilder {
    let mut inner = InternalLinkTypeUnsupportedProxy::default();
    inner.td_name = "internalLinkTypeUnsupportedProxy".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInternalLinkTypeUnsupportedProxyBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDInternalLinkTypeUnsupportedProxyBuilder {
  inner: InternalLinkTypeUnsupportedProxy
}

impl RTDInternalLinkTypeUnsupportedProxyBuilder {
  pub fn build(&self) -> InternalLinkTypeUnsupportedProxy { self.inner.clone() }

}

impl AsRef<InternalLinkTypeUnsupportedProxy> for InternalLinkTypeUnsupportedProxy {
  fn as_ref(&self) -> &InternalLinkTypeUnsupportedProxy { self }
}

impl AsRef<InternalLinkTypeUnsupportedProxy> for RTDInternalLinkTypeUnsupportedProxyBuilder {
  fn as_ref(&self) -> &InternalLinkTypeUnsupportedProxy { &self.inner }
}







/// The link is a link to a video chat. Call searchPublicChat with the given chat username, and then joinGoupCall with the given invite hash to process the link
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "internalLinkTypeVideoChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInternalLinkType for InternalLinkTypeVideoChat {}



impl InternalLinkTypeVideoChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInternalLinkTypeVideoChatBuilder {
    let mut inner = InternalLinkTypeVideoChat::default();
    inner.td_name = "internalLinkTypeVideoChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInternalLinkTypeVideoChatBuilder { inner }
  }

  pub fn chat_username(&self) -> &String { &self.chat_username }

  pub fn invite_hash(&self) -> &String { &self.invite_hash }

  pub fn is_live_stream(&self) -> bool { self.is_live_stream }

}

#[doc(hidden)]
pub struct RTDInternalLinkTypeVideoChatBuilder {
  inner: InternalLinkTypeVideoChat
}

impl RTDInternalLinkTypeVideoChatBuilder {
  pub fn build(&self) -> InternalLinkTypeVideoChat { self.inner.clone() }

   
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
  fn as_ref(&self) -> &InternalLinkTypeVideoChat { self }
}

impl AsRef<InternalLinkTypeVideoChat> for RTDInternalLinkTypeVideoChatBuilder {
  fn as_ref(&self) -> &InternalLinkTypeVideoChat { &self.inner }
}



