use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents outline of an image
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Outline {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The list of closed vector paths
  paths: Vec<ClosedVectorPath>,
}

impl RObject for Outline {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "outline"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl Outline {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDOutlineBuilder {
    let inner = Outline {
      td_name: "outline".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDOutlineBuilder { inner }
  }

  pub fn paths(&self) -> &Vec<ClosedVectorPath> {
    &self.paths
  }
}

#[doc(hidden)]
pub struct RTDOutlineBuilder {
  inner: Outline,
}

impl RTDOutlineBuilder {
  pub fn build(&self) -> Outline {
    self.inner.clone()
  }

  pub fn paths(&mut self, paths: Vec<ClosedVectorPath>) -> &mut Self {
    self.inner.paths = paths;
    self
  }
}

impl AsRef<Outline> for Outline {
  fn as_ref(&self) -> &Outline {
    self
  }
}

impl AsRef<Outline> for RTDOutlineBuilder {
  fn as_ref(&self) -> &Outline {
    &self.inner
  }
}
