use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about public post search limits
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PublicPostSearchLimits {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Number of queries that can be sent daily for free
  daily_free_query_count: i64,
  /// Number of remaining free queries today
  remaining_free_query_count: i64,
  /// Amount of time till the next free query can be sent; 0 if it can be sent now
  next_free_query_in: i64,
  /// Number of Telegram Stars that must be paid for each non-free query
  star_count: i64,
  /// True, if the search for the specified query isn't charged
  is_current_query_free: bool,
}

impl RObject for PublicPostSearchLimits {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "publicPostSearchLimits"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PublicPostSearchLimits {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPublicPostSearchLimitsBuilder {
    let inner = PublicPostSearchLimits {
      td_name: "publicPostSearchLimits".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPublicPostSearchLimitsBuilder { inner }
  }

  pub fn daily_free_query_count(&self) -> i64 {
    self.daily_free_query_count
  }

  pub fn remaining_free_query_count(&self) -> i64 {
    self.remaining_free_query_count
  }

  pub fn next_free_query_in(&self) -> i64 {
    self.next_free_query_in
  }

  pub fn star_count(&self) -> i64 {
    self.star_count
  }

  pub fn is_current_query_free(&self) -> bool {
    self.is_current_query_free
  }
}

#[doc(hidden)]
pub struct RTDPublicPostSearchLimitsBuilder {
  inner: PublicPostSearchLimits,
}

impl RTDPublicPostSearchLimitsBuilder {
  pub fn build(&self) -> PublicPostSearchLimits {
    self.inner.clone()
  }

  pub fn daily_free_query_count(&mut self, daily_free_query_count: i64) -> &mut Self {
    self.inner.daily_free_query_count = daily_free_query_count;
    self
  }

  pub fn remaining_free_query_count(&mut self, remaining_free_query_count: i64) -> &mut Self {
    self.inner.remaining_free_query_count = remaining_free_query_count;
    self
  }

  pub fn next_free_query_in(&mut self, next_free_query_in: i64) -> &mut Self {
    self.inner.next_free_query_in = next_free_query_in;
    self
  }

  pub fn star_count(&mut self, star_count: i64) -> &mut Self {
    self.inner.star_count = star_count;
    self
  }

  pub fn is_current_query_free(&mut self, is_current_query_free: bool) -> &mut Self {
    self.inner.is_current_query_free = is_current_query_free;
    self
  }
}

impl AsRef<PublicPostSearchLimits> for PublicPostSearchLimits {
  fn as_ref(&self) -> &PublicPostSearchLimits {
    self
  }
}

impl AsRef<PublicPostSearchLimits> for RTDPublicPostSearchLimitsBuilder {
  fn as_ref(&self) -> &PublicPostSearchLimits {
    &self.inner
  }
}
