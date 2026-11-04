use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a closed vector path. The path begins at the end point of the last command. The coordinate system origin is in the upper-left corner
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClosedVectorPath {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of vector path commands
  commands: Vec<VectorPathCommand>,
}

impl RObject for ClosedVectorPath {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "closedVectorPath"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ClosedVectorPath {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDClosedVectorPathBuilder {
    let inner = ClosedVectorPath {
      td_name: "closedVectorPath".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDClosedVectorPathBuilder { inner }
  }

  pub fn commands(&self) -> &Vec<VectorPathCommand> {
    &self.commands
  }
}

#[doc(hidden)]
pub struct RTDClosedVectorPathBuilder {
  inner: ClosedVectorPath,
}

impl RTDClosedVectorPathBuilder {
  pub fn build(&self) -> ClosedVectorPath {
    self.inner.clone()
  }

  pub fn commands(&mut self, commands: Vec<VectorPathCommand>) -> &mut Self {
    self.inner.commands = commands;
    self
  }
}

impl AsRef<ClosedVectorPath> for ClosedVectorPath {
  fn as_ref(&self) -> &ClosedVectorPath {
    self
  }
}

impl AsRef<ClosedVectorPath> for RTDClosedVectorPathBuilder {
  fn as_ref(&self) -> &ClosedVectorPath {
    &self.inner
  }
}
