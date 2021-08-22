
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains a chat invite link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatInviteLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat invite link
  invite_link: String,
  /// User identifier of an administrator created the link
  creator_user_id: i64,
  /// Point in time (Unix timestamp) when the link was created
  date: i64,
  /// Point in time (Unix timestamp) when the link was last edited; 0 if never or unknown
  edit_date: i64,
  /// Point in time (Unix timestamp) when the link will expire; 0 if never
  expire_date: i64,
  /// The maximum number of members, which can join the chat using the link simultaneously; 0 if not limited
  member_limit: i64,
  /// Number of chat members, which joined the chat using the link
  member_count: i64,
  /// True, if the link is primary. Primary invite link can't have expire date or usage limit. There is exactly one primary invite link for each administrator with can_invite_users right at a given time
  is_primary: bool,
  /// True, if the link was revoked
  is_revoked: bool,
  
}

impl RObject for ChatInviteLink {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatInviteLink" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatInviteLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatInviteLinkBuilder {
    let mut inner = ChatInviteLink::default();
    inner.td_name = "chatInviteLink".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatInviteLinkBuilder { inner }
  }

  pub fn invite_link(&self) -> &String { &self.invite_link }

  pub fn creator_user_id(&self) -> i64 { self.creator_user_id }

  pub fn date(&self) -> i64 { self.date }

  pub fn edit_date(&self) -> i64 { self.edit_date }

  pub fn expire_date(&self) -> i64 { self.expire_date }

  pub fn member_limit(&self) -> i64 { self.member_limit }

  pub fn member_count(&self) -> i64 { self.member_count }

  pub fn is_primary(&self) -> bool { self.is_primary }

  pub fn is_revoked(&self) -> bool { self.is_revoked }

}

#[doc(hidden)]
pub struct RTDChatInviteLinkBuilder {
  inner: ChatInviteLink
}

impl RTDChatInviteLinkBuilder {
  pub fn build(&self) -> ChatInviteLink { self.inner.clone() }

   
  pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
    self.inner.invite_link = invite_link.as_ref().to_string();
    self
  }

   
  pub fn creator_user_id(&mut self, creator_user_id: i64) -> &mut Self {
    self.inner.creator_user_id = creator_user_id;
    self
  }

   
  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

   
  pub fn edit_date(&mut self, edit_date: i64) -> &mut Self {
    self.inner.edit_date = edit_date;
    self
  }

   
  pub fn expire_date(&mut self, expire_date: i64) -> &mut Self {
    self.inner.expire_date = expire_date;
    self
  }

   
  pub fn member_limit(&mut self, member_limit: i64) -> &mut Self {
    self.inner.member_limit = member_limit;
    self
  }

   
  pub fn member_count(&mut self, member_count: i64) -> &mut Self {
    self.inner.member_count = member_count;
    self
  }

   
  pub fn is_primary(&mut self, is_primary: bool) -> &mut Self {
    self.inner.is_primary = is_primary;
    self
  }

   
  pub fn is_revoked(&mut self, is_revoked: bool) -> &mut Self {
    self.inner.is_revoked = is_revoked;
    self
  }

}

impl AsRef<ChatInviteLink> for ChatInviteLink {
  fn as_ref(&self) -> &ChatInviteLink { self }
}

impl AsRef<ChatInviteLink> for RTDChatInviteLinkBuilder {
  fn as_ref(&self) -> &ChatInviteLink { &self.inner }
}



