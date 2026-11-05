use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a connection of the bot with a business account
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessConnection {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the connection
  id: String,
  /// Identifier of the business user who created the connection
  user_id: i64,
  /// Chat identifier of the private chat with the user
  user_chat_id: i64,
  /// Point in time (Unix timestamp) when the connection was established
  date: i64,
  /// Rights of the bot; may be null if the connection was disabled
  rights: Option<BusinessBotRights>,
  /// True, if the connection is enabled; false otherwise
  is_enabled: bool,
}

impl RObject for BusinessConnection {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessConnection"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BusinessConnection {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessConnectionBuilder {
    let inner = BusinessConnection {
      td_name: "businessConnection".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBusinessConnectionBuilder { inner }
  }

  pub fn id(&self) -> &String {
    &self.id
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn user_chat_id(&self) -> i64 {
    self.user_chat_id
  }

  pub fn date(&self) -> i64 {
    self.date
  }

  pub fn rights(&self) -> &Option<BusinessBotRights> {
    &self.rights
  }

  pub fn is_enabled(&self) -> bool {
    self.is_enabled
  }
}

#[doc(hidden)]
pub struct RTDBusinessConnectionBuilder {
  inner: BusinessConnection,
}

impl RTDBusinessConnectionBuilder {
  pub fn build(&self) -> BusinessConnection {
    self.inner.clone()
  }

  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn user_chat_id(&mut self, user_chat_id: i64) -> &mut Self {
    self.inner.user_chat_id = user_chat_id;
    self
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

  pub fn rights<T: AsRef<BusinessBotRights>>(&mut self, rights: T) -> &mut Self {
    self.inner.rights = Some(rights.as_ref().clone());
    self
  }

  pub fn is_enabled(&mut self, is_enabled: bool) -> &mut Self {
    self.inner.is_enabled = is_enabled;
    self
  }
}

impl AsRef<BusinessConnection> for BusinessConnection {
  fn as_ref(&self) -> &BusinessConnection {
    self
  }
}

impl AsRef<BusinessConnection> for RTDBusinessConnectionBuilder {
  fn as_ref(&self) -> &BusinessConnection {
    &self.inner
  }
}
