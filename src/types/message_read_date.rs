use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes read date of a recent outgoing message in a private chat
pub trait TDMessageReadDate: Debug + RObject {}

/// Describes read date of a recent outgoing message in a private chat
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum MessageReadDate {
  #[doc(hidden)]
  _Default(()),
  /// Returns read date of a recent outgoing message in a private chat. The method can be called if messageProperties.can_get_read_date == true
  GetMessageReadDate(GetMessageReadDate),
  /// The read date is unknown due to privacy settings of the current user, but will be known if the user subscribes to Telegram Premium
  MyPrivacyRestricted(MessageReadDateMyPrivacyRestricted),
  /// Contains read date of the message
  Read(MessageReadDateRead),
  /// The message is too old to get read date
  TooOld(MessageReadDateTooOld),
  /// The message is unread yet
  Unread(MessageReadDateUnread),
  /// The read date is unknown due to privacy settings of the other user
  UserPrivacyRestricted(MessageReadDateUserPrivacyRestricted),
}

impl Default for MessageReadDate {
  fn default() -> Self {
    MessageReadDate::_Default(())
  }
}

impl<'de> Deserialize<'de> for MessageReadDate {
  fn deserialize<D>(deserializer: D) -> Result<MessageReadDate, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      MessageReadDate,
      (getMessageReadDate, GetMessageReadDate);
      (messageReadDateMyPrivacyRestricted, MyPrivacyRestricted);
      (messageReadDateRead, Read);
      (messageReadDateTooOld, TooOld);
      (messageReadDateUnread, Unread);
      (messageReadDateUserPrivacyRestricted, UserPrivacyRestricted);

    )(deserializer)
  }
}

