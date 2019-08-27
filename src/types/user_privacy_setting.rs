
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes available user privacy settings
pub trait TDUserPrivacySetting: Debug + RObject {}

/// Describes available user privacy settings
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum UserPrivacySetting {
  #[doc(hidden)] _Default(()),
  /// A privacy setting for managing whether the user's online status is visible
  ShowStatus(UserPrivacySettingShowStatus),
  /// A privacy setting for managing whether the user can be invited to chats
  AllowChatInvites(UserPrivacySettingAllowChatInvites),
  /// A privacy setting for managing whether the user can be called
  AllowCalls(UserPrivacySettingAllowCalls),
  /// A privacy setting for managing whether peer-to-peer connections can be used for calls
  AllowPeerToPeerCalls(UserPrivacySettingAllowPeerToPeerCalls),

}

impl Default for UserPrivacySetting {
  fn default() -> Self { UserPrivacySetting::_Default(()) }
}

impl<'de> Deserialize<'de> for UserPrivacySetting {
  fn deserialize<D>(deserializer: D) -> Result<UserPrivacySetting, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      UserPrivacySetting,
      (userPrivacySettingShowStatus, ShowStatus);
      (userPrivacySettingAllowChatInvites, AllowChatInvites);
      (userPrivacySettingAllowCalls, AllowCalls);
      (userPrivacySettingAllowPeerToPeerCalls, AllowPeerToPeerCalls);

    )(deserializer)
  }
}

impl RObject for UserPrivacySetting {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      UserPrivacySetting::ShowStatus(t) => t.td_name(),
      UserPrivacySetting::AllowChatInvites(t) => t.td_name(),
      UserPrivacySetting::AllowCalls(t) => t.td_name(),
      UserPrivacySetting::AllowPeerToPeerCalls(t) => t.td_name(),

      _ => "-1",
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl UserPrivacySetting {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let UserPrivacySetting::_Default(_) = self { true } else { false } }

  pub fn is_show_status(&self) -> bool { if let UserPrivacySetting::ShowStatus(_) = self { true } else { false } }
  pub fn is_allow_chat_invites(&self) -> bool { if let UserPrivacySetting::AllowChatInvites(_) = self { true } else { false } }
  pub fn is_allow_calls(&self) -> bool { if let UserPrivacySetting::AllowCalls(_) = self { true } else { false } }
  pub fn is_allow_peer_to_peer_calls(&self) -> bool { if let UserPrivacySetting::AllowPeerToPeerCalls(_) = self { true } else { false } }

  pub fn on_show_status<F: FnOnce(&UserPrivacySettingShowStatus)>(&self, fnc: F) -> &Self { if let UserPrivacySetting::ShowStatus(t) = self { fnc(t) }; self }
  pub fn on_allow_chat_invites<F: FnOnce(&UserPrivacySettingAllowChatInvites)>(&self, fnc: F) -> &Self { if let UserPrivacySetting::AllowChatInvites(t) = self { fnc(t) }; self }
  pub fn on_allow_calls<F: FnOnce(&UserPrivacySettingAllowCalls)>(&self, fnc: F) -> &Self { if let UserPrivacySetting::AllowCalls(t) = self { fnc(t) }; self }
  pub fn on_allow_peer_to_peer_calls<F: FnOnce(&UserPrivacySettingAllowPeerToPeerCalls)>(&self, fnc: F) -> &Self { if let UserPrivacySetting::AllowPeerToPeerCalls(t) = self { fnc(t) }; self }

  pub fn as_show_status(&self) -> Option<&UserPrivacySettingShowStatus> { if let UserPrivacySetting::ShowStatus(t) = self { return Some(t) } None }
  pub fn as_allow_chat_invites(&self) -> Option<&UserPrivacySettingAllowChatInvites> { if let UserPrivacySetting::AllowChatInvites(t) = self { return Some(t) } None }
  pub fn as_allow_calls(&self) -> Option<&UserPrivacySettingAllowCalls> { if let UserPrivacySetting::AllowCalls(t) = self { return Some(t) } None }
  pub fn as_allow_peer_to_peer_calls(&self) -> Option<&UserPrivacySettingAllowPeerToPeerCalls> { if let UserPrivacySetting::AllowPeerToPeerCalls(t) = self { return Some(t) } None }



  pub fn show_status<T: AsRef<UserPrivacySettingShowStatus>>(t: T) -> Self { UserPrivacySetting::ShowStatus(t.as_ref().clone()) }

  pub fn allow_chat_invites<T: AsRef<UserPrivacySettingAllowChatInvites>>(t: T) -> Self { UserPrivacySetting::AllowChatInvites(t.as_ref().clone()) }

  pub fn allow_calls<T: AsRef<UserPrivacySettingAllowCalls>>(t: T) -> Self { UserPrivacySetting::AllowCalls(t.as_ref().clone()) }

  pub fn allow_peer_to_peer_calls<T: AsRef<UserPrivacySettingAllowPeerToPeerCalls>>(t: T) -> Self { UserPrivacySetting::AllowPeerToPeerCalls(t.as_ref().clone()) }

}

impl AsRef<UserPrivacySetting> for UserPrivacySetting {
  fn as_ref(&self) -> &UserPrivacySetting { self }
}







