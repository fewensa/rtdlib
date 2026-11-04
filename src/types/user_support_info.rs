use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains custom information about the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserSupportInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Information message
  message: FormattedText,
  /// Information author
  author: String,
  /// Information change date
  date: i64,
}

impl RObject for UserSupportInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userSupportInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UserSupportInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserSupportInfoBuilder {
    let inner = UserSupportInfo {
      td_name: "userSupportInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUserSupportInfoBuilder { inner }
  }

  pub fn message(&self) -> &FormattedText {
    &self.message
  }

  pub fn author(&self) -> &String {
    &self.author
  }

  pub fn date(&self) -> i64 {
    self.date
  }
}

#[doc(hidden)]
pub struct RTDUserSupportInfoBuilder {
  inner: UserSupportInfo,
}

impl RTDUserSupportInfoBuilder {
  pub fn build(&self) -> UserSupportInfo {
    self.inner.clone()
  }

  pub fn message<T: AsRef<FormattedText>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().clone();
    self
  }

  pub fn author<T: AsRef<str>>(&mut self, author: T) -> &mut Self {
    self.inner.author = author.as_ref().to_string();
    self
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }
}

impl AsRef<UserSupportInfo> for UserSupportInfo {
  fn as_ref(&self) -> &UserSupportInfo {
    self
  }
}

impl AsRef<UserSupportInfo> for RTDUserSupportInfoBuilder {
  fn as_ref(&self) -> &UserSupportInfo {
    &self.inner
  }
}
