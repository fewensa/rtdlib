use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a message passed by the page of a web proxy carrier to postMessage of the bridge object
pub trait TDWebProxyCarrierMessage: Debug + RObject {}

/// Describes a message passed by the page of a web proxy carrier to postMessage of the bridge object
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum WebProxyCarrierMessage {
  #[doc(hidden)]
  _Default(()),
  /// A binary message, passed as an ArrayBuffer or a typed array
  Binary(WebProxyCarrierMessageBinary),
  /// A string message
  Text(WebProxyCarrierMessageText),
}

impl Default for WebProxyCarrierMessage {
  fn default() -> Self {
    WebProxyCarrierMessage::_Default(())
  }
}

impl<'de> Deserialize<'de> for WebProxyCarrierMessage {
  fn deserialize<D>(deserializer: D) -> Result<WebProxyCarrierMessage, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      WebProxyCarrierMessage,
      (webProxyCarrierMessageBinary, Binary);
      (webProxyCarrierMessageText, Text);

    )(deserializer)
  }
}

impl RObject for WebProxyCarrierMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      WebProxyCarrierMessage::Binary(t) => t.td_name(),
      WebProxyCarrierMessage::Text(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      WebProxyCarrierMessage::Binary(t) => t.extra(),
      WebProxyCarrierMessage::Text(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl WebProxyCarrierMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, WebProxyCarrierMessage::_Default(_))
  }

  pub fn is_binary(&self) -> bool {
    matches!(self, WebProxyCarrierMessage::Binary(_))
  }
  pub fn is_text(&self) -> bool {
    matches!(self, WebProxyCarrierMessage::Text(_))
  }

  pub fn on_binary<F: FnOnce(&WebProxyCarrierMessageBinary)>(&self, fnc: F) -> &Self {
    if let WebProxyCarrierMessage::Binary(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_text<F: FnOnce(&WebProxyCarrierMessageText)>(&self, fnc: F) -> &Self {
    if let WebProxyCarrierMessage::Text(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_binary(&self) -> Option<&WebProxyCarrierMessageBinary> {
    if let WebProxyCarrierMessage::Binary(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_text(&self) -> Option<&WebProxyCarrierMessageText> {
    if let WebProxyCarrierMessage::Text(t) = self {
      return Some(t);
    }
    None
  }

  pub fn binary<T: AsRef<WebProxyCarrierMessageBinary>>(t: T) -> Self {
    WebProxyCarrierMessage::Binary(t.as_ref().clone())
  }

  pub fn text<T: AsRef<WebProxyCarrierMessageText>>(t: T) -> Self {
    WebProxyCarrierMessage::Text(t.as_ref().clone())
  }
}

impl AsRef<WebProxyCarrierMessage> for WebProxyCarrierMessage {
  fn as_ref(&self) -> &WebProxyCarrierMessage {
    self
  }
}

/// A binary message, passed as an ArrayBuffer or a typed array
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebProxyCarrierMessageBinary {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Data of the message
  data: String,
}

impl RObject for WebProxyCarrierMessageBinary {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "webProxyCarrierMessageBinary"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDWebProxyCarrierMessage for WebProxyCarrierMessageBinary {}

impl WebProxyCarrierMessageBinary {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDWebProxyCarrierMessageBinaryBuilder {
    let inner = WebProxyCarrierMessageBinary {
      td_name: "webProxyCarrierMessageBinary".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDWebProxyCarrierMessageBinaryBuilder { inner }
  }

  pub fn data(&self) -> &String {
    &self.data
  }
}

#[doc(hidden)]
pub struct RTDWebProxyCarrierMessageBinaryBuilder {
  inner: WebProxyCarrierMessageBinary,
}

impl RTDWebProxyCarrierMessageBinaryBuilder {
  pub fn build(&self) -> WebProxyCarrierMessageBinary {
    self.inner.clone()
  }

  pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
    self.inner.data = data.as_ref().to_string();
    self
  }
}

impl AsRef<WebProxyCarrierMessageBinary> for WebProxyCarrierMessageBinary {
  fn as_ref(&self) -> &WebProxyCarrierMessageBinary {
    self
  }
}

impl AsRef<WebProxyCarrierMessageBinary> for RTDWebProxyCarrierMessageBinaryBuilder {
  fn as_ref(&self) -> &WebProxyCarrierMessageBinary {
    &self.inner
  }
}

/// A string message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebProxyCarrierMessageText {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The string
  text: String,
}

impl RObject for WebProxyCarrierMessageText {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "webProxyCarrierMessageText"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDWebProxyCarrierMessage for WebProxyCarrierMessageText {}

impl WebProxyCarrierMessageText {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDWebProxyCarrierMessageTextBuilder {
    let inner = WebProxyCarrierMessageText {
      td_name: "webProxyCarrierMessageText".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDWebProxyCarrierMessageTextBuilder { inner }
  }

  pub fn text(&self) -> &String {
    &self.text
  }
}

#[doc(hidden)]
pub struct RTDWebProxyCarrierMessageTextBuilder {
  inner: WebProxyCarrierMessageText,
}

impl RTDWebProxyCarrierMessageTextBuilder {
  pub fn build(&self) -> WebProxyCarrierMessageText {
    self.inner.clone()
  }

  pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().to_string();
    self
  }
}

impl AsRef<WebProxyCarrierMessageText> for WebProxyCarrierMessageText {
  fn as_ref(&self) -> &WebProxyCarrierMessageText {
    self
  }
}

impl AsRef<WebProxyCarrierMessageText> for RTDWebProxyCarrierMessageTextBuilder {
  fn as_ref(&self) -> &WebProxyCarrierMessageText {
    &self.inner
  }
}
