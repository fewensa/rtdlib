use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains privacy settings for message read date in private chats. Read dates are always shown to the users that can see online status of the current user regardless of this setting
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReadDatePrivacySettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if message read date is shown to other users in private chats. If false and the current user isn't a Telegram Premium user, then they will not be able to see others' message read date
  show_read_date: bool,
}

impl RObject for ReadDatePrivacySettings {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "readDatePrivacySettings"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ReadDatePrivacySettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReadDatePrivacySettingsBuilder {
    let inner = ReadDatePrivacySettings {
      td_name: "readDatePrivacySettings".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDReadDatePrivacySettingsBuilder { inner }
  }

  pub fn show_read_date(&self) -> bool {
    self.show_read_date
  }
}

#[doc(hidden)]
pub struct RTDReadDatePrivacySettingsBuilder {
  inner: ReadDatePrivacySettings,
}

impl RTDReadDatePrivacySettingsBuilder {
  pub fn build(&self) -> ReadDatePrivacySettings {
    self.inner.clone()
  }

  pub fn show_read_date(&mut self, show_read_date: bool) -> &mut Self {
    self.inner.show_read_date = show_read_date;
    self
  }
}

impl AsRef<ReadDatePrivacySettings> for ReadDatePrivacySettings {
  fn as_ref(&self) -> &ReadDatePrivacySettings {
    self
  }
}

impl AsRef<ReadDatePrivacySettings> for RTDReadDatePrivacySettingsBuilder {
  fn as_ref(&self) -> &ReadDatePrivacySettings {
    &self.inner
  }
}
