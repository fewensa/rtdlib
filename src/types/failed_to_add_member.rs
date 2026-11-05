use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a user who has failed to be added to a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FailedToAddMember {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier
  user_id: i64,
  /// True, if subscription to Telegram Premium would have allowed to add the user to the chat
  premium_would_allow_invite: bool,
  /// True, if subscription to Telegram Premium is required to send the user chat invite link
  premium_required_to_send_messages: bool,
}

impl RObject for FailedToAddMember {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "failedToAddMember"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl FailedToAddMember {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFailedToAddMemberBuilder {
    let inner = FailedToAddMember {
      td_name: "failedToAddMember".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDFailedToAddMemberBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn premium_would_allow_invite(&self) -> bool {
    self.premium_would_allow_invite
  }

  pub fn premium_required_to_send_messages(&self) -> bool {
    self.premium_required_to_send_messages
  }
}

#[doc(hidden)]
pub struct RTDFailedToAddMemberBuilder {
  inner: FailedToAddMember,
}

impl RTDFailedToAddMemberBuilder {
  pub fn build(&self) -> FailedToAddMember {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn premium_would_allow_invite(&mut self, premium_would_allow_invite: bool) -> &mut Self {
    self.inner.premium_would_allow_invite = premium_would_allow_invite;
    self
  }

  pub fn premium_required_to_send_messages(
    &mut self,
    premium_required_to_send_messages: bool,
  ) -> &mut Self {
    self.inner.premium_required_to_send_messages = premium_required_to_send_messages;
    self
  }
}

impl AsRef<FailedToAddMember> for FailedToAddMember {
  fn as_ref(&self) -> &FailedToAddMember {
    self
  }
}

impl AsRef<FailedToAddMember> for RTDFailedToAddMemberBuilder {
  fn as_ref(&self) -> &FailedToAddMember {
    &self.inner
  }
}
