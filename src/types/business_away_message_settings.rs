use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes settings for messages that are automatically sent by a Telegram Business account when it is away
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessAwayMessageSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique quick reply shortcut identifier for the away messages
  shortcut_id: i64,
  /// Chosen recipients of the away messages
  recipients: BusinessRecipients,
  /// Settings used to check whether the current user is away
  schedule: BusinessAwayMessageSchedule,
  /// True, if the messages must not be sent if the account was online in the last 10 minutes
  offline_only: bool,
}

impl RObject for BusinessAwayMessageSettings {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessAwayMessageSettings"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BusinessAwayMessageSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessAwayMessageSettingsBuilder {
    let inner = BusinessAwayMessageSettings {
      td_name: "businessAwayMessageSettings".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBusinessAwayMessageSettingsBuilder { inner }
  }

  pub fn shortcut_id(&self) -> i64 {
    self.shortcut_id
  }

  pub fn recipients(&self) -> &BusinessRecipients {
    &self.recipients
  }

  pub fn schedule(&self) -> &BusinessAwayMessageSchedule {
    &self.schedule
  }

  pub fn offline_only(&self) -> bool {
    self.offline_only
  }
}

#[doc(hidden)]
pub struct RTDBusinessAwayMessageSettingsBuilder {
  inner: BusinessAwayMessageSettings,
}

impl RTDBusinessAwayMessageSettingsBuilder {
  pub fn build(&self) -> BusinessAwayMessageSettings {
    self.inner.clone()
  }

  pub fn shortcut_id(&mut self, shortcut_id: i64) -> &mut Self {
    self.inner.shortcut_id = shortcut_id;
    self
  }

  pub fn recipients<T: AsRef<BusinessRecipients>>(&mut self, recipients: T) -> &mut Self {
    self.inner.recipients = recipients.as_ref().clone();
    self
  }

  pub fn schedule<T: AsRef<BusinessAwayMessageSchedule>>(&mut self, schedule: T) -> &mut Self {
    self.inner.schedule = schedule.as_ref().clone();
    self
  }

  pub fn offline_only(&mut self, offline_only: bool) -> &mut Self {
    self.inner.offline_only = offline_only;
    self
  }
}

impl AsRef<BusinessAwayMessageSettings> for BusinessAwayMessageSettings {
  fn as_ref(&self) -> &BusinessAwayMessageSettings {
    self
  }
}

impl AsRef<BusinessAwayMessageSettings> for RTDBusinessAwayMessageSettingsBuilder {
  fn as_ref(&self) -> &BusinessAwayMessageSettings {
    &self.inner
  }
}
