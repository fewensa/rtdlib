use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes the type of inline keyboard button
pub trait TDInlineKeyboardButtonType: Debug + RObject {}

/// Describes the type of inline keyboard button
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InlineKeyboardButtonType {
  #[doc(hidden)]
  _Default(()),
  /// A button to buy something. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type messageInvoice
  Buy(InlineKeyboardButtonTypeBuy),
  /// A button that sends a callback query to a bot
  Callback(InlineKeyboardButtonTypeCallback),
  /// A button with a game that sends a callback query to a bot. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type messageGame
  CallbackGame(InlineKeyboardButtonTypeCallbackGame),
  /// A button that asks for the 2-step verification password of the current user and then sends a callback query to a bot
  CallbackWithPassword(InlineKeyboardButtonTypeCallbackWithPassword),
  /// A button that copies specified text to clipboard
  CopyText(InlineKeyboardButtonTypeCopyText),
  /// A disabled button
  Disabled(InlineKeyboardButtonTypeDisabled),
  /// A button that opens a specified URL and automatically authorize the current user by calling getLoginUrlInfo; not supported in ephemeral messages
  LoginUrl(InlineKeyboardButtonTypeLoginUrl),
  /// A button that forces an inline query to the bot to be inserted in the input field
  SwitchInline(InlineKeyboardButtonTypeSwitchInline),
  /// A button that opens a specified URL
  Url(InlineKeyboardButtonTypeUrl),
  /// A button with a user reference to be handled in the same way as textEntityTypeMentionName entities
  User(InlineKeyboardButtonTypeUser),
  /// A button that opens a Web App by calling openWebApp
  WebApp(InlineKeyboardButtonTypeWebApp),
}

impl Default for InlineKeyboardButtonType {
  fn default() -> Self {
    InlineKeyboardButtonType::_Default(())
  }
}

impl<'de> Deserialize<'de> for InlineKeyboardButtonType {
  fn deserialize<D>(deserializer: D) -> Result<InlineKeyboardButtonType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InlineKeyboardButtonType,
      (inlineKeyboardButtonTypeBuy, Buy);
      (inlineKeyboardButtonTypeCallback, Callback);
      (inlineKeyboardButtonTypeCallbackGame, CallbackGame);
      (inlineKeyboardButtonTypeCallbackWithPassword, CallbackWithPassword);
      (inlineKeyboardButtonTypeCopyText, CopyText);
      (inlineKeyboardButtonTypeDisabled, Disabled);
      (inlineKeyboardButtonTypeLoginUrl, LoginUrl);
      (inlineKeyboardButtonTypeSwitchInline, SwitchInline);
      (inlineKeyboardButtonTypeUrl, Url);
      (inlineKeyboardButtonTypeUser, User);
      (inlineKeyboardButtonTypeWebApp, WebApp);

    )(deserializer)
  }
}

