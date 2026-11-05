use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a user or a chat as a member of another chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMember {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat member. Currently, other chats can be only Left or Banned. Only supergroups and channels can have other chats as Left or Banned members and these chats must be supergroups or channels
  member_id: MessageSender,
  /// Tag of the chat member or its custom title if the member is an administrator of the chat; 0-16 characters without emoji; applicable to basic groups and supergroups only
  tag: String,
  /// Identifier of a user who invited/promoted/banned this member in the chat; 0 if unknown
  inviter_user_id: i64,
  /// Point in time (Unix timestamp) when the user joined/was promoted/was banned in the chat
  joined_chat_date: i64,
  /// Status of the member in the chat
  status: ChatMemberStatus,
}

impl RObject for ChatMember {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatMember"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatMember {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatMemberBuilder {
    let inner = ChatMember {
      td_name: "chatMember".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatMemberBuilder { inner }
  }

  pub fn member_id(&self) -> &MessageSender {
    &self.member_id
  }

  pub fn tag(&self) -> &String {
    &self.tag
  }

  pub fn inviter_user_id(&self) -> i64 {
    self.inviter_user_id
  }

  pub fn joined_chat_date(&self) -> i64 {
    self.joined_chat_date
  }

  pub fn status(&self) -> &ChatMemberStatus {
    &self.status
  }
}

#[doc(hidden)]
pub struct RTDChatMemberBuilder {
  inner: ChatMember,
}

impl RTDChatMemberBuilder {
  pub fn build(&self) -> ChatMember {
    self.inner.clone()
  }

  pub fn member_id<T: AsRef<MessageSender>>(&mut self, member_id: T) -> &mut Self {
    self.inner.member_id = member_id.as_ref().clone();
    self
  }

  pub fn tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.tag = tag.as_ref().to_string();
    self
  }

  pub fn inviter_user_id(&mut self, inviter_user_id: i64) -> &mut Self {
    self.inner.inviter_user_id = inviter_user_id;
    self
  }

  pub fn joined_chat_date(&mut self, joined_chat_date: i64) -> &mut Self {
    self.inner.joined_chat_date = joined_chat_date;
    self
  }

  pub fn status<T: AsRef<ChatMemberStatus>>(&mut self, status: T) -> &mut Self {
    self.inner.status = status.as_ref().clone();
    self
  }
}

impl AsRef<ChatMember> for ChatMember {
  fn as_ref(&self) -> &ChatMember {
    self
  }
}

impl AsRef<ChatMember> for RTDChatMemberBuilder {
  fn as_ref(&self) -> &ChatMember {
    &self.inner
  }
}
