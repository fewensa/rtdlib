use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes source of a keyboard button
pub trait TDKeyboardButtonSource: Debug + RObject {}

/// Describes source of a keyboard button
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum KeyboardButtonSource {
  #[doc(hidden)]
  _Default(()),
  /// The button is from a bot's message
  Message(KeyboardButtonSourceMessage),
  /// The button is a prepared keyboard button from a Mini App received via getPreparedKeyboardButton
  WebApp(KeyboardButtonSourceWebApp),
}

impl Default for KeyboardButtonSource {
  fn default() -> Self {
    KeyboardButtonSource::_Default(())
  }
}

impl<'de> Deserialize<'de> for KeyboardButtonSource {
  fn deserialize<D>(deserializer: D) -> Result<KeyboardButtonSource, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      KeyboardButtonSource,
      (keyboardButtonSourceMessage, Message);
      (keyboardButtonSourceWebApp, WebApp);

    )(deserializer)
  }
}

impl RObject for KeyboardButtonSource {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      KeyboardButtonSource::Message(t) => t.td_name(),
      KeyboardButtonSource::WebApp(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      KeyboardButtonSource::Message(t) => t.extra(),
      KeyboardButtonSource::WebApp(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl KeyboardButtonSource {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, KeyboardButtonSource::_Default(_))
  }

  pub fn is_message(&self) -> bool {
    matches!(self, KeyboardButtonSource::Message(_))
  }
  pub fn is_web_app(&self) -> bool {
    matches!(self, KeyboardButtonSource::WebApp(_))
  }

  pub fn on_message<F: FnOnce(&KeyboardButtonSourceMessage)>(&self, fnc: F) -> &Self {
    if let KeyboardButtonSource::Message(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_web_app<F: FnOnce(&KeyboardButtonSourceWebApp)>(&self, fnc: F) -> &Self {
    if let KeyboardButtonSource::WebApp(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_message(&self) -> Option<&KeyboardButtonSourceMessage> {
    if let KeyboardButtonSource::Message(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_web_app(&self) -> Option<&KeyboardButtonSourceWebApp> {
    if let KeyboardButtonSource::WebApp(t) = self {
      return Some(t);
    }
    None
  }

  pub fn message<T: AsRef<KeyboardButtonSourceMessage>>(t: T) -> Self {
    KeyboardButtonSource::Message(t.as_ref().clone())
  }

  pub fn web_app<T: AsRef<KeyboardButtonSourceWebApp>>(t: T) -> Self {
    KeyboardButtonSource::WebApp(t.as_ref().clone())
  }
}

impl AsRef<KeyboardButtonSource> for KeyboardButtonSource {
  fn as_ref(&self) -> &KeyboardButtonSource {
    self
  }
}

/// The button is from a bot's message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButtonSourceMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat with the message
  chat_id: i64,
  /// Identifier of the message with the button
  message_id: i64,
}

impl RObject for KeyboardButtonSourceMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "keyboardButtonSourceMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDKeyboardButtonSource for KeyboardButtonSourceMessage {}

impl KeyboardButtonSourceMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDKeyboardButtonSourceMessageBuilder {
    let inner = KeyboardButtonSourceMessage {
      td_name: "keyboardButtonSourceMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDKeyboardButtonSourceMessageBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }
}

#[doc(hidden)]
pub struct RTDKeyboardButtonSourceMessageBuilder {
  inner: KeyboardButtonSourceMessage,
}

impl RTDKeyboardButtonSourceMessageBuilder {
  pub fn build(&self) -> KeyboardButtonSourceMessage {
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

impl AsRef<KeyboardButtonSourceMessage> for KeyboardButtonSourceMessage {
  fn as_ref(&self) -> &KeyboardButtonSourceMessage {
    self
  }
}

impl AsRef<KeyboardButtonSourceMessage> for RTDKeyboardButtonSourceMessageBuilder {
  fn as_ref(&self) -> &KeyboardButtonSourceMessage {
    &self.inner
  }
}

/// The button is a prepared keyboard button from a Mini App received via getPreparedKeyboardButton
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButtonSourceWebApp {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the bot that created the button
  bot_user_id: i64,
  /// Identifier of the prepared button
  prepared_button_id: String,
}

impl RObject for KeyboardButtonSourceWebApp {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "keyboardButtonSourceWebApp"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDKeyboardButtonSource for KeyboardButtonSourceWebApp {}

impl KeyboardButtonSourceWebApp {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDKeyboardButtonSourceWebAppBuilder {
    let inner = KeyboardButtonSourceWebApp {
      td_name: "keyboardButtonSourceWebApp".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDKeyboardButtonSourceWebAppBuilder { inner }
  }

  pub fn bot_user_id(&self) -> i64 {
    self.bot_user_id
  }

  pub fn prepared_button_id(&self) -> &String {
    &self.prepared_button_id
  }
}

#[doc(hidden)]
pub struct RTDKeyboardButtonSourceWebAppBuilder {
  inner: KeyboardButtonSourceWebApp,
}

impl RTDKeyboardButtonSourceWebAppBuilder {
  pub fn build(&self) -> KeyboardButtonSourceWebApp {
    self.inner.clone()
  }

  pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
    self.inner.bot_user_id = bot_user_id;
    self
  }

  pub fn prepared_button_id<T: AsRef<str>>(&mut self, prepared_button_id: T) -> &mut Self {
    self.inner.prepared_button_id = prepared_button_id.as_ref().to_string();
    self
  }
}

impl AsRef<KeyboardButtonSourceWebApp> for KeyboardButtonSourceWebApp {
  fn as_ref(&self) -> &KeyboardButtonSourceWebApp {
    self
  }
}

impl AsRef<KeyboardButtonSourceWebApp> for RTDKeyboardButtonSourceWebAppBuilder {
  fn as_ref(&self) -> &KeyboardButtonSourceWebApp {
    &self.inner
  }
}
