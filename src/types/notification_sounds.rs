use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of notification sounds
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationSounds {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A list of notification sounds
  notification_sounds: Vec<NotificationSound>,
}

impl RObject for NotificationSounds {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "notificationSounds"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl NotificationSounds {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDNotificationSoundsBuilder {
    let inner = NotificationSounds {
      td_name: "notificationSounds".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDNotificationSoundsBuilder { inner }
  }

  pub fn notification_sounds(&self) -> &Vec<NotificationSound> {
    &self.notification_sounds
  }
}

#[doc(hidden)]
pub struct RTDNotificationSoundsBuilder {
  inner: NotificationSounds,
}

impl RTDNotificationSoundsBuilder {
  pub fn build(&self) -> NotificationSounds {
    self.inner.clone()
  }

  pub fn notification_sounds(&mut self, notification_sounds: Vec<NotificationSound>) -> &mut Self {
    self.inner.notification_sounds = notification_sounds;
    self
  }
}

impl AsRef<NotificationSounds> for NotificationSounds {
  fn as_ref(&self) -> &NotificationSounds {
    self
  }
}

impl AsRef<NotificationSounds> for RTDNotificationSoundsBuilder {
  fn as_ref(&self) -> &NotificationSounds {
    &self.inner
  }
}