/// A privacy setting for managing whether the user's online status is visible
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingShowStatus {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for UserPrivacySettingShowStatus {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userPrivacySettingShowStatus" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUserPrivacySetting for UserPrivacySettingShowStatus {}



impl UserPrivacySettingShowStatus {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserPrivacySettingShowStatusBuilder {
    let mut inner = UserPrivacySettingShowStatus::default();
    inner.td_name = "userPrivacySettingShowStatus".to_string();
    RTDUserPrivacySettingShowStatusBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDUserPrivacySettingShowStatusBuilder {
  inner: UserPrivacySettingShowStatus
}

impl RTDUserPrivacySettingShowStatusBuilder {
  pub fn build(&self) -> UserPrivacySettingShowStatus { self.inner.clone() }

}

impl AsRef<UserPrivacySettingShowStatus> for UserPrivacySettingShowStatus {
  fn as_ref(&self) -> &UserPrivacySettingShowStatus { self }
}

impl AsRef<UserPrivacySettingShowStatus> for RTDUserPrivacySettingShowStatusBuilder {
  fn as_ref(&self) -> &UserPrivacySettingShowStatus { &self.inner }
}







/// A privacy setting for managing whether the user can be invited to chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingAllowChatInvites {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for UserPrivacySettingAllowChatInvites {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userPrivacySettingAllowChatInvites" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUserPrivacySetting for UserPrivacySettingAllowChatInvites {}



impl UserPrivacySettingAllowChatInvites {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserPrivacySettingAllowChatInvitesBuilder {
    let mut inner = UserPrivacySettingAllowChatInvites::default();
    inner.td_name = "userPrivacySettingAllowChatInvites".to_string();
    RTDUserPrivacySettingAllowChatInvitesBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDUserPrivacySettingAllowChatInvitesBuilder {
  inner: UserPrivacySettingAllowChatInvites
}

impl RTDUserPrivacySettingAllowChatInvitesBuilder {
  pub fn build(&self) -> UserPrivacySettingAllowChatInvites { self.inner.clone() }

}

impl AsRef<UserPrivacySettingAllowChatInvites> for UserPrivacySettingAllowChatInvites {
  fn as_ref(&self) -> &UserPrivacySettingAllowChatInvites { self }
}

impl AsRef<UserPrivacySettingAllowChatInvites> for RTDUserPrivacySettingAllowChatInvitesBuilder {
  fn as_ref(&self) -> &UserPrivacySettingAllowChatInvites { &self.inner }
}







/// A privacy setting for managing whether the user can be called
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingAllowCalls {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for UserPrivacySettingAllowCalls {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userPrivacySettingAllowCalls" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUserPrivacySetting for UserPrivacySettingAllowCalls {}



impl UserPrivacySettingAllowCalls {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserPrivacySettingAllowCallsBuilder {
    let mut inner = UserPrivacySettingAllowCalls::default();
    inner.td_name = "userPrivacySettingAllowCalls".to_string();
    RTDUserPrivacySettingAllowCallsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDUserPrivacySettingAllowCallsBuilder {
  inner: UserPrivacySettingAllowCalls
}

impl RTDUserPrivacySettingAllowCallsBuilder {
  pub fn build(&self) -> UserPrivacySettingAllowCalls { self.inner.clone() }

}

impl AsRef<UserPrivacySettingAllowCalls> for UserPrivacySettingAllowCalls {
  fn as_ref(&self) -> &UserPrivacySettingAllowCalls { self }
}

impl AsRef<UserPrivacySettingAllowCalls> for RTDUserPrivacySettingAllowCallsBuilder {
  fn as_ref(&self) -> &UserPrivacySettingAllowCalls { &self.inner }
}







/// A privacy setting for managing whether peer-to-peer connections can be used for calls
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingAllowPeerToPeerCalls {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for UserPrivacySettingAllowPeerToPeerCalls {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userPrivacySettingAllowPeerToPeerCalls" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUserPrivacySetting for UserPrivacySettingAllowPeerToPeerCalls {}



impl UserPrivacySettingAllowPeerToPeerCalls {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserPrivacySettingAllowPeerToPeerCallsBuilder {
    let mut inner = UserPrivacySettingAllowPeerToPeerCalls::default();
    inner.td_name = "userPrivacySettingAllowPeerToPeerCalls".to_string();
    RTDUserPrivacySettingAllowPeerToPeerCallsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDUserPrivacySettingAllowPeerToPeerCallsBuilder {
  inner: UserPrivacySettingAllowPeerToPeerCalls
}

impl RTDUserPrivacySettingAllowPeerToPeerCallsBuilder {
  pub fn build(&self) -> UserPrivacySettingAllowPeerToPeerCalls { self.inner.clone() }

}

impl AsRef<UserPrivacySettingAllowPeerToPeerCalls> for UserPrivacySettingAllowPeerToPeerCalls {
  fn as_ref(&self) -> &UserPrivacySettingAllowPeerToPeerCalls { self }
}

impl AsRef<UserPrivacySettingAllowPeerToPeerCalls> for RTDUserPrivacySettingAllowPeerToPeerCallsBuilder {
  fn as_ref(&self) -> &UserPrivacySettingAllowPeerToPeerCalls { &self.inner }
}



