use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a number of Telegram Stars
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Number of Telegram Stars
  star_count: i64,
}

impl RObject for StarCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StarCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarCountBuilder {
    let inner = StarCount {
      td_name: "starCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarCountBuilder { inner }
  }

  pub fn star_count(&self) -> i64 {
    self.star_count
  }
}

#[doc(hidden)]
pub struct RTDStarCountBuilder {
  inner: StarCount,
}

impl RTDStarCountBuilder {
  pub fn build(&self) -> StarCount {
    self.inner.clone()
  }

  pub fn star_count(&mut self, star_count: i64) -> &mut Self {
    self.inner.star_count = star_count;
    self
  }
}

impl AsRef<StarCount> for StarCount {
  fn as_ref(&self) -> &StarCount {
    self
  }
}

impl AsRef<StarCount> for RTDStarCountBuilder {
  fn as_ref(&self) -> &StarCount {
    &self.inner
  }
}
