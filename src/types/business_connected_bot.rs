use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a business bot connected to an account
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessConnectedBot {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier of the bot
  bot_user_id: i64,
  /// Private chats that will be accessible to the bot
  recipients: BusinessRecipients,
  /// Rights of the bot
  rights: BusinessBotRights,
}

impl RObject for BusinessConnectedBot {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessConnectedBot"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BusinessConnectedBot {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessConnectedBotBuilder {
    let inner = BusinessConnectedBot {
      td_name: "businessConnectedBot".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBusinessConnectedBotBuilder { inner }
  }

  pub fn bot_user_id(&self) -> i64 {
    self.bot_user_id
  }

  pub fn recipients(&self) -> &BusinessRecipients {
    &self.recipients
  }

  pub fn rights(&self) -> &BusinessBotRights {
    &self.rights
  }
}

#[doc(hidden)]
pub struct RTDBusinessConnectedBotBuilder {
  inner: BusinessConnectedBot,
}

impl RTDBusinessConnectedBotBuilder {
  pub fn build(&self) -> BusinessConnectedBot {
    self.inner.clone()
  }

  pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
    self.inner.bot_user_id = bot_user_id;
    self
  }

  pub fn recipients<T: AsRef<BusinessRecipients>>(&mut self, recipients: T) -> &mut Self {
    self.inner.recipients = recipients.as_ref().clone();
    self
  }

  pub fn rights<T: AsRef<BusinessBotRights>>(&mut self, rights: T) -> &mut Self {
    self.inner.rights = rights.as_ref().clone();
    self
  }
}

impl AsRef<BusinessConnectedBot> for BusinessConnectedBot {
  fn as_ref(&self) -> &BusinessConnectedBot {
    self
  }
}

impl AsRef<BusinessConnectedBot> for RTDBusinessConnectedBotBuilder {
  fn as_ref(&self) -> &BusinessConnectedBot {
    &self.inner
  }
}
