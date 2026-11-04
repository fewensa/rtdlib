use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A live location
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LiveLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The current location
  location: Location,
  /// Time relative to the message send date, for which the location can be updated, in seconds; if 0x7FFFFFFF, then location can be updated forever
  live_period: i64,
  /// The direction in which the location moves, in degrees; 1-360; 0 if unknown
  heading: i64,
  /// The maximum distance to another chat member for proximity alerts, in meters (0-100000). 0 if the notification is disabled. Can't be enabled in direct messages chats, channels and Saved Messages. Available only to the message sender
  proximity_alert_radius: i64,
}

impl RObject for LiveLocation {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "liveLocation"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl LiveLocation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLiveLocationBuilder {
    let inner = LiveLocation {
      td_name: "liveLocation".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLiveLocationBuilder { inner }
  }

  pub fn location(&self) -> &Location {
    &self.location
  }

  pub fn live_period(&self) -> i64 {
    self.live_period
  }

  pub fn heading(&self) -> i64 {
    self.heading
  }

  pub fn proximity_alert_radius(&self) -> i64 {
    self.proximity_alert_radius
  }
}

#[doc(hidden)]
pub struct RTDLiveLocationBuilder {
  inner: LiveLocation,
}

impl RTDLiveLocationBuilder {
  pub fn build(&self) -> LiveLocation {
    self.inner.clone()
  }

  pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
    self.inner.location = location.as_ref().clone();
    self
  }

  pub fn live_period(&mut self, live_period: i64) -> &mut Self {
    self.inner.live_period = live_period;
    self
  }

  pub fn heading(&mut self, heading: i64) -> &mut Self {
    self.inner.heading = heading;
    self
  }

  pub fn proximity_alert_radius(&mut self, proximity_alert_radius: i64) -> &mut Self {
    self.inner.proximity_alert_radius = proximity_alert_radius;
    self
  }
}

impl AsRef<LiveLocation> for LiveLocation {
  fn as_ref(&self) -> &LiveLocation {
    self
  }
}

impl AsRef<LiveLocation> for RTDLiveLocationBuilder {
  fn as_ref(&self) -> &LiveLocation {
    &self.inner
  }
}
