use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a user who had or will have a birthday soon
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CloseBirthdayUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier
  user_id: i64,
  /// Birthdate of the user
  birthdate: Birthdate,
}

impl RObject for CloseBirthdayUser {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "closeBirthdayUser"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl CloseBirthdayUser {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCloseBirthdayUserBuilder {
    let inner = CloseBirthdayUser {
      td_name: "closeBirthdayUser".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCloseBirthdayUserBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn birthdate(&self) -> &Birthdate {
    &self.birthdate
  }
}

#[doc(hidden)]
pub struct RTDCloseBirthdayUserBuilder {
  inner: CloseBirthdayUser,
}

impl RTDCloseBirthdayUserBuilder {
  pub fn build(&self) -> CloseBirthdayUser {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn birthdate<T: AsRef<Birthdate>>(&mut self, birthdate: T) -> &mut Self {
    self.inner.birthdate = birthdate.as_ref().clone();
    self
  }
}

impl AsRef<CloseBirthdayUser> for CloseBirthdayUser {
  fn as_ref(&self) -> &CloseBirthdayUser {
    self
  }
}

impl AsRef<CloseBirthdayUser> for RTDCloseBirthdayUserBuilder {
  fn as_ref(&self) -> &CloseBirthdayUser {
    &self.inner
  }
}
