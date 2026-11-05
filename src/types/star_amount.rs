use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a possibly non-integer Telegram Star amount
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarAmount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The integer Telegram Star amount rounded to 0
  star_count: i64,
  /// The number of 1/1000000000 shares of Telegram Stars; from 999999999 to 999999999
  nanostar_count: i64,
}

impl RObject for StarAmount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starAmount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StarAmount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarAmountBuilder {
    let inner = StarAmount {
      td_name: "starAmount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarAmountBuilder { inner }
  }

  pub fn star_count(&self) -> i64 {
    self.star_count
  }

  pub fn nanostar_count(&self) -> i64 {
    self.nanostar_count
  }
}

#[doc(hidden)]
pub struct RTDStarAmountBuilder {
  inner: StarAmount,
}

impl RTDStarAmountBuilder {
  pub fn build(&self) -> StarAmount {
    self.inner.clone()
  }

  pub fn star_count(&mut self, star_count: i64) -> &mut Self {
    self.inner.star_count = star_count;
    self
  }

  pub fn nanostar_count(&mut self, nanostar_count: i64) -> &mut Self {
    self.inner.nanostar_count = nanostar_count;
    self
  }
}

impl AsRef<StarAmount> for StarAmount {
  fn as_ref(&self) -> &StarAmount {
    self
  }
}

impl AsRef<StarAmount> for RTDStarAmountBuilder {
  fn as_ref(&self) -> &StarAmount {
    &self.inner
  }
}
