use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes scope of autosave settings
pub trait TDAutosaveSettingsScope: Debug + RObject {}

/// Describes scope of autosave settings
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum AutosaveSettingsScope {
  #[doc(hidden)]
  _Default(()),
  /// Autosave settings applied to all channel chats without chat-specific settings
  ChannelChats(AutosaveSettingsScopeChannelChats),
  /// Autosave settings applied to a chat
  Chat(AutosaveSettingsScopeChat),
  /// Autosave settings applied to all basic group and supergroup chats without chat-specific settings
  GroupChats(AutosaveSettingsScopeGroupChats),
  /// Autosave settings applied to all private chats without chat-specific settings
  PrivateChats(AutosaveSettingsScopePrivateChats),
}

impl Default for AutosaveSettingsScope {
  fn default() -> Self {
    AutosaveSettingsScope::_Default(())
  }
}

impl<'de> Deserialize<'de> for AutosaveSettingsScope {
  fn deserialize<D>(deserializer: D) -> Result<AutosaveSettingsScope, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      AutosaveSettingsScope,
      (autosaveSettingsScopeChannelChats, ChannelChats);
      (autosaveSettingsScopeChat, Chat);
      (autosaveSettingsScopeGroupChats, GroupChats);
      (autosaveSettingsScopePrivateChats, PrivateChats);

    )(deserializer)
  }
}

