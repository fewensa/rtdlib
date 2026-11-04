use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an interval of time when the business is open
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessOpeningHoursInterval {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The minute's sequence number in a week, starting on Monday, marking the start of the time interval during which the business is open; 0-7*24*60
  start_minute: i64,
  /// The minute's sequence number in a week, starting on Monday, marking the end of the time interval during which the business is open; 1-8*24*60
  end_minute: i64,
}

impl RObject for BusinessOpeningHoursInterval {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessOpeningHoursInterval"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BusinessOpeningHoursInterval {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessOpeningHoursIntervalBuilder {
    let inner = BusinessOpeningHoursInterval {
      td_name: "businessOpeningHoursInterval".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBusinessOpeningHoursIntervalBuilder { inner }
  }

  pub fn start_minute(&self) -> i64 {
    self.start_minute
  }

  pub fn end_minute(&self) -> i64 {
    self.end_minute
  }
}

#[doc(hidden)]
pub struct RTDBusinessOpeningHoursIntervalBuilder {
  inner: BusinessOpeningHoursInterval,
}

impl RTDBusinessOpeningHoursIntervalBuilder {
  pub fn build(&self) -> BusinessOpeningHoursInterval {
    self.inner.clone()
  }

  pub fn start_minute(&mut self, start_minute: i64) -> &mut Self {
    self.inner.start_minute = start_minute;
    self
  }

  pub fn end_minute(&mut self, end_minute: i64) -> &mut Self {
    self.inner.end_minute = end_minute;
    self
  }
}

impl AsRef<BusinessOpeningHoursInterval> for BusinessOpeningHoursInterval {
  fn as_ref(&self) -> &BusinessOpeningHoursInterval {
    self
  }
}

impl AsRef<BusinessOpeningHoursInterval> for RTDBusinessOpeningHoursIntervalBuilder {
  fn as_ref(&self) -> &BusinessOpeningHoursInterval {
    &self.inner
  }
}
