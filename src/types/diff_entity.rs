use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a change of a text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiffEntity {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Offset of the entity, in UTF-16 code units
  offset: i64,
  /// Length of the entity, in UTF-16 code units
  length: i64,
  /// Type of the entity
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: DiffEntityType,
}

impl RObject for DiffEntity {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "diffEntity"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl DiffEntity {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDiffEntityBuilder {
    let inner = DiffEntity {
      td_name: "diffEntity".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDDiffEntityBuilder { inner }
  }

  pub fn offset(&self) -> i64 {
    self.offset
  }

  pub fn length(&self) -> i64 {
    self.length
  }

  pub fn type_(&self) -> &DiffEntityType {
    &self.type_
  }
}

#[doc(hidden)]
pub struct RTDDiffEntityBuilder {
  inner: DiffEntity,
}

impl RTDDiffEntityBuilder {
  pub fn build(&self) -> DiffEntity {
    self.inner.clone()
  }

  pub fn offset(&mut self, offset: i64) -> &mut Self {
    self.inner.offset = offset;
    self
  }

  pub fn length(&mut self, length: i64) -> &mut Self {
    self.inner.length = length;
    self
  }

  pub fn type_<T: AsRef<DiffEntityType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }
}

impl AsRef<DiffEntity> for DiffEntity {
  fn as_ref(&self) -> &DiffEntity {
    self
  }
}

impl AsRef<DiffEntity> for RTDDiffEntityBuilder {
  fn as_ref(&self) -> &DiffEntity {
    &self.inner
  }
}
