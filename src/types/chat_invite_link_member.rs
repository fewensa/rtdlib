
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes a chat member joined a chat by an invite link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatInviteLinkMember {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier
  user_id: i64,
  /// Point in time (Unix timestamp) when the user joined the chat
  joined_chat_date: i64,
  /// User identifier of the chat administrator, approved user join request
  approver_user_id: i64,
  
}

impl RObject for ChatInviteLinkMember {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatInviteLinkMember" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatInviteLinkMember {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatInviteLinkMemberBuilder {
    let mut inner = ChatInviteLinkMember::default();
    inner.td_name = "chatInviteLinkMember".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatInviteLinkMemberBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn joined_chat_date(&self) -> i64 { self.joined_chat_date }

  pub fn approver_user_id(&self) -> i64 { self.approver_user_id }

}

#[doc(hidden)]
pub struct RTDChatInviteLinkMemberBuilder {
  inner: ChatInviteLinkMember
}

impl RTDChatInviteLinkMemberBuilder {
  pub fn build(&self) -> ChatInviteLinkMember { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn joined_chat_date(&mut self, joined_chat_date: i64) -> &mut Self {
    self.inner.joined_chat_date = joined_chat_date;
    self
  }

   
  pub fn approver_user_id(&mut self, approver_user_id: i64) -> &mut Self {
    self.inner.approver_user_id = approver_user_id;
    self
  }

}

impl AsRef<ChatInviteLinkMember> for ChatInviteLinkMember {
  fn as_ref(&self) -> &ChatInviteLinkMember { self }
}

impl AsRef<ChatInviteLinkMember> for RTDChatInviteLinkMemberBuilder {
  fn as_ref(&self) -> &ChatInviteLinkMember { &self.inner }
}



