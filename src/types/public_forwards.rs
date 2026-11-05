use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of public forwards and reposts as a story of a message or a story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PublicForwards {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Approximate total number of messages and stories found
  total_count: i64,
  /// List of found public forwards and reposts
  forwards: Vec<PublicForward>,
  /// The offset for the next request. If empty, then there are no more results
  next_offset: String,
}

impl RObject for PublicForwards {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "publicForwards"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PublicForwards {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPublicForwardsBuilder {
    let inner = PublicForwards {
      td_name: "publicForwards".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPublicForwardsBuilder { inner }
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }

  pub fn forwards(&self) -> &Vec<PublicForward> {
    &self.forwards
  }

  pub fn next_offset(&self) -> &String {
    &self.next_offset
  }
}

#[doc(hidden)]
pub struct RTDPublicForwardsBuilder {
  inner: PublicForwards,
}

impl RTDPublicForwardsBuilder {
  pub fn build(&self) -> PublicForwards {
    self.inner.clone()
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

  pub fn forwards(&mut self, forwards: Vec<PublicForward>) -> &mut Self {
    self.inner.forwards = forwards;
    self
  }

  pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
    self.inner.next_offset = next_offset.as_ref().to_string();
    self
  }
}

impl AsRef<PublicForwards> for PublicForwards {
  fn as_ref(&self) -> &PublicForwards {
    self
  }
}

impl AsRef<PublicForwards> for RTDPublicForwardsBuilder {
  fn as_ref(&self) -> &PublicForwards {
    &self.inner
  }
}
