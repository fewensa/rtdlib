use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes allowed types for the target chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TargetChatTypes {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if private chats with ordinary users are allowed
  allow_user_chats: bool,
  /// True, if private chats with other bots are allowed
  allow_bot_chats: bool,
  /// True, if basic group and supergroup chats are allowed
  allow_group_chats: bool,
  /// True, if channel chats are allowed
  allow_channel_chats: bool,
}

impl RObject for TargetChatTypes {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "targetChatTypes"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TargetChatTypes {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTargetChatTypesBuilder {
    let inner = TargetChatTypes {
      td_name: "targetChatTypes".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTargetChatTypesBuilder { inner }
  }

  pub fn allow_user_chats(&self) -> bool {
    self.allow_user_chats
  }

  pub fn allow_bot_chats(&self) -> bool {
    self.allow_bot_chats
  }

  pub fn allow_group_chats(&self) -> bool {
    self.allow_group_chats
  }

  pub fn allow_channel_chats(&self) -> bool {
    self.allow_channel_chats
  }
}

#[doc(hidden)]
pub struct RTDTargetChatTypesBuilder {
  inner: TargetChatTypes,
}

impl RTDTargetChatTypesBuilder {
  pub fn build(&self) -> TargetChatTypes {
    self.inner.clone()
  }

  pub fn allow_user_chats(&mut self, allow_user_chats: bool) -> &mut Self {
    self.inner.allow_user_chats = allow_user_chats;
    self
  }

  pub fn allow_bot_chats(&mut self, allow_bot_chats: bool) -> &mut Self {
    self.inner.allow_bot_chats = allow_bot_chats;
    self
  }

  pub fn allow_group_chats(&mut self, allow_group_chats: bool) -> &mut Self {
    self.inner.allow_group_chats = allow_group_chats;
    self
  }

  pub fn allow_channel_chats(&mut self, allow_channel_chats: bool) -> &mut Self {
    self.inner.allow_channel_chats = allow_channel_chats;
    self
  }
}

impl AsRef<TargetChatTypes> for TargetChatTypes {
  fn as_ref(&self) -> &TargetChatTypes {
    self
  }
}

impl AsRef<TargetChatTypes> for RTDTargetChatTypesBuilder {
  fn as_ref(&self) -> &TargetChatTypes {
    &self.inner
  }
}
