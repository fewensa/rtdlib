use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a user shared with a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SharedUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier
  user_id: i64,
  /// First name of the user; for bots only
  first_name: String,
  /// Last name of the user; for bots only
  last_name: String,
  /// Username of the user; for bots only
  username: String,
  /// Profile photo of the user; for bots only; may be null
  photo: Option<Photo>,
}

impl RObject for SharedUser {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sharedUser"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl SharedUser {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSharedUserBuilder {
    let inner = SharedUser {
      td_name: "sharedUser".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSharedUserBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn first_name(&self) -> &String {
    &self.first_name
  }

  pub fn last_name(&self) -> &String {
    &self.last_name
  }

  pub fn username(&self) -> &String {
    &self.username
  }

  pub fn photo(&self) -> &Option<Photo> {
    &self.photo
  }
}

#[doc(hidden)]
pub struct RTDSharedUserBuilder {
  inner: SharedUser,
}

impl RTDSharedUserBuilder {
  pub fn build(&self) -> SharedUser {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn first_name<T: AsRef<str>>(&mut self, first_name: T) -> &mut Self {
    self.inner.first_name = first_name.as_ref().to_string();
    self
  }

  pub fn last_name<T: AsRef<str>>(&mut self, last_name: T) -> &mut Self {
    self.inner.last_name = last_name.as_ref().to_string();
    self
  }

  pub fn username<T: AsRef<str>>(&mut self, username: T) -> &mut Self {
    self.inner.username = username.as_ref().to_string();
    self
  }

  pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = Some(photo.as_ref().clone());
    self
  }
}

impl AsRef<SharedUser> for SharedUser {
  fn as_ref(&self) -> &SharedUser {
    self
  }
}

impl AsRef<SharedUser> for RTDSharedUserBuilder {
  fn as_ref(&self) -> &SharedUser {
    &self.inner
  }
}