impl RObject for MessageReadDate {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      MessageReadDate::GetMessageReadDate(t) => t.td_name(),
      MessageReadDate::MyPrivacyRestricted(t) => t.td_name(),
      MessageReadDate::Read(t) => t.td_name(),
      MessageReadDate::TooOld(t) => t.td_name(),
      MessageReadDate::Unread(t) => t.td_name(),
      MessageReadDate::UserPrivacyRestricted(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      MessageReadDate::GetMessageReadDate(t) => t.extra(),
      MessageReadDate::MyPrivacyRestricted(t) => t.extra(),
      MessageReadDate::Read(t) => t.extra(),
      MessageReadDate::TooOld(t) => t.extra(),
      MessageReadDate::Unread(t) => t.extra(),
      MessageReadDate::UserPrivacyRestricted(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl MessageReadDate {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, MessageReadDate::_Default(_))
  }

  pub fn is_get_message_read_date(&self) -> bool {
    matches!(self, MessageReadDate::GetMessageReadDate(_))
  }
  pub fn is_my_privacy_restricted(&self) -> bool {
    matches!(self, MessageReadDate::MyPrivacyRestricted(_))
  }
  pub fn is_read(&self) -> bool {
    matches!(self, MessageReadDate::Read(_))
  }
  pub fn is_too_old(&self) -> bool {
    matches!(self, MessageReadDate::TooOld(_))
  }
  pub fn is_unread(&self) -> bool {
    matches!(self, MessageReadDate::Unread(_))
  }
  pub fn is_user_privacy_restricted(&self) -> bool {
    matches!(self, MessageReadDate::UserPrivacyRestricted(_))
  }

  pub fn on_get_message_read_date<F: FnOnce(&GetMessageReadDate)>(&self, fnc: F) -> &Self {
    if let MessageReadDate::GetMessageReadDate(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_my_privacy_restricted<F: FnOnce(&MessageReadDateMyPrivacyRestricted)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let MessageReadDate::MyPrivacyRestricted(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_read<F: FnOnce(&MessageReadDateRead)>(&self, fnc: F) -> &Self {
    if let MessageReadDate::Read(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_too_old<F: FnOnce(&MessageReadDateTooOld)>(&self, fnc: F) -> &Self {
    if let MessageReadDate::TooOld(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_unread<F: FnOnce(&MessageReadDateUnread)>(&self, fnc: F) -> &Self {
    if let MessageReadDate::Unread(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_user_privacy_restricted<F: FnOnce(&MessageReadDateUserPrivacyRestricted)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let MessageReadDate::UserPrivacyRestricted(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_get_message_read_date(&self) -> Option<&GetMessageReadDate> {
    if let MessageReadDate::GetMessageReadDate(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_my_privacy_restricted(&self) -> Option<&MessageReadDateMyPrivacyRestricted> {
    if let MessageReadDate::MyPrivacyRestricted(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_read(&self) -> Option<&MessageReadDateRead> {
    if let MessageReadDate::Read(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_too_old(&self) -> Option<&MessageReadDateTooOld> {
    if let MessageReadDate::TooOld(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_unread(&self) -> Option<&MessageReadDateUnread> {
    if let MessageReadDate::Unread(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_user_privacy_restricted(&self) -> Option<&MessageReadDateUserPrivacyRestricted> {
    if let MessageReadDate::UserPrivacyRestricted(t) = self {
      return Some(t);
    }
    None
  }

  pub fn get_message_read_date<T: AsRef<GetMessageReadDate>>(t: T) -> Self {
    MessageReadDate::GetMessageReadDate(t.as_ref().clone())
  }

  pub fn my_privacy_restricted<T: AsRef<MessageReadDateMyPrivacyRestricted>>(t: T) -> Self {
    MessageReadDate::MyPrivacyRestricted(t.as_ref().clone())
  }

  pub fn read<T: AsRef<MessageReadDateRead>>(t: T) -> Self {
    MessageReadDate::Read(t.as_ref().clone())
  }

  pub fn too_old<T: AsRef<MessageReadDateTooOld>>(t: T) -> Self {
    MessageReadDate::TooOld(t.as_ref().clone())
  }

  pub fn unread<T: AsRef<MessageReadDateUnread>>(t: T) -> Self {
    MessageReadDate::Unread(t.as_ref().clone())
  }

  pub fn user_privacy_restricted<T: AsRef<MessageReadDateUserPrivacyRestricted>>(t: T) -> Self {
    MessageReadDate::UserPrivacyRestricted(t.as_ref().clone())
  }
}

impl AsRef<MessageReadDate> for MessageReadDate {
  fn as_ref(&self) -> &MessageReadDate {
    self
  }
}

/// The read date is unknown due to privacy settings of the current user, but will be known if the user subscribes to Telegram Premium
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageReadDateMyPrivacyRestricted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for MessageReadDateMyPrivacyRestricted {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageReadDateMyPrivacyRestricted"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageReadDate for MessageReadDateMyPrivacyRestricted {}

impl MessageReadDateMyPrivacyRestricted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageReadDateMyPrivacyRestrictedBuilder {
    let inner = MessageReadDateMyPrivacyRestricted {
      td_name: "messageReadDateMyPrivacyRestricted".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDMessageReadDateMyPrivacyRestrictedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDMessageReadDateMyPrivacyRestrictedBuilder {
  inner: MessageReadDateMyPrivacyRestricted,
}

impl RTDMessageReadDateMyPrivacyRestrictedBuilder {
  pub fn build(&self) -> MessageReadDateMyPrivacyRestricted {
    self.inner.clone()
  }
}

impl AsRef<MessageReadDateMyPrivacyRestricted> for MessageReadDateMyPrivacyRestricted {
  fn as_ref(&self) -> &MessageReadDateMyPrivacyRestricted {
    self
  }
}

impl AsRef<MessageReadDateMyPrivacyRestricted> for RTDMessageReadDateMyPrivacyRestrictedBuilder {
  fn as_ref(&self) -> &MessageReadDateMyPrivacyRestricted {
    &self.inner
  }
}

/// Contains read date of the message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageReadDateRead {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Point in time (Unix timestamp) when the message was read by the other user
  read_date: i64,
}

impl RObject for MessageReadDateRead {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageReadDateRead"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageReadDate for MessageReadDateRead {}

impl MessageReadDateRead {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageReadDateReadBuilder {
    let inner = MessageReadDateRead {
      td_name: "messageReadDateRead".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageReadDateReadBuilder { inner }
  }

  pub fn read_date(&self) -> i64 {
    self.read_date
  }
}

#[doc(hidden)]
pub struct RTDMessageReadDateReadBuilder {
  inner: MessageReadDateRead,
}

impl RTDMessageReadDateReadBuilder {
  pub fn build(&self) -> MessageReadDateRead {
    self.inner.clone()
  }

  pub fn read_date(&mut self, read_date: i64) -> &mut Self {
    self.inner.read_date = read_date;
    self
  }
}

impl AsRef<MessageReadDateRead> for MessageReadDateRead {
  fn as_ref(&self) -> &MessageReadDateRead {
    self
  }
}

impl AsRef<MessageReadDateRead> for RTDMessageReadDateReadBuilder {
  fn as_ref(&self) -> &MessageReadDateRead {
    &self.inner
  }
}

/// The message is too old to get read date
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageReadDateTooOld {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for MessageReadDateTooOld {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageReadDateTooOld"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageReadDate for MessageReadDateTooOld {}

impl MessageReadDateTooOld {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageReadDateTooOldBuilder {
    let inner = MessageReadDateTooOld {
      td_name: "messageReadDateTooOld".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDMessageReadDateTooOldBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDMessageReadDateTooOldBuilder {
  inner: MessageReadDateTooOld,
}

impl RTDMessageReadDateTooOldBuilder {
  pub fn build(&self) -> MessageReadDateTooOld {
    self.inner.clone()
  }
}

impl AsRef<MessageReadDateTooOld> for MessageReadDateTooOld {
  fn as_ref(&self) -> &MessageReadDateTooOld {
    self
  }
}

impl AsRef<MessageReadDateTooOld> for RTDMessageReadDateTooOldBuilder {
  fn as_ref(&self) -> &MessageReadDateTooOld {
    &self.inner
  }
}

/// The message is unread yet
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageReadDateUnread {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for MessageReadDateUnread {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageReadDateUnread"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageReadDate for MessageReadDateUnread {}

impl MessageReadDateUnread {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageReadDateUnreadBuilder {
    let inner = MessageReadDateUnread {
      td_name: "messageReadDateUnread".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDMessageReadDateUnreadBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDMessageReadDateUnreadBuilder {
  inner: MessageReadDateUnread,
}

impl RTDMessageReadDateUnreadBuilder {
  pub fn build(&self) -> MessageReadDateUnread {
    self.inner.clone()
  }
}

impl AsRef<MessageReadDateUnread> for MessageReadDateUnread {
  fn as_ref(&self) -> &MessageReadDateUnread {
    self
  }
}

impl AsRef<MessageReadDateUnread> for RTDMessageReadDateUnreadBuilder {
  fn as_ref(&self) -> &MessageReadDateUnread {
    &self.inner
  }
}

/// The read date is unknown due to privacy settings of the other user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageReadDateUserPrivacyRestricted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for MessageReadDateUserPrivacyRestricted {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageReadDateUserPrivacyRestricted"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageReadDate for MessageReadDateUserPrivacyRestricted {}

impl MessageReadDateUserPrivacyRestricted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageReadDateUserPrivacyRestrictedBuilder {
    let inner = MessageReadDateUserPrivacyRestricted {
      td_name: "messageReadDateUserPrivacyRestricted".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDMessageReadDateUserPrivacyRestrictedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDMessageReadDateUserPrivacyRestrictedBuilder {
  inner: MessageReadDateUserPrivacyRestricted,
}

impl RTDMessageReadDateUserPrivacyRestrictedBuilder {
  pub fn build(&self) -> MessageReadDateUserPrivacyRestricted {
    self.inner.clone()
  }
}

impl AsRef<MessageReadDateUserPrivacyRestricted> for MessageReadDateUserPrivacyRestricted {
  fn as_ref(&self) -> &MessageReadDateUserPrivacyRestricted {
    self
  }
}

impl AsRef<MessageReadDateUserPrivacyRestricted>
  for RTDMessageReadDateUserPrivacyRestrictedBuilder
{
  fn as_ref(&self) -> &MessageReadDateUserPrivacyRestricted {
    &self.inner
  }
}
