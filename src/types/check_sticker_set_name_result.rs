
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Represents result of checking whether a name can be used for a new sticker set
pub trait TDCheckStickerSetNameResult: Debug + RObject {}

/// Represents result of checking whether a name can be used for a new sticker set
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CheckStickerSetNameResult {
  #[doc(hidden)] _Default(()),
  /// Checks whether a name can be used for a new sticker set
  CheckStickerSetName(CheckStickerSetName),
  /// The name is invalid
  NameInvalid(CheckStickerSetNameResultNameInvalid),
  /// The name is occupied
  NameOccupied(CheckStickerSetNameResultNameOccupied),
  /// The name can be set
  Ok(CheckStickerSetNameResultOk),

}

impl Default for CheckStickerSetNameResult {
  fn default() -> Self { CheckStickerSetNameResult::_Default(()) }
}

impl<'de> Deserialize<'de> for CheckStickerSetNameResult {
  fn deserialize<D>(deserializer: D) -> Result<CheckStickerSetNameResult, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      CheckStickerSetNameResult,
      (checkStickerSetName, CheckStickerSetName);
      (checkStickerSetNameResultNameInvalid, NameInvalid);
      (checkStickerSetNameResultNameOccupied, NameOccupied);
      (checkStickerSetNameResultOk, Ok);

    )(deserializer)
  }
}

