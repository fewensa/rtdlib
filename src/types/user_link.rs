use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains an HTTPS URL, which can be used to get information about a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The URL
  url: String,
  /// Left time for which the link is valid, in seconds; 0 if the link is permanent
  expires_in: i64,
  /// True, if a permanent link to the current user can be received; false if all links to the current user expire
  can_get_permanent_link: bool,
}

impl RObject for UserLink {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userLink"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UserLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserLinkBuilder {
    let inner = UserLink {
      td_name: "userLink".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUserLinkBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }

  pub fn expires_in(&self) -> i64 {
    self.expires_in
  }

  pub fn can_get_permanent_link(&self) -> bool {
    self.can_get_permanent_link
  }
}

#[doc(hidden)]
pub struct RTDUserLinkBuilder {
  inner: UserLink,
}

impl RTDUserLinkBuilder {
  pub fn build(&self) -> UserLink {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

  pub fn expires_in(&mut self, expires_in: i64) -> &mut Self {
    self.inner.expires_in = expires_in;
    self
  }

  pub fn can_get_permanent_link(&mut self, can_get_permanent_link: bool) -> &mut Self {
    self.inner.can_get_permanent_link = can_get_permanent_link;
    self
  }
}

impl AsRef<UserLink> for UserLink {
  fn as_ref(&self) -> &UserLink {
    self
  }
}

impl AsRef<UserLink> for RTDUserLinkBuilder {
  fn as_ref(&self) -> &UserLink {
    &self.inner
  }
}
