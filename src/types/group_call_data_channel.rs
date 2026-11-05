use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes data channel for a group call
pub trait TDGroupCallDataChannel: Debug + RObject {}

/// Describes data channel for a group call
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GroupCallDataChannel {
  #[doc(hidden)]
  _Default(()),
  /// The main data channel for audio and video data
  Main(GroupCallDataChannelMain),
  /// The data channel for screen sharing
  ScreenSharing(GroupCallDataChannelScreenSharing),
}

impl Default for GroupCallDataChannel {
  fn default() -> Self {
    GroupCallDataChannel::_Default(())
  }
}

impl<'de> Deserialize<'de> for GroupCallDataChannel {
  fn deserialize<D>(deserializer: D) -> Result<GroupCallDataChannel, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      GroupCallDataChannel,
      (groupCallDataChannelMain, Main);
      (groupCallDataChannelScreenSharing, ScreenSharing);

    )(deserializer)
  }
}

impl RObject for GroupCallDataChannel {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      GroupCallDataChannel::Main(t) => t.td_name(),
      GroupCallDataChannel::ScreenSharing(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      GroupCallDataChannel::Main(t) => t.extra(),
      GroupCallDataChannel::ScreenSharing(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GroupCallDataChannel {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, GroupCallDataChannel::_Default(_))
  }

  pub fn is_main(&self) -> bool {
    matches!(self, GroupCallDataChannel::Main(_))
  }
  pub fn is_screen_sharing(&self) -> bool {
    matches!(self, GroupCallDataChannel::ScreenSharing(_))
  }

  pub fn on_main<F: FnOnce(&GroupCallDataChannelMain)>(&self, fnc: F) -> &Self {
    if let GroupCallDataChannel::Main(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_screen_sharing<F: FnOnce(&GroupCallDataChannelScreenSharing)>(&self, fnc: F) -> &Self {
    if let GroupCallDataChannel::ScreenSharing(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_main(&self) -> Option<&GroupCallDataChannelMain> {
    if let GroupCallDataChannel::Main(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_screen_sharing(&self) -> Option<&GroupCallDataChannelScreenSharing> {
    if let GroupCallDataChannel::ScreenSharing(t) = self {
      return Some(t);
    }
    None
  }

  pub fn main<T: AsRef<GroupCallDataChannelMain>>(t: T) -> Self {
    GroupCallDataChannel::Main(t.as_ref().clone())
  }

  pub fn screen_sharing<T: AsRef<GroupCallDataChannelScreenSharing>>(t: T) -> Self {
    GroupCallDataChannel::ScreenSharing(t.as_ref().clone())
  }
}

impl AsRef<GroupCallDataChannel> for GroupCallDataChannel {
  fn as_ref(&self) -> &GroupCallDataChannel {
    self
  }
}

/// The main data channel for audio and video data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallDataChannelMain {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for GroupCallDataChannelMain {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "groupCallDataChannelMain"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDGroupCallDataChannel for GroupCallDataChannelMain {}

impl GroupCallDataChannelMain {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGroupCallDataChannelMainBuilder {
    let inner = GroupCallDataChannelMain {
      td_name: "groupCallDataChannelMain".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDGroupCallDataChannelMainBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDGroupCallDataChannelMainBuilder {
  inner: GroupCallDataChannelMain,
}

impl RTDGroupCallDataChannelMainBuilder {
  pub fn build(&self) -> GroupCallDataChannelMain {
    self.inner.clone()
  }
}

impl AsRef<GroupCallDataChannelMain> for GroupCallDataChannelMain {
  fn as_ref(&self) -> &GroupCallDataChannelMain {
    self
  }
}

impl AsRef<GroupCallDataChannelMain> for RTDGroupCallDataChannelMainBuilder {
  fn as_ref(&self) -> &GroupCallDataChannelMain {
    &self.inner
  }
}

/// The data channel for screen sharing
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallDataChannelScreenSharing {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for GroupCallDataChannelScreenSharing {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "groupCallDataChannelScreenSharing"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDGroupCallDataChannel for GroupCallDataChannelScreenSharing {}

impl GroupCallDataChannelScreenSharing {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGroupCallDataChannelScreenSharingBuilder {
    let inner = GroupCallDataChannelScreenSharing {
      td_name: "groupCallDataChannelScreenSharing".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDGroupCallDataChannelScreenSharingBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDGroupCallDataChannelScreenSharingBuilder {
  inner: GroupCallDataChannelScreenSharing,
}

impl RTDGroupCallDataChannelScreenSharingBuilder {
  pub fn build(&self) -> GroupCallDataChannelScreenSharing {
    self.inner.clone()
  }
}

impl AsRef<GroupCallDataChannelScreenSharing> for GroupCallDataChannelScreenSharing {
  fn as_ref(&self) -> &GroupCallDataChannelScreenSharing {
    self
  }
}

impl AsRef<GroupCallDataChannelScreenSharing> for RTDGroupCallDataChannelScreenSharingBuilder {
  fn as_ref(&self) -> &GroupCallDataChannelScreenSharing {
    &self.inner
  }
}
