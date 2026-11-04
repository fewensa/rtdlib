use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes settings for greeting messages that are automatically sent by a Telegram Business account as response to incoming messages in an inactive private chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessGreetingMessageSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique quick reply shortcut identifier for the greeting messages
  shortcut_id: i64,
  /// Chosen recipients of the greeting messages
  recipients: BusinessRecipients,
  /// The number of days after which a chat will be considered as inactive; currently, must be one of 7, 14, 21, or 28
  inactivity_days: i64,
}

impl RObject for BusinessGreetingMessageSettings {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessGreetingMessageSettings"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BusinessGreetingMessageSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessGreetingMessageSettingsBuilder {
    let inner = BusinessGreetingMessageSettings {
      td_name: "businessGreetingMessageSettings".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBusinessGreetingMessageSettingsBuilder { inner }
  }

  pub fn shortcut_id(&self) -> i64 {
    self.shortcut_id
  }

  pub fn recipients(&self) -> &BusinessRecipients {
    &self.recipients
  }

  pub fn inactivity_days(&self) -> i64 {
    self.inactivity_days
  }
}

#[doc(hidden)]
pub struct RTDBusinessGreetingMessageSettingsBuilder {
  inner: BusinessGreetingMessageSettings,
}

impl RTDBusinessGreetingMessageSettingsBuilder {
  pub fn build(&self) -> BusinessGreetingMessageSettings {
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

  pub fn inactivity_days(&mut self, inactivity_days: i64) -> &mut Self {
    self.inner.inactivity_days = inactivity_days;
    self
  }
}

impl AsRef<BusinessGreetingMessageSettings> for BusinessGreetingMessageSettings {
  fn as_ref(&self) -> &BusinessGreetingMessageSettings {
    self
  }
}

impl AsRef<BusinessGreetingMessageSettings> for RTDBusinessGreetingMessageSettingsBuilder {
  fn as_ref(&self) -> &BusinessGreetingMessageSettings {
    &self.inner
  }
}
