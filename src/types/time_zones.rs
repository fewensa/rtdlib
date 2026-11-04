use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of time zones
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimeZones {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A list of time zones
  time_zones: Vec<TimeZone>,
}

impl RObject for TimeZones {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "timeZones"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TimeZones {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTimeZonesBuilder {
    let inner = TimeZones {
      td_name: "timeZones".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTimeZonesBuilder { inner }
  }

  pub fn time_zones(&self) -> &Vec<TimeZone> {
    &self.time_zones
  }
}

#[doc(hidden)]
pub struct RTDTimeZonesBuilder {
  inner: TimeZones,
}

impl RTDTimeZonesBuilder {
  pub fn build(&self) -> TimeZones {
    self.inner.clone()
  }

  pub fn time_zones(&mut self, time_zones: Vec<TimeZone>) -> &mut Self {
    self.inner.time_zones = time_zones;
    self
  }
}

impl AsRef<TimeZones> for TimeZones {
  fn as_ref(&self) -> &TimeZones {
    self
  }
}

impl AsRef<TimeZones> for RTDTimeZonesBuilder {
  fn as_ref(&self) -> &TimeZones {
    &self.inner
  }
}
