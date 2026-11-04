use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes opening hours of a business
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessOpeningHours {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique time zone identifier
  time_zone_id: String,
  /// Intervals of the time when the business is open
  opening_hours: Vec<BusinessOpeningHoursInterval>,
}

impl RObject for BusinessOpeningHours {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessOpeningHours"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BusinessOpeningHours {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessOpeningHoursBuilder {
    let inner = BusinessOpeningHours {
      td_name: "businessOpeningHours".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBusinessOpeningHoursBuilder { inner }
  }

  pub fn time_zone_id(&self) -> &String {
    &self.time_zone_id
  }

  pub fn opening_hours(&self) -> &Vec<BusinessOpeningHoursInterval> {
    &self.opening_hours
  }
}

#[doc(hidden)]
pub struct RTDBusinessOpeningHoursBuilder {
  inner: BusinessOpeningHours,
}

impl RTDBusinessOpeningHoursBuilder {
  pub fn build(&self) -> BusinessOpeningHours {
    self.inner.clone()
  }

  pub fn time_zone_id<T: AsRef<str>>(&mut self, time_zone_id: T) -> &mut Self {
    self.inner.time_zone_id = time_zone_id.as_ref().to_string();
    self
  }

  pub fn opening_hours(&mut self, opening_hours: Vec<BusinessOpeningHoursInterval>) -> &mut Self {
    self.inner.opening_hours = opening_hours;
    self
  }
}

impl AsRef<BusinessOpeningHours> for BusinessOpeningHours {
  fn as_ref(&self) -> &BusinessOpeningHours {
    self
  }
}

impl AsRef<BusinessOpeningHours> for RTDBusinessOpeningHoursBuilder {
  fn as_ref(&self) -> &BusinessOpeningHours {
    &self.inner
  }
}
