use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a notification sound in MP3 format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationSound {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the notification sound
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  id: isize,
  /// Duration of the sound, in seconds
  duration: i64,
  /// Point in time (Unix timestamp) when the sound was created
  date: i64,
  /// Title of the notification sound
  title: String,
  /// Arbitrary data, defined while the sound was uploaded
  data: String,
  /// File containing the sound
  sound: File,
}

impl RObject for NotificationSound {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "notificationSound"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl NotificationSound {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDNotificationSoundBuilder {
    let inner = NotificationSound {
      td_name: "notificationSound".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDNotificationSoundBuilder { inner }
  }

  pub fn id(&self) -> isize {
    self.id
  }

  pub fn duration(&self) -> i64 {
    self.duration
  }

  pub fn date(&self) -> i64 {
    self.date
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn data(&self) -> &String {
    &self.data
  }

  pub fn sound(&self) -> &File {
    &self.sound
  }
}

#[doc(hidden)]
pub struct RTDNotificationSoundBuilder {
  inner: NotificationSound,
}

impl RTDNotificationSoundBuilder {
  pub fn build(&self) -> NotificationSound {
    self.inner.clone()
  }

  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

  pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
    self.inner.data = data.as_ref().to_string();
    self
  }

  pub fn sound<T: AsRef<File>>(&mut self, sound: T) -> &mut Self {
    self.inner.sound = sound.as_ref().clone();
    self
  }
}

impl AsRef<NotificationSound> for NotificationSound {
  fn as_ref(&self) -> &NotificationSound {
    self
  }
}

impl AsRef<NotificationSound> for RTDNotificationSoundBuilder {
  fn as_ref(&self) -> &NotificationSound {
    &self.inner
  }
}
