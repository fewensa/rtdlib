use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes users that have access to a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotAccessSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if access to the bot is restricted to its owner and selected users
  is_restricted: bool,
  /// Identifiers of the users who can use the bot additionally to the owner of the bot
  added_user_ids: Vec<i64>,
}

impl RObject for BotAccessSettings {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "botAccessSettings"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BotAccessSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBotAccessSettingsBuilder {
    let inner = BotAccessSettings {
      td_name: "botAccessSettings".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBotAccessSettingsBuilder { inner }
  }

  pub fn is_restricted(&self) -> bool {
    self.is_restricted
  }

  pub fn added_user_ids(&self) -> &Vec<i64> {
    &self.added_user_ids
  }
}

#[doc(hidden)]
pub struct RTDBotAccessSettingsBuilder {
  inner: BotAccessSettings,
}

impl RTDBotAccessSettingsBuilder {
  pub fn build(&self) -> BotAccessSettings {
    self.inner.clone()
  }

  pub fn is_restricted(&mut self, is_restricted: bool) -> &mut Self {
    self.inner.is_restricted = is_restricted;
    self
  }

  pub fn added_user_ids(&mut self, added_user_ids: Vec<i64>) -> &mut Self {
    self.inner.added_user_ids = added_user_ids;
    self
  }
}

impl AsRef<BotAccessSettings> for BotAccessSettings {
  fn as_ref(&self) -> &BotAccessSettings {
    self
  }
}

impl AsRef<BotAccessSettings> for RTDBotAccessSettingsBuilder {
  fn as_ref(&self) -> &BotAccessSettings {
    &self.inner
  }
}
