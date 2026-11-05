use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a business bot that manages the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessBotManageBar {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier of the bot
  bot_user_id: i64,
  /// URL to be opened to manage the bot
  manage_url: String,
  /// True, if the bot is paused. Use toggleBusinessConnectedBotChatIsPaused to change the value of the field
  is_bot_paused: bool,
  /// True, if the bot can reply
  can_bot_reply: bool,
}

impl RObject for BusinessBotManageBar {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessBotManageBar"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BusinessBotManageBar {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessBotManageBarBuilder {
    let inner = BusinessBotManageBar {
      td_name: "businessBotManageBar".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBusinessBotManageBarBuilder { inner }
  }

  pub fn bot_user_id(&self) -> i64 {
    self.bot_user_id
  }

  pub fn manage_url(&self) -> &String {
    &self.manage_url
  }

  pub fn is_bot_paused(&self) -> bool {
    self.is_bot_paused
  }

  pub fn can_bot_reply(&self) -> bool {
    self.can_bot_reply
  }
}

#[doc(hidden)]
pub struct RTDBusinessBotManageBarBuilder {
  inner: BusinessBotManageBar,
}

impl RTDBusinessBotManageBarBuilder {
  pub fn build(&self) -> BusinessBotManageBar {
    self.inner.clone()
  }

  pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
    self.inner.bot_user_id = bot_user_id;
    self
  }

  pub fn manage_url<T: AsRef<str>>(&mut self, manage_url: T) -> &mut Self {
    self.inner.manage_url = manage_url.as_ref().to_string();
    self
  }

  pub fn is_bot_paused(&mut self, is_bot_paused: bool) -> &mut Self {
    self.inner.is_bot_paused = is_bot_paused;
    self
  }

  pub fn can_bot_reply(&mut self, can_bot_reply: bool) -> &mut Self {
    self.inner.can_bot_reply = can_bot_reply;
    self
  }
}

impl AsRef<BusinessBotManageBar> for BusinessBotManageBar {
  fn as_ref(&self) -> &BusinessBotManageBar {
    self
  }
}

impl AsRef<BusinessBotManageBar> for RTDBusinessBotManageBarBuilder {
  fn as_ref(&self) -> &BusinessBotManageBar {
    &self.inner
  }
}
