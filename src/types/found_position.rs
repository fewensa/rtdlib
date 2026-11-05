use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains 0-based match position
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoundPosition {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The position of the match
  position: i64,
}

impl RObject for FoundPosition {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "foundPosition"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl FoundPosition {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFoundPositionBuilder {
    let inner = FoundPosition {
      td_name: "foundPosition".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDFoundPositionBuilder { inner }
  }

  pub fn position(&self) -> i64 {
    self.position
  }
}

#[doc(hidden)]
pub struct RTDFoundPositionBuilder {
  inner: FoundPosition,
}

impl RTDFoundPositionBuilder {
  pub fn build(&self) -> FoundPosition {
    self.inner.clone()
  }

  pub fn position(&mut self, position: i64) -> &mut Self {
    self.inner.position = position;
    self
  }
}

impl AsRef<FoundPosition> for FoundPosition {
  fn as_ref(&self) -> &FoundPosition {
    self
  }
}

impl AsRef<FoundPosition> for RTDFoundPositionBuilder {
  fn as_ref(&self) -> &FoundPosition {
    &self.inner
  }
}
