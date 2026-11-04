use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a keyboard button type
pub trait TDKeyboardButtonType: Debug + RObject {}

/// Describes a keyboard button type
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum KeyboardButtonType {
  #[doc(hidden)]
  _Default(()),
  /// A button that requests a chat to be shared by the current user; available only in private chats. Use the method shareChatWithBot to complete the request
  RequestChat(KeyboardButtonTypeRequestChat),
  /// A button that sends the user's location when pressed; available only in private chats
  RequestLocation(KeyboardButtonTypeRequestLocation),
  /// A button that requests creation of a managed bot by the current user; available only in private chats. Use the method createBot to complete the request
  RequestManagedBot(KeyboardButtonTypeRequestManagedBot),
  /// A button that sends the user's phone number when pressed; available only in private chats
  RequestPhoneNumber(KeyboardButtonTypeRequestPhoneNumber),
  /// A button that allows the user to create and send a poll when pressed; available only in private chats
  RequestPoll(KeyboardButtonTypeRequestPoll),
  /// A button that requests users to be shared by the current user; available only in private chats. Use the method shareUsersWithBot to complete the request
  RequestUsers(KeyboardButtonTypeRequestUsers),
  /// A simple button, with text that must be sent when the button is pressed
  Text(KeyboardButtonTypeText),
  /// A button that opens a Web App by calling getWebAppUrl
  WebApp(KeyboardButtonTypeWebApp),
}

impl Default for KeyboardButtonType {
  fn default() -> Self {
    KeyboardButtonType::_Default(())
  }
}

impl<'de> Deserialize<'de> for KeyboardButtonType {
  fn deserialize<D>(deserializer: D) -> Result<KeyboardButtonType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      KeyboardButtonType,
      (keyboardButtonTypeRequestChat, RequestChat);
      (keyboardButtonTypeRequestLocation, RequestLocation);
      (keyboardButtonTypeRequestManagedBot, RequestManagedBot);
      (keyboardButtonTypeRequestPhoneNumber, RequestPhoneNumber);
      (keyboardButtonTypeRequestPoll, RequestPoll);
      (keyboardButtonTypeRequestUsers, RequestUsers);
      (keyboardButtonTypeText, Text);
      (keyboardButtonTypeWebApp, WebApp);

    )(deserializer)
  }
}

