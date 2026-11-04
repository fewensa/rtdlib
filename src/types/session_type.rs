use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes type of user session
pub trait TDSessionType: Debug + RObject {}

/// Describes type of user session
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum SessionType {
  #[doc(hidden)]
  _Default(()),
  /// A business bot connected to the current user's account
  ConnectedBot(SessionTypeConnectedBot),
  /// A regular session from a device
  Device(SessionTypeDevice),
}

impl Default for SessionType {
  fn default() -> Self {
    SessionType::_Default(())
  }
}

impl<'de> Deserialize<'de> for SessionType {
  fn deserialize<D>(deserializer: D) -> Result<SessionType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      SessionType,
      (sessionTypeConnectedBot, ConnectedBot);
      (sessionTypeDevice, Device);

    )(deserializer)
  }
}

impl RObject for SessionType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      SessionType::ConnectedBot(t) => t.td_name(),
      SessionType::Device(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      SessionType::ConnectedBot(t) => t.extra(),
      SessionType::Device(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl SessionType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, SessionType::_Default(_))
  }

  pub fn is_connected_bot(&self) -> bool {
    matches!(self, SessionType::ConnectedBot(_))
  }
  pub fn is_device(&self) -> bool {
    matches!(self, SessionType::Device(_))
  }

  pub fn on_connected_bot<F: FnOnce(&SessionTypeConnectedBot)>(&self, fnc: F) -> &Self {
    if let SessionType::ConnectedBot(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_device<F: FnOnce(&SessionTypeDevice)>(&self, fnc: F) -> &Self {
    if let SessionType::Device(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_connected_bot(&self) -> Option<&SessionTypeConnectedBot> {
    if let SessionType::ConnectedBot(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_device(&self) -> Option<&SessionTypeDevice> {
    if let SessionType::Device(t) = self {
      return Some(t);
    }
    None
  }

  pub fn connected_bot<T: AsRef<SessionTypeConnectedBot>>(t: T) -> Self {
    SessionType::ConnectedBot(t.as_ref().clone())
  }

  pub fn device<T: AsRef<SessionTypeDevice>>(t: T) -> Self {
    SessionType::Device(t.as_ref().clone())
  }
}

impl AsRef<SessionType> for SessionType {
  fn as_ref(&self) -> &SessionType {
    self
  }
}

/// A business bot connected to the current user's account
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionTypeConnectedBot {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier of the bot. Use deleteBusinessConnectedBot to remove it or confirmBusinessConnectedBot to confirm it if it isn't confirmed yet
  bot_user_id: i64,
}

impl RObject for SessionTypeConnectedBot {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sessionTypeConnectedBot"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSessionType for SessionTypeConnectedBot {}

impl SessionTypeConnectedBot {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSessionTypeConnectedBotBuilder {
    let inner = SessionTypeConnectedBot {
      td_name: "sessionTypeConnectedBot".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSessionTypeConnectedBotBuilder { inner }
  }

  pub fn bot_user_id(&self) -> i64 {
    self.bot_user_id
  }
}

#[doc(hidden)]
pub struct RTDSessionTypeConnectedBotBuilder {
  inner: SessionTypeConnectedBot,
}

impl RTDSessionTypeConnectedBotBuilder {
  pub fn build(&self) -> SessionTypeConnectedBot {
    self.inner.clone()
  }

  pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
    self.inner.bot_user_id = bot_user_id;
    self
  }
}

impl AsRef<SessionTypeConnectedBot> for SessionTypeConnectedBot {
  fn as_ref(&self) -> &SessionTypeConnectedBot {
    self
  }
}

impl AsRef<SessionTypeConnectedBot> for RTDSessionTypeConnectedBotBuilder {
  fn as_ref(&self) -> &SessionTypeConnectedBot {
    &self.inner
  }
}

/// A regular session from a device
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionTypeDevice {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the session. Use terminateSession to terminate it or confirmSession to confirm it if it isn't confirmed yet
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  session_id: isize,
}

impl RObject for SessionTypeDevice {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sessionTypeDevice"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSessionType for SessionTypeDevice {}

impl SessionTypeDevice {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSessionTypeDeviceBuilder {
    let inner = SessionTypeDevice {
      td_name: "sessionTypeDevice".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSessionTypeDeviceBuilder { inner }
  }

  pub fn session_id(&self) -> isize {
    self.session_id
  }
}

#[doc(hidden)]
pub struct RTDSessionTypeDeviceBuilder {
  inner: SessionTypeDevice,
}

impl RTDSessionTypeDeviceBuilder {
  pub fn build(&self) -> SessionTypeDevice {
    self.inner.clone()
  }

  pub fn session_id(&mut self, session_id: isize) -> &mut Self {
    self.inner.session_id = session_id;
    self
  }
}

impl AsRef<SessionTypeDevice> for SessionTypeDevice {
  fn as_ref(&self) -> &SessionTypeDevice {
    self
  }
}

impl AsRef<SessionTypeDevice> for RTDSessionTypeDeviceBuilder {
  fn as_ref(&self) -> &SessionTypeDevice {
    &self.inner
  }
}
