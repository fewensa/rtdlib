use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Represents a change of a text
pub trait TDDiffEntityType: Debug + RObject {}

/// Represents a change of a text
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum DiffEntityType {
  #[doc(hidden)]
  _Default(()),
  /// Removal of some text
  Delete(DiffEntityTypeDelete),
  /// Addition of some text
  Insert(DiffEntityTypeInsert),
  /// Change of some text
  Replace(DiffEntityTypeReplace),
}

impl Default for DiffEntityType {
  fn default() -> Self {
    DiffEntityType::_Default(())
  }
}

impl<'de> Deserialize<'de> for DiffEntityType {
  fn deserialize<D>(deserializer: D) -> Result<DiffEntityType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      DiffEntityType,
      (diffEntityTypeDelete, Delete);
      (diffEntityTypeInsert, Insert);
      (diffEntityTypeReplace, Replace);

    )(deserializer)
  }
}

impl RObject for DiffEntityType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      DiffEntityType::Delete(t) => t.td_name(),
      DiffEntityType::Insert(t) => t.td_name(),
      DiffEntityType::Replace(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      DiffEntityType::Delete(t) => t.extra(),
      DiffEntityType::Insert(t) => t.extra(),
      DiffEntityType::Replace(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl DiffEntityType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, DiffEntityType::_Default(_))
  }

  pub fn is_delete(&self) -> bool {
    matches!(self, DiffEntityType::Delete(_))
  }
  pub fn is_insert(&self) -> bool {
    matches!(self, DiffEntityType::Insert(_))
  }
  pub fn is_replace(&self) -> bool {
    matches!(self, DiffEntityType::Replace(_))
  }

  pub fn on_delete<F: FnOnce(&DiffEntityTypeDelete)>(&self, fnc: F) -> &Self {
    if let DiffEntityType::Delete(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_insert<F: FnOnce(&DiffEntityTypeInsert)>(&self, fnc: F) -> &Self {
    if let DiffEntityType::Insert(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_replace<F: FnOnce(&DiffEntityTypeReplace)>(&self, fnc: F) -> &Self {
    if let DiffEntityType::Replace(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_delete(&self) -> Option<&DiffEntityTypeDelete> {
    if let DiffEntityType::Delete(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_insert(&self) -> Option<&DiffEntityTypeInsert> {
    if let DiffEntityType::Insert(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_replace(&self) -> Option<&DiffEntityTypeReplace> {
    if let DiffEntityType::Replace(t) = self {
      return Some(t);
    }
    None
  }

  pub fn delete<T: AsRef<DiffEntityTypeDelete>>(t: T) -> Self {
    DiffEntityType::Delete(t.as_ref().clone())
  }

  pub fn insert<T: AsRef<DiffEntityTypeInsert>>(t: T) -> Self {
    DiffEntityType::Insert(t.as_ref().clone())
  }

  pub fn replace<T: AsRef<DiffEntityTypeReplace>>(t: T) -> Self {
    DiffEntityType::Replace(t.as_ref().clone())
  }
}

impl AsRef<DiffEntityType> for DiffEntityType {
  fn as_ref(&self) -> &DiffEntityType {
    self
  }
}

/// Removal of some text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiffEntityTypeDelete {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for DiffEntityTypeDelete {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "diffEntityTypeDelete"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDDiffEntityType for DiffEntityTypeDelete {}

impl DiffEntityTypeDelete {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDiffEntityTypeDeleteBuilder {
    let inner = DiffEntityTypeDelete {
      td_name: "diffEntityTypeDelete".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDDiffEntityTypeDeleteBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDDiffEntityTypeDeleteBuilder {
  inner: DiffEntityTypeDelete,
}

impl RTDDiffEntityTypeDeleteBuilder {
  pub fn build(&self) -> DiffEntityTypeDelete {
    self.inner.clone()
  }
}

impl AsRef<DiffEntityTypeDelete> for DiffEntityTypeDelete {
  fn as_ref(&self) -> &DiffEntityTypeDelete {
    self
  }
}

impl AsRef<DiffEntityTypeDelete> for RTDDiffEntityTypeDeleteBuilder {
  fn as_ref(&self) -> &DiffEntityTypeDelete {
    &self.inner
  }
}

/// Addition of some text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiffEntityTypeInsert {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for DiffEntityTypeInsert {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "diffEntityTypeInsert"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDDiffEntityType for DiffEntityTypeInsert {}

impl DiffEntityTypeInsert {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDiffEntityTypeInsertBuilder {
    let inner = DiffEntityTypeInsert {
      td_name: "diffEntityTypeInsert".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDDiffEntityTypeInsertBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDDiffEntityTypeInsertBuilder {
  inner: DiffEntityTypeInsert,
}

impl RTDDiffEntityTypeInsertBuilder {
  pub fn build(&self) -> DiffEntityTypeInsert {
    self.inner.clone()
  }
}

impl AsRef<DiffEntityTypeInsert> for DiffEntityTypeInsert {
  fn as_ref(&self) -> &DiffEntityTypeInsert {
    self
  }
}

impl AsRef<DiffEntityTypeInsert> for RTDDiffEntityTypeInsertBuilder {
  fn as_ref(&self) -> &DiffEntityTypeInsert {
    &self.inner
  }
}

/// Change of some text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiffEntityTypeReplace {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The old text
  old_text: String,
}

impl RObject for DiffEntityTypeReplace {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "diffEntityTypeReplace"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDDiffEntityType for DiffEntityTypeReplace {}

impl DiffEntityTypeReplace {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDiffEntityTypeReplaceBuilder {
    let inner = DiffEntityTypeReplace {
      td_name: "diffEntityTypeReplace".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDDiffEntityTypeReplaceBuilder { inner }
  }

  pub fn old_text(&self) -> &String {
    &self.old_text
  }
}

#[doc(hidden)]
pub struct RTDDiffEntityTypeReplaceBuilder {
  inner: DiffEntityTypeReplace,
}

impl RTDDiffEntityTypeReplaceBuilder {
  pub fn build(&self) -> DiffEntityTypeReplace {
    self.inner.clone()
  }

  pub fn old_text<T: AsRef<str>>(&mut self, old_text: T) -> &mut Self {
    self.inner.old_text = old_text.as_ref().to_string();
    self
  }
}

impl AsRef<DiffEntityTypeReplace> for DiffEntityTypeReplace {
  fn as_ref(&self) -> &DiffEntityTypeReplace {
    self
  }
}

impl AsRef<DiffEntityTypeReplace> for RTDDiffEntityTypeReplaceBuilder {
  fn as_ref(&self) -> &DiffEntityTypeReplace {
    &self.inner
  }
}
