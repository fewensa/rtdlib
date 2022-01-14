
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes an action suggested to the current user
pub trait TDSuggestedAction: Debug + RObject {}

/// Describes an action suggested to the current user
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum SuggestedAction {
  #[doc(hidden)] _Default(()),
  /// Suggests the user to check whether they still remember their 2-step verification password
  CheckPassword(SuggestedActionCheckPassword),
  /// Suggests the user to check whether authorization phone number is correct and change the phone number if it is inaccessible
  CheckPhoneNumber(SuggestedActionCheckPhoneNumber),
  /// Suggests the user to convert specified supergroup to a broadcast group
  ConvertToBroadcastGroup(SuggestedActionConvertToBroadcastGroup),
  /// Suggests the user to enable "archive_and_mute_new_chats_from_unknown_users" option
  EnableArchiveAndMuteNewChats(SuggestedActionEnableArchiveAndMuteNewChats),
  /// Suggests the user to set a 2-step verification password to be able to log in again
  SetPassword(SuggestedActionSetPassword),
  /// Suggests the user to view a hint about the meaning of one and two check marks on sent messages
  ViewChecksHint(SuggestedActionViewChecksHint),

}

impl Default for SuggestedAction {
  fn default() -> Self { SuggestedAction::_Default(()) }
}

impl<'de> Deserialize<'de> for SuggestedAction {
  fn deserialize<D>(deserializer: D) -> Result<SuggestedAction, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      SuggestedAction,
      (suggestedActionCheckPassword, CheckPassword);
      (suggestedActionCheckPhoneNumber, CheckPhoneNumber);
      (suggestedActionConvertToBroadcastGroup, ConvertToBroadcastGroup);
      (suggestedActionEnableArchiveAndMuteNewChats, EnableArchiveAndMuteNewChats);
      (suggestedActionSetPassword, SetPassword);
      (suggestedActionViewChecksHint, ViewChecksHint);

    )(deserializer)
  }
}

