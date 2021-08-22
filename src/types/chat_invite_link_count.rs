
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes a chat administrator with a number of active and revoked chat invite links
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatInviteLinkCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Administrator's user identifier
  user_id: i64,
  /// Number of active invite links
  invite_link_count: i64,
  /// Number of revoked invite links
  revoked_invite_link_count: i64,
  
}

impl RObject for ChatInviteLinkCount {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatInviteLinkCount" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatInviteLinkCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatInviteLinkCountBuilder {
    let mut inner = ChatInviteLinkCount::default();
    inner.td_name = "chatInviteLinkCount".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatInviteLinkCountBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn invite_link_count(&self) -> i64 { self.invite_link_count }

  pub fn revoked_invite_link_count(&self) -> i64 { self.revoked_invite_link_count }

}

#[doc(hidden)]
pub struct RTDChatInviteLinkCountBuilder {
  inner: ChatInviteLinkCount
}

impl RTDChatInviteLinkCountBuilder {
  pub fn build(&self) -> ChatInviteLinkCount { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn invite_link_count(&mut self, invite_link_count: i64) -> &mut Self {
    self.inner.invite_link_count = invite_link_count;
    self
  }

   
  pub fn revoked_invite_link_count(&mut self, revoked_invite_link_count: i64) -> &mut Self {
    self.inner.revoked_invite_link_count = revoked_invite_link_count;
    self
  }

}

impl AsRef<ChatInviteLinkCount> for ChatInviteLinkCount {
  fn as_ref(&self) -> &ChatInviteLinkCount { self }
}

impl AsRef<ChatInviteLinkCount> for RTDChatInviteLinkCountBuilder {
  fn as_ref(&self) -> &ChatInviteLinkCount { &self.inner }
}



