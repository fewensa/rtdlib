use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a connection of a bot to an account
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessConnectedBotInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Information about the bot
  bot: BusinessConnectedBot,
  /// Point in time (Unix timestamp) when the bot was added; may be 0 if unknown
  connection_date: i64,
  /// Model of the device that was used for the bot connection, as provided by the application; may be empty if unknown
  device_model: String,
  /// A human-readable description of the location from which the bot was connected, based on the IP address; may be empty if unknown
  location: String,
}

impl RObject for BusinessConnectedBotInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessConnectedBotInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BusinessConnectedBotInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessConnectedBotInfoBuilder {
    let inner = BusinessConnectedBotInfo {
      td_name: "businessConnectedBotInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBusinessConnectedBotInfoBuilder { inner }
  }

  pub fn bot(&self) -> &BusinessConnectedBot {
    &self.bot
  }

  pub fn connection_date(&self) -> i64 {
    self.connection_date
  }

  pub fn device_model(&self) -> &String {
    &self.device_model
  }

  pub fn location(&self) -> &String {
    &self.location
  }
}

#[doc(hidden)]
pub struct RTDBusinessConnectedBotInfoBuilder {
  inner: BusinessConnectedBotInfo,
}

impl RTDBusinessConnectedBotInfoBuilder {
  pub fn build(&self) -> BusinessConnectedBotInfo {
    self.inner.clone()
  }

  pub fn bot<T: AsRef<BusinessConnectedBot>>(&mut self, bot: T) -> &mut Self {
    self.inner.bot = bot.as_ref().clone();
    self
  }

  pub fn connection_date(&mut self, connection_date: i64) -> &mut Self {
    self.inner.connection_date = connection_date;
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

impl AsRef<BusinessConnectedBotInfo> for BusinessConnectedBotInfo {
  fn as_ref(&self) -> &BusinessConnectedBotInfo {
    self
  }
}

impl AsRef<BusinessConnectedBotInfo> for RTDBusinessConnectedBotInfoBuilder {
  fn as_ref(&self) -> &BusinessConnectedBotInfo {
    &self.inner
  }
}
