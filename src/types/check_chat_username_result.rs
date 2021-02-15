
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Represents result of checking whether a username can be set for a chat
pub trait TDCheckChatUsernameResult: Debug + RObject {}

/// Represents result of checking whether a username can be set for a chat
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CheckChatUsernameResult {
  #[doc(hidden)] _Default(()),
  /// Checks whether a username can be set for a chat
  CheckChatUsername(CheckChatUsername),
  /// The username can be set
  Ok(CheckChatUsernameResultOk),
  /// The user has too much chats with username, one of them should be made private first
  PublicChatsTooMuch(CheckChatUsernameResultPublicChatsTooMuch),
  /// The user can't be a member of a public supergroup
  PublicGroupsUnavailable(CheckChatUsernameResultPublicGroupsUnavailable),
  /// The username is invalid
  UsernameInvalid(CheckChatUsernameResultUsernameInvalid),
  /// The username is occupied
  UsernameOccupied(CheckChatUsernameResultUsernameOccupied),

}

impl Default for CheckChatUsernameResult {
  fn default() -> Self { CheckChatUsernameResult::_Default(()) }
}

impl<'de> Deserialize<'de> for CheckChatUsernameResult {
  fn deserialize<D>(deserializer: D) -> Result<CheckChatUsernameResult, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      CheckChatUsernameResult,
      (checkChatUsername, CheckChatUsername);
      (checkChatUsernameResultOk, Ok);
      (checkChatUsernameResultPublicChatsTooMuch, PublicChatsTooMuch);
      (checkChatUsernameResultPublicGroupsUnavailable, PublicGroupsUnavailable);
      (checkChatUsernameResultUsernameInvalid, UsernameInvalid);
      (checkChatUsernameResultUsernameOccupied, UsernameOccupied);

    )(deserializer)
  }
}

