
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains a list of chat members joined a chat via an invite link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatInviteLinkMembers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Approximate total count of chat members found
  total_count: i64,
  /// List of chat members, joined a chat via an invite link
  members: Vec<ChatInviteLinkMember>,
  
}

impl RObject for ChatInviteLinkMembers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatInviteLinkMembers" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatInviteLinkMembers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatInviteLinkMembersBuilder {
    let mut inner = ChatInviteLinkMembers::default();
    inner.td_name = "chatInviteLinkMembers".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatInviteLinkMembersBuilder { inner }
  }

  pub fn total_count(&self) -> i64 { self.total_count }

  pub fn members(&self) -> &Vec<ChatInviteLinkMember> { &self.members }

}

#[doc(hidden)]
pub struct RTDChatInviteLinkMembersBuilder {
  inner: ChatInviteLinkMembers
}

impl RTDChatInviteLinkMembersBuilder {
  pub fn build(&self) -> ChatInviteLinkMembers { self.inner.clone() }

   
  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

   
  pub fn members(&mut self, members: Vec<ChatInviteLinkMember>) -> &mut Self {
    self.inner.members = members;
    self
  }

}

impl AsRef<ChatInviteLinkMembers> for ChatInviteLinkMembers {
  fn as_ref(&self) -> &ChatInviteLinkMembers { self }
}

impl AsRef<ChatInviteLinkMembers> for RTDChatInviteLinkMembersBuilder {
  fn as_ref(&self) -> &ChatInviteLinkMembers { &self.inner }
}