impl RObject for CheckStickerSetNameResult {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      CheckStickerSetNameResult::CheckStickerSetName(t) => t.td_name(),
      CheckStickerSetNameResult::NameInvalid(t) => t.td_name(),
      CheckStickerSetNameResult::NameOccupied(t) => t.td_name(),
      CheckStickerSetNameResult::Ok(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      CheckStickerSetNameResult::CheckStickerSetName(t) => t.extra(),
      CheckStickerSetNameResult::NameInvalid(t) => t.extra(),
      CheckStickerSetNameResult::NameOccupied(t) => t.extra(),
      CheckStickerSetNameResult::Ok(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl CheckStickerSetNameResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let CheckStickerSetNameResult::_Default(_) = self { true } else { false } }

  pub fn is_check_sticker_set_name(&self) -> bool { if let CheckStickerSetNameResult::CheckStickerSetName(_) = self { true } else { false } }
  pub fn is_name_invalid(&self) -> bool { if let CheckStickerSetNameResult::NameInvalid(_) = self { true } else { false } }
  pub fn is_name_occupied(&self) -> bool { if let CheckStickerSetNameResult::NameOccupied(_) = self { true } else { false } }
  pub fn is_ok(&self) -> bool { if let CheckStickerSetNameResult::Ok(_) = self { true } else { false } }

  pub fn on_check_sticker_set_name<F: FnOnce(&CheckStickerSetName)>(&self, fnc: F) -> &Self { if let CheckStickerSetNameResult::CheckStickerSetName(t) = self { fnc(t) }; self }
  pub fn on_name_invalid<F: FnOnce(&CheckStickerSetNameResultNameInvalid)>(&self, fnc: F) -> &Self { if let CheckStickerSetNameResult::NameInvalid(t) = self { fnc(t) }; self }
  pub fn on_name_occupied<F: FnOnce(&CheckStickerSetNameResultNameOccupied)>(&self, fnc: F) -> &Self { if let CheckStickerSetNameResult::NameOccupied(t) = self { fnc(t) }; self }
  pub fn on_ok<F: FnOnce(&CheckStickerSetNameResultOk)>(&self, fnc: F) -> &Self { if let CheckStickerSetNameResult::Ok(t) = self { fnc(t) }; self }

  pub fn as_check_sticker_set_name(&self) -> Option<&CheckStickerSetName> { if let CheckStickerSetNameResult::CheckStickerSetName(t) = self { return Some(t) } None }
  pub fn as_name_invalid(&self) -> Option<&CheckStickerSetNameResultNameInvalid> { if let CheckStickerSetNameResult::NameInvalid(t) = self { return Some(t) } None }
  pub fn as_name_occupied(&self) -> Option<&CheckStickerSetNameResultNameOccupied> { if let CheckStickerSetNameResult::NameOccupied(t) = self { return Some(t) } None }
  pub fn as_ok(&self) -> Option<&CheckStickerSetNameResultOk> { if let CheckStickerSetNameResult::Ok(t) = self { return Some(t) } None }



  pub fn check_sticker_set_name<T: AsRef<CheckStickerSetName>>(t: T) -> Self { CheckStickerSetNameResult::CheckStickerSetName(t.as_ref().clone()) }

  pub fn name_invalid<T: AsRef<CheckStickerSetNameResultNameInvalid>>(t: T) -> Self { CheckStickerSetNameResult::NameInvalid(t.as_ref().clone()) }

  pub fn name_occupied<T: AsRef<CheckStickerSetNameResultNameOccupied>>(t: T) -> Self { CheckStickerSetNameResult::NameOccupied(t.as_ref().clone()) }

  pub fn ok<T: AsRef<CheckStickerSetNameResultOk>>(t: T) -> Self { CheckStickerSetNameResult::Ok(t.as_ref().clone()) }

}

impl AsRef<CheckStickerSetNameResult> for CheckStickerSetNameResult {
  fn as_ref(&self) -> &CheckStickerSetNameResult { self }
}







/// The name is invalid
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckStickerSetNameResultNameInvalid {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for CheckStickerSetNameResultNameInvalid {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkStickerSetNameResultNameInvalid" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDCheckStickerSetNameResult for CheckStickerSetNameResultNameInvalid {}



impl CheckStickerSetNameResultNameInvalid {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCheckStickerSetNameResultNameInvalidBuilder {
    let mut inner = CheckStickerSetNameResultNameInvalid::default();
    inner.td_name = "checkStickerSetNameResultNameInvalid".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCheckStickerSetNameResultNameInvalidBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDCheckStickerSetNameResultNameInvalidBuilder {
  inner: CheckStickerSetNameResultNameInvalid
}

impl RTDCheckStickerSetNameResultNameInvalidBuilder {
  pub fn build(&self) -> CheckStickerSetNameResultNameInvalid { self.inner.clone() }

}

impl AsRef<CheckStickerSetNameResultNameInvalid> for CheckStickerSetNameResultNameInvalid {
  fn as_ref(&self) -> &CheckStickerSetNameResultNameInvalid { self }
}

impl AsRef<CheckStickerSetNameResultNameInvalid> for RTDCheckStickerSetNameResultNameInvalidBuilder {
  fn as_ref(&self) -> &CheckStickerSetNameResultNameInvalid { &self.inner }
}







/// The name is occupied
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckStickerSetNameResultNameOccupied {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for CheckStickerSetNameResultNameOccupied {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkStickerSetNameResultNameOccupied" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDCheckStickerSetNameResult for CheckStickerSetNameResultNameOccupied {}



impl CheckStickerSetNameResultNameOccupied {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCheckStickerSetNameResultNameOccupiedBuilder {
    let mut inner = CheckStickerSetNameResultNameOccupied::default();
    inner.td_name = "checkStickerSetNameResultNameOccupied".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCheckStickerSetNameResultNameOccupiedBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDCheckStickerSetNameResultNameOccupiedBuilder {
  inner: CheckStickerSetNameResultNameOccupied
}

impl RTDCheckStickerSetNameResultNameOccupiedBuilder {
  pub fn build(&self) -> CheckStickerSetNameResultNameOccupied { self.inner.clone() }

}

impl AsRef<CheckStickerSetNameResultNameOccupied> for CheckStickerSetNameResultNameOccupied {
  fn as_ref(&self) -> &CheckStickerSetNameResultNameOccupied { self }
}

impl AsRef<CheckStickerSetNameResultNameOccupied> for RTDCheckStickerSetNameResultNameOccupiedBuilder {
  fn as_ref(&self) -> &CheckStickerSetNameResultNameOccupied { &self.inner }
}







/// The name can be set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckStickerSetNameResultOk {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for CheckStickerSetNameResultOk {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkStickerSetNameResultOk" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDCheckStickerSetNameResult for CheckStickerSetNameResultOk {}



impl CheckStickerSetNameResultOk {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCheckStickerSetNameResultOkBuilder {
    let mut inner = CheckStickerSetNameResultOk::default();
    inner.td_name = "checkStickerSetNameResultOk".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCheckStickerSetNameResultOkBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDCheckStickerSetNameResultOkBuilder {
  inner: CheckStickerSetNameResultOk
}

impl RTDCheckStickerSetNameResultOkBuilder {
  pub fn build(&self) -> CheckStickerSetNameResultOk { self.inner.clone() }

}

impl AsRef<CheckStickerSetNameResultOk> for CheckStickerSetNameResultOk {
  fn as_ref(&self) -> &CheckStickerSetNameResultOk { self }
}

impl AsRef<CheckStickerSetNameResultOk> for RTDCheckStickerSetNameResultOkBuilder {
  fn as_ref(&self) -> &CheckStickerSetNameResultOk { &self.inner }
}



