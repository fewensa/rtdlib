use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a time zone
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimeZone {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique time zone identifier
  id: String,
  /// Time zone name
  name: String,
  /// Current UTC time offset for the time zone
  utc_time_offset: i64,
}

impl RObject for TimeZone {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "timeZone"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TimeZone {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTimeZoneBuilder {
    let inner = TimeZone {
      td_name: "timeZone".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTimeZoneBuilder { inner }
  }

  pub fn id(&self) -> &String {
    &self.id
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn utc_time_offset(&self) -> i64 {
    self.utc_time_offset
  }
}

#[doc(hidden)]
pub struct RTDTimeZoneBuilder {
  inner: TimeZone,
}

impl RTDTimeZoneBuilder {
  pub fn build(&self) -> TimeZone {
    self.inner.clone()
  }

  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

  pub fn utc_time_offset(&mut self, utc_time_offset: i64) -> &mut Self {
    self.inner.utc_time_offset = utc_time_offset;
    self
  }
}

impl AsRef<TimeZone> for TimeZone {
  fn as_ref(&self) -> &TimeZone {
    self
  }
}

impl AsRef<TimeZone> for RTDTimeZoneBuilder {
  fn as_ref(&self) -> &TimeZone {
    &self.inner
  }
}
