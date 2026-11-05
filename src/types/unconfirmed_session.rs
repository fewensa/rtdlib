use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about an unconfirmed session
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnconfirmedSession {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Session type
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: SessionType,
  /// Point in time (Unix timestamp) when the user has logged in or the business bot was connected
  date: i64,
  /// Model of the device that was used for the session creation, as provided by the application
  device_model: String,
  /// A human-readable description of the location from which the session was created, based on the IP address
  location: String,
}

impl RObject for UnconfirmedSession {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "unconfirmedSession"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UnconfirmedSession {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUnconfirmedSessionBuilder {
    let inner = UnconfirmedSession {
      td_name: "unconfirmedSession".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUnconfirmedSessionBuilder { inner }
  }

  pub fn type_(&self) -> &SessionType {
    &self.type_
  }

  pub fn date(&self) -> i64 {
    self.date
  }

  pub fn device_model(&self) -> &String {
    &self.device_model
  }

  pub fn location(&self) -> &String {
    &self.location
  }
}

#[doc(hidden)]
pub struct RTDUnconfirmedSessionBuilder {
  inner: UnconfirmedSession,
}

impl RTDUnconfirmedSessionBuilder {
  pub fn build(&self) -> UnconfirmedSession {
    self.inner.clone()
  }

  pub fn type_<T: AsRef<SessionType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

  pub fn device_model<T: AsRef<str>>(&mut self, device_model: T) -> &mut Self {
    self.inner.device_model = device_model.as_ref().to_string();
    self
  }

  pub fn location<T: AsRef<str>>(&mut self, location: T) -> &mut Self {
    self.inner.location = location.as_ref().to_string();
    self
  }
}

impl AsRef<UnconfirmedSession> for UnconfirmedSession {
  fn as_ref(&self) -> &UnconfirmedSession {
    self
  }
}

impl AsRef<UnconfirmedSession> for RTDUnconfirmedSessionBuilder {
  fn as_ref(&self) -> &UnconfirmedSession {
    &self.inner
  }
}
