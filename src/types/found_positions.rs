use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains 0-based positions of matched objects
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoundPositions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Total number of matched objects
  total_count: i64,
  /// The positions of the matched objects
  positions: Vec<i64>,
}

impl RObject for FoundPositions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "foundPositions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl FoundPositions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFoundPositionsBuilder {
    let inner = FoundPositions {
      td_name: "foundPositions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDFoundPositionsBuilder { inner }
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }

  pub fn positions(&self) -> &Vec<i64> {
    &self.positions
  }
}

#[doc(hidden)]
pub struct RTDFoundPositionsBuilder {
  inner: FoundPositions,
}

impl RTDFoundPositionsBuilder {
  pub fn build(&self) -> FoundPositions {
    self.inner.clone()
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

  pub fn positions(&mut self, positions: Vec<i64>) -> &mut Self {
    self.inner.positions = positions;
    self
  }
}

impl AsRef<FoundPositions> for FoundPositions {
  fn as_ref(&self) -> &FoundPositions {
    self
  }
}

impl AsRef<FoundPositions> for RTDFoundPositionsBuilder {
  fn as_ref(&self) -> &FoundPositions {
    &self.inner
  }
}
