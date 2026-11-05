use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a chat folder invite link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatFolderInviteLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The chat folder invite link
  invite_link: String,
  /// Name of the link
  name: String,
  /// Identifiers of chats, included in the link
  chat_ids: Vec<i64>,
}

impl RObject for ChatFolderInviteLink {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatFolderInviteLink"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatFolderInviteLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatFolderInviteLinkBuilder {
    let inner = ChatFolderInviteLink {
      td_name: "chatFolderInviteLink".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatFolderInviteLinkBuilder { inner }
  }

  pub fn invite_link(&self) -> &String {
    &self.invite_link
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn chat_ids(&self) -> &Vec<i64> {
    &self.chat_ids
  }
}

#[doc(hidden)]
pub struct RTDChatFolderInviteLinkBuilder {
  inner: ChatFolderInviteLink,
}

impl RTDChatFolderInviteLinkBuilder {
  pub fn build(&self) -> ChatFolderInviteLink {
    self.inner.clone()
  }

  pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
    self.inner.invite_link = invite_link.as_ref().to_string();
    self
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

  pub fn chat_ids(&mut self, chat_ids: Vec<i64>) -> &mut Self {
    self.inner.chat_ids = chat_ids;
    self
  }
}

impl AsRef<ChatFolderInviteLink> for ChatFolderInviteLink {
  fn as_ref(&self) -> &ChatFolderInviteLink {
    self
  }
}

impl AsRef<ChatFolderInviteLink> for RTDChatFolderInviteLinkBuilder {
  fn as_ref(&self) -> &ChatFolderInviteLink {
    &self.inner
  }
}