impl RObject for KeyboardButtonType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      KeyboardButtonType::RequestChat(t) => t.td_name(),
      KeyboardButtonType::RequestLocation(t) => t.td_name(),
      KeyboardButtonType::RequestManagedBot(t) => t.td_name(),
      KeyboardButtonType::RequestPhoneNumber(t) => t.td_name(),
      KeyboardButtonType::RequestPoll(t) => t.td_name(),
      KeyboardButtonType::RequestUsers(t) => t.td_name(),
      KeyboardButtonType::Text(t) => t.td_name(),
      KeyboardButtonType::WebApp(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      KeyboardButtonType::RequestChat(t) => t.extra(),
      KeyboardButtonType::RequestLocation(t) => t.extra(),
      KeyboardButtonType::RequestManagedBot(t) => t.extra(),
      KeyboardButtonType::RequestPhoneNumber(t) => t.extra(),
      KeyboardButtonType::RequestPoll(t) => t.extra(),
      KeyboardButtonType::RequestUsers(t) => t.extra(),
      KeyboardButtonType::Text(t) => t.extra(),
      KeyboardButtonType::WebApp(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl KeyboardButtonType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, KeyboardButtonType::_Default(_))
  }

  pub fn is_request_chat(&self) -> bool {
    matches!(self, KeyboardButtonType::RequestChat(_))
  }
  pub fn is_request_location(&self) -> bool {
    matches!(self, KeyboardButtonType::RequestLocation(_))
  }
  pub fn is_request_managed_bot(&self) -> bool {
    matches!(self, KeyboardButtonType::RequestManagedBot(_))
  }
  pub fn is_request_phone_number(&self) -> bool {
    matches!(self, KeyboardButtonType::RequestPhoneNumber(_))
  }
  pub fn is_request_poll(&self) -> bool {
    matches!(self, KeyboardButtonType::RequestPoll(_))
  }
  pub fn is_request_users(&self) -> bool {
    matches!(self, KeyboardButtonType::RequestUsers(_))
  }
  pub fn is_text(&self) -> bool {
    matches!(self, KeyboardButtonType::Text(_))
  }
  pub fn is_web_app(&self) -> bool {
    matches!(self, KeyboardButtonType::WebApp(_))
  }

  pub fn on_request_chat<F: FnOnce(&KeyboardButtonTypeRequestChat)>(&self, fnc: F) -> &Self {
    if let KeyboardButtonType::RequestChat(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_request_location<F: FnOnce(&KeyboardButtonTypeRequestLocation)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let KeyboardButtonType::RequestLocation(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_request_managed_bot<F: FnOnce(&KeyboardButtonTypeRequestManagedBot)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let KeyboardButtonType::RequestManagedBot(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_request_phone_number<F: FnOnce(&KeyboardButtonTypeRequestPhoneNumber)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let KeyboardButtonType::RequestPhoneNumber(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_request_poll<F: FnOnce(&KeyboardButtonTypeRequestPoll)>(&self, fnc: F) -> &Self {
    if let KeyboardButtonType::RequestPoll(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_request_users<F: FnOnce(&KeyboardButtonTypeRequestUsers)>(&self, fnc: F) -> &Self {
    if let KeyboardButtonType::RequestUsers(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_text<F: FnOnce(&KeyboardButtonTypeText)>(&self, fnc: F) -> &Self {
    if let KeyboardButtonType::Text(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_web_app<F: FnOnce(&KeyboardButtonTypeWebApp)>(&self, fnc: F) -> &Self {
    if let KeyboardButtonType::WebApp(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_request_chat(&self) -> Option<&KeyboardButtonTypeRequestChat> {
    if let KeyboardButtonType::RequestChat(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_request_location(&self) -> Option<&KeyboardButtonTypeRequestLocation> {
    if let KeyboardButtonType::RequestLocation(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_request_managed_bot(&self) -> Option<&KeyboardButtonTypeRequestManagedBot> {
    if let KeyboardButtonType::RequestManagedBot(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_request_phone_number(&self) -> Option<&KeyboardButtonTypeRequestPhoneNumber> {
    if let KeyboardButtonType::RequestPhoneNumber(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_request_poll(&self) -> Option<&KeyboardButtonTypeRequestPoll> {
    if let KeyboardButtonType::RequestPoll(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_request_users(&self) -> Option<&KeyboardButtonTypeRequestUsers> {
    if let KeyboardButtonType::RequestUsers(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_text(&self) -> Option<&KeyboardButtonTypeText> {
    if let KeyboardButtonType::Text(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_web_app(&self) -> Option<&KeyboardButtonTypeWebApp> {
    if let KeyboardButtonType::WebApp(t) = self {
      return Some(t);
    }
    None
  }

  pub fn request_chat<T: AsRef<KeyboardButtonTypeRequestChat>>(t: T) -> Self {
    KeyboardButtonType::RequestChat(t.as_ref().clone())
  }

  pub fn request_location<T: AsRef<KeyboardButtonTypeRequestLocation>>(t: T) -> Self {
    KeyboardButtonType::RequestLocation(t.as_ref().clone())
  }

  pub fn request_managed_bot<T: AsRef<KeyboardButtonTypeRequestManagedBot>>(t: T) -> Self {
    KeyboardButtonType::RequestManagedBot(t.as_ref().clone())
  }

  pub fn request_phone_number<T: AsRef<KeyboardButtonTypeRequestPhoneNumber>>(t: T) -> Self {
    KeyboardButtonType::RequestPhoneNumber(t.as_ref().clone())
  }

  pub fn request_poll<T: AsRef<KeyboardButtonTypeRequestPoll>>(t: T) -> Self {
    KeyboardButtonType::RequestPoll(t.as_ref().clone())
  }

  pub fn request_users<T: AsRef<KeyboardButtonTypeRequestUsers>>(t: T) -> Self {
    KeyboardButtonType::RequestUsers(t.as_ref().clone())
  }

  pub fn text<T: AsRef<KeyboardButtonTypeText>>(t: T) -> Self {
    KeyboardButtonType::Text(t.as_ref().clone())
  }

  pub fn web_app<T: AsRef<KeyboardButtonTypeWebApp>>(t: T) -> Self {
    KeyboardButtonType::WebApp(t.as_ref().clone())
  }
}

impl AsRef<KeyboardButtonType> for KeyboardButtonType {
  fn as_ref(&self) -> &KeyboardButtonType {
    self
  }
}

/// A button that requests a chat to be shared by the current user; available only in private chats. Use the method shareChatWithBot to complete the request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButtonTypeRequestChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique button identifier
  id: i64,
  /// True, if the chat must be a channel; otherwise, a basic group or a supergroup chat is shared
  chat_is_channel: bool,
  /// True, if the chat must or must not be a forum supergroup
  restrict_chat_is_forum: bool,
  /// True, if the chat must be a forum supergroup; otherwise, the chat must not be a forum supergroup. Ignored if restrict_chat_is_forum is false
  chat_is_forum: bool,
  /// True, if the chat must or must not have a username
  restrict_chat_has_username: bool,
  /// True, if the chat must have a username; otherwise, the chat must not have a username. Ignored if restrict_chat_has_username is false
  chat_has_username: bool,
  /// True, if the chat must be created by the current user
  chat_is_created: bool,
  /// Expected user administrator rights in the chat; may be null if they aren't restricted
  user_administrator_rights: Option<ChatAdministratorRights>,
  /// Expected bot administrator rights in the chat; may be null if they aren't restricted
  bot_administrator_rights: Option<ChatAdministratorRights>,
  /// True, if the bot must be a member of the chat; for basic group and supergroup chats only
  bot_is_member: bool,
  /// Pass true to request title of the chat; bots only
  request_title: bool,
  /// Pass true to request username of the chat; bots only
  request_username: bool,
  /// Pass true to request photo of the chat; bots only
  request_photo: bool,
}

impl RObject for KeyboardButtonTypeRequestChat {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "keyboardButtonTypeRequestChat"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDKeyboardButtonType for KeyboardButtonTypeRequestChat {}

impl KeyboardButtonTypeRequestChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDKeyboardButtonTypeRequestChatBuilder {
    let inner = KeyboardButtonTypeRequestChat {
      td_name: "keyboardButtonTypeRequestChat".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDKeyboardButtonTypeRequestChatBuilder { inner }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn chat_is_channel(&self) -> bool {
    self.chat_is_channel
  }

  pub fn restrict_chat_is_forum(&self) -> bool {
    self.restrict_chat_is_forum
  }

  pub fn chat_is_forum(&self) -> bool {
    self.chat_is_forum
  }

  pub fn restrict_chat_has_username(&self) -> bool {
    self.restrict_chat_has_username
  }

  pub fn chat_has_username(&self) -> bool {
    self.chat_has_username
  }

  pub fn chat_is_created(&self) -> bool {
    self.chat_is_created
  }

  pub fn user_administrator_rights(&self) -> &Option<ChatAdministratorRights> {
    &self.user_administrator_rights
  }

  pub fn bot_administrator_rights(&self) -> &Option<ChatAdministratorRights> {
    &self.bot_administrator_rights
  }

  pub fn bot_is_member(&self) -> bool {
    self.bot_is_member
  }

  pub fn request_title(&self) -> bool {
    self.request_title
  }

  pub fn request_username(&self) -> bool {
    self.request_username
  }

  pub fn request_photo(&self) -> bool {
    self.request_photo
  }
}

#[doc(hidden)]
pub struct RTDKeyboardButtonTypeRequestChatBuilder {
  inner: KeyboardButtonTypeRequestChat,
}

impl RTDKeyboardButtonTypeRequestChatBuilder {
  pub fn build(&self) -> KeyboardButtonTypeRequestChat {
    self.inner.clone()
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn chat_is_channel(&mut self, chat_is_channel: bool) -> &mut Self {
    self.inner.chat_is_channel = chat_is_channel;
    self
  }

  pub fn restrict_chat_is_forum(&mut self, restrict_chat_is_forum: bool) -> &mut Self {
    self.inner.restrict_chat_is_forum = restrict_chat_is_forum;
    self
  }

  pub fn chat_is_forum(&mut self, chat_is_forum: bool) -> &mut Self {
    self.inner.chat_is_forum = chat_is_forum;
    self
  }

  pub fn restrict_chat_has_username(&mut self, restrict_chat_has_username: bool) -> &mut Self {
    self.inner.restrict_chat_has_username = restrict_chat_has_username;
    self
  }

  pub fn chat_has_username(&mut self, chat_has_username: bool) -> &mut Self {
    self.inner.chat_has_username = chat_has_username;
    self
  }

  pub fn chat_is_created(&mut self, chat_is_created: bool) -> &mut Self {
    self.inner.chat_is_created = chat_is_created;
    self
  }

  pub fn user_administrator_rights<T: AsRef<ChatAdministratorRights>>(
    &mut self,
    user_administrator_rights: T,
  ) -> &mut Self {
    self.inner.user_administrator_rights = Some(user_administrator_rights.as_ref().clone());
    self
  }

  pub fn bot_administrator_rights<T: AsRef<ChatAdministratorRights>>(
    &mut self,
    bot_administrator_rights: T,
  ) -> &mut Self {
    self.inner.bot_administrator_rights = Some(bot_administrator_rights.as_ref().clone());
    self
  }

  pub fn bot_is_member(&mut self, bot_is_member: bool) -> &mut Self {
    self.inner.bot_is_member = bot_is_member;
    self
  }

  pub fn request_title(&mut self, request_title: bool) -> &mut Self {
    self.inner.request_title = request_title;
    self
  }

  pub fn request_username(&mut self, request_username: bool) -> &mut Self {
    self.inner.request_username = request_username;
    self
  }

  pub fn request_photo(&mut self, request_photo: bool) -> &mut Self {
    self.inner.request_photo = request_photo;
    self
  }
}

impl AsRef<KeyboardButtonTypeRequestChat> for KeyboardButtonTypeRequestChat {
  fn as_ref(&self) -> &KeyboardButtonTypeRequestChat {
    self
  }
}

impl AsRef<KeyboardButtonTypeRequestChat> for RTDKeyboardButtonTypeRequestChatBuilder {
  fn as_ref(&self) -> &KeyboardButtonTypeRequestChat {
    &self.inner
  }
}

/// A button that sends the user's location when pressed; available only in private chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButtonTypeRequestLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for KeyboardButtonTypeRequestLocation {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "keyboardButtonTypeRequestLocation"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDKeyboardButtonType for KeyboardButtonTypeRequestLocation {}

impl KeyboardButtonTypeRequestLocation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDKeyboardButtonTypeRequestLocationBuilder {
    let inner = KeyboardButtonTypeRequestLocation {
      td_name: "keyboardButtonTypeRequestLocation".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDKeyboardButtonTypeRequestLocationBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDKeyboardButtonTypeRequestLocationBuilder {
  inner: KeyboardButtonTypeRequestLocation,
}

impl RTDKeyboardButtonTypeRequestLocationBuilder {
  pub fn build(&self) -> KeyboardButtonTypeRequestLocation {
    self.inner.clone()
  }
}

impl AsRef<KeyboardButtonTypeRequestLocation> for KeyboardButtonTypeRequestLocation {
  fn as_ref(&self) -> &KeyboardButtonTypeRequestLocation {
    self
  }
}

impl AsRef<KeyboardButtonTypeRequestLocation> for RTDKeyboardButtonTypeRequestLocationBuilder {
  fn as_ref(&self) -> &KeyboardButtonTypeRequestLocation {
    &self.inner
  }
}

/// A button that requests creation of a managed bot by the current user; available only in private chats. Use the method createBot to complete the request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButtonTypeRequestManagedBot {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique button identifier
  id: i64,
  /// Suggested name for the bot; may be empty if not specified
  suggested_name: String,
  /// Suggested username for the bot; may be empty if not specified
  suggested_username: String,
}

impl RObject for KeyboardButtonTypeRequestManagedBot {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "keyboardButtonTypeRequestManagedBot"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDKeyboardButtonType for KeyboardButtonTypeRequestManagedBot {}

impl KeyboardButtonTypeRequestManagedBot {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDKeyboardButtonTypeRequestManagedBotBuilder {
    let inner = KeyboardButtonTypeRequestManagedBot {
      td_name: "keyboardButtonTypeRequestManagedBot".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDKeyboardButtonTypeRequestManagedBotBuilder { inner }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn suggested_name(&self) -> &String {
    &self.suggested_name
  }

  pub fn suggested_username(&self) -> &String {
    &self.suggested_username
  }
}

#[doc(hidden)]
pub struct RTDKeyboardButtonTypeRequestManagedBotBuilder {
  inner: KeyboardButtonTypeRequestManagedBot,
}

impl RTDKeyboardButtonTypeRequestManagedBotBuilder {
  pub fn build(&self) -> KeyboardButtonTypeRequestManagedBot {
    self.inner.clone()
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn suggested_name<T: AsRef<str>>(&mut self, suggested_name: T) -> &mut Self {
    self.inner.suggested_name = suggested_name.as_ref().to_string();
    self
  }

  pub fn suggested_username<T: AsRef<str>>(&mut self, suggested_username: T) -> &mut Self {
    self.inner.suggested_username = suggested_username.as_ref().to_string();
    self
  }
}

impl AsRef<KeyboardButtonTypeRequestManagedBot> for KeyboardButtonTypeRequestManagedBot {
  fn as_ref(&self) -> &KeyboardButtonTypeRequestManagedBot {
    self
  }
}

impl AsRef<KeyboardButtonTypeRequestManagedBot> for RTDKeyboardButtonTypeRequestManagedBotBuilder {
  fn as_ref(&self) -> &KeyboardButtonTypeRequestManagedBot {
    &self.inner
  }
}

/// A button that sends the user's phone number when pressed; available only in private chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButtonTypeRequestPhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for KeyboardButtonTypeRequestPhoneNumber {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "keyboardButtonTypeRequestPhoneNumber"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDKeyboardButtonType for KeyboardButtonTypeRequestPhoneNumber {}

impl KeyboardButtonTypeRequestPhoneNumber {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDKeyboardButtonTypeRequestPhoneNumberBuilder {
    let inner = KeyboardButtonTypeRequestPhoneNumber {
      td_name: "keyboardButtonTypeRequestPhoneNumber".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDKeyboardButtonTypeRequestPhoneNumberBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDKeyboardButtonTypeRequestPhoneNumberBuilder {
  inner: KeyboardButtonTypeRequestPhoneNumber,
}

impl RTDKeyboardButtonTypeRequestPhoneNumberBuilder {
  pub fn build(&self) -> KeyboardButtonTypeRequestPhoneNumber {
    self.inner.clone()
  }
}

impl AsRef<KeyboardButtonTypeRequestPhoneNumber> for KeyboardButtonTypeRequestPhoneNumber {
  fn as_ref(&self) -> &KeyboardButtonTypeRequestPhoneNumber {
    self
  }
}

impl AsRef<KeyboardButtonTypeRequestPhoneNumber>
  for RTDKeyboardButtonTypeRequestPhoneNumberBuilder
{
  fn as_ref(&self) -> &KeyboardButtonTypeRequestPhoneNumber {
    &self.inner
  }
}

/// A button that allows the user to create and send a poll when pressed; available only in private chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButtonTypeRequestPoll {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// If true, only regular polls must be allowed to create
  force_regular: bool,
  /// If true, only polls in quiz mode must be allowed to create
  force_quiz: bool,
}

impl RObject for KeyboardButtonTypeRequestPoll {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "keyboardButtonTypeRequestPoll"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDKeyboardButtonType for KeyboardButtonTypeRequestPoll {}

impl KeyboardButtonTypeRequestPoll {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDKeyboardButtonTypeRequestPollBuilder {
    let inner = KeyboardButtonTypeRequestPoll {
      td_name: "keyboardButtonTypeRequestPoll".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDKeyboardButtonTypeRequestPollBuilder { inner }
  }

  pub fn force_regular(&self) -> bool {
    self.force_regular
  }

  pub fn force_quiz(&self) -> bool {
    self.force_quiz
  }
}

#[doc(hidden)]
pub struct RTDKeyboardButtonTypeRequestPollBuilder {
  inner: KeyboardButtonTypeRequestPoll,
}

impl RTDKeyboardButtonTypeRequestPollBuilder {
  pub fn build(&self) -> KeyboardButtonTypeRequestPoll {
    self.inner.clone()
  }

  pub fn force_regular(&mut self, force_regular: bool) -> &mut Self {
    self.inner.force_regular = force_regular;
    self
  }

  pub fn force_quiz(&mut self, force_quiz: bool) -> &mut Self {
    self.inner.force_quiz = force_quiz;
    self
  }
}

impl AsRef<KeyboardButtonTypeRequestPoll> for KeyboardButtonTypeRequestPoll {
  fn as_ref(&self) -> &KeyboardButtonTypeRequestPoll {
    self
  }
}

impl AsRef<KeyboardButtonTypeRequestPoll> for RTDKeyboardButtonTypeRequestPollBuilder {
  fn as_ref(&self) -> &KeyboardButtonTypeRequestPoll {
    &self.inner
  }
}

/// A button that requests users to be shared by the current user; available only in private chats. Use the method shareUsersWithBot to complete the request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButtonTypeRequestUsers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique button identifier
  id: i64,
  /// True, if the shared users must or must not be bots
  restrict_user_is_bot: bool,
  /// True, if the shared users must be bots; otherwise, the shared users must not be bots. Ignored if restrict_user_is_bot is false
  user_is_bot: bool,
  /// True, if the shared users must or must not be Telegram Premium users
  restrict_user_is_premium: bool,
  /// True, if the shared users must be Telegram Premium users; otherwise, the shared users must not be Telegram Premium users. Ignored if restrict_user_is_premium is false
  user_is_premium: bool,
  /// The maximum number of users to share
  max_quantity: i64,
  /// Pass true to request name of the users; bots only
  request_name: bool,
  /// Pass true to request username of the users; bots only
  request_username: bool,
  /// Pass true to request photo of the users; bots only
  request_photo: bool,
}

impl RObject for KeyboardButtonTypeRequestUsers {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "keyboardButtonTypeRequestUsers"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDKeyboardButtonType for KeyboardButtonTypeRequestUsers {}

impl KeyboardButtonTypeRequestUsers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDKeyboardButtonTypeRequestUsersBuilder {
    let inner = KeyboardButtonTypeRequestUsers {
      td_name: "keyboardButtonTypeRequestUsers".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDKeyboardButtonTypeRequestUsersBuilder { inner }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn restrict_user_is_bot(&self) -> bool {
    self.restrict_user_is_bot
  }

  pub fn user_is_bot(&self) -> bool {
    self.user_is_bot
  }

  pub fn restrict_user_is_premium(&self) -> bool {
    self.restrict_user_is_premium
  }

  pub fn user_is_premium(&self) -> bool {
    self.user_is_premium
  }

  pub fn max_quantity(&self) -> i64 {
    self.max_quantity
  }

  pub fn request_name(&self) -> bool {
    self.request_name
  }

  pub fn request_username(&self) -> bool {
    self.request_username
  }

  pub fn request_photo(&self) -> bool {
    self.request_photo
  }
}

#[doc(hidden)]
pub struct RTDKeyboardButtonTypeRequestUsersBuilder {
  inner: KeyboardButtonTypeRequestUsers,
}

impl RTDKeyboardButtonTypeRequestUsersBuilder {
  pub fn build(&self) -> KeyboardButtonTypeRequestUsers {
    self.inner.clone()
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn restrict_user_is_bot(&mut self, restrict_user_is_bot: bool) -> &mut Self {
    self.inner.restrict_user_is_bot = restrict_user_is_bot;
    self
  }

  pub fn user_is_bot(&mut self, user_is_bot: bool) -> &mut Self {
    self.inner.user_is_bot = user_is_bot;
    self
  }

  pub fn restrict_user_is_premium(&mut self, restrict_user_is_premium: bool) -> &mut Self {
    self.inner.restrict_user_is_premium = restrict_user_is_premium;
    self
  }

  pub fn user_is_premium(&mut self, user_is_premium: bool) -> &mut Self {
    self.inner.user_is_premium = user_is_premium;
    self
  }

  pub fn max_quantity(&mut self, max_quantity: i64) -> &mut Self {
    self.inner.max_quantity = max_quantity;
    self
  }

  pub fn request_name(&mut self, request_name: bool) -> &mut Self {
    self.inner.request_name = request_name;
    self
  }

  pub fn request_username(&mut self, request_username: bool) -> &mut Self {
    self.inner.request_username = request_username;
    self
  }

  pub fn request_photo(&mut self, request_photo: bool) -> &mut Self {
    self.inner.request_photo = request_photo;
    self
  }
}

impl AsRef<KeyboardButtonTypeRequestUsers> for KeyboardButtonTypeRequestUsers {
  fn as_ref(&self) -> &KeyboardButtonTypeRequestUsers {
    self
  }
}

impl AsRef<KeyboardButtonTypeRequestUsers> for RTDKeyboardButtonTypeRequestUsersBuilder {
  fn as_ref(&self) -> &KeyboardButtonTypeRequestUsers {
    &self.inner
  }
}

/// A simple button, with text that must be sent when the button is pressed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButtonTypeText {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for KeyboardButtonTypeText {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "keyboardButtonTypeText"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDKeyboardButtonType for KeyboardButtonTypeText {}

impl KeyboardButtonTypeText {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDKeyboardButtonTypeTextBuilder {
    let inner = KeyboardButtonTypeText {
      td_name: "keyboardButtonTypeText".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDKeyboardButtonTypeTextBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDKeyboardButtonTypeTextBuilder {
  inner: KeyboardButtonTypeText,
}

impl RTDKeyboardButtonTypeTextBuilder {
  pub fn build(&self) -> KeyboardButtonTypeText {
    self.inner.clone()
  }
}

impl AsRef<KeyboardButtonTypeText> for KeyboardButtonTypeText {
  fn as_ref(&self) -> &KeyboardButtonTypeText {
    self
  }
}

impl AsRef<KeyboardButtonTypeText> for RTDKeyboardButtonTypeTextBuilder {
  fn as_ref(&self) -> &KeyboardButtonTypeText {
    &self.inner
  }
}

/// A button that opens a Web App by calling getWebAppUrl
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButtonTypeWebApp {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// An HTTP URL to pass to getWebAppUrl
  url: String,
}

impl RObject for KeyboardButtonTypeWebApp {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "keyboardButtonTypeWebApp"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDKeyboardButtonType for KeyboardButtonTypeWebApp {}

impl KeyboardButtonTypeWebApp {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDKeyboardButtonTypeWebAppBuilder {
    let inner = KeyboardButtonTypeWebApp {
      td_name: "keyboardButtonTypeWebApp".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDKeyboardButtonTypeWebAppBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }
}

#[doc(hidden)]
pub struct RTDKeyboardButtonTypeWebAppBuilder {
  inner: KeyboardButtonTypeWebApp,
}

impl RTDKeyboardButtonTypeWebAppBuilder {
  pub fn build(&self) -> KeyboardButtonTypeWebApp {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }
}

impl AsRef<KeyboardButtonTypeWebApp> for KeyboardButtonTypeWebApp {
  fn as_ref(&self) -> &KeyboardButtonTypeWebApp {
    self
  }
}

impl AsRef<KeyboardButtonTypeWebApp> for RTDKeyboardButtonTypeWebAppBuilder {
  fn as_ref(&self) -> &KeyboardButtonTypeWebApp {
    &self.inner
  }
}
