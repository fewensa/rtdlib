use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains privacy settings for chats with non-contacts
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NewChatPrivacySettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if non-contacts users are able to write first to the current user. Telegram Premium subscribers are able to write first regardless of this setting
  allow_new_chats_from_unknown_users: bool,
  /// Number of Telegram Stars that must be paid for every incoming private message by non-contacts; 0-getOption("paid_message_star_count_max"). If positive, then allow_new_chats_from_unknown_users must be true. The current user will receive getOption("paid_message_earnings_per_mille") Telegram Stars for each 1000 Telegram Stars paid for message sending. Can be positive, only if getOption("can_enable_paid_messages") is true
  incoming_paid_message_star_count: i64,
}

impl RObject for NewChatPrivacySettings {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "newChatPrivacySettings"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl NewChatPrivacySettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDNewChatPrivacySettingsBuilder {
    let inner = NewChatPrivacySettings {
      td_name: "newChatPrivacySettings".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDNewChatPrivacySettingsBuilder { inner }
  }

  pub fn allow_new_chats_from_unknown_users(&self) -> bool {
    self.allow_new_chats_from_unknown_users
  }

  pub fn incoming_paid_message_star_count(&self) -> i64 {
    self.incoming_paid_message_star_count
  }
}

#[doc(hidden)]
pub struct RTDNewChatPrivacySettingsBuilder {
  inner: NewChatPrivacySettings,
}

impl RTDNewChatPrivacySettingsBuilder {
  pub fn build(&self) -> NewChatPrivacySettings {
    self.inner.clone()
  }

  pub fn allow_new_chats_from_unknown_users(
    &mut self,
    allow_new_chats_from_unknown_users: bool,
  ) -> &mut Self {
    self.inner.allow_new_chats_from_unknown_users = allow_new_chats_from_unknown_users;
    self
  }

  pub fn incoming_paid_message_star_count(
    &mut self,
    incoming_paid_message_star_count: i64,
  ) -> &mut Self {
    self.inner.incoming_paid_message_star_count = incoming_paid_message_star_count;
    self
  }
}

impl AsRef<NewChatPrivacySettings> for NewChatPrivacySettings {
  fn as_ref(&self) -> &NewChatPrivacySettings {
    self
  }
}

impl AsRef<NewChatPrivacySettings> for RTDNewChatPrivacySettingsBuilder {
  fn as_ref(&self) -> &NewChatPrivacySettings {
    &self.inner
  }
}
