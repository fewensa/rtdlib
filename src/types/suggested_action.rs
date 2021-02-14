
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
  /// Suggests the user to check authorization phone number and change the phone number if it is inaccessible
  CheckPhoneNumber(SuggestedActionCheckPhoneNumber),
  /// Suggests the user to enable "archive_and_mute_new_chats_from_unknown_users" option
  EnableArchiveAndMuteNewChats(SuggestedActionEnableArchiveAndMuteNewChats),
  /// Suggests the user to see a hint about meaning of one and two ticks on sent message
  SeeTicksHint(SuggestedActionSeeTicksHint),

}

impl Default for SuggestedAction {
  fn default() -> Self { SuggestedAction::_Default(()) }
}

impl<'de> Deserialize<'de> for SuggestedAction {
  fn deserialize<D>(deserializer: D) -> Result<SuggestedAction, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      SuggestedAction,
      (suggestedActionCheckPhoneNumber, CheckPhoneNumber);
      (suggestedActionEnableArchiveAndMuteNewChats, EnableArchiveAndMuteNewChats);
      (suggestedActionSeeTicksHint, SeeTicksHint);

    )(deserializer)
  }
}

impl RObject for SuggestedAction {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      SuggestedAction::CheckPhoneNumber(t) => t.td_name(),
      SuggestedAction::EnableArchiveAndMuteNewChats(t) => t.td_name(),
      SuggestedAction::SeeTicksHint(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      SuggestedAction::CheckPhoneNumber(t) => t.extra(),
      SuggestedAction::EnableArchiveAndMuteNewChats(t) => t.extra(),
      SuggestedAction::SeeTicksHint(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl SuggestedAction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let SuggestedAction::_Default(_) = self { true } else { false } }

  pub fn is_check_phone_number(&self) -> bool { if let SuggestedAction::CheckPhoneNumber(_) = self { true } else { false } }
  pub fn is_enable_archive_and_mute_new_chats(&self) -> bool { if let SuggestedAction::EnableArchiveAndMuteNewChats(_) = self { true } else { false } }
  pub fn is_see_ticks_hint(&self) -> bool { if let SuggestedAction::SeeTicksHint(_) = self { true } else { false } }

  pub fn on_check_phone_number<F: FnOnce(&SuggestedActionCheckPhoneNumber)>(&self, fnc: F) -> &Self { if let SuggestedAction::CheckPhoneNumber(t) = self { fnc(t) }; self }
  pub fn on_enable_archive_and_mute_new_chats<F: FnOnce(&SuggestedActionEnableArchiveAndMuteNewChats)>(&self, fnc: F) -> &Self { if let SuggestedAction::EnableArchiveAndMuteNewChats(t) = self { fnc(t) }; self }
  pub fn on_see_ticks_hint<F: FnOnce(&SuggestedActionSeeTicksHint)>(&self, fnc: F) -> &Self { if let SuggestedAction::SeeTicksHint(t) = self { fnc(t) }; self }

  pub fn as_check_phone_number(&self) -> Option<&SuggestedActionCheckPhoneNumber> { if let SuggestedAction::CheckPhoneNumber(t) = self { return Some(t) } None }
  pub fn as_enable_archive_and_mute_new_chats(&self) -> Option<&SuggestedActionEnableArchiveAndMuteNewChats> { if let SuggestedAction::EnableArchiveAndMuteNewChats(t) = self { return Some(t) } None }
  pub fn as_see_ticks_hint(&self) -> Option<&SuggestedActionSeeTicksHint> { if let SuggestedAction::SeeTicksHint(t) = self { return Some(t) } None }



  pub fn check_phone_number<T: AsRef<SuggestedActionCheckPhoneNumber>>(t: T) -> Self { SuggestedAction::CheckPhoneNumber(t.as_ref().clone()) }

  pub fn enable_archive_and_mute_new_chats<T: AsRef<SuggestedActionEnableArchiveAndMuteNewChats>>(t: T) -> Self { SuggestedAction::EnableArchiveAndMuteNewChats(t.as_ref().clone()) }

  pub fn see_ticks_hint<T: AsRef<SuggestedActionSeeTicksHint>>(t: T) -> Self { SuggestedAction::SeeTicksHint(t.as_ref().clone()) }

}

impl AsRef<SuggestedAction> for SuggestedAction {
  fn as_ref(&self) -> &SuggestedAction { self }
}







/// Suggests the user to check authorization phone number and change the phone number if it is inaccessible
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







/// Suggests the user to see a hint about meaning of one and two ticks on sent message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionSeeTicksHint {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for SuggestedActionSeeTicksHint {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "suggestedActionSeeTicksHint" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDSuggestedAction for SuggestedActionSeeTicksHint {}



impl SuggestedActionSeeTicksHint {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSuggestedActionSeeTicksHintBuilder {
    let mut inner = SuggestedActionSeeTicksHint::default();
    inner.td_name = "suggestedActionSeeTicksHint".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSuggestedActionSeeTicksHintBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDSuggestedActionSeeTicksHintBuilder {
  inner: SuggestedActionSeeTicksHint
}

impl RTDSuggestedActionSeeTicksHintBuilder {
  pub fn build(&self) -> SuggestedActionSeeTicksHint { self.inner.clone() }

}

impl AsRef<SuggestedActionSeeTicksHint> for SuggestedActionSeeTicksHint {
  fn as_ref(&self) -> &SuggestedActionSeeTicksHint { self }
}

impl AsRef<SuggestedActionSeeTicksHint> for RTDSuggestedActionSeeTicksHintBuilder {
  fn as_ref(&self) -> &SuggestedActionSeeTicksHint { &self.inner }
}



