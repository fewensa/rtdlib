use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes an invoice to process
pub trait TDInputInvoice: Debug + RObject {}

/// Describes an invoice to process
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputInvoice {
  #[doc(hidden)]
  _Default(()),
  /// An invoice from a message of the type messageInvoice or paid media purchase from messagePaidMedia
  Message(InputInvoiceMessage),
  /// An invoice from a link of the type internalLinkTypeInvoice
  Name(InputInvoiceName),
  /// An invoice for a payment toward Telegram; must not be used in the in-store apps
  Telegram(InputInvoiceTelegram),
}

impl Default for InputInvoice {
  fn default() -> Self {
    InputInvoice::_Default(())
  }
}

impl<'de> Deserialize<'de> for InputInvoice {
  fn deserialize<D>(deserializer: D) -> Result<InputInvoice, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InputInvoice,
      (inputInvoiceMessage, Message);
      (inputInvoiceName, Name);
      (inputInvoiceTelegram, Telegram);

    )(deserializer)
  }
}

impl RObject for InputInvoice {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      InputInvoice::Message(t) => t.td_name(),
      InputInvoice::Name(t) => t.td_name(),
      InputInvoice::Telegram(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      InputInvoice::Message(t) => t.extra(),
      InputInvoice::Name(t) => t.extra(),
      InputInvoice::Telegram(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputInvoice {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, InputInvoice::_Default(_))
  }

  pub fn is_message(&self) -> bool {
    matches!(self, InputInvoice::Message(_))
  }
  pub fn is_name(&self) -> bool {
    matches!(self, InputInvoice::Name(_))
  }
  pub fn is_telegram(&self) -> bool {
    matches!(self, InputInvoice::Telegram(_))
  }

  pub fn on_message<F: FnOnce(&InputInvoiceMessage)>(&self, fnc: F) -> &Self {
    if let InputInvoice::Message(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_name<F: FnOnce(&InputInvoiceName)>(&self, fnc: F) -> &Self {
    if let InputInvoice::Name(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_telegram<F: FnOnce(&InputInvoiceTelegram)>(&self, fnc: F) -> &Self {
    if let InputInvoice::Telegram(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_message(&self) -> Option<&InputInvoiceMessage> {
    if let InputInvoice::Message(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_name(&self) -> Option<&InputInvoiceName> {
    if let InputInvoice::Name(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_telegram(&self) -> Option<&InputInvoiceTelegram> {
    if let InputInvoice::Telegram(t) = self {
      return Some(t);
    }
    None
  }

  pub fn message<T: AsRef<InputInvoiceMessage>>(t: T) -> Self {
    InputInvoice::Message(t.as_ref().clone())
  }

  pub fn name<T: AsRef<InputInvoiceName>>(t: T) -> Self {
    InputInvoice::Name(t.as_ref().clone())
  }

  pub fn telegram<T: AsRef<InputInvoiceTelegram>>(t: T) -> Self {
    InputInvoice::Telegram(t.as_ref().clone())
  }
}

impl AsRef<InputInvoice> for InputInvoice {
  fn as_ref(&self) -> &InputInvoice {
    self
  }
}

/// An invoice from a message of the type messageInvoice or paid media purchase from messagePaidMedia
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputInvoiceMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier of the message
  chat_id: i64,
  /// Message identifier. Use messageProperties.can_be_paid to check whether the message can be used in the method
  message_id: i64,
}

impl RObject for InputInvoiceMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputInvoiceMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputInvoice for InputInvoiceMessage {}

impl InputInvoiceMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputInvoiceMessageBuilder {
    let inner = InputInvoiceMessage {
      td_name: "inputInvoiceMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputInvoiceMessageBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }
}

#[doc(hidden)]
pub struct RTDInputInvoiceMessageBuilder {
  inner: InputInvoiceMessage,
}

impl RTDInputInvoiceMessageBuilder {
  pub fn build(&self) -> InputInvoiceMessage {
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

impl AsRef<InputInvoiceMessage> for InputInvoiceMessage {
  fn as_ref(&self) -> &InputInvoiceMessage {
    self
  }
}

impl AsRef<InputInvoiceMessage> for RTDInputInvoiceMessageBuilder {
  fn as_ref(&self) -> &InputInvoiceMessage {
    &self.inner
  }
}

/// An invoice from a link of the type internalLinkTypeInvoice
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputInvoiceName {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the invoice
  name: String,
}

impl RObject for InputInvoiceName {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputInvoiceName"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputInvoice for InputInvoiceName {}

impl InputInvoiceName {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputInvoiceNameBuilder {
    let inner = InputInvoiceName {
      td_name: "inputInvoiceName".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputInvoiceNameBuilder { inner }
  }

  pub fn name(&self) -> &String {
    &self.name
  }
}

#[doc(hidden)]
pub struct RTDInputInvoiceNameBuilder {
  inner: InputInvoiceName,
}

impl RTDInputInvoiceNameBuilder {
  pub fn build(&self) -> InputInvoiceName {
    self.inner.clone()
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }
}

impl AsRef<InputInvoiceName> for InputInvoiceName {
  fn as_ref(&self) -> &InputInvoiceName {
    self
  }
}

impl AsRef<InputInvoiceName> for RTDInputInvoiceNameBuilder {
  fn as_ref(&self) -> &InputInvoiceName {
    &self.inner
  }
}

/// An invoice for a payment toward Telegram; must not be used in the in-store apps
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputInvoiceTelegram {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Transaction purpose
  purpose: TelegramPaymentPurpose,
}

impl RObject for InputInvoiceTelegram {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputInvoiceTelegram"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputInvoice for InputInvoiceTelegram {}

impl InputInvoiceTelegram {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputInvoiceTelegramBuilder {
    let inner = InputInvoiceTelegram {
      td_name: "inputInvoiceTelegram".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputInvoiceTelegramBuilder { inner }
  }

  pub fn purpose(&self) -> &TelegramPaymentPurpose {
    &self.purpose
  }
}

#[doc(hidden)]
pub struct RTDInputInvoiceTelegramBuilder {
  inner: InputInvoiceTelegram,
}

impl RTDInputInvoiceTelegramBuilder {
  pub fn build(&self) -> InputInvoiceTelegram {
    self.inner.clone()
  }

  pub fn purpose<T: AsRef<TelegramPaymentPurpose>>(&mut self, purpose: T) -> &mut Self {
    self.inner.purpose = purpose.as_ref().clone();
    self
  }
}

impl AsRef<InputInvoiceTelegram> for InputInvoiceTelegram {
  fn as_ref(&self) -> &InputInvoiceTelegram {
    self
  }
}

impl AsRef<InputInvoiceTelegram> for RTDInputInvoiceTelegramBuilder {
  fn as_ref(&self) -> &InputInvoiceTelegram {
    &self.inner
  }
}