impl RObject for InlineKeyboardButtonType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      InlineKeyboardButtonType::Buy(t) => t.td_name(),
      InlineKeyboardButtonType::Callback(t) => t.td_name(),
      InlineKeyboardButtonType::CallbackGame(t) => t.td_name(),
      InlineKeyboardButtonType::CallbackWithPassword(t) => t.td_name(),
      InlineKeyboardButtonType::CopyText(t) => t.td_name(),
      InlineKeyboardButtonType::Disabled(t) => t.td_name(),
      InlineKeyboardButtonType::LoginUrl(t) => t.td_name(),
      InlineKeyboardButtonType::SwitchInline(t) => t.td_name(),
      InlineKeyboardButtonType::Url(t) => t.td_name(),
      InlineKeyboardButtonType::User(t) => t.td_name(),
      InlineKeyboardButtonType::WebApp(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      InlineKeyboardButtonType::Buy(t) => t.extra(),
      InlineKeyboardButtonType::Callback(t) => t.extra(),
      InlineKeyboardButtonType::CallbackGame(t) => t.extra(),
      InlineKeyboardButtonType::CallbackWithPassword(t) => t.extra(),
      InlineKeyboardButtonType::CopyText(t) => t.extra(),
      InlineKeyboardButtonType::Disabled(t) => t.extra(),
      InlineKeyboardButtonType::LoginUrl(t) => t.extra(),
      InlineKeyboardButtonType::SwitchInline(t) => t.extra(),
      InlineKeyboardButtonType::Url(t) => t.extra(),
      InlineKeyboardButtonType::User(t) => t.extra(),
      InlineKeyboardButtonType::WebApp(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InlineKeyboardButtonType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, InlineKeyboardButtonType::_Default(_))
  }

  pub fn is_buy(&self) -> bool {
    matches!(self, InlineKeyboardButtonType::Buy(_))
  }
  pub fn is_callback(&self) -> bool {
    matches!(self, InlineKeyboardButtonType::Callback(_))
  }
  pub fn is_callback_game(&self) -> bool {
    matches!(self, InlineKeyboardButtonType::CallbackGame(_))
  }
  pub fn is_callback_with_password(&self) -> bool {
    matches!(self, InlineKeyboardButtonType::CallbackWithPassword(_))
  }
  pub fn is_copy_text(&self) -> bool {
    matches!(self, InlineKeyboardButtonType::CopyText(_))
  }
  pub fn is_disabled(&self) -> bool {
    matches!(self, InlineKeyboardButtonType::Disabled(_))
  }
  pub fn is_login_url(&self) -> bool {
    matches!(self, InlineKeyboardButtonType::LoginUrl(_))
  }
  pub fn is_switch_inline(&self) -> bool {
    matches!(self, InlineKeyboardButtonType::SwitchInline(_))
  }
  pub fn is_url(&self) -> bool {
    matches!(self, InlineKeyboardButtonType::Url(_))
  }
  pub fn is_user(&self) -> bool {
    matches!(self, InlineKeyboardButtonType::User(_))
  }
  pub fn is_web_app(&self) -> bool {
    matches!(self, InlineKeyboardButtonType::WebApp(_))
  }

  pub fn on_buy<F: FnOnce(&InlineKeyboardButtonTypeBuy)>(&self, fnc: F) -> &Self {
    if let InlineKeyboardButtonType::Buy(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_callback<F: FnOnce(&InlineKeyboardButtonTypeCallback)>(&self, fnc: F) -> &Self {
    if let InlineKeyboardButtonType::Callback(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_callback_game<F: FnOnce(&InlineKeyboardButtonTypeCallbackGame)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InlineKeyboardButtonType::CallbackGame(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_callback_with_password<F: FnOnce(&InlineKeyboardButtonTypeCallbackWithPassword)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InlineKeyboardButtonType::CallbackWithPassword(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_copy_text<F: FnOnce(&InlineKeyboardButtonTypeCopyText)>(&self, fnc: F) -> &Self {
    if let InlineKeyboardButtonType::CopyText(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_disabled<F: FnOnce(&InlineKeyboardButtonTypeDisabled)>(&self, fnc: F) -> &Self {
    if let InlineKeyboardButtonType::Disabled(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_login_url<F: FnOnce(&InlineKeyboardButtonTypeLoginUrl)>(&self, fnc: F) -> &Self {
    if let InlineKeyboardButtonType::LoginUrl(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_switch_inline<F: FnOnce(&InlineKeyboardButtonTypeSwitchInline)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InlineKeyboardButtonType::SwitchInline(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_url<F: FnOnce(&InlineKeyboardButtonTypeUrl)>(&self, fnc: F) -> &Self {
    if let InlineKeyboardButtonType::Url(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_user<F: FnOnce(&InlineKeyboardButtonTypeUser)>(&self, fnc: F) -> &Self {
    if let InlineKeyboardButtonType::User(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_web_app<F: FnOnce(&InlineKeyboardButtonTypeWebApp)>(&self, fnc: F) -> &Self {
    if let InlineKeyboardButtonType::WebApp(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_buy(&self) -> Option<&InlineKeyboardButtonTypeBuy> {
    if let InlineKeyboardButtonType::Buy(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_callback(&self) -> Option<&InlineKeyboardButtonTypeCallback> {
    if let InlineKeyboardButtonType::Callback(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_callback_game(&self) -> Option<&InlineKeyboardButtonTypeCallbackGame> {
    if let InlineKeyboardButtonType::CallbackGame(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_callback_with_password(&self) -> Option<&InlineKeyboardButtonTypeCallbackWithPassword> {
    if let InlineKeyboardButtonType::CallbackWithPassword(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_copy_text(&self) -> Option<&InlineKeyboardButtonTypeCopyText> {
    if let InlineKeyboardButtonType::CopyText(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_disabled(&self) -> Option<&InlineKeyboardButtonTypeDisabled> {
    if let InlineKeyboardButtonType::Disabled(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_login_url(&self) -> Option<&InlineKeyboardButtonTypeLoginUrl> {
    if let InlineKeyboardButtonType::LoginUrl(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_switch_inline(&self) -> Option<&InlineKeyboardButtonTypeSwitchInline> {
    if let InlineKeyboardButtonType::SwitchInline(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_url(&self) -> Option<&InlineKeyboardButtonTypeUrl> {
    if let InlineKeyboardButtonType::Url(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_user(&self) -> Option<&InlineKeyboardButtonTypeUser> {
    if let InlineKeyboardButtonType::User(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_web_app(&self) -> Option<&InlineKeyboardButtonTypeWebApp> {
    if let InlineKeyboardButtonType::WebApp(t) = self {
      return Some(t);
    }
    None
  }

  pub fn buy<T: AsRef<InlineKeyboardButtonTypeBuy>>(t: T) -> Self {
    InlineKeyboardButtonType::Buy(t.as_ref().clone())
  }

  pub fn callback<T: AsRef<InlineKeyboardButtonTypeCallback>>(t: T) -> Self {
    InlineKeyboardButtonType::Callback(t.as_ref().clone())
  }

  pub fn callback_game<T: AsRef<InlineKeyboardButtonTypeCallbackGame>>(t: T) -> Self {
    InlineKeyboardButtonType::CallbackGame(t.as_ref().clone())
  }

  pub fn callback_with_password<T: AsRef<InlineKeyboardButtonTypeCallbackWithPassword>>(
    t: T,
  ) -> Self {
    InlineKeyboardButtonType::CallbackWithPassword(t.as_ref().clone())
  }

  pub fn copy_text<T: AsRef<InlineKeyboardButtonTypeCopyText>>(t: T) -> Self {
    InlineKeyboardButtonType::CopyText(t.as_ref().clone())
  }

  pub fn disabled<T: AsRef<InlineKeyboardButtonTypeDisabled>>(t: T) -> Self {
    InlineKeyboardButtonType::Disabled(t.as_ref().clone())
  }

  pub fn login_url<T: AsRef<InlineKeyboardButtonTypeLoginUrl>>(t: T) -> Self {
    InlineKeyboardButtonType::LoginUrl(t.as_ref().clone())
  }

  pub fn switch_inline<T: AsRef<InlineKeyboardButtonTypeSwitchInline>>(t: T) -> Self {
    InlineKeyboardButtonType::SwitchInline(t.as_ref().clone())
  }

  pub fn url<T: AsRef<InlineKeyboardButtonTypeUrl>>(t: T) -> Self {
    InlineKeyboardButtonType::Url(t.as_ref().clone())
  }

  pub fn user<T: AsRef<InlineKeyboardButtonTypeUser>>(t: T) -> Self {
    InlineKeyboardButtonType::User(t.as_ref().clone())
  }

  pub fn web_app<T: AsRef<InlineKeyboardButtonTypeWebApp>>(t: T) -> Self {
    InlineKeyboardButtonType::WebApp(t.as_ref().clone())
  }
}

impl AsRef<InlineKeyboardButtonType> for InlineKeyboardButtonType {
  fn as_ref(&self) -> &InlineKeyboardButtonType {
    self
  }
}

/// A button to buy something. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type messageInvoice
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeBuy {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for InlineKeyboardButtonTypeBuy {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inlineKeyboardButtonTypeBuy"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeBuy {}

impl InlineKeyboardButtonTypeBuy {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInlineKeyboardButtonTypeBuyBuilder {
    let inner = InlineKeyboardButtonTypeBuy {
      td_name: "inlineKeyboardButtonTypeBuy".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDInlineKeyboardButtonTypeBuyBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDInlineKeyboardButtonTypeBuyBuilder {
  inner: InlineKeyboardButtonTypeBuy,
}

impl RTDInlineKeyboardButtonTypeBuyBuilder {
  pub fn build(&self) -> InlineKeyboardButtonTypeBuy {
    self.inner.clone()
  }
}

impl AsRef<InlineKeyboardButtonTypeBuy> for InlineKeyboardButtonTypeBuy {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeBuy {
    self
  }
}

impl AsRef<InlineKeyboardButtonTypeBuy> for RTDInlineKeyboardButtonTypeBuyBuilder {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeBuy {
    &self.inner
  }
}

/// A button that sends a callback query to a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeCallback {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Data to be sent to the bot via a callback query
  data: String,
}

impl RObject for InlineKeyboardButtonTypeCallback {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inlineKeyboardButtonTypeCallback"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeCallback {}

impl InlineKeyboardButtonTypeCallback {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInlineKeyboardButtonTypeCallbackBuilder {
    let inner = InlineKeyboardButtonTypeCallback {
      td_name: "inlineKeyboardButtonTypeCallback".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInlineKeyboardButtonTypeCallbackBuilder { inner }
  }

  pub fn data(&self) -> &String {
    &self.data
  }
}

#[doc(hidden)]
pub struct RTDInlineKeyboardButtonTypeCallbackBuilder {
  inner: InlineKeyboardButtonTypeCallback,
}

impl RTDInlineKeyboardButtonTypeCallbackBuilder {
  pub fn build(&self) -> InlineKeyboardButtonTypeCallback {
    self.inner.clone()
  }

  pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
    self.inner.data = data.as_ref().to_string();
    self
  }
}

impl AsRef<InlineKeyboardButtonTypeCallback> for InlineKeyboardButtonTypeCallback {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeCallback {
    self
  }
}

impl AsRef<InlineKeyboardButtonTypeCallback> for RTDInlineKeyboardButtonTypeCallbackBuilder {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeCallback {
    &self.inner
  }
}

/// A button with a game that sends a callback query to a bot. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type messageGame
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeCallbackGame {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for InlineKeyboardButtonTypeCallbackGame {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inlineKeyboardButtonTypeCallbackGame"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeCallbackGame {}

impl InlineKeyboardButtonTypeCallbackGame {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInlineKeyboardButtonTypeCallbackGameBuilder {
    let inner = InlineKeyboardButtonTypeCallbackGame {
      td_name: "inlineKeyboardButtonTypeCallbackGame".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDInlineKeyboardButtonTypeCallbackGameBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDInlineKeyboardButtonTypeCallbackGameBuilder {
  inner: InlineKeyboardButtonTypeCallbackGame,
}

impl RTDInlineKeyboardButtonTypeCallbackGameBuilder {
  pub fn build(&self) -> InlineKeyboardButtonTypeCallbackGame {
    self.inner.clone()
  }
}

impl AsRef<InlineKeyboardButtonTypeCallbackGame> for InlineKeyboardButtonTypeCallbackGame {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeCallbackGame {
    self
  }
}

impl AsRef<InlineKeyboardButtonTypeCallbackGame>
  for RTDInlineKeyboardButtonTypeCallbackGameBuilder
{
  fn as_ref(&self) -> &InlineKeyboardButtonTypeCallbackGame {
    &self.inner
  }
}

/// A button that asks for the 2-step verification password of the current user and then sends a callback query to a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeCallbackWithPassword {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Data to be sent to the bot via a callback query
  data: String,
}

impl RObject for InlineKeyboardButtonTypeCallbackWithPassword {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inlineKeyboardButtonTypeCallbackWithPassword"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeCallbackWithPassword {}

impl InlineKeyboardButtonTypeCallbackWithPassword {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInlineKeyboardButtonTypeCallbackWithPasswordBuilder {
    let inner = InlineKeyboardButtonTypeCallbackWithPassword {
      td_name: "inlineKeyboardButtonTypeCallbackWithPassword".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInlineKeyboardButtonTypeCallbackWithPasswordBuilder { inner }
  }

  pub fn data(&self) -> &String {
    &self.data
  }
}

#[doc(hidden)]
pub struct RTDInlineKeyboardButtonTypeCallbackWithPasswordBuilder {
  inner: InlineKeyboardButtonTypeCallbackWithPassword,
}

impl RTDInlineKeyboardButtonTypeCallbackWithPasswordBuilder {
  pub fn build(&self) -> InlineKeyboardButtonTypeCallbackWithPassword {
    self.inner.clone()
  }

  pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
    self.inner.data = data.as_ref().to_string();
    self
  }
}

impl AsRef<InlineKeyboardButtonTypeCallbackWithPassword>
  for InlineKeyboardButtonTypeCallbackWithPassword
{
  fn as_ref(&self) -> &InlineKeyboardButtonTypeCallbackWithPassword {
    self
  }
}

impl AsRef<InlineKeyboardButtonTypeCallbackWithPassword>
  for RTDInlineKeyboardButtonTypeCallbackWithPasswordBuilder
{
  fn as_ref(&self) -> &InlineKeyboardButtonTypeCallbackWithPassword {
    &self.inner
  }
}

/// A button that copies specified text to clipboard
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeCopyText {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The text to copy to clipboard
  text: String,
}

impl RObject for InlineKeyboardButtonTypeCopyText {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inlineKeyboardButtonTypeCopyText"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeCopyText {}

impl InlineKeyboardButtonTypeCopyText {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInlineKeyboardButtonTypeCopyTextBuilder {
    let inner = InlineKeyboardButtonTypeCopyText {
      td_name: "inlineKeyboardButtonTypeCopyText".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInlineKeyboardButtonTypeCopyTextBuilder { inner }
  }

  pub fn text(&self) -> &String {
    &self.text
  }
}

#[doc(hidden)]
pub struct RTDInlineKeyboardButtonTypeCopyTextBuilder {
  inner: InlineKeyboardButtonTypeCopyText,
}

impl RTDInlineKeyboardButtonTypeCopyTextBuilder {
  pub fn build(&self) -> InlineKeyboardButtonTypeCopyText {
    self.inner.clone()
  }

  pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().to_string();
    self
  }
}

impl AsRef<InlineKeyboardButtonTypeCopyText> for InlineKeyboardButtonTypeCopyText {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeCopyText {
    self
  }
}

impl AsRef<InlineKeyboardButtonTypeCopyText> for RTDInlineKeyboardButtonTypeCopyTextBuilder {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeCopyText {
    &self.inner
  }
}

/// A disabled button
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeDisabled {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for InlineKeyboardButtonTypeDisabled {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inlineKeyboardButtonTypeDisabled"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeDisabled {}

impl InlineKeyboardButtonTypeDisabled {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInlineKeyboardButtonTypeDisabledBuilder {
    let inner = InlineKeyboardButtonTypeDisabled {
      td_name: "inlineKeyboardButtonTypeDisabled".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDInlineKeyboardButtonTypeDisabledBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDInlineKeyboardButtonTypeDisabledBuilder {
  inner: InlineKeyboardButtonTypeDisabled,
}

impl RTDInlineKeyboardButtonTypeDisabledBuilder {
  pub fn build(&self) -> InlineKeyboardButtonTypeDisabled {
    self.inner.clone()
  }
}

impl AsRef<InlineKeyboardButtonTypeDisabled> for InlineKeyboardButtonTypeDisabled {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeDisabled {
    self
  }
}

impl AsRef<InlineKeyboardButtonTypeDisabled> for RTDInlineKeyboardButtonTypeDisabledBuilder {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeDisabled {
    &self.inner
  }
}

/// A button that opens a specified URL and automatically authorize the current user by calling getLoginUrlInfo; not supported in ephemeral messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeLoginUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// An HTTP URL to pass to getLoginUrlInfo
  url: String,
  /// Unique button identifier
  id: i64,
  /// If non-empty, new text of the button in forwarded messages
  forward_text: String,
}

impl RObject for InlineKeyboardButtonTypeLoginUrl {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inlineKeyboardButtonTypeLoginUrl"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeLoginUrl {}

impl InlineKeyboardButtonTypeLoginUrl {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInlineKeyboardButtonTypeLoginUrlBuilder {
    let inner = InlineKeyboardButtonTypeLoginUrl {
      td_name: "inlineKeyboardButtonTypeLoginUrl".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInlineKeyboardButtonTypeLoginUrlBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn forward_text(&self) -> &String {
    &self.forward_text
  }
}

#[doc(hidden)]
pub struct RTDInlineKeyboardButtonTypeLoginUrlBuilder {
  inner: InlineKeyboardButtonTypeLoginUrl,
}

impl RTDInlineKeyboardButtonTypeLoginUrlBuilder {
  pub fn build(&self) -> InlineKeyboardButtonTypeLoginUrl {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn forward_text<T: AsRef<str>>(&mut self, forward_text: T) -> &mut Self {
    self.inner.forward_text = forward_text.as_ref().to_string();
    self
  }
}

impl AsRef<InlineKeyboardButtonTypeLoginUrl> for InlineKeyboardButtonTypeLoginUrl {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeLoginUrl {
    self
  }
}

impl AsRef<InlineKeyboardButtonTypeLoginUrl> for RTDInlineKeyboardButtonTypeLoginUrlBuilder {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeLoginUrl {
    &self.inner
  }
}

/// A button that forces an inline query to the bot to be inserted in the input field
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeSwitchInline {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Inline query to be sent to the bot
  query: String,
  /// Target chat from which to send the inline query
  target_chat: TargetChat,
}

impl RObject for InlineKeyboardButtonTypeSwitchInline {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inlineKeyboardButtonTypeSwitchInline"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeSwitchInline {}

impl InlineKeyboardButtonTypeSwitchInline {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInlineKeyboardButtonTypeSwitchInlineBuilder {
    let inner = InlineKeyboardButtonTypeSwitchInline {
      td_name: "inlineKeyboardButtonTypeSwitchInline".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInlineKeyboardButtonTypeSwitchInlineBuilder { inner }
  }

  pub fn query(&self) -> &String {
    &self.query
  }

  pub fn target_chat(&self) -> &TargetChat {
    &self.target_chat
  }
}

#[doc(hidden)]
pub struct RTDInlineKeyboardButtonTypeSwitchInlineBuilder {
  inner: InlineKeyboardButtonTypeSwitchInline,
}

impl RTDInlineKeyboardButtonTypeSwitchInlineBuilder {
  pub fn build(&self) -> InlineKeyboardButtonTypeSwitchInline {
    self.inner.clone()
  }

  pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
    self.inner.query = query.as_ref().to_string();
    self
  }

  pub fn target_chat<T: AsRef<TargetChat>>(&mut self, target_chat: T) -> &mut Self {
    self.inner.target_chat = target_chat.as_ref().clone();
    self
  }
}

impl AsRef<InlineKeyboardButtonTypeSwitchInline> for InlineKeyboardButtonTypeSwitchInline {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeSwitchInline {
    self
  }
}

impl AsRef<InlineKeyboardButtonTypeSwitchInline>
  for RTDInlineKeyboardButtonTypeSwitchInlineBuilder
{
  fn as_ref(&self) -> &InlineKeyboardButtonTypeSwitchInline {
    &self.inner
  }
}

/// A button that opens a specified URL
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// HTTP or tg:// URL to open. If the link is of the type internalLinkTypeWebApp, then the button must be marked as a Web App button
  url: String,
}

impl RObject for InlineKeyboardButtonTypeUrl {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inlineKeyboardButtonTypeUrl"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeUrl {}

impl InlineKeyboardButtonTypeUrl {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInlineKeyboardButtonTypeUrlBuilder {
    let inner = InlineKeyboardButtonTypeUrl {
      td_name: "inlineKeyboardButtonTypeUrl".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInlineKeyboardButtonTypeUrlBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }
}

#[doc(hidden)]
pub struct RTDInlineKeyboardButtonTypeUrlBuilder {
  inner: InlineKeyboardButtonTypeUrl,
}

impl RTDInlineKeyboardButtonTypeUrlBuilder {
  pub fn build(&self) -> InlineKeyboardButtonTypeUrl {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }
}

impl AsRef<InlineKeyboardButtonTypeUrl> for InlineKeyboardButtonTypeUrl {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeUrl {
    self
  }
}

impl AsRef<InlineKeyboardButtonTypeUrl> for RTDInlineKeyboardButtonTypeUrlBuilder {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeUrl {
    &self.inner
  }
}

/// A button with a user reference to be handled in the same way as textEntityTypeMentionName entities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier
  user_id: i64,
}

impl RObject for InlineKeyboardButtonTypeUser {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inlineKeyboardButtonTypeUser"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeUser {}

impl InlineKeyboardButtonTypeUser {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInlineKeyboardButtonTypeUserBuilder {
    let inner = InlineKeyboardButtonTypeUser {
      td_name: "inlineKeyboardButtonTypeUser".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInlineKeyboardButtonTypeUserBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }
}

#[doc(hidden)]
pub struct RTDInlineKeyboardButtonTypeUserBuilder {
  inner: InlineKeyboardButtonTypeUser,
}

impl RTDInlineKeyboardButtonTypeUserBuilder {
  pub fn build(&self) -> InlineKeyboardButtonTypeUser {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }
}

impl AsRef<InlineKeyboardButtonTypeUser> for InlineKeyboardButtonTypeUser {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeUser {
    self
  }
}

impl AsRef<InlineKeyboardButtonTypeUser> for RTDInlineKeyboardButtonTypeUserBuilder {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeUser {
    &self.inner
  }
}

/// A button that opens a Web App by calling openWebApp
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeWebApp {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// An HTTP URL to pass to openWebApp
  url: String,
}

impl RObject for InlineKeyboardButtonTypeWebApp {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inlineKeyboardButtonTypeWebApp"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeWebApp {}

impl InlineKeyboardButtonTypeWebApp {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInlineKeyboardButtonTypeWebAppBuilder {
    let inner = InlineKeyboardButtonTypeWebApp {
      td_name: "inlineKeyboardButtonTypeWebApp".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInlineKeyboardButtonTypeWebAppBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }
}

#[doc(hidden)]
pub struct RTDInlineKeyboardButtonTypeWebAppBuilder {
  inner: InlineKeyboardButtonTypeWebApp,
}

impl RTDInlineKeyboardButtonTypeWebAppBuilder {
  pub fn build(&self) -> InlineKeyboardButtonTypeWebApp {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }
}

impl AsRef<InlineKeyboardButtonTypeWebApp> for InlineKeyboardButtonTypeWebApp {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeWebApp {
    self
  }
}

impl AsRef<InlineKeyboardButtonTypeWebApp> for RTDInlineKeyboardButtonTypeWebAppBuilder {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeWebApp {
    &self.inner
  }
}
