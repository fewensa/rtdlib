use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of found users
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoundUsers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifiers of the found users
  user_ids: Vec<i64>,
  /// The offset for the next request. If empty, then there are no more results
  next_offset: String,
}

impl RObject for FoundUsers {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "foundUsers"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl FoundUsers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFoundUsersBuilder {
    let inner = FoundUsers {
      td_name: "foundUsers".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDFoundUsersBuilder { inner }
  }

  pub fn user_ids(&self) -> &Vec<i64> {
    &self.user_ids
  }

  pub fn next_offset(&self) -> &String {
    &self.next_offset
  }
}

#[doc(hidden)]
pub struct RTDFoundUsersBuilder {
  inner: FoundUsers,
}

impl RTDFoundUsersBuilder {
  pub fn build(&self) -> FoundUsers {
    self.inner.clone()
  }

  pub fn user_ids(&mut self, user_ids: Vec<i64>) -> &mut Self {
    self.inner.user_ids = user_ids;
    self
  }

  pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
    self.inner.next_offset = next_offset.as_ref().to_string();
    self
  }
}

impl AsRef<FoundUsers> for FoundUsers {
  fn as_ref(&self) -> &FoundUsers {
    self
  }
}

impl AsRef<FoundUsers> for RTDFoundUsersBuilder {
  fn as_ref(&self) -> &FoundUsers {
    &self.inner
  }
}