impl RObject for AutosaveSettingsScope {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      AutosaveSettingsScope::ChannelChats(t) => t.td_name(),
      AutosaveSettingsScope::Chat(t) => t.td_name(),
      AutosaveSettingsScope::GroupChats(t) => t.td_name(),
      AutosaveSettingsScope::PrivateChats(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      AutosaveSettingsScope::ChannelChats(t) => t.extra(),
      AutosaveSettingsScope::Chat(t) => t.extra(),
      AutosaveSettingsScope::GroupChats(t) => t.extra(),
      AutosaveSettingsScope::PrivateChats(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AutosaveSettingsScope {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, AutosaveSettingsScope::_Default(_))
  }

  pub fn is_channel_chats(&self) -> bool {
    matches!(self, AutosaveSettingsScope::ChannelChats(_))
  }
  pub fn is_chat(&self) -> bool {
    matches!(self, AutosaveSettingsScope::Chat(_))
  }
  pub fn is_group_chats(&self) -> bool {
    matches!(self, AutosaveSettingsScope::GroupChats(_))
  }
  pub fn is_private_chats(&self) -> bool {
    matches!(self, AutosaveSettingsScope::PrivateChats(_))
  }

  pub fn on_channel_chats<F: FnOnce(&AutosaveSettingsScopeChannelChats)>(&self, fnc: F) -> &Self {
    if let AutosaveSettingsScope::ChannelChats(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat<F: FnOnce(&AutosaveSettingsScopeChat)>(&self, fnc: F) -> &Self {
    if let AutosaveSettingsScope::Chat(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_group_chats<F: FnOnce(&AutosaveSettingsScopeGroupChats)>(&self, fnc: F) -> &Self {
    if let AutosaveSettingsScope::GroupChats(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_private_chats<F: FnOnce(&AutosaveSettingsScopePrivateChats)>(&self, fnc: F) -> &Self {
    if let AutosaveSettingsScope::PrivateChats(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_channel_chats(&self) -> Option<&AutosaveSettingsScopeChannelChats> {
    if let AutosaveSettingsScope::ChannelChats(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat(&self) -> Option<&AutosaveSettingsScopeChat> {
    if let AutosaveSettingsScope::Chat(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_group_chats(&self) -> Option<&AutosaveSettingsScopeGroupChats> {
    if let AutosaveSettingsScope::GroupChats(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_private_chats(&self) -> Option<&AutosaveSettingsScopePrivateChats> {
    if let AutosaveSettingsScope::PrivateChats(t) = self {
      return Some(t);
    }
    None
  }

  pub fn channel_chats<T: AsRef<AutosaveSettingsScopeChannelChats>>(t: T) -> Self {
    AutosaveSettingsScope::ChannelChats(t.as_ref().clone())
  }

  pub fn chat<T: AsRef<AutosaveSettingsScopeChat>>(t: T) -> Self {
    AutosaveSettingsScope::Chat(t.as_ref().clone())
  }

  pub fn group_chats<T: AsRef<AutosaveSettingsScopeGroupChats>>(t: T) -> Self {
    AutosaveSettingsScope::GroupChats(t.as_ref().clone())
  }

  pub fn private_chats<T: AsRef<AutosaveSettingsScopePrivateChats>>(t: T) -> Self {
    AutosaveSettingsScope::PrivateChats(t.as_ref().clone())
  }
}

impl AsRef<AutosaveSettingsScope> for AutosaveSettingsScope {
  fn as_ref(&self) -> &AutosaveSettingsScope {
    self
  }
}

/// Autosave settings applied to all channel chats without chat-specific settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutosaveSettingsScopeChannelChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for AutosaveSettingsScopeChannelChats {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "autosaveSettingsScopeChannelChats"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAutosaveSettingsScope for AutosaveSettingsScopeChannelChats {}

impl AutosaveSettingsScopeChannelChats {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAutosaveSettingsScopeChannelChatsBuilder {
    let inner = AutosaveSettingsScopeChannelChats {
      td_name: "autosaveSettingsScopeChannelChats".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDAutosaveSettingsScopeChannelChatsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDAutosaveSettingsScopeChannelChatsBuilder {
  inner: AutosaveSettingsScopeChannelChats,
}

impl RTDAutosaveSettingsScopeChannelChatsBuilder {
  pub fn build(&self) -> AutosaveSettingsScopeChannelChats {
    self.inner.clone()
  }
}

impl AsRef<AutosaveSettingsScopeChannelChats> for AutosaveSettingsScopeChannelChats {
  fn as_ref(&self) -> &AutosaveSettingsScopeChannelChats {
    self
  }
}

impl AsRef<AutosaveSettingsScopeChannelChats> for RTDAutosaveSettingsScopeChannelChatsBuilder {
  fn as_ref(&self) -> &AutosaveSettingsScopeChannelChats {
    &self.inner
  }
}

/// Autosave settings applied to a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutosaveSettingsScopeChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
}

impl RObject for AutosaveSettingsScopeChat {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "autosaveSettingsScopeChat"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAutosaveSettingsScope for AutosaveSettingsScopeChat {}

impl AutosaveSettingsScopeChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAutosaveSettingsScopeChatBuilder {
    let inner = AutosaveSettingsScopeChat {
      td_name: "autosaveSettingsScopeChat".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAutosaveSettingsScopeChatBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }
}

#[doc(hidden)]
pub struct RTDAutosaveSettingsScopeChatBuilder {
  inner: AutosaveSettingsScopeChat,
}

impl RTDAutosaveSettingsScopeChatBuilder {
  pub fn build(&self) -> AutosaveSettingsScopeChat {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }
}

impl AsRef<AutosaveSettingsScopeChat> for AutosaveSettingsScopeChat {
  fn as_ref(&self) -> &AutosaveSettingsScopeChat {
    self
  }
}

impl AsRef<AutosaveSettingsScopeChat> for RTDAutosaveSettingsScopeChatBuilder {
  fn as_ref(&self) -> &AutosaveSettingsScopeChat {
    &self.inner
  }
}

/// Autosave settings applied to all basic group and supergroup chats without chat-specific settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutosaveSettingsScopeGroupChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for AutosaveSettingsScopeGroupChats {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "autosaveSettingsScopeGroupChats"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAutosaveSettingsScope for AutosaveSettingsScopeGroupChats {}

impl AutosaveSettingsScopeGroupChats {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAutosaveSettingsScopeGroupChatsBuilder {
    let inner = AutosaveSettingsScopeGroupChats {
      td_name: "autosaveSettingsScopeGroupChats".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDAutosaveSettingsScopeGroupChatsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDAutosaveSettingsScopeGroupChatsBuilder {
  inner: AutosaveSettingsScopeGroupChats,
}

impl RTDAutosaveSettingsScopeGroupChatsBuilder {
  pub fn build(&self) -> AutosaveSettingsScopeGroupChats {
    self.inner.clone()
  }
}

impl AsRef<AutosaveSettingsScopeGroupChats> for AutosaveSettingsScopeGroupChats {
  fn as_ref(&self) -> &AutosaveSettingsScopeGroupChats {
    self
  }
}

impl AsRef<AutosaveSettingsScopeGroupChats> for RTDAutosaveSettingsScopeGroupChatsBuilder {
  fn as_ref(&self) -> &AutosaveSettingsScopeGroupChats {
    &self.inner
  }
}

/// Autosave settings applied to all private chats without chat-specific settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutosaveSettingsScopePrivateChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for AutosaveSettingsScopePrivateChats {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "autosaveSettingsScopePrivateChats"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAutosaveSettingsScope for AutosaveSettingsScopePrivateChats {}

impl AutosaveSettingsScopePrivateChats {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAutosaveSettingsScopePrivateChatsBuilder {
    let inner = AutosaveSettingsScopePrivateChats {
      td_name: "autosaveSettingsScopePrivateChats".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDAutosaveSettingsScopePrivateChatsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDAutosaveSettingsScopePrivateChatsBuilder {
  inner: AutosaveSettingsScopePrivateChats,
}

impl RTDAutosaveSettingsScopePrivateChatsBuilder {
  pub fn build(&self) -> AutosaveSettingsScopePrivateChats {
    self.inner.clone()
  }
}

impl AsRef<AutosaveSettingsScopePrivateChats> for AutosaveSettingsScopePrivateChats {
  fn as_ref(&self) -> &AutosaveSettingsScopePrivateChats {
    self
  }
}

impl AsRef<AutosaveSettingsScopePrivateChats> for RTDAutosaveSettingsScopePrivateChatsBuilder {
  fn as_ref(&self) -> &AutosaveSettingsScopePrivateChats {
    &self.inner
  }
}