impl RObject for CheckChatUsernameResult {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      CheckChatUsernameResult::CheckChatUsername(t) => t.td_name(),
      CheckChatUsernameResult::Ok(t) => t.td_name(),
      CheckChatUsernameResult::PublicChatsTooMuch(t) => t.td_name(),
      CheckChatUsernameResult::PublicGroupsUnavailable(t) => t.td_name(),
      CheckChatUsernameResult::UsernameInvalid(t) => t.td_name(),
      CheckChatUsernameResult::UsernameOccupied(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      CheckChatUsernameResult::CheckChatUsername(t) => t.extra(),
      CheckChatUsernameResult::Ok(t) => t.extra(),
      CheckChatUsernameResult::PublicChatsTooMuch(t) => t.extra(),
      CheckChatUsernameResult::PublicGroupsUnavailable(t) => t.extra(),
      CheckChatUsernameResult::UsernameInvalid(t) => t.extra(),
      CheckChatUsernameResult::UsernameOccupied(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl CheckChatUsernameResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let CheckChatUsernameResult::_Default(_) = self { true } else { false } }

  pub fn is_check_chat_username(&self) -> bool { if let CheckChatUsernameResult::CheckChatUsername(_) = self { true } else { false } }
  pub fn is_ok(&self) -> bool { if let CheckChatUsernameResult::Ok(_) = self { true } else { false } }
  pub fn is_public_chats_too_much(&self) -> bool { if let CheckChatUsernameResult::PublicChatsTooMuch(_) = self { true } else { false } }
  pub fn is_public_groups_unavailable(&self) -> bool { if let CheckChatUsernameResult::PublicGroupsUnavailable(_) = self { true } else { false } }
  pub fn is_username_invalid(&self) -> bool { if let CheckChatUsernameResult::UsernameInvalid(_) = self { true } else { false } }
  pub fn is_username_occupied(&self) -> bool { if let CheckChatUsernameResult::UsernameOccupied(_) = self { true } else { false } }

  pub fn on_check_chat_username<F: FnOnce(&CheckChatUsername)>(&self, fnc: F) -> &Self { if let CheckChatUsernameResult::CheckChatUsername(t) = self { fnc(t) }; self }
  pub fn on_ok<F: FnOnce(&CheckChatUsernameResultOk)>(&self, fnc: F) -> &Self { if let CheckChatUsernameResult::Ok(t) = self { fnc(t) }; self }
  pub fn on_public_chats_too_much<F: FnOnce(&CheckChatUsernameResultPublicChatsTooMuch)>(&self, fnc: F) -> &Self { if let CheckChatUsernameResult::PublicChatsTooMuch(t) = self { fnc(t) }; self }
  pub fn on_public_groups_unavailable<F: FnOnce(&CheckChatUsernameResultPublicGroupsUnavailable)>(&self, fnc: F) -> &Self { if let CheckChatUsernameResult::PublicGroupsUnavailable(t) = self { fnc(t) }; self }
  pub fn on_username_invalid<F: FnOnce(&CheckChatUsernameResultUsernameInvalid)>(&self, fnc: F) -> &Self { if let CheckChatUsernameResult::UsernameInvalid(t) = self { fnc(t) }; self }
  pub fn on_username_occupied<F: FnOnce(&CheckChatUsernameResultUsernameOccupied)>(&self, fnc: F) -> &Self { if let CheckChatUsernameResult::UsernameOccupied(t) = self { fnc(t) }; self }

  pub fn as_check_chat_username(&self) -> Option<&CheckChatUsername> { if let CheckChatUsernameResult::CheckChatUsername(t) = self { return Some(t) } None }
  pub fn as_ok(&self) -> Option<&CheckChatUsernameResultOk> { if let CheckChatUsernameResult::Ok(t) = self { return Some(t) } None }
  pub fn as_public_chats_too_much(&self) -> Option<&CheckChatUsernameResultPublicChatsTooMuch> { if let CheckChatUsernameResult::PublicChatsTooMuch(t) = self { return Some(t) } None }
  pub fn as_public_groups_unavailable(&self) -> Option<&CheckChatUsernameResultPublicGroupsUnavailable> { if let CheckChatUsernameResult::PublicGroupsUnavailable(t) = self { return Some(t) } None }
  pub fn as_username_invalid(&self) -> Option<&CheckChatUsernameResultUsernameInvalid> { if let CheckChatUsernameResult::UsernameInvalid(t) = self { return Some(t) } None }
  pub fn as_username_occupied(&self) -> Option<&CheckChatUsernameResultUsernameOccupied> { if let CheckChatUsernameResult::UsernameOccupied(t) = self { return Some(t) } None }



  pub fn check_chat_username<T: AsRef<CheckChatUsername>>(t: T) -> Self { CheckChatUsernameResult::CheckChatUsername(t.as_ref().clone()) }

  pub fn ok<T: AsRef<CheckChatUsernameResultOk>>(t: T) -> Self { CheckChatUsernameResult::Ok(t.as_ref().clone()) }

  pub fn public_chats_too_much<T: AsRef<CheckChatUsernameResultPublicChatsTooMuch>>(t: T) -> Self { CheckChatUsernameResult::PublicChatsTooMuch(t.as_ref().clone()) }

  pub fn public_groups_unavailable<T: AsRef<CheckChatUsernameResultPublicGroupsUnavailable>>(t: T) -> Self { CheckChatUsernameResult::PublicGroupsUnavailable(t.as_ref().clone()) }

  pub fn username_invalid<T: AsRef<CheckChatUsernameResultUsernameInvalid>>(t: T) -> Self { CheckChatUsernameResult::UsernameInvalid(t.as_ref().clone()) }

  pub fn username_occupied<T: AsRef<CheckChatUsernameResultUsernameOccupied>>(t: T) -> Self { CheckChatUsernameResult::UsernameOccupied(t.as_ref().clone()) }

}

impl AsRef<CheckChatUsernameResult> for CheckChatUsernameResult {
  fn as_ref(&self) -> &CheckChatUsernameResult { self }
}







/// The username can be set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckChatUsernameResultOk {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for CheckChatUsernameResultOk {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkChatUsernameResultOk" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDCheckChatUsernameResult for CheckChatUsernameResultOk {}



impl CheckChatUsernameResultOk {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCheckChatUsernameResultOkBuilder {
    let mut inner = CheckChatUsernameResultOk::default();
    inner.td_name = "checkChatUsernameResultOk".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCheckChatUsernameResultOkBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDCheckChatUsernameResultOkBuilder {
  inner: CheckChatUsernameResultOk
}

impl RTDCheckChatUsernameResultOkBuilder {
  pub fn build(&self) -> CheckChatUsernameResultOk { self.inner.clone() }

}

impl AsRef<CheckChatUsernameResultOk> for CheckChatUsernameResultOk {
  fn as_ref(&self) -> &CheckChatUsernameResultOk { self }
}

impl AsRef<CheckChatUsernameResultOk> for RTDCheckChatUsernameResultOkBuilder {
  fn as_ref(&self) -> &CheckChatUsernameResultOk { &self.inner }
}







/// The user has too much chats with username, one of them should be made private first
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckChatUsernameResultPublicChatsTooMuch {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for CheckChatUsernameResultPublicChatsTooMuch {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkChatUsernameResultPublicChatsTooMuch" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDCheckChatUsernameResult for CheckChatUsernameResultPublicChatsTooMuch {}



impl CheckChatUsernameResultPublicChatsTooMuch {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCheckChatUsernameResultPublicChatsTooMuchBuilder {
    let mut inner = CheckChatUsernameResultPublicChatsTooMuch::default();
    inner.td_name = "checkChatUsernameResultPublicChatsTooMuch".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCheckChatUsernameResultPublicChatsTooMuchBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDCheckChatUsernameResultPublicChatsTooMuchBuilder {
  inner: CheckChatUsernameResultPublicChatsTooMuch
}

impl RTDCheckChatUsernameResultPublicChatsTooMuchBuilder {
  pub fn build(&self) -> CheckChatUsernameResultPublicChatsTooMuch { self.inner.clone() }

}

impl AsRef<CheckChatUsernameResultPublicChatsTooMuch> for CheckChatUsernameResultPublicChatsTooMuch {
  fn as_ref(&self) -> &CheckChatUsernameResultPublicChatsTooMuch { self }
}

impl AsRef<CheckChatUsernameResultPublicChatsTooMuch> for RTDCheckChatUsernameResultPublicChatsTooMuchBuilder {
  fn as_ref(&self) -> &CheckChatUsernameResultPublicChatsTooMuch { &self.inner }
}







/// The user can't be a member of a public supergroup
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckChatUsernameResultPublicGroupsUnavailable {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for CheckChatUsernameResultPublicGroupsUnavailable {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkChatUsernameResultPublicGroupsUnavailable" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDCheckChatUsernameResult for CheckChatUsernameResultPublicGroupsUnavailable {}



impl CheckChatUsernameResultPublicGroupsUnavailable {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCheckChatUsernameResultPublicGroupsUnavailableBuilder {
    let mut inner = CheckChatUsernameResultPublicGroupsUnavailable::default();
    inner.td_name = "checkChatUsernameResultPublicGroupsUnavailable".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCheckChatUsernameResultPublicGroupsUnavailableBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDCheckChatUsernameResultPublicGroupsUnavailableBuilder {
  inner: CheckChatUsernameResultPublicGroupsUnavailable
}

impl RTDCheckChatUsernameResultPublicGroupsUnavailableBuilder {
  pub fn build(&self) -> CheckChatUsernameResultPublicGroupsUnavailable { self.inner.clone() }

}

impl AsRef<CheckChatUsernameResultPublicGroupsUnavailable> for CheckChatUsernameResultPublicGroupsUnavailable {
  fn as_ref(&self) -> &CheckChatUsernameResultPublicGroupsUnavailable { self }
}

impl AsRef<CheckChatUsernameResultPublicGroupsUnavailable> for RTDCheckChatUsernameResultPublicGroupsUnavailableBuilder {
  fn as_ref(&self) -> &CheckChatUsernameResultPublicGroupsUnavailable { &self.inner }
}







/// The username is invalid
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckChatUsernameResultUsernameInvalid {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for CheckChatUsernameResultUsernameInvalid {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkChatUsernameResultUsernameInvalid" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDCheckChatUsernameResult for CheckChatUsernameResultUsernameInvalid {}



impl CheckChatUsernameResultUsernameInvalid {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCheckChatUsernameResultUsernameInvalidBuilder {
    let mut inner = CheckChatUsernameResultUsernameInvalid::default();
    inner.td_name = "checkChatUsernameResultUsernameInvalid".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCheckChatUsernameResultUsernameInvalidBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDCheckChatUsernameResultUsernameInvalidBuilder {
  inner: CheckChatUsernameResultUsernameInvalid
}

impl RTDCheckChatUsernameResultUsernameInvalidBuilder {
  pub fn build(&self) -> CheckChatUsernameResultUsernameInvalid { self.inner.clone() }

}

impl AsRef<CheckChatUsernameResultUsernameInvalid> for CheckChatUsernameResultUsernameInvalid {
  fn as_ref(&self) -> &CheckChatUsernameResultUsernameInvalid { self }
}

impl AsRef<CheckChatUsernameResultUsernameInvalid> for RTDCheckChatUsernameResultUsernameInvalidBuilder {
  fn as_ref(&self) -> &CheckChatUsernameResultUsernameInvalid { &self.inner }
}







/// The username is occupied
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckChatUsernameResultUsernameOccupied {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for CheckChatUsernameResultUsernameOccupied {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkChatUsernameResultUsernameOccupied" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDCheckChatUsernameResult for CheckChatUsernameResultUsernameOccupied {}



impl CheckChatUsernameResultUsernameOccupied {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCheckChatUsernameResultUsernameOccupiedBuilder {
    let mut inner = CheckChatUsernameResultUsernameOccupied::default();
    inner.td_name = "checkChatUsernameResultUsernameOccupied".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCheckChatUsernameResultUsernameOccupiedBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDCheckChatUsernameResultUsernameOccupiedBuilder {
  inner: CheckChatUsernameResultUsernameOccupied
}

impl RTDCheckChatUsernameResultUsernameOccupiedBuilder {
  pub fn build(&self) -> CheckChatUsernameResultUsernameOccupied { self.inner.clone() }

}

impl AsRef<CheckChatUsernameResultUsernameOccupied> for CheckChatUsernameResultUsernameOccupied {
  fn as_ref(&self) -> &CheckChatUsernameResultUsernameOccupied { self }
}

impl AsRef<CheckChatUsernameResultUsernameOccupied> for RTDCheckChatUsernameResultUsernameOccupiedBuilder {
  fn as_ref(&self) -> &CheckChatUsernameResultUsernameOccupied { &self.inner }
}