impl RObject for SuggestedAction {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      SuggestedAction::CheckPassword(t) => t.td_name(),
      SuggestedAction::CheckPhoneNumber(t) => t.td_name(),
      SuggestedAction::ConvertToBroadcastGroup(t) => t.td_name(),
      SuggestedAction::EnableArchiveAndMuteNewChats(t) => t.td_name(),
      SuggestedAction::SetPassword(t) => t.td_name(),
      SuggestedAction::ViewChecksHint(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      SuggestedAction::CheckPassword(t) => t.extra(),
      SuggestedAction::CheckPhoneNumber(t) => t.extra(),
      SuggestedAction::ConvertToBroadcastGroup(t) => t.extra(),
      SuggestedAction::EnableArchiveAndMuteNewChats(t) => t.extra(),
      SuggestedAction::SetPassword(t) => t.extra(),
      SuggestedAction::ViewChecksHint(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl SuggestedAction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let SuggestedAction::_Default(_) = self { true } else { false } }

  pub fn is_check_password(&self) -> bool { if let SuggestedAction::CheckPassword(_) = self { true } else { false } }
  pub fn is_check_phone_number(&self) -> bool { if let SuggestedAction::CheckPhoneNumber(_) = self { true } else { false } }
  pub fn is_convert_to_broadcast_group(&self) -> bool { if let SuggestedAction::ConvertToBroadcastGroup(_) = self { true } else { false } }
  pub fn is_enable_archive_and_mute_new_chats(&self) -> bool { if let SuggestedAction::EnableArchiveAndMuteNewChats(_) = self { true } else { false } }
  pub fn is_set_password(&self) -> bool { if let SuggestedAction::SetPassword(_) = self { true } else { false } }
  pub fn is_view_checks_hint(&self) -> bool { if let SuggestedAction::ViewChecksHint(_) = self { true } else { false } }

  pub fn on_check_password<F: FnOnce(&SuggestedActionCheckPassword)>(&self, fnc: F) -> &Self { if let SuggestedAction::CheckPassword(t) = self { fnc(t) }; self }
  pub fn on_check_phone_number<F: FnOnce(&SuggestedActionCheckPhoneNumber)>(&self, fnc: F) -> &Self { if let SuggestedAction::CheckPhoneNumber(t) = self { fnc(t) }; self }
  pub fn on_convert_to_broadcast_group<F: FnOnce(&SuggestedActionConvertToBroadcastGroup)>(&self, fnc: F) -> &Self { if let SuggestedAction::ConvertToBroadcastGroup(t) = self { fnc(t) }; self }
  pub fn on_enable_archive_and_mute_new_chats<F: FnOnce(&SuggestedActionEnableArchiveAndMuteNewChats)>(&self, fnc: F) -> &Self { if let SuggestedAction::EnableArchiveAndMuteNewChats(t) = self { fnc(t) }; self }
  pub fn on_set_password<F: FnOnce(&SuggestedActionSetPassword)>(&self, fnc: F) -> &Self { if let SuggestedAction::SetPassword(t) = self { fnc(t) }; self }
  pub fn on_view_checks_hint<F: FnOnce(&SuggestedActionViewChecksHint)>(&self, fnc: F) -> &Self { if let SuggestedAction::ViewChecksHint(t) = self { fnc(t) }; self }

  pub fn as_check_password(&self) -> Option<&SuggestedActionCheckPassword> { if let SuggestedAction::CheckPassword(t) = self { return Some(t) } None }
  pub fn as_check_phone_number(&self) -> Option<&SuggestedActionCheckPhoneNumber> { if let SuggestedAction::CheckPhoneNumber(t) = self { return Some(t) } None }
  pub fn as_convert_to_broadcast_group(&self) -> Option<&SuggestedActionConvertToBroadcastGroup> { if let SuggestedAction::ConvertToBroadcastGroup(t) = self { return Some(t) } None }
  pub fn as_enable_archive_and_mute_new_chats(&self) -> Option<&SuggestedActionEnableArchiveAndMuteNewChats> { if let SuggestedAction::EnableArchiveAndMuteNewChats(t) = self { return Some(t) } None }
  pub fn as_set_password(&self) -> Option<&SuggestedActionSetPassword> { if let SuggestedAction::SetPassword(t) = self { return Some(t) } None }
  pub fn as_view_checks_hint(&self) -> Option<&SuggestedActionViewChecksHint> { if let SuggestedAction::ViewChecksHint(t) = self { return Some(t) } None }



  pub fn check_password<T: AsRef<SuggestedActionCheckPassword>>(t: T) -> Self { SuggestedAction::CheckPassword(t.as_ref().clone()) }

  pub fn check_phone_number<T: AsRef<SuggestedActionCheckPhoneNumber>>(t: T) -> Self { SuggestedAction::CheckPhoneNumber(t.as_ref().clone()) }

  pub fn convert_to_broadcast_group<T: AsRef<SuggestedActionConvertToBroadcastGroup>>(t: T) -> Self { SuggestedAction::ConvertToBroadcastGroup(t.as_ref().clone()) }

  pub fn enable_archive_and_mute_new_chats<T: AsRef<SuggestedActionEnableArchiveAndMuteNewChats>>(t: T) -> Self { SuggestedAction::EnableArchiveAndMuteNewChats(t.as_ref().clone()) }

  pub fn set_password<T: AsRef<SuggestedActionSetPassword>>(t: T) -> Self { SuggestedAction::SetPassword(t.as_ref().clone()) }

  pub fn view_checks_hint<T: AsRef<SuggestedActionViewChecksHint>>(t: T) -> Self { SuggestedAction::ViewChecksHint(t.as_ref().clone()) }

}

impl AsRef<SuggestedAction> for SuggestedAction {
  fn as_ref(&self) -> &SuggestedAction { self }
}







/// Suggests the user to check whether they still remember their 2-step verification password
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionCheckPassword {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for SuggestedActionCheckPassword {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "suggestedActionCheckPassword" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDSuggestedAction for SuggestedActionCheckPassword {}



impl SuggestedActionCheckPassword {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSuggestedActionCheckPasswordBuilder {
    let mut inner = SuggestedActionCheckPassword::default();
    inner.td_name = "suggestedActionCheckPassword".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSuggestedActionCheckPasswordBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDSuggestedActionCheckPasswordBuilder {
  inner: SuggestedActionCheckPassword
}

impl RTDSuggestedActionCheckPasswordBuilder {
  pub fn build(&self) -> SuggestedActionCheckPassword { self.inner.clone() }

}

impl AsRef<SuggestedActionCheckPassword> for SuggestedActionCheckPassword {
  fn as_ref(&self) -> &SuggestedActionCheckPassword { self }
}

impl AsRef<SuggestedActionCheckPassword> for RTDSuggestedActionCheckPasswordBuilder {
  fn as_ref(&self) -> &SuggestedActionCheckPassword { &self.inner }
}







/// Suggests the user to check whether authorization phone number is correct and change the phone number if it is inaccessible
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionCheckPhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for SuggestedActionCheckPhoneNumber {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "suggestedActionCheckPhoneNumber" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDSuggestedAction for SuggestedActionCheckPhoneNumber {}



impl SuggestedActionCheckPhoneNumber {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSuggestedActionCheckPhoneNumberBuilder {
    let mut inner = SuggestedActionCheckPhoneNumber::default();
    inner.td_name = "suggestedActionCheckPhoneNumber".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSuggestedActionCheckPhoneNumberBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDSuggestedActionCheckPhoneNumberBuilder {
  inner: SuggestedActionCheckPhoneNumber
}

impl RTDSuggestedActionCheckPhoneNumberBuilder {
  pub fn build(&self) -> SuggestedActionCheckPhoneNumber { self.inner.clone() }

}

impl AsRef<SuggestedActionCheckPhoneNumber> for SuggestedActionCheckPhoneNumber {
  fn as_ref(&self) -> &SuggestedActionCheckPhoneNumber { self }
}

impl AsRef<SuggestedActionCheckPhoneNumber> for RTDSuggestedActionCheckPhoneNumberBuilder {
  fn as_ref(&self) -> &SuggestedActionCheckPhoneNumber { &self.inner }
}







/// Suggests the user to convert specified supergroup to a broadcast group
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionConvertToBroadcastGroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Supergroup identifier
  supergroup_id: i64,
  
}

impl RObject for SuggestedActionConvertToBroadcastGroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "suggestedActionConvertToBroadcastGroup" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDSuggestedAction for SuggestedActionConvertToBroadcastGroup {}



impl SuggestedActionConvertToBroadcastGroup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSuggestedActionConvertToBroadcastGroupBuilder {
    let mut inner = SuggestedActionConvertToBroadcastGroup::default();
    inner.td_name = "suggestedActionConvertToBroadcastGroup".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSuggestedActionConvertToBroadcastGroupBuilder { inner }
  }

  pub fn supergroup_id(&self) -> i64 { self.supergroup_id }

}

#[doc(hidden)]
pub struct RTDSuggestedActionConvertToBroadcastGroupBuilder {
  inner: SuggestedActionConvertToBroadcastGroup
}

impl RTDSuggestedActionConvertToBroadcastGroupBuilder {
  pub fn build(&self) -> SuggestedActionConvertToBroadcastGroup { self.inner.clone() }

   
  pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
    self.inner.supergroup_id = supergroup_id;
    self
  }

}

impl AsRef<SuggestedActionConvertToBroadcastGroup> for SuggestedActionConvertToBroadcastGroup {
  fn as_ref(&self) -> &SuggestedActionConvertToBroadcastGroup { self }
}

impl AsRef<SuggestedActionConvertToBroadcastGroup> for RTDSuggestedActionConvertToBroadcastGroupBuilder {
  fn as_ref(&self) -> &SuggestedActionConvertToBroadcastGroup { &self.inner }
}







/// Suggests the user to enable "archive_and_mute_new_chats_from_unknown_users" option
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionEnableArchiveAndMuteNewChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for SuggestedActionEnableArchiveAndMuteNewChats {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "suggestedActionEnableArchiveAndMuteNewChats" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDSuggestedAction for SuggestedActionEnableArchiveAndMuteNewChats {}



impl SuggestedActionEnableArchiveAndMuteNewChats {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSuggestedActionEnableArchiveAndMuteNewChatsBuilder {
    let mut inner = SuggestedActionEnableArchiveAndMuteNewChats::default();
    inner.td_name = "suggestedActionEnableArchiveAndMuteNewChats".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSuggestedActionEnableArchiveAndMuteNewChatsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDSuggestedActionEnableArchiveAndMuteNewChatsBuilder {
  inner: SuggestedActionEnableArchiveAndMuteNewChats
}

impl RTDSuggestedActionEnableArchiveAndMuteNewChatsBuilder {
  pub fn build(&self) -> SuggestedActionEnableArchiveAndMuteNewChats { self.inner.clone() }

}

impl AsRef<SuggestedActionEnableArchiveAndMuteNewChats> for SuggestedActionEnableArchiveAndMuteNewChats {
  fn as_ref(&self) -> &SuggestedActionEnableArchiveAndMuteNewChats { self }
}

impl AsRef<SuggestedActionEnableArchiveAndMuteNewChats> for RTDSuggestedActionEnableArchiveAndMuteNewChatsBuilder {
  fn as_ref(&self) -> &SuggestedActionEnableArchiveAndMuteNewChats { &self.inner }
}







/// Suggests the user to set a 2-step verification password to be able to log in again
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionSetPassword {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The number of days to pass between consecutive authorizations if the user declines to set password
  authorization_delay: i64,
  
}

impl RObject for SuggestedActionSetPassword {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "suggestedActionSetPassword" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDSuggestedAction for SuggestedActionSetPassword {}



impl SuggestedActionSetPassword {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSuggestedActionSetPasswordBuilder {
    let mut inner = SuggestedActionSetPassword::default();
    inner.td_name = "suggestedActionSetPassword".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSuggestedActionSetPasswordBuilder { inner }
  }

  pub fn authorization_delay(&self) -> i64 { self.authorization_delay }

}

#[doc(hidden)]
pub struct RTDSuggestedActionSetPasswordBuilder {
  inner: SuggestedActionSetPassword
}

impl RTDSuggestedActionSetPasswordBuilder {
  pub fn build(&self) -> SuggestedActionSetPassword { self.inner.clone() }

   
  pub fn authorization_delay(&mut self, authorization_delay: i64) -> &mut Self {
    self.inner.authorization_delay = authorization_delay;
    self
  }

}

impl AsRef<SuggestedActionSetPassword> for SuggestedActionSetPassword {
  fn as_ref(&self) -> &SuggestedActionSetPassword { self }
}

impl AsRef<SuggestedActionSetPassword> for RTDSuggestedActionSetPasswordBuilder {
  fn as_ref(&self) -> &SuggestedActionSetPassword { &self.inner }
}







/// Suggests the user to view a hint about the meaning of one and two check marks on sent messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionViewChecksHint {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for SuggestedActionViewChecksHint {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "suggestedActionViewChecksHint" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDSuggestedAction for SuggestedActionViewChecksHint {}



impl SuggestedActionViewChecksHint {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSuggestedActionViewChecksHintBuilder {
    let mut inner = SuggestedActionViewChecksHint::default();
    inner.td_name = "suggestedActionViewChecksHint".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSuggestedActionViewChecksHintBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDSuggestedActionViewChecksHintBuilder {
  inner: SuggestedActionViewChecksHint
}

impl RTDSuggestedActionViewChecksHintBuilder {
  pub fn build(&self) -> SuggestedActionViewChecksHint { self.inner.clone() }

}

impl AsRef<SuggestedActionViewChecksHint> for SuggestedActionViewChecksHint {
  fn as_ref(&self) -> &SuggestedActionViewChecksHint { self }
}

impl AsRef<SuggestedActionViewChecksHint> for RTDSuggestedActionViewChecksHintBuilder {
  fn as_ref(&self) -> &SuggestedActionViewChecksHint { &self.inner }
}



