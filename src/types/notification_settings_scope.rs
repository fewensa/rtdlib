
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes the types of chats to which notification settings are applied
pub trait TDNotificationSettingsScope: Debug + RObject {}

/// Describes the types of chats to which notification settings are applied
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum NotificationSettingsScope {
  #[doc(hidden)] _Default(()),
  /// Notification settings applied to all basic groups, supergroups and channels when the corresponding chat setting has a default value
  GroupChats(NotificationSettingsScopeGroupChats),
  /// Notification settings applied to all private and secret chats when the corresponding chat setting has a default value
  PrivateChats(NotificationSettingsScopePrivateChats),

}

impl Default for NotificationSettingsScope {
  fn default() -> Self { NotificationSettingsScope::_Default(()) }
}

impl<'de> Deserialize<'de> for NotificationSettingsScope {
  fn deserialize<D>(deserializer: D) -> Result<NotificationSettingsScope, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      NotificationSettingsScope,
      (notificationSettingsScopeGroupChats, GroupChats);
      (notificationSettingsScopePrivateChats, PrivateChats);

    )(deserializer)
  }
}

impl RObject for NotificationSettingsScope {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      NotificationSettingsScope::GroupChats(t) => t.td_name(),
      NotificationSettingsScope::PrivateChats(t) => t.td_name(),

      _ => "-1",
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl NotificationSettingsScope {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let NotificationSettingsScope::_Default(_) = self { true } else { false } }

  pub fn is_group_chats(&self) -> bool { if let NotificationSettingsScope::GroupChats(_) = self { true } else { false } }
  pub fn is_private_chats(&self) -> bool { if let NotificationSettingsScope::PrivateChats(_) = self { true } else { false } }

  pub fn on_group_chats<F: FnOnce(&NotificationSettingsScopeGroupChats)>(&self, fnc: F) -> &Self { if let NotificationSettingsScope::GroupChats(t) = self { fnc(t) }; self }
  pub fn on_private_chats<F: FnOnce(&NotificationSettingsScopePrivateChats)>(&self, fnc: F) -> &Self { if let NotificationSettingsScope::PrivateChats(t) = self { fnc(t) }; self }

  pub fn as_group_chats(&self) -> Option<&NotificationSettingsScopeGroupChats> { if let NotificationSettingsScope::GroupChats(t) = self { return Some(t) } None }
  pub fn as_private_chats(&self) -> Option<&NotificationSettingsScopePrivateChats> { if let NotificationSettingsScope::PrivateChats(t) = self { return Some(t) } None }



  pub fn group_chats<T: AsRef<NotificationSettingsScopeGroupChats>>(t: T) -> Self { NotificationSettingsScope::GroupChats(t.as_ref().clone()) }

  pub fn private_chats<T: AsRef<NotificationSettingsScopePrivateChats>>(t: T) -> Self { NotificationSettingsScope::PrivateChats(t.as_ref().clone()) }

}

impl AsRef<NotificationSettingsScope> for NotificationSettingsScope {
  fn as_ref(&self) -> &NotificationSettingsScope { self }
}







/// Notification settings applied to all basic groups, supergroups and channels when the corresponding chat setting has a default value
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationSettingsScopeGroupChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for NotificationSettingsScopeGroupChats {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "notificationSettingsScopeGroupChats" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDNotificationSettingsScope for NotificationSettingsScopeGroupChats {}



impl NotificationSettingsScopeGroupChats {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDNotificationSettingsScopeGroupChatsBuilder {
    let mut inner = NotificationSettingsScopeGroupChats::default();
    inner.td_name = "notificationSettingsScopeGroupChats".to_string();
    RTDNotificationSettingsScopeGroupChatsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDNotificationSettingsScopeGroupChatsBuilder {
  inner: NotificationSettingsScopeGroupChats
}

impl RTDNotificationSettingsScopeGroupChatsBuilder {
  pub fn build(&self) -> NotificationSettingsScopeGroupChats { self.inner.clone() }

}

impl AsRef<NotificationSettingsScopeGroupChats> for NotificationSettingsScopeGroupChats {
  fn as_ref(&self) -> &NotificationSettingsScopeGroupChats { self }
}

impl AsRef<NotificationSettingsScopeGroupChats> for RTDNotificationSettingsScopeGroupChatsBuilder {
  fn as_ref(&self) -> &NotificationSettingsScopeGroupChats { &self.inner }
}







/// Notification settings applied to all private and secret chats when the corresponding chat setting has a default value
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationSettingsScopePrivateChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for NotificationSettingsScopePrivateChats {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "notificationSettingsScopePrivateChats" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDNotificationSettingsScope for NotificationSettingsScopePrivateChats {}



impl NotificationSettingsScopePrivateChats {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDNotificationSettingsScopePrivateChatsBuilder {
    let mut inner = NotificationSettingsScopePrivateChats::default();
    inner.td_name = "notificationSettingsScopePrivateChats".to_string();
    RTDNotificationSettingsScopePrivateChatsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDNotificationSettingsScopePrivateChatsBuilder {
  inner: NotificationSettingsScopePrivateChats
}

impl RTDNotificationSettingsScopePrivateChatsBuilder {
  pub fn build(&self) -> NotificationSettingsScopePrivateChats { self.inner.clone() }

}

impl AsRef<NotificationSettingsScopePrivateChats> for NotificationSettingsScopePrivateChats {
  fn as_ref(&self) -> &NotificationSettingsScopePrivateChats { self }
}

impl AsRef<NotificationSettingsScopePrivateChats> for RTDNotificationSettingsScopePrivateChatsBuilder {
  fn as_ref(&self) -> &NotificationSettingsScopePrivateChats { &self.inner }
}



